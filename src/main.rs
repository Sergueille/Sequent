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
    sequent_position: ScreenSize,
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

/// Part of the screen, next to left and right borders, where focused element shouldn't be (screen space) 
pub const SEQUENT_SAFE_ZONE_SIDES: f32 = 0.3;
/// Part of the screen, next to the top, where focused element shouldn't be (screen space) 
pub const SEQUENT_SAFE_ZONE_TOP: f32 = 0.5;
pub const CAMERA_MOVEMENT_SPEED: f32 = 5.0;


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
        .add_plugin(notan::extra::FpsLimit::new(100))
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
        },
        0.0
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
            },
            sequent_position: ScreenSize::zero(),
        }),
        bindings: action::get_default_bindings(),
    };
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    // draw.clear(Color::BLACK);

    let time_seconds = app.timer.elapsed().as_secs_f32();

    // Draw FPS
    draw.text(&state.text_font, &format!("{}ms / {}FPS", ((1.0 / app.timer.fps()) * 1000.0).round(), app.timer.fps().round()))
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

    let (w, h) = gfx.size();
    let screen_ratio = w as f32 / h as f32;

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

            let special_mode = action::is_down(action::Action::SpecialRuleMode, &state.bindings, &app);

            game_ui::render_ui(special_mode, &state.bindings, &state.symbol_font, &mut draw, &gfx, &game_state);

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
                        for i in 0..game_state.logic_system.rules.len() {
                            if action::was_pressed(action::Action::InsertRule(i as u32), &state.bindings, app) {

                                let rule = if special_mode {
                                    match &game_state.logic_system.special_rules[i] {
                                        Some(rule) => rule,
                                        None => &game_state.logic_system.rules[i],
                                    }
                                }
                                else {
                                    &game_state.logic_system.rules[i]
                                };

                                let (branches, field_count) = rule.as_ref().create_branches(&current_proof.root);
                                match branches {
                                    Some(new_branches) => {
                                        current_proof.branches = new_branches.into_iter().map(|s|
                                            proof::sequent_as_empty_proof(s, app.timer.elapsed_f32())
                                        ).collect();

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
                focus_rect: ScreenRect::nothing()
            };

            let proof_width = get_proof_width(&game_state.state.proof, &mut render_info);
            let base_position = ScreenPosition {
                x: -proof_width * 0.5,
                y: -0.7,
            };

            draw_proof(&game_state.state.proof, base_position.add(game_state.sequent_position), &mut render_info);

            // If larger than screen move to center focused element, otherwise center the sequent
            let current_x_shift = if render_info.focus_rect == ScreenRect::nothing() {
                game_state.sequent_position.x
            }
            else if proof_width > (screen_ratio - SEQUENT_SAFE_ZONE_SIDES) * 2.0 {
                render_info.focus_rect.center().x
            }
            else {
                game_state.sequent_position.x
            };

            /* Other possible behavior: if the focused element is near borders or outside screen, move sequent

            let safe_left = SEQUENT_SAFE_ZONE_SIDES - screen_ratio;
            let safe_right = - SEQUENT_SAFE_ZONE_SIDES + screen_ratio;

            let overflow_left = render_info.focus_rect.bottom_left.x < safe_left;
            let overflow_right = render_info.focus_rect.top_right.x > safe_right;

            let current_x_shift = if render_info.focus_rect == ScreenRect::nothing() {
                0.0
            }
            else if overflow_left && overflow_right {
                render_info.focus_rect.center().x
            }
            else if overflow_left {
                render_info.focus_rect.bottom_left.x - safe_left
            }
            else if overflow_right {
                render_info.focus_rect.top_right.x - safe_right
            }
            else {
                0.0
            };
            */

            let current_y_shift = if render_info.focus_rect == ScreenRect::nothing() {
                game_state.sequent_position.y
            }
            else if -game_state.sequent_position.y + render_info.focus_rect.top_right.y > 1.0 - SEQUENT_SAFE_ZONE_TOP {
                render_info.focus_rect.top_right.y - 1.0 + SEQUENT_SAFE_ZONE_TOP
            }
            else {
                game_state.sequent_position.y
            };

            game_state.sequent_position.x -= CAMERA_MOVEMENT_SPEED * current_x_shift * app.timer.delta_f32();
            game_state.sequent_position.y -= CAMERA_MOVEMENT_SPEED * current_y_shift * app.timer.delta_f32();
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
    };
    let test = proof_or_fake(seq);

    println!("seq = {}", test);
}
