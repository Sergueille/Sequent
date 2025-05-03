
// Ingame UI rendering

use crate::coord::*;
use crate::{State, Theme, GameMode};
use notan::prelude::*;
use notan::draw::*;

/// Number of letters for which the key is displayed 
pub const NB_LETTERS_DISPLAYED: u32 = 6;

pub const ACTION_RECT_SIZE: f32 = 0.07;
pub const ACTION_LARGE_RECT_WIDTH: f32 = 0.14;
pub const ACTION_RECT_COLOR: u32 = 0x222222ff;
pub const ACTION_RIGHT_MARGIN: f32 = 0.001;
pub const ACTION_TEXT_SIZE: f32 = 30.0;

pub const KEYS_COLUMN_SIZE: f32 = 0.3;
pub const KEYS_Y: f32 = 0.85;
pub const KEYS_LINE_HEIGHT: f32 = 0.1;
pub const KEYS_SCALE_SHIFT_X: f32 = 0.1;
pub const BORDER_MARGIN: f32 = 0.08;
pub const MISC_KEYS_Y: f32 = 0.95;
pub const MISC_KEYS_COLUMN_SIZE: f32 = 0.3;
pub const MISC_KEYS_SCALE: f32 = 0.6;
pub const MISC_KEYS_SCALE_SHIFT_X: f32 = 0.07;


pub fn render_ui(special: bool, symbol_font: &Font, draw: &mut Draw, gfx: &Graphics, state: &State) {
    
    let game_state = match &state.mode {
        GameMode::Ingame(s) => s,
        _ => unreachable!()
    };

    if game_state.state.editing_formulas {

        let total_size = KEYS_COLUMN_SIZE * game_state.logic_system.operators.len() as f32;

        for (i, op) in game_state.logic_system.operators.iter().enumerate() {
            draw_action_and_text(
                ScreenPosition { x: -total_size * 0.5 + KEYS_COLUMN_SIZE * i as f32 + KEYS_SCALE_SHIFT_X, y: KEYS_Y - KEYS_LINE_HEIGHT },
                crate::action::Action::InsertOperator(i as u32),
                crate::proof::get_operator_symbol(*op),
                1.0,
                state.theme, &state.bindings, symbol_font, draw, gfx
            );
        }

        for i in 0..NB_LETTERS_DISPLAYED  {
            draw_action_and_text(
                ScreenPosition { x: -total_size * 0.5 + KEYS_COLUMN_SIZE * i as f32 + KEYS_SCALE_SHIFT_X, y: KEYS_Y },
                crate::action::Action::InsertVariable(i),
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
                ScreenPosition { x: -total_size * 0.5 + KEYS_COLUMN_SIZE * (i/2) as f32 + KEYS_SCALE_SHIFT_X, y: KEYS_Y - KEYS_LINE_HEIGHT * ((i%2) as f32) },
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
        crate::action::Action::Restart,
        crate::action::Action::ToggleKeys,
        crate::action::Action::SpecialRuleMode,
    ];

    let left_text = [
        "Exit",
        "Undo",
        "Redo",
        "Restart",
        "Hide UI",
        "Alt. rules",
    ];

    for i in 0..left_actions.len() {
        let total_size = MISC_KEYS_COLUMN_SIZE * (left_actions.len() as f32);

        draw_action_and_text(
            ScreenPosition { x: -total_size * 0.5 + MISC_KEYS_COLUMN_SIZE * i as f32 + MISC_KEYS_SCALE_SHIFT_X, y: MISC_KEYS_Y },
            left_actions[i],
            left_text[i],
            MISC_KEYS_SCALE,
            state.theme, &state.bindings, symbol_font, draw, gfx
        );
    }

}


fn draw_action_and_text(pos: ScreenPosition, action: crate::action::Action, text: &str, text_scale: f32, theme: Theme, bindings: &crate::action::Bindings, 
    symbol_font: &Font, draw: &mut Draw, gfx: &Graphics
) {
    let x = pos.x; // - ACTION_RECT_SIZE * 0.5 - ACTION_RIGHT_MARGIN * 0.5;
    let pos = ScreenPosition { x, y: pos.y };

    draw_action(action, pos, text_scale, theme, bindings, symbol_font, draw, gfx);

    let text_pos = ScreenPosition { x: x + (ACTION_RECT_SIZE + ACTION_RIGHT_MARGIN) * text_scale, y: pos.y }.to_pixel(gfx);

    let operator_symbol = text;
    let mut text = draw.text(symbol_font, operator_symbol);
    text.position(text_pos.x, text_pos.y)
        .color(theme.ui_text)
        .v_align_middle()
        .h_align_left();

    set_text_size(&mut text, ACTION_TEXT_SIZE * text_scale, gfx);
}


pub fn draw_action(action: crate::action::Action, position: ScreenPosition, scale: f32, theme: Theme, bindings: &crate::action::Bindings, 
    font: &Font, draw: &mut Draw, gfx: &Graphics
) {
    let rect;
    let mut bl;
    let mut center;
    
    let action_text = match bindings.get(&action) {
        Some(key_code) => {
            &crate::action::key_code_display(*key_code)
        },
        None => "??",
    };

    if action_text.chars().count() <= 3 {
        rect = ScreenSize { x: ACTION_RECT_SIZE * scale, y: ACTION_RECT_SIZE * scale };
        bl = position.subtract(rect.scale(0.5));
        center = position;
    }
    else {
        rect = ScreenSize { x: ACTION_LARGE_RECT_WIDTH * scale, y: ACTION_RECT_SIZE * scale };
        bl = position.subtract(rect.scale(0.5));
        center = position;

        center.x -= (ACTION_LARGE_RECT_WIDTH - ACTION_RECT_SIZE) * 0.25;
        bl.x -= (ACTION_LARGE_RECT_WIDTH - ACTION_RECT_SIZE) * 0.25;
    }

    let text_size = ACTION_TEXT_SIZE * scale * match action_text.chars().count() {
        1 => 1.0,
        2 => 0.8,
        _ => 0.5,
    };

    draw.rect(bl.to_pixel(gfx).as_couple(), rect.to_pixel(gfx))
        .color(theme.ui_bg);

    let screen_pos_pixel = center.to_pixel(gfx);

    let mut txt = draw.text(font, action_text);
    txt.position(screen_pos_pixel.x, screen_pos_pixel.y)
        .color(theme.ui_text)
        .v_align_middle()
        .h_align_center();

    set_text_size(&mut txt, text_size, gfx);
}

