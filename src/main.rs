#![allow(dead_code)]

use notan::prelude::*;
use notan::draw::*;

mod proofs;
mod coord;


#[derive(AppState)]
struct State {
    text_font: Font,
    symbol_font: Font,
    text_calculator: notan::text::Calculator,
}

#[notan_main]
fn main() -> Result<(), String> {
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

    draw.text(&state.text_font, "J'aime dériver des sequents")
        .position(viewport_x / 2.0, viewport_y / 2.0)
        .size(50.0)
        .color(color)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}

fn test_sine(app: &App, phase: f32) -> f32 {
    return f32::sin(app.timer.elapsed().as_secs_f32() * 5.0 + phase * 2.0 * std::f32::consts::PI) * 0.5 + 0.5;
}

