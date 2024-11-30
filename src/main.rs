#![allow(dead_code)]

use coord::*;
use notan::prelude::*;
use notan::draw::*;

mod proof;
mod coord;


#[derive(AppState)]
struct State {
    text_font: notan::glyph::ab_glyph::FontVec,
    symbol_font: notan::glyph::ab_glyph::FontVec,
    text_calculator: notan::text::Calculator,
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
    /*
    let font = gfx
        .create_font(include_bytes!("../assets/fonts/cmunrm.ttf"))
        .unwrap();
    */

    let font_data = include_bytes!("../assets/fonts/cmunrm.ttf").to_vec();
    let font = notan::glyph::ab_glyph::FontVec::try_from_vec(font_data).expect("Unable to read font!");

    /*
    let symbol_font = gfx
        .create_font(include_bytes!("../assets/fonts/JuliaMono.ttf"))
        .unwrap();
    */

    let symbol_font_data = include_bytes!("../assets/fonts/JuliaMono.ttf").to_vec();
    let symbol_font = notan::glyph::ab_glyph::FontVec::try_from_vec(symbol_font_data).expect("Unable to read font!");

    State {
        text_font: font,
        symbol_font, 
        text_calculator: notan::text::Calculator::new(),
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
    
    let mut test_proof = proof::Proof {
        root: proof::Sequent {
            before: vec![],
            after: vec![
                proof::Formula::Operator(proof::Operator { 
                    operator_type: proof::OperatorType::Top,
                    arg1: None,
                    arg2: None
                })
            ],
            cached_text_section: None,
        },
        branches: vec![],
        rule: Box::new(proof::NoRule {}),
    };

    proof::rendering::compute_sequent_text_section(&mut test_proof.root, state);
    
    proof::rendering::draw_proof(&test_proof, ScreenPosition::new(0.0, 0.0), gfx, &mut draw, state);

    gfx.render(&draw);
}

fn test_sine(app: &App, phase: f32) -> f32 {
    return f32::sin(app.timer.elapsed().as_secs_f32() * 5.0 + phase * 2.0 * std::f32::consts::PI) * 0.5 + 0.5;
}

