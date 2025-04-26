#![allow(dead_code)]

use std::collections::HashMap;

use proof::*;
use calcul::*;
use coord::*;
use notan::prelude::*;
use notan::draw::*;
use notan::egui::{self, *};
use rendering::draw_proof;
use rendering::get_proof_width;

mod proof;
mod coord;
mod action;
mod game_ui;

/// Current global state of the game.
enum GameMode {
    Ingame(GameState),
}


struct GameState {
    logic_system: LogicSystem,
    state: UndoState,
    undo_stack: Vec<UndoState>,
    redo_stack: Vec<UndoState>,
}

#[derive(Clone)]
struct UndoState {
    proof: Proof,
    editing_formulas: bool,

    /// ID of the current focused formula field
    formulas_position: u32,

    /// Next id that will be assigned to empty fields, to make sure they are unique. This means all fields in proof will have an id below this .
    next_formula_index: u32,
}


#[derive(AppState)]
struct State {
    text_font: Font,
    symbol_font: Font,
    cached_sizes: HashMap<char, f32>,
    mode: GameMode,
    bindings: HashMap<action::Action, KeyCode>,
}


#[notan_main]
fn main() -> Result<(), String> {
    calculation_test();

    // Get backtraces
    std::env::set_var("RUST_BACKTRACE", "1");

    let window_config = WindowConfig::new()
        .set_title("Hello worlds")
        .set_fullscreen(true)
        .set_maximized(true)
        .set_resizable(true)
        .set_vsync(false);

    return notan::init_with(setup)
        .draw(draw)
        .add_config(window_config)
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .build();
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("../assets/fonts/cmunrm.ttf"))
        .unwrap();

    let symbol_font = gfx
        .create_font(include_bytes!("../assets/fonts/JuliaMono.ttf"))
        .unwrap();

    let test_proof = sequent_as_empty_proof(
        Sequent {
            before: vec![],
            after: vec![
                Formula::NotCompleted(FormulaField {
                    id: 0,
                    prev_id: 0,
                    next_id: 0,
                }),
            ],
            cached_text_section: None,
        }
    );

    return State {
        text_font: font,
        symbol_font, 
        cached_sizes: proof::rendering::compute_char_sizes(&font, &symbol_font),
        mode: GameMode::Ingame(GameState {
            logic_system: proof::natural_logic::get_system(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            state: UndoState {
                proof: test_proof,
                editing_formulas: true,
                formulas_position: 0,
                next_formula_index: 1,
            }
        }),
        bindings: action::get_default_bindings(),
    };
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    // draw.clear(Color::BLACK);

    let time_seconds = app.timer.elapsed().as_secs_f32();

    // Draw FPS
    draw.text(&state.text_font, &format!("{}ms / {}FPS", app.timer.delta().as_millis(), app.timer.fps().round()))
        .position(2.0, 2.0)
        .size(20.0)
        .v_align_top()
        .h_align_left();

    // EGUI can be used here, maybe later for forms or menus
    let mut ui_output = plugins.egui(|ctx| {
        let frame = egui::containers::Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |_ui| {
            // UI code here
        });
    });
    
    ui_output.clear_color(Color::from_hex(0x000000ff));

    match &mut state.mode {
        GameMode::Ingame(game_state) => {

            // Handle undo/redo
            if action::was_pressed(action::Action::Undo, &state.bindings, &app) {
                if !undo(game_state) {
                    // TODO: undo failed; feedback
                }
            }
            else if action::was_pressed(action::Action::Redo, &state.bindings, &app) {
                if !redo(game_state) {
                    // TODO: redo failed; feedback
                }
            }

            game_ui::render_ui(&state.bindings, &state.symbol_font, &mut draw, &gfx, &game_state);

            if game_state.state.editing_formulas {
                // Check for operator insertion
                for (i, op) in game_state.logic_system.operators.clone().into_iter().enumerate() {
                    if action::was_pressed(action::Action::InsertOperator(i as u32), &state.bindings, app) {
                        
                        record_undo_entry(game_state);
                        match proof::place_uncompleted_operator(op, game_state.state.formulas_position, &mut game_state.state.proof, &mut game_state.state.next_formula_index) {
                            Some(new_field) => game_state.state.formulas_position = new_field,
                            None => game_state.state.editing_formulas = false,
                        }
                        
                        break;
                    } 
                }

                // Check for variable insertion
                for i in 0..MAX_VARIABLE_COUNT {
                    if action::was_pressed(action::Action::InsertVariable(i as u32), &state.bindings, app) {
                        
                        record_undo_entry(game_state);
                        match proof::place_variable(i, game_state.state.formulas_position, &mut game_state.state.proof) {
                            Some(new_field) => game_state.state.formulas_position = new_field,
                            None => game_state.state.editing_formulas = false,
                        }
                        
                        break;     
                    } 
                }

                // Previous and next fields
                if action::was_pressed(action::Action::NextField, &state.bindings, app) {
                    game_state.state.formulas_position = proof::formula_as_field(
                        proof::search_fields_by_id_in_proof(&mut game_state.state.proof, Some(game_state.state.formulas_position))[0]
                    ).next_id;
                }
                if action::was_pressed(action::Action::PreviousField, &state.bindings, app) {
                    game_state.state.formulas_position = proof::formula_as_field(
                        proof::search_fields_by_id_in_proof(&mut game_state.state.proof, Some(game_state.state.formulas_position))[0]
                    ).prev_id;
                }
            }
            else { // Not editing formulas

                let undo_entry = game_state.state.clone(); 

                match proof::get_first_unfinished_proof(&mut game_state.state.proof) {
                    Some(current_proof) => {
                        current_proof.last_focused_time = time_seconds;

                        // Check for rules insertion
                        for (i, rule) in game_state.logic_system.rules.iter().enumerate() {
                            if action::was_pressed(action::Action::InsertRule(i as u32), &state.bindings, app) {

                                let (branches, field_count) = rule.as_ref().create_branches(&current_proof.root);
                                match branches {
                                    Some(new_branches) => {
                                        current_proof.branches = new_branches.into_iter().map(proof::sequent_as_empty_proof).collect(); 
                                        current_proof.rule_id = Some(i as u32);

                                        add_undo_entry(undo_entry, game_state);
                                    },
                                    None => {
                                        // TODO: feedback
                                    },
                                } 

                                if field_count > 0 {
                                    game_state.state.next_formula_index = field_count;
                                    game_state.state.formulas_position = 0;

                                    game_state.state.editing_formulas = true;
                                }
        
                                break;
                            } 
                        }
                    },
                    None => (), // TODO: show proof is finished
                }
            }

            // Draw the proof

            let mut render_info = proof::rendering::RenderInfo {
                draw: &mut draw,
                gfx,
                text_font: &state.text_font,
                symbol_font: &state.symbol_font,
                cached_sizes: &state.cached_sizes,
                focused_formula_field: game_state.state.formulas_position,
                editing_formulas: game_state.state.editing_formulas,
                logic_system: &game_state.logic_system,
                time: time_seconds,
            };

            let proof_width = get_proof_width(&game_state.state.proof, &mut render_info);
            let position = ScreenPosition {
                x: -proof_width * 0.5,
                y: -0.7,
            };

            draw_proof(&game_state.state.proof, position, &mut render_info);
        }
    }

    gfx.render(&ui_output);
    gfx.render(&draw);
}


