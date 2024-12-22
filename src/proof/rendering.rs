
use std::collections::HashMap;

use crate::proof::*;
use crate::coord::*;
use notan::prelude::*;
use notan::draw::*;

// Screen units
pub const PROOF_MARGIN: f32 = 100e-3;
pub const SEQUENT_MARGIN: f32 = 30e-3;
pub const COMMA_MARGIN: f32 = 10e-3;
pub const FIELD_SIZE: f32 = 70e-3;
pub const FIELD_HEIGHT: f32 = 70e-3;
pub const TEXT_SCALE: f32 = 50.0;
pub const LINE_HEIGHT: f32 = 120e-3;
pub const BAR_HEIGHT: f32 = 5e-3;
pub const PAR_POSITION: f32 = 100e-3;
pub const OPERATOR_MARGIN: f32 = 10e-3;

pub const FOCUSED_FILED_COLOR: u32 = 0x442200ff;
pub const FILED_COLOR: u32 = 0x000044ff;

pub const SYMBOLS: &str = "¬→∧∨⊤⊥⊢";

/// Letters used for variables, in order
pub const VARIABLE_LETTERS: &str = "ABCDEFGH";

pub struct RenderInfo<'a> {
    pub draw: &'a mut Draw,
    pub gfx: &'a mut Graphics,
    pub text_font: &'a Font,
    pub symbol_font: &'a Font,
    pub cached_sizes: &'a HashMap<char, f32>,
    pub focused_formula_field: u32,
    pub editing_formulas: bool,
}


pub fn draw_proof(p: &Proof, bottom_left: ScreenPosition, info: &mut RenderInfo) {
    let branches_width = get_proof_branches_width(p, info);
    let root_width = get_sequent_width(&p.root, info);
    let total_width = f32::max(branches_width, root_width);

    let root_left_space = (total_width - root_width) * 0.5;

    let mut pos = bottom_left;
    pos.x += root_left_space;

    draw_sequent(&p.root, pos, info);

    // Draw bar
    let mut branches_base_size = (f32::max(0.0, p.branches.len() as f32 - 1.0)) * PROOF_MARGIN;
    for b in p.branches.iter() {
        let seq_width = get_sequent_width(&b.root, info);
        branches_base_size += seq_width + f32::max(0.0, get_proof_width(b, info) - seq_width) * 0.5;
    }

    let bar_left_pos = if p.branches.len() == 0 { 0.0 } else {
        (get_proof_width(&p.branches[0], info) - get_sequent_width(&p.branches[0].root, info) ) * 0.5
    };
    let bar_right_pos = if p.branches.len() == 0 { 0.0 } else {
        (get_proof_width(&p.branches[p.branches.len() - 1], info) - get_sequent_width(&p.branches[p.branches.len() - 1].root, info)) * 0.5
    };

    let mut bl_pos = bottom_left;
    bl_pos.x += bar_left_pos;
    bl_pos.y += PAR_POSITION;

    let mut tr_pos = bl_pos.clone();
    tr_pos.x += total_width - bar_right_pos - bar_left_pos;
    tr_pos.y += BAR_HEIGHT;

    let bl = bl_pos.to_pixel(info.gfx).as_f32_couple();
    let size = tr_pos.to_pixel(info.gfx).difference_with_f32(bl_pos.to_pixel(info.gfx));
    info.draw.rect(bl, size)
        .color(Color::from_hex(0xffffffff));

    let top_left_space = (total_width - branches_width) * 0.5;
    pos = bottom_left;
    pos.x += top_left_space;
    pos.y += LINE_HEIGHT;

    for child in p.branches.iter() {
        draw_proof(&child, pos, info);

        pos.x += get_proof_width(child, info);
        pos.x += PROOF_MARGIN;
    }
}


pub fn draw_sequent(s: &Sequent, bottom_left: ScreenPosition, info: &mut RenderInfo) {
    let mut pos = bottom_left;

    for (i, f) in s.before.iter().enumerate() {
        if i != 0 {
            pos.x += draw_text(",", pos, info.text_font, info);
            pos.x += COMMA_MARGIN;
        }

        draw_formula(f, pos, info);
        pos.x += get_formula_width(f, info);
    }

    if s.before.len() > 0 { pos.x += SEQUENT_MARGIN };

    pos.x += draw_text("⊢", pos, info.symbol_font, info);

    if s.after.len() > 0 { pos.x += SEQUENT_MARGIN };

    for (i, f) in s.after.iter().enumerate() {
        if i != 0 {
            pos.x += draw_text(",", pos, info.text_font, info);
            pos.x += COMMA_MARGIN;
        }

        draw_formula(f, pos, info);
        pos.x += get_formula_width(f, info);
    }
}


