
use std::collections::HashMap;

use crate::animation;
use crate::VerticalAlign;
use crate::proof::*;
use crate::coord::*;
use notan::prelude::*;
use notan::draw::*;

// Screen units
pub const PROOF_MARGIN: f32 = 200e-3;
pub const SEQUENT_MARGIN: f32 = 30e-3;
pub const COMMA_MARGIN: f32 = 10e-3;
pub const FIELD_WIDTH: f32 = 55e-3;
pub const FIELD_HEIGHT: f32 = 70e-3;
pub const FIELD_Y_SHIFT: f32 = 3e-3;
pub const TEXT_SCALE: f32 = 50.0;
pub const LINE_HEIGHT: f32 = 120e-3;
pub const BAR_HEIGHT: f32 = 5e-3;
pub const PAR_POSITION: f32 = 100e-3;
pub const OPERATOR_MARGIN: f32 = 10e-3;
pub const VARIABLE_Y_SHIFT: f32 = -7e-3;

pub const RULE_MARGIN: f32 = 10e-3;
pub const RULE_TEXT_SCALE: f32 = 0.5; // 1 is normal text

pub const APPEAR_TAU: f32 = 0.05;
pub const APPEAR_OVERSHOOT: f32 = 1.0;
pub const APPEAR_RULE_OVERSHOOT: f32 = 2.0;
pub const FIELD_APPEAR_TAU: f32 = 0.02;

pub const SYMBOLS: &str = "¬→∧∨⊤⊥⊢";

/// Letters used for variables, in order
pub const VARIABLE_LETTERS: &str = "ABCDEFGHIJK";

pub struct RenderInfo<'a> {
    pub draw: &'a mut Draw,
    pub gfx: &'a mut Graphics,
    pub text_font: &'a Font,
    pub symbol_font: &'a Font,
    pub cached_sizes: &'a HashMap<char, f32>,
    pub focused_formula_field: Option<u32>,
    pub editing_formulas: bool,
    pub logic_system: &'a LogicSystem,
    pub scale: f32,
    pub time: f32,
    pub theme: crate::Theme,
    // Position of the currently focused element. Set by the draw_proof function
    pub focus_rect: ScreenRect,
    pub fields_creation_time: &'a mut HashMap<u32, f32>,
}


pub fn draw_proof(p: &Proof, bottom_left: ScreenPosition, info: &mut RenderInfo) {
    let branches_width = get_proof_branches_width(p, info);
    let root_width = get_sequent_width(&p.root, info);
    let total_width = f32::max(branches_width, root_width);

    let root_left_space = (total_width - root_width) * 0.5;

    let appear_scale = crate::animation::ease_out_exp_second(info.time - p.creation_time, APPEAR_TAU, APPEAR_OVERSHOOT);

    let mut pos = bottom_left;
    pos.x += root_left_space;

    draw_sequent(&p.root, pos, appear_scale, info);

    // Draw bar
    let bar_left_pos = if p.branches.len() == 0 { 0.0 } else {
        f32::min(
            root_left_space,
            (get_proof_width(&p.branches[0], info) - get_proof_root_width(&p.branches[0], info)) * 0.5
        )
    };
    let bar_right_pos = if p.branches.len() == 0 { 0.0 } else {
        f32::min(
            root_left_space,
            (get_proof_width(&p.branches[p.branches.len() - 1], info) - get_proof_root_width(&p.branches[p.branches.len() - 1], info)) * 0.5
        )
    };

    let mut bl_pos = bottom_left;
    bl_pos.x += bar_left_pos;
    bl_pos.y += PAR_POSITION * info.scale;

    let mut tr_pos = bl_pos;
    tr_pos.x += total_width - bar_right_pos - bar_left_pos;
    tr_pos.y += BAR_HEIGHT * info.scale;

    let bar_color = if p.is_rule_invalid {
        info.theme.seq_invalid
    } else if p.last_focused_time == info.time { 
        info.theme.seq_bar_focused
    } else { 
        info.theme.seq_bar 
    };

    let mut bl = bl_pos.to_pixel(info.gfx).as_couple();
    let mut size = tr_pos.difference_with(bl_pos);
    size.x *= appear_scale;

    bl.1 = f32::round(bl.1); // Make sur the bar have a consistent size by snapping no the nearest pixel

    info.draw.rect(bl, size.to_pixel(info.gfx))
        .color(bar_color);

    let rule_scale = crate::animation::ease_out_exp_second(info.time - p.rule_set_time , APPEAR_TAU, APPEAR_RULE_OVERSHOOT);

    let rule_color = if p.is_rule_invalid {
        info.theme.seq_invalid
    } else {
        info.theme.seq_text
    };

    // Draw rule name
    match p.rule_id {
        Some(id) => {
            let text = format!("({})", info.logic_system.rules[id as usize].display_text());
            let mut position = tr_pos;
            position.x += RULE_MARGIN;

            draw_text_more_params(&text, position, info.symbol_font, RULE_TEXT_SCALE * rule_scale, VerticalAlign::Middle, rule_color, info);
        },
        None => (),
    }

    // Draw 
    let branches_left_space = (total_width - branches_width) * 0.5;
    pos = bottom_left;
    pos.x += branches_left_space;
    pos.y += LINE_HEIGHT * info.scale;

    for child in p.branches.iter() {
        draw_proof(child, pos, info);

        pos.x += get_proof_width(child, info);
        pos.x += PROOF_MARGIN * info.scale;
    }

    // Update focus position
    if p.last_focused_time == info.time {
        info.focus_rect = ScreenRect {
            bottom_left,
            top_right: ScreenPosition { x: bottom_left.x + total_width, y: bottom_left.y + LINE_HEIGHT * info.scale }
        }
    }

}


