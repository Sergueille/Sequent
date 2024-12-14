#![allow(dead_code)]

use std::collections::HashMap;

use proof::*;
use coord::*;
use notan::prelude::*;
use notan::draw::*;

mod proof;
mod coord;


#[derive(AppState)]
struct State {
    text_font: Font,
    symbol_font: Font,
    cached_sizes: HashMap<char, f32>,
}

#[notan_main]
fn main() -> Result<(), String> {
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

    State {
        text_font: font,
        symbol_font, 
        cached_sizes: proof::rendering::compute_char_sizes(&font, &symbol_font),
    }
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

    let no_rule = NoRule {};
        
    let mut test_proof = Proof {
        root: Sequent {
            before: vec![
                Formula::Operator(Operator { 
                    operator_type: OperatorType::Not,
                    arg1: Some(Box::new(
                        Formula::Variable(3)
                    )),
                    arg2: None
                }),
                Formula::Variable(0),
            ],
            after: vec![
                Formula::Operator(Operator { 
                    operator_type: OperatorType::Not,
                    arg1: Some(Box::new(
                        Formula::Operator(Operator { 
                            operator_type: OperatorType::And,
                            arg1: Some(Box::new(Formula::Operator(Operator { operator_type: OperatorType::Top, arg1: None, arg2: None }))),
                            arg2: Some(Box::new(Formula::Operator(Operator { 
                                operator_type: OperatorType::Not, 
                                arg1: Some(Box::new(Formula::Variable(1))),
                                arg2: None 
                            })))
                        })
                    )),
                    arg2: None
                }),
                Formula::Variable(0),
            ],
            cached_text_section: None,
        },
        branches: vec![],
        rule: &no_rule,
    };

    let copy_1 = test_proof.clone();
    let copy_2 = test_proof.clone();
    test_proof.branches = vec![copy_1, copy_2];

    let w = rendering::get_proof_width(&test_proof, state, gfx);

    rendering::draw_proof(
        &test_proof, 
        ScreenPosition { x: -w * 0.5, y: -0.8 }, 
        gfx, 
        &mut draw, 
        state
    );

    gfx.render(&draw);
}


