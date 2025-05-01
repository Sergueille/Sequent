#![allow(dead_code)]
#![deny(clippy::disallowed_methods)]

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
mod background;
mod animation;
mod ingame;

/// Current global state of the game.
#[allow(clippy::large_enum_variant)]
enum GameMode {
    Ingame(GameState),
    Other,
}


struct GameState {
    logic_system: LogicSystem,
    state: UndoState,
    undo_stack: Vec<UndoState>,
    redo_stack: Vec<UndoState>,
    sequent_position: ScreenSize,
    sequent_scale: f32,
    keys_visibility: bool,
    last_shake_time: f32,
}

#[derive(Clone)]
struct UndoState {
    proof: Proof,
    editing_formulas: bool,

    /// ID of the current focused formula field
    formulas_position: u32,

    /// Next id that will be assigned to empty fields, to make sure they are unique. This means all fields in proof will have an id below this .
    next_formula_index: u32,

    /// Creation times of the fields in the sequent, indexed by their id
    fields_creation_time: HashMap<u32, f32>,
}


#[derive(AppState)]
struct State {
    text_font: Font,
    symbol_font: Font,
    cached_sizes: HashMap<char, f32>,
    mode: GameMode,
    bindings: HashMap<action::Action, KeyCode>,
    screen_ratio: f32,
    theme: Theme,
    background_state: background::BackgroundState,
}

#[derive(Clone, Copy)]
struct Theme {
    ui_text: Color,
    ui_bg: Color,
    bg_text: Color,
    bg: Color,
    seq_text: Color,
    seq_bar: Color,
    seq_bar_focused: Color,
    seq_field: Color,
    seq_field_focused: Color,
} 


/// Part of the screen, next to left and right borders, where focused element shouldn't be (screen space) 
pub const SEQUENT_SAFE_ZONE_SIDES: f32 = 0.3;
/// Part of the screen, next to the top, where focused element shouldn't be (screen space) 
pub const SEQUENT_SAFE_ZONE_TOP: f32 = 0.5;
pub const CAMERA_MOVEMENT_SPEED_X: f32 = 5.0;
pub const CAMERA_MOVEMENT_SPEED_Y: f32 = 6.0;
pub const CAMERA_MOVEMENT_SPEED_SCALE: f32 = 7.0;


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

    let test_theme = Theme {
        ui_text: Color::from_hex(0xeeeeeeff),
        ui_bg: Color::from_hex(0x222222ff),
        bg_text: Color::from_hex(0x151515ff),
        bg: Color::from_hex(0x080808ff),
        seq_text: Color::from_hex(0xeeeeeeff),
        seq_bar: Color::from_hex(0xeeeeeeff),
        seq_bar_focused: Color::from_hex(0xffccaaff),
        seq_field: Color::from_hex(0x30308050),
        seq_field_focused: Color::from_hex(0xffffdd50),
    };

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
                fields_creation_time: HashMap::with_capacity(20),
            },
            sequent_position: ScreenSize::zero(),
            sequent_scale: 1.0,
            keys_visibility: true,
            last_shake_time: f32::NEG_INFINITY
        }),
        bindings: action::get_default_bindings(),
        screen_ratio: 1.0,
        background_state: background::init_background_state(),
        theme: test_theme,
    };
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    // draw.clear(Color::BLACK);

    // Draw FPS
    {
        let fps_text = format!("{}ms / {}FPS", ((1.0 / app.timer.fps()) * 1000.0).round(), app.timer.fps().round());
        let mut txt = draw.text(&state.text_font, &fps_text);
        txt.position(2.0, 2.0)
            .v_align_top()
            .h_align_left();

        set_text_size(&mut txt, 20.0, gfx);
    }

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
    
    ui_output.clear_color(state.theme.bg);

    let (w, h) = gfx.size();
    state.screen_ratio = w as f32 / h as f32;

    background::draw_background(app.timer.elapsed_f32(), &mut draw, gfx, state);

    match &mut state.mode {
        GameMode::Ingame(_) => {
            ingame::game_frame(state, app, gfx, &mut draw);
        },
        _ => ()
    }

    gfx.render(&ui_output);
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
    };
    let test = proof_or_fake(seq);

    println!("seq = {}", test);
}
