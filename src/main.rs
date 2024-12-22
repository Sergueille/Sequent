#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::Rc;

use proof::*;
use calcul::*;
use coord::*;
use notan::prelude::*;
use notan::draw::*;
use rendering::draw_proof;
use rendering::get_proof_width;

mod proof;
mod coord;
mod action;

/// Current global state of the game.
enum GameMode {
    Ingame(GameState),
}


struct GameState {
    logic_system: LogicSystem,
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
        .build();
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("../assets/fonts/cmunrm.ttf"))
        .unwrap();

    let symbol_font = gfx
        .create_font(include_bytes!("../assets/fonts/JuliaMono.ttf"))
        .unwrap();

    let test_proof = Proof {
        root: Sequent {
            before: vec![],
            after: vec![
                Formula::NotCompleted(FormulaField {
                    id: 0,
                    prev_id: 0,
                    next_id: 0,
                }),
            ],
            cached_text_section: None,
        },
        branches: vec![],
        rule_id: None,
    };

    return State {
        text_font: font,
        symbol_font, 
        cached_sizes: proof::rendering::compute_char_sizes(&font, &symbol_font),
        mode: GameMode::Ingame(GameState {
            logic_system: proof::natural_logic::get_system(),
            proof: test_proof,
            editing_formulas: true,
            formulas_position: 0,
            next_formula_index: 1
        }),
        bindings: action::get_default_bindings(),
    };
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Draw FPS
    draw.text(&state.text_font, &format!("{}ms / {}FPS", app.timer.delta().as_millis(), app.timer.fps().round()))
        .position(2.0, 2.0)
        .size(20.0)
        .v_align_top()
        .h_align_left();

    match &mut state.mode {
        GameMode::Ingame(game_state) => {
            let mut render_info = proof::rendering::RenderInfo {
                draw: &mut draw,
                gfx,
                text_font: &state.text_font,
                symbol_font: &state.symbol_font,
                cached_sizes: &state.cached_sizes,
                focused_formula_field: game_state.formulas_position,
                editing_formulas: game_state.editing_formulas
            };

            if game_state.editing_formulas {
                // Check for operator insertion
                for (i, op) in game_state.logic_system.operators.iter().enumerate() {
                    if action::was_pressed(action::Action::InsertOperator(i as u32), &state.bindings, app) {
                        match proof::place_uncompleted_operator(*op, game_state.formulas_position, &mut game_state.proof, &mut game_state.next_formula_index) {
                            Some(new_field) => game_state.formulas_position = new_field,
                            None => game_state.editing_formulas = false,
                        }
                        
                        break;
                    } 
                }

                // Check for variable insertion
                for i in 0..MAX_VARIABLE_COUNT {
                    if action::was_pressed(action::Action::InsertVariable(i as u32), &state.bindings, app) {
                        match proof::place_variable(i, game_state.formulas_position, &mut game_state.proof) {
                            Some(new_field) => game_state.formulas_position = new_field,
                            None => game_state.editing_formulas = false,
                        }
                        
                        break;     
                    } 
                }

                // Previous and next fields
                if action::was_pressed(action::Action::NextField, &state.bindings, app) {
                    game_state.formulas_position = proof::formula_as_field(
                        proof::search_field_id_in_proof(&mut game_state.proof, Some(game_state.formulas_position)).unwrap()
                    ).next_id;
                }
                if action::was_pressed(action::Action::PreviousField, &state.bindings, app) {
                    game_state.formulas_position = proof::formula_as_field(
                        proof::search_field_id_in_proof(&mut game_state.proof, Some(game_state.formulas_position)).unwrap()
                    ).prev_id;
                }
            }
            else { // Not editing formulas
                match proof::get_first_unfinished_proof(&mut game_state.proof) {
                    Some(p) => {
                        let current_proof = p;

                        // Check for rules insertion
                        for (i, rule) in game_state.logic_system.rules.iter().enumerate() {
                            if action::was_pressed(action::Action::InsertRule(i as u32), &state.bindings, app) {
        
                                match rule.as_ref().create_branches(&current_proof.root, &mut game_state.formulas_position) {
                                    Some(new_branches) => {
                                        current_proof.branches = new_branches; 
                                        current_proof.rule_id = Some(i as u32);
                                    },
                                    None => {
                                        // TODO: feedback
                                    },
                                }
        
                                break;
                            } 
                        }
                    },
                    None => (), // TODO: show proof is finished
                }
            }

            let proof_width = get_proof_width(&game_state.proof, &mut render_info);
            let position = ScreenPosition {
                x: -proof_width * 0.5,
                y: -0.7,
            };

            draw_proof(&game_state.proof, position, &mut render_info);
        }
    }

    gfx.render(&draw);
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
