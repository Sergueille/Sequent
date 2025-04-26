
// Ingame UI rendering

use crate::coord::*;
use crate::GameState;
use notan::prelude::*;
use notan::draw::*;


pub const ACTION_RECT_SIZE: f32 = 0.07;
pub const ACTION_RECT_COLOR: u32 = 0x222222ff;
pub const ACTION_RIGHT_MARGIN: f32 = 0.001;
pub const ACTION_TEXT_SIZE: f32 = 30.0;

pub const KEYS_COLUMN_SIZE: f32 = 0.3;
pub const KEYS_Y: f32 = 0.85;
pub const KEYS_LINE_HEIGHT: f32 = 0.1;


pub fn render_ui(special: bool, bindings: &crate::action::Bindings, symbol_font: &Font, draw: &mut Draw, gfx: &Graphics, game_state: &GameState) {
    
    if game_state.state.editing_formulas {

        let total_size = KEYS_COLUMN_SIZE * game_state.logic_system.operators.len() as f32;

        for (i, op) in game_state.logic_system.operators.iter().enumerate() {
            draw_action_and_text(
                ScreenPosition { x: -total_size * 0.5 + KEYS_COLUMN_SIZE * (i as f32 + 0.5), y: KEYS_Y },
                crate::action::Action::InsertOperator(i as u32),
                crate::proof::get_operator_symbol(*op),
                bindings, symbol_font, draw, gfx
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
                bindings, symbol_font, draw, gfx
            );
        }
    }

}


fn draw_action_and_text(pos: ScreenPosition, action: crate::action::Action, text: &str, bindings: &crate::action::Bindings, 
    symbol_font: &Font, draw: &mut Draw, gfx: &Graphics
) {
    let x = pos.x - ACTION_RECT_SIZE * 0.5 - ACTION_RIGHT_MARGIN * 0.5;
    let pos = ScreenPosition { x, y: pos.y };

    draw_action(action, pos, bindings, symbol_font, draw, gfx);

    let sym_pos = ScreenPosition { x: x + ACTION_RECT_SIZE + ACTION_RIGHT_MARGIN, y: pos.y }.to_pixel(gfx);

    let operator_symbol = text;
    draw.text(symbol_font, operator_symbol)
        .size(ACTION_TEXT_SIZE)
        .position(sym_pos.x as f32, sym_pos.y as f32)
        .v_align_middle()
        .h_align_left();
}


pub fn draw_action(action: crate::action::Action, position: ScreenPosition, bindings: &crate::action::Bindings, font: &Font, draw: &mut Draw, gfx: &Graphics) {
    
    let rect = ScreenRect { x: ACTION_RECT_SIZE, y: ACTION_RECT_SIZE };
    let bl = position.subtract(rect.scale(0.5));

    let action_text = match bindings.get(&action) {
        Some(key_code) => {
            &crate::action::key_code_display(*key_code)
        },
        None => "No key",
    };

    let text_size = match bindings.get(&action) {
        Some(_) => ACTION_TEXT_SIZE,
        None => ACTION_TEXT_SIZE * 0.5,
    };


    let screen_pos_pixel = position.to_pixel(gfx);

    draw.rect(bl.to_pixel(gfx).as_f32_couple(), rect.to_pixel_f32(gfx))
        .color(Color::from_hex(ACTION_RECT_COLOR));

    draw.text(font, action_text)
        .size(text_size)
        .position(screen_pos_pixel.x as f32, screen_pos_pixel.y as f32)
        .v_align_middle()
        .h_align_center();
}