pub fn draw_sequent(s: &Sequent, bottom_left: ScreenPosition, squish_x: f32, info: &mut RenderInfo) {
    let mut pos = bottom_left;

    for (i, f) in s.before.iter().enumerate() {
        if i != 0 {
            pos.x += draw_text(",", pos, info.text_font, info) * squish_x;
            pos.x += COMMA_MARGIN * info.scale;
        }

        draw_formula(f, pos, squish_x, info);
        pos.x += get_formula_width(f, info) * squish_x;
    }

    if s.before.len() > 0 { pos.x += SEQUENT_MARGIN * info.scale * squish_x };

    pos.x += draw_text("⊢", pos, info.symbol_font, info);

    if s.after.len() > 0 { pos.x += SEQUENT_MARGIN * info.scale * squish_x };

    for (i, f) in s.after.iter().enumerate() {
        if i != 0 {
            pos.x += draw_text(",", pos, info.text_font, info) * squish_x;
            pos.x += COMMA_MARGIN * info.scale * squish_x;
        }

        draw_formula(f, pos, squish_x, info);
        pos.x += get_formula_width(f, info) * squish_x;
    }
}


pub fn draw_formula(f: &Formula, bottom_left: ScreenPosition, squish_x: f32, info: &mut RenderInfo) {
    match f {
        Formula::Operator(operator) => {
            let arity = get_operator_arity(operator.operator_type);
            let mut draw_pos = bottom_left;

            let left_f = if arity == 2 { operator.arg1.as_ref() } else { None };
            let right_f = if arity == 1 { operator.arg1.as_ref() } else { operator.arg2.as_ref() };

            let priority = get_operator_priority(operator.operator_type);
            let opening_parenthesis = '('.to_string();
            let closing_parenthesis = ')'.to_string();

            match left_f {
                Some(f) => {
                    let need_p = needs_parentheses(priority, f);

                    if need_p {
                        draw_pos.x += draw_text(&opening_parenthesis, draw_pos, info.text_font, info) * squish_x;
                    }

                    draw_formula(f, draw_pos, squish_x, info);
                    draw_pos.x += get_formula_width(f, info) * squish_x;

                    if need_p {
                        draw_pos.x += draw_text(&closing_parenthesis, draw_pos, info.text_font, info) * squish_x;
                    }

                    draw_pos.x += OPERATOR_MARGIN * info.scale * squish_x;
                }
                None => {},
            }
            
            draw_pos.x += draw_text(get_operator_symbol(operator.operator_type), draw_pos, info.symbol_font, info) * squish_x;

            match right_f {
                Some(f) => {
                    let need_p = needs_parentheses(priority, f);

                    draw_pos.x += OPERATOR_MARGIN * info.scale * squish_x;

                    if need_p {
                        draw_pos.x += draw_text(&opening_parenthesis, draw_pos, info.text_font, info) * squish_x;
                    }

                    draw_formula(f, draw_pos, squish_x, info);
                    draw_pos.x += get_formula_width(f, info) * squish_x;

                    if need_p {
                        draw_pos.x += draw_text(&closing_parenthesis, draw_pos, info.text_font, info) * squish_x;
                    }
                }
                None => {},
            }
        },
        Formula::Variable(id) => {
            let actual_pos = ScreenPosition { x: bottom_left.x, y: bottom_left.y + VARIABLE_Y_SHIFT * info.scale };
            draw_text(&VARIABLE_LETTERS.chars().nth(*id as usize).unwrap().to_string(), actual_pos, info.text_font, info);
        },
        Formula::NotCompleted(field_info) => {
            let color = if info.editing_formulas && Some(field_info.id) == info.focused_formula_field { 
                info.theme.seq_field_focused 
            } else { 
                info.theme.seq_field
            };

            let mut bl = bottom_left;
            bl.y += FIELD_Y_SHIFT * info.scale;

            let mut top_right = bl;
            top_right.x += FIELD_WIDTH * info.scale;
            top_right.y += FIELD_HEIGHT * info.scale;

            let mut size = top_right.difference_with(bottom_left);
            size.x *= get_or_create_field_size(field_info.id, info.fields_creation_time, info.time);
            
            info.draw.rect(bl.to_pixel(info.gfx).as_couple(), size.to_pixel(info.gfx)).color(color);

            // Update focus position
            if info.editing_formulas && Some(field_info.id) == info.focused_formula_field {
                let rect = ScreenRect {
                    bottom_left, top_right
                };

                info.focus_rect = ScreenRect::merge(info.focus_rect, rect);
            }
        },
    }
}


