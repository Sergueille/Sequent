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
        
    let test_proof = Proof {
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
        rule: Box::new(NoRule {}),
    };

    let w = rendering::get_sequent_width(&test_proof.root, state, gfx);

    rendering::draw_sequent(
        &test_proof.root, 
        ScreenPosition { x: -w * 0.5, y: test_sine(app, 0.0) * 0.1 }, 
        gfx, 
        &mut draw, 
        state
    );

    gfx.render(&draw);
}

fn test_sine(app: &App, phase: f32) -> f32 {
    return f32::sin(app.timer.elapsed().as_secs_f32() * 5.0 + phase * 2.0 * std::f32::consts::PI) * 0.5 + 0.5;
}