pub fn draw_formula(f: &Formula, bottom_left: ScreenPosition, info: &mut RenderInfo) {
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
                    let need_p = needs_parentheses(priority, &f);

                    if need_p {
                        draw_pos.x += draw_text(&opening_parenthesis, draw_pos, info.text_font, info);
                    }

                    draw_formula(f, draw_pos, info);
                    draw_pos.x += get_formula_width(f, info);

                    if need_p {
                        draw_pos.x += draw_text(&closing_parenthesis, draw_pos, info.text_font, info);
                    }

                    draw_pos.x += OPERATOR_MARGIN;
                }
                None => {},
            }
            
            draw_pos.x += draw_text(&get_operator_symbol(operator.operator_type), draw_pos, info.symbol_font, info);

            match right_f {
                Some(f) => {
                    let need_p = needs_parentheses(priority, &f);

                    draw_pos.x += OPERATOR_MARGIN;

                    if need_p {
                        draw_pos.x += draw_text(&opening_parenthesis, draw_pos, info.text_font, info);
                    }

                    draw_formula(f, draw_pos, info);
                    draw_pos.x += get_formula_width(f, info);

                    if need_p {
                        draw_pos.x += draw_text(&closing_parenthesis, draw_pos, info.text_font, info);
                    }
                }
                None => {},
            }
        },
        Formula::Variable(id) => {
            draw_text(&VARIABLE_LETTERS.chars().nth(*id as usize).unwrap().to_string(), bottom_left, info.text_font, info);
        },
        Formula::NotCompleted(field_info) => {
            let color = if info.editing_formulas && field_info.id == info.focused_formula_field { 
                FOCUSED_FILED_COLOR 
            } else { 
                FILED_COLOR 
            };

            let bl = bottom_left.to_pixel(info.gfx).as_f32_couple();
            let mut top_right = bottom_left.clone();
            top_right.x += FIELD_SIZE;
            top_right.y += FIELD_HEIGHT;

            let size = top_right.to_pixel(&info.gfx).difference_with_f32(bottom_left.to_pixel(&info.gfx));
            info.draw.rect(bl, size).color(Color::from_hex(color));
        },
    }
}


pub fn get_proof_width(p: &Proof, info: &mut RenderInfo) -> f32 {
    return f32::max(get_proof_branches_width(p, info), get_sequent_width(&p.root, info));
}


fn get_proof_branches_width(p: &Proof, info: &mut RenderInfo) -> f32 {
    let mut sum = if p.branches.len() > 0 { (p.branches.len() - 1) as f32 * PROOF_MARGIN } else { 0.0 };

    for proof in p.branches.iter() {
        sum += get_proof_width(&proof, info);
    }

    return sum;
}


pub fn get_sequent_width(s: &Sequent, info: &RenderInfo) -> f32 {
    let mut sum = get_character_width('⊢', info);

    if s.before.len() > 0 { sum += SEQUENT_MARGIN };
    if s.after.len() > 0 { sum += SEQUENT_MARGIN };

    let comma_size = COMMA_MARGIN + get_character_width(',', info);
    if s.before.len() > 0 { sum += (s.before.len() as f32 - 1.0) * comma_size };
    if s.before.len() > 0 { sum += (s.after.len() as f32 - 1.0) * comma_size };

    for f in s.before.iter().chain(s.after.iter()) {
        sum += get_formula_width(&f, info);
    }

    return sum;
}


pub fn get_formula_width(f: &Formula, info: &RenderInfo) -> f32 {
    match f {
        Formula::Operator(operator) => {
            let mut sum = get_character_width(get_operator_symbol(operator.operator_type).chars().next().unwrap(), info);
            sum += get_operator_arity(operator.operator_type) as f32 * OPERATOR_MARGIN;

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
        Formula::NotCompleted(_) => {
            return FIELD_SIZE;
        },
    }
}

pub fn get_character_width(char: char, info: &RenderInfo) -> f32 {
    let (_vw, vh) = info.gfx.size();
    
    match info.cached_sizes.get(&char) {
        Some(w) => *w / vh as f32 * 2.0,
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

    for c in 'A'..(('Z' as u8 + 1) as char) {
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
    info.draw.text(&font, text)
        .position(position.to_pixel(info.gfx).x as f32, position.to_pixel(info.gfx).y as f32)
        .size(TEXT_SCALE)
        .v_align_bottom()
        .h_align_left();

    return info.draw.last_text_bounds().width / info.gfx.size().1 as f32 * 2.0;
}