pub fn get_proof_width(p: &Proof, info: &mut RenderInfo) -> f32 {
    let x_scale = crate::animation::ease_out_exp_second(info.time - p.creation_time, APPEAR_TAU, APPEAR_OVERSHOOT);
    return f32::max(get_proof_branches_width(p, info), get_sequent_width(&p.root, info)) * x_scale;
}


fn get_proof_branches_width(p: &Proof, info: &mut RenderInfo) -> f32 {
    let mut sum = if p.branches.len() > 0 { (p.branches.len() - 1) as f32 * PROOF_MARGIN * info.scale } else { 0.0 };

    for proof in p.branches.iter() {
        sum += get_proof_width(proof, info);
    }

    return sum;
}

pub fn get_proof_root_width(p: &Proof, info: &mut RenderInfo) -> f32 {
    let x_scale = crate::animation::ease_out_exp_second(info.time - p.creation_time, APPEAR_TAU, APPEAR_OVERSHOOT);
    return get_sequent_width(&p.root, info) * x_scale;
}

pub fn get_sequent_width(s: &Sequent, info: &mut RenderInfo) -> f32 {
    let mut sum = get_character_width('⊢', info);

    if s.before.len() > 0 { sum += SEQUENT_MARGIN * info.scale };
    if s.after.len() > 0 { sum += SEQUENT_MARGIN * info.scale };

    let comma_size = COMMA_MARGIN + get_character_width(',', info);
    if s.before.len() > 0 { sum += (s.before.len() as f32 - 1.0) * comma_size };
    if s.before.len() > 0 { sum += (s.after.len() as f32 - 1.0) * comma_size };

    for f in s.before.iter().chain(s.after.iter()) {
        sum += get_formula_width(f, info);
    }

    return sum;
}


pub fn get_formula_width(f: &Formula, info: &mut RenderInfo) -> f32 {
    match f {
        Formula::Operator(operator) => {
            let mut sum = get_character_width(get_operator_symbol(operator.operator_type).chars().next().unwrap(), info);
            sum += get_operator_arity(operator.operator_type) as f32 * OPERATOR_MARGIN * info.scale;

            let priority = get_operator_priority(operator.operator_type);
            let parentheses_width = get_character_width('(', info) + get_character_width(')', info) ;

            if operator.arg1.is_some() { 
                if needs_parentheses(priority, operator.arg1.as_ref().unwrap()) {
                    sum += parentheses_width;
                }

                sum += get_formula_width(operator.arg1.as_ref().unwrap(), info);
            }
            if operator.arg2.is_some() { 
                if needs_parentheses(priority, operator.arg2.as_ref().unwrap()) {
                    sum += parentheses_width;
                }

                sum += get_formula_width(operator.arg2.as_ref().unwrap(), info);
            }
            
            return sum;
        },
        Formula::Variable(id) => {
            return get_character_width(VARIABLE_LETTERS.chars().nth(*id as usize).unwrap(), info);
        },
        Formula::NotCompleted(field_info) => {
            return FIELD_WIDTH * info.scale * get_or_create_field_size(field_info.id, info.fields_creation_time, info.time);
        },
    }
}


