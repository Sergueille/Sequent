#![allow(dead_code)]

use std::collections::HashMap;

use proof::*;
use calcul::*;
use coord::*;
use notan::prelude::*;
use notan::draw::*;
use rendering::draw_proof;
use rendering::get_proof_width;

mod proof;
mod coord;

/// Current global state of the game.
enum GameMode<'a> {
    Ingame(GameState<'a>),
}


struct GameState<'a> {
    proof: Proof<'a>,
    editing_formulas: bool,

    /// ID of the current focused formula field
    formulas_position: u32,
}


#[derive(AppState)]
struct State<'a> {
    text_font: Font,
    symbol_font: Font,
    cached_sizes: HashMap<char, f32>,
    rules: Vec<Box<dyn Rule>>,
    mode: GameMode<'a>
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

fn setup<'a>(gfx: &mut Graphics) -> State<'a> {
    let font = gfx
        .create_font(include_bytes!("../assets/fonts/cmunrm.ttf"))
        .unwrap();

    let symbol_font = gfx
        .create_font(include_bytes!("../assets/fonts/JuliaMono.ttf"))
        .unwrap();

    let rules: Vec<Box<dyn Rule>> = vec![];

    let test_proof = Proof {
        root: Sequent {
            before: vec![],
            after: vec![
                Formula::NotCompleted(0),
            ],
            cached_text_section: None,
        },
        branches: vec![],
        rule: None,
    };

    return State {
        text_font: font,
        symbol_font, 
        cached_sizes: proof::rendering::compute_char_sizes(&font, &symbol_font),
        rules,
        mode: GameMode::Ingame(GameState {
            proof: test_proof,
            editing_formulas: true,
            formulas_position: 0
        })
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
                focused_formula_field: game_state.formulas_position
            };

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