fn record_undo_entry(gs: &mut GameState) {
    add_undo_entry(gs.state.clone(), gs);
}

fn add_undo_entry(entry: UndoState, gs: &mut GameState) {
    gs.undo_stack.push(entry);
    gs.redo_stack = Vec::new();
}

fn undo(gs: &mut GameState) -> bool {
    match gs.undo_stack.pop() {
        Some(mut last_state) => {
            std::mem::swap(&mut gs.state, &mut last_state);
            gs.redo_stack.push(last_state); 
            return true;
        },
        None => return false
    }
}

fn redo(gs: &mut GameState) -> bool {
    match gs.redo_stack.pop() {
        Some(mut last_state) => {
            std::mem::swap(&mut gs.state, &mut last_state);
            gs.undo_stack.push(last_state); 
            return true;
        },
        None => return false
    }
}


fn calculation_test() {
    let seq = Sequent {
        before: vec![],
        after: vec![
            Formula::Operator(Operator {
                operator_type: OperatorType::Or,
                arg1: Some(Box::new(Formula::Operator(Operator { 
                    operator_type: OperatorType::Not, 
                    arg1: Some(Box::new(Formula::Variable(1))), arg2: None }))),
                    arg2: Some(Box::new(Formula::Variable(1))),
            })
        ],
        cached_text_section: None,
    };
    let test = proof_or_fake(seq);

    println!("seq = {}", test);
}