pub fn get_character_width(char: char, info: &RenderInfo) -> f32 {    
    match info.cached_sizes.get(&char) {
        Some(w) => *w / 1080.0 * 2.0 * info.scale, // Why 1080.0? No one knows...
        None => panic!("Unknown char width. Add it to SYMBOLS constant!"),
    }
}


/// Computes the width of the chars
pub fn compute_char_sizes(text_font: &notan::text::Font, symbol_font: &notan::text::Font) -> HashMap<char, f32> {
    let mut res = HashMap::new();

    let mut calculator = notan::text::Calculator::new();

    fn insert_char(c: char, font: &notan::text::Font, res: &mut HashMap<char, f32>, calculator: &mut notan::text::Calculator) {
        let str = String::from(c);

        let section = notan::glyph::Section::new().add_text(
            notan::glyph::Text::new(&str)
                .with_scale(TEXT_SCALE) // TODO: set font
                .with_font_id(font)
        );

        res.insert(
            c,
            calculator.bounds(&section).width
        );
    }

    for c in 'A'..((b'Z' + 1) as char) {
        insert_char(c, text_font, &mut res, &mut calculator)
    }
    for c in 'a'..((b'z' + 1) as char) {
        insert_char(c, text_font, &mut res, &mut calculator)
    }

    for c in SYMBOLS.chars() {
        insert_char(c, symbol_font, &mut res, &mut calculator)
    }

    insert_char('(', text_font, &mut res, &mut calculator);
    insert_char(')', text_font, &mut res, &mut calculator);
    insert_char(',', text_font, &mut res, &mut calculator);

    return res;
}


fn needs_parentheses(parent_priority: f32, f: &Formula) -> bool {
    match f {
        Formula::Operator(operator) => {
            get_operator_priority(operator.operator_type) >= parent_priority
        },
        Formula::Variable(_) => false,
        Formula::NotCompleted(_) => false,
    }
}


fn draw_text(text: &str, position: ScreenPosition, font: &Font, info: &mut RenderInfo) -> f32 {
    draw_text_more_params(text, position, font, 1.0, VerticalAlign::Bottom, info.theme.seq_text, info)
}


fn draw_text_more_params(text: &str, position: ScreenPosition, font: &Font, scale: f32, vertical_align: VerticalAlign, color: Color, info: &mut RenderInfo) -> f32 {
    let align_fn = match vertical_align {
        VerticalAlign::Top => TextSection::v_align_top,
        VerticalAlign::Middle => TextSection::v_align_middle,
        VerticalAlign::Bottom => TextSection::v_align_bottom,
    };

    { // Actual text to be rendered
        let mut builder = info.draw.text(font, text);
        
        builder.position(position.to_pixel(info.gfx).x, position.to_pixel(info.gfx).y)
            .color(color)
            .h_align_left();

        set_text_size(&mut builder, TEXT_SCALE * scale * info.scale, info.gfx);

        align_fn(&mut builder);
    }

    { // Fake, transparent text to get the correct text width
        let mut builder = info.draw.text(font, text);
        
        builder.position(f32::round(position.to_pixel(info.gfx).x), position.to_pixel(info.gfx).y)
            .color(Color::from_hex(0x00000000))
            .h_align_left();

        set_text_size(&mut builder, TEXT_SCALE * scale * info.scale, info.gfx);
    
        align_fn(&mut builder);
    }

    return info.draw.last_text_bounds().width / info.gfx.size().1 as f32 * 2.0;
}

fn get_or_create_field_size(field_id: u32, table: &mut HashMap<u32, f32>, time: f32) -> f32 {
    let creation_time = match table.get(&field_id) {
        Some(t) => { 
            *t
        }
        None => {
            table.insert(field_id, time);
            time
        },
    };

    return animation::ease_out_exp(time - creation_time, FIELD_APPEAR_TAU);
}

