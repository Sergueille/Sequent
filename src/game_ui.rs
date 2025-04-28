
// Ingame UI rendering

use crate::coord::*;
use crate::{State, Theme, GameMode};
use notan::prelude::*;
use notan::draw::*;

/// Number of letters for which the key is displayed 
pub const NB_LETTERS_DISPLAYED: u32 = 6;

pub const ACTION_RECT_SIZE: f32 = 0.07;
pub const ACTION_RECT_COLOR: u32 = 0x222222ff;
pub const ACTION_RIGHT_MARGIN: f32 = 0.001;
pub const ACTION_TEXT_SIZE: f32 = 30.0;

pub const KEYS_COLUMN_SIZE: f32 = 0.3;
pub const KEYS_Y: f32 = 0.85;
pub const KEYS_LINE_HEIGHT: f32 = 0.1;
pub const BORDER_MARGIN: f32 = 0.08;


pub fn render_ui(special: bool, symbol_font: &Font, draw: &mut Draw, gfx: &Graphics, state: &State) {
    
    #[allow(unreachable_patterns)] // To be removed when other game modes will be implemented
    let game_state = match &state.mode {
        GameMode::Ingame(s) => s,
        _ => unreachable!()
    };

    if game_state.state.editing_formulas {

        let total_size = KEYS_COLUMN_SIZE * game_state.logic_system.operators.len() as f32;

        for (i, op) in game_state.logic_system.operators.iter().enumerate() {
            draw_action_and_text(
                ScreenPosition { x: -total_size * 0.5 + KEYS_COLUMN_SIZE * (i as f32 + 0.5), y: KEYS_Y - KEYS_LINE_HEIGHT },
                crate::action::Action::InsertOperator(i as u32),
                crate::proof::get_operator_symbol(*op),
                1.0,
                state.theme, &state.bindings, symbol_font, draw, gfx
            );
        }

        for i in 0..NB_LETTERS_DISPLAYED  {
            draw_action_and_text(
                ScreenPosition { x: -total_size * 0.5 + KEYS_COLUMN_SIZE * (i as f32 + 0.5), y: KEYS_Y },
                crate::action::Action::InsertVariable(i as u32),
                &crate::proof::rendering::VARIABLE_LETTERS.chars().nth(i as usize).unwrap().to_string(),
                1.0,
                state.theme, &state.bindings, symbol_font, draw, gfx
            );
        }
    }
    else {
        let total_size = KEYS_COLUMN_SIZE * ((game_state.logic_system.rules.len() + 1) / 2) as f32;

        for i in 0..game_state.logic_system.rules.len() {
            let rule = if special {
                match &game_state.logic_system.special_rules[i] {
                    Some(rule) => rule,
                    None => &game_state.logic_system.rules[i],
                }
            }
            else {
                &game_state.logic_system.rules[i]
            };

            draw_action_and_text(
                ScreenPosition { x: -total_size * 0.5 + KEYS_COLUMN_SIZE * ((i/2) as f32 + 0.5), y: KEYS_Y - KEYS_LINE_HEIGHT * ((i%2) as f32) },
                crate::action::Action::InsertRule(i as u32),
                rule.display_text(),
                1.0,
                state.theme, &state.bindings, symbol_font, draw, gfx
            );
        }
    }

    let left_actions = [
        crate::action::Action::Exit,
        crate::action::Action::Undo,
        crate::action::Action::Redo,
        crate::action::Action::ToggleKeys,
        crate::action::Action::SpecialRuleMode,
    ];

    let left_text = [
        "Exit",
        "Undo",
        "Redo",
        "Hide ui",
        "Alt. rules",
    ];

    for i in 0..5 {
        draw_action_and_text(
            ScreenPosition { 
                x: -state.screen_ratio + BORDER_MARGIN + ACTION_RECT_SIZE * 0.5 + ACTION_RIGHT_MARGIN * 0.5, 
                y: 1.0 - BORDER_MARGIN - KEYS_LINE_HEIGHT * i as f32
            },
            left_actions[i],
            left_text[i],
            0.7,
            state.theme, &state.bindings, symbol_font, draw, gfx
        );
    }

}


fn draw_action_and_text(pos: ScreenPosition, action: crate::action::Action, text: &str, text_scale: f32, theme: Theme, bindings: &crate::action::Bindings, 
    symbol_font: &Font, draw: &mut Draw, gfx: &Graphics
) {
    let x = pos.x - ACTION_RECT_SIZE * 0.5 - ACTION_RIGHT_MARGIN * 0.5;
    let pos = ScreenPosition { x, y: pos.y };

    draw_action(action, pos, theme, bindings, symbol_font, draw, gfx);

    let sym_pos = ScreenPosition { x: x + ACTION_RECT_SIZE + ACTION_RIGHT_MARGIN, y: pos.y }.to_pixel(gfx);

    let operator_symbol = text;
    draw.text(symbol_font, operator_symbol)
        .size(ACTION_TEXT_SIZE * text_scale)
        .position(sym_pos.x as f32, sym_pos.y as f32)
        .color(theme.ui_text)
        .v_align_middle()
        .h_align_left();
}


pub fn draw_action(action: crate::action::Action, position: ScreenPosition, theme: Theme, bindings: &crate::action::Bindings, 
    font: &Font, draw: &mut Draw, gfx: &Graphics
) {
    
    let rect = ScreenSize { x: ACTION_RECT_SIZE, y: ACTION_RECT_SIZE };
    let bl = position.subtract(rect.scale(0.5));

    let action_text = match bindings.get(&action) {
        Some(key_code) => {
            &crate::action::key_code_display(*key_code)
        },
        None => "No key",
    };

    let text_size = ACTION_TEXT_SIZE * match action_text.chars().count() {
        1 => 1.0,
        2 => 0.8,
        3 => 0.5,
        4 => 0.4,
        _ => 0.3,
    };

    let screen_pos_pixel = position.to_pixel(gfx);

    draw.rect(bl.to_pixel(gfx).as_f32_couple(), rect.to_pixel_f32(gfx))
        .color(theme.ui_bg);

    draw.text(font, action_text)
        .size(text_size)
        .position(screen_pos_pixel.x as f32, screen_pos_pixel.y as f32)
        .color(theme.ui_text)
        .v_align_middle()
        .h_align_center();
}

