#![allow(dead_code)]

use std::char::ToUppercase;
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
    let (viewport_x, viewport_y) = draw.size();
    draw.clear(Color::BLACK);

    let color = Color::from_rgb(
        test_sine(app, 0.0),
        test_sine(app, 1.0 / 3.0),
        test_sine(app, 2.0 / 3.0),
    );

    /*
    draw.text(&state.text_font, "J'aime dériver des sequents")
        .position(viewport_x / 2.0, viewport_y / 2.0)
        .size(50.0)
        .color(color)
        .h_align_center()
        .v_align_middle();
    */
    
    let mut test_proof = Proof {
        root: Sequent {
            before: vec![],
            after: vec![
                Formula::Operator(Operator { 
                    operator_type: OperatorType::And,
                    arg1: Some(Box::new(Formula::Operator(Operator { operator_type: OperatorType::Top, arg1: None, arg2: None }))),
                    arg2: Some(Box::new(Formula::Operator(Operator { 
                        operator_type: OperatorType::Not, 
                        arg1: Some(Box::new(Formula::Variable(1))),
                        arg2: None 
                    })))
                })
            ],
            cached_text_section: None,
        },
        branches: vec![],
        rule: Box::new(NoRule {}),
    };

    println!("{}", proof::rendering::get_proof_width(&test_proof, &state, &gfx));

    rendering::draw_formula(
        &test_proof.root.after[0], 
        ScreenPosition { x: 0.0, y: 0.0 }, 
        gfx, 
        &mut draw, 
        state
    );

    gfx.render(&draw);
}

fn test_sine(app: &App, phase: f32) -> f32 {
    return f32::sin(app.timer.elapsed().as_secs_f32() * 5.0 + phase * 2.0 * std::f32::consts::PI) * 0.5 + 0.5;
}

