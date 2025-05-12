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
mod menus;
mod settings;
mod misc;

/// Current global state of the game.
#[allow(clippy::large_enum_variant)]
enum GameMode {
    Ingame(ingame::GameState),
    Menu(menus::MenuState),
    SetKey(menus::KeyRecordState),
    None,
}


enum VerticalAlign {
    Top, Middle, Bottom
}

enum HorizontalAlign {
    Top, Middle, Bottom
}


#[derive(AppState)]
struct State {
    text_font: Font,
    symbol_font: Font,
    cached_sizes: HashMap<char, f32>,
    mode: GameMode,
    screen_ratio: f32,
    background_state: background::BackgroundState,
    settings: settings::Settings,
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

    let settings = match settings::load_settings() {
        Ok(s) => s,
        Err(e) => {
            panic!("{}", e.message);
        },
    };

    let mut state = State {
        text_font: font,
        symbol_font, 
        cached_sizes: proof::rendering::compute_char_sizes(&font, &symbol_font),
        mode: GameMode::None,
        // mode: ingame::get_initial_state(proof::get_empty_sequent(), 0.0),
        screen_ratio: 1.0,
        background_state: background::init_background_state(),
        settings,
    };

    state.mode = GameMode::Menu(menus::MenuState {
        current_menu: menus::main_menu(&state),
        focused_element: 0,
        y_scroll: 0.0,
    });

    return state;
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
    
    ui_output.clear_color(state.settings.theme().bg);

    let (w, h) = gfx.size();
    state.screen_ratio = w as f32 / h as f32;

    background::draw_background(app.timer.elapsed_f32(), &mut draw, gfx, state);

    match &mut state.mode {
        GameMode::Ingame(_) => {
            ingame::game_frame(state, app, gfx, &mut draw);
        },
        GameMode::Menu(_) => {
            menus::draw_menu(state, app, gfx, &mut draw);
        },
        GameMode::SetKey(s) => {
            menus::handle_key_input(s.clone(), state, &mut draw, gfx, app);
        }
        GameMode::None => { }
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
