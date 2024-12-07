
use std::collections::HashMap;

use crate::proof::*;
use crate::coord::*;
use crate::State;
use notan::prelude::*;
use notan::draw::*;

// Screen units
pub const PROOF_MARGIN: f32 = 50e-3;
pub const SEQUENT_MARGIN: f32 = 30e-3;
pub const COMMA_MARGIN: f32 = 10e-3;
pub const FIELD_SIZE: f32 = 70e-3;
pub const TEXT_SCALE: f32 = 50.0;
pub const LINE_HEIGHT: f32 = 90e-3;
pub const OPERATOR_MARGIN: f32 = 10e-3;

pub const SYMBOLS: &str = "¬→∧∨⊤⊥⊢";

/// Letters used for variables, in order
pub const VARIABLE_LETTERS: &str = "AZERTYUIOP";


pub fn draw_sequent(s: &Sequent, bottom_left: ScreenPosition, gfx: &mut Graphics, draw: &mut Draw, state: &State) {
    let mut pos = bottom_left;

    for (i, f) in s.before.iter().enumerate() {
        if i != 0 {
            pos.x += draw_text(",", pos, state.text_font, gfx, draw);
            pos.x += COMMA_MARGIN;
        }

        draw_formula(f, pos, gfx, draw, state);
        pos.x += get_formula_width(f, state, gfx);
    }

    if s.before.len() > 0 { pos.x += SEQUENT_MARGIN };

    pos.x += draw_text("⊢", pos, state.symbol_font, gfx, draw);

    if s.after.len() > 0 { pos.x += SEQUENT_MARGIN };

    for (i, f) in s.after.iter().enumerate() {
        if i != 0 {
            pos.x += draw_text(",", pos, state.text_font, gfx, draw);
            pos.x += COMMA_MARGIN;
        }

        draw_formula(f, pos, gfx, draw, state);
        pos.x += get_formula_width(f, state, gfx);
    }
}


pub fn draw_formula(f: &Formula, bottom_left: ScreenPosition, gfx: &mut Graphics, draw: &mut Draw, state: &State) {
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
                        draw_pos.x += draw_text(&opening_parenthesis, draw_pos, state.text_font, gfx, draw);
                    }

                    draw_formula(f, draw_pos, gfx, draw, state);
                    draw_pos.x += get_formula_width(f, state, &gfx);

                    if need_p {
                        draw_pos.x += draw_text(&closing_parenthesis, draw_pos, state.text_font, gfx, draw);
                    }

                    draw_pos.x += OPERATOR_MARGIN;
                }
                None => {},
            }
            
            draw_pos.x += draw_text(&get_operator_symbol(operator.operator_type), draw_pos, state.symbol_font, gfx, draw);

            match right_f {
                Some(f) => {
                    let need_p = needs_parentheses(priority, &f);

                    draw_pos.x += OPERATOR_MARGIN;

                    if need_p {
                        draw_pos.x += draw_text(&opening_parenthesis, draw_pos, state.text_font, gfx, draw);
                    }

                    draw_formula(f, draw_pos, gfx, draw, state);
                    draw_pos.x += get_formula_width(f, state, &gfx);

                    if need_p {
                        draw_pos.x += draw_text(&closing_parenthesis, draw_pos, state.text_font, gfx, draw);
                    }
                }
                None => {},
            }
        },
        Formula::Variable(id) => {
            draw_text(&VARIABLE_LETTERS.chars().nth(*id as usize).unwrap().to_string(), bottom_left, state.text_font, gfx, draw);
        },
        Formula::NotCompleted(_) => {
            // TODO
        },
    }
}


pub fn get_proof_width(p: &Proof, state: &State, gfx: &Graphics) -> f32 {
    let mut top_sum = if p.branches.len() > 0 { (p.branches.len() - 1) as f32 * PROOF_MARGIN } else { 0.0 };

    for proof in p.branches.iter() {
        top_sum += get_proof_width(&proof, &state, &gfx);
    }

    return f32::max(top_sum, get_sequent_width(&p.root, &state, &gfx));
}


pub fn get_sequent_width(s: &Sequent, state: &State, gfx: &Graphics) -> f32 {
    let mut sum = get_character_width('⊢', &state, &gfx);

    if s.before.len() > 0 { sum += SEQUENT_MARGIN };
    if s.after.len() > 0 { sum += SEQUENT_MARGIN };

    let comma_size = COMMA_MARGIN + get_character_width(',', &state, &gfx);
    if s.before.len() > 0 { sum += (s.before.len() as f32 - 1.0) * comma_size };
    if s.before.len() > 0 { sum += (s.after.len() as f32 - 1.0) * comma_size };

    for f in s.before.iter().chain(s.after.iter()) {
        sum += get_formula_width(&f, &state, &gfx);
    }

    return sum;
}


pub fn get_formula_width(f: &Formula, state: &State, gfx: &Graphics) -> f32 {
    match f {
        Formula::Operator(operator) => {
            let mut sum = get_character_width(get_operator_symbol(operator.operator_type).chars().next().unwrap(), &state, &gfx);
            sum += get_operator_arity(operator.operator_type) as f32 * OPERATOR_MARGIN;

            let priority = get_operator_priority(operator.operator_type);
            let parentheses_width = get_character_width('(', state, gfx) + get_character_width(')', state, gfx) ;

            if operator.arg1.is_some() { 
                if needs_parentheses(priority, operator.arg1.as_ref().unwrap()) {
                    sum += parentheses_width;
                }

                sum += get_formula_width(operator.arg1.as_ref().unwrap(), &state, &gfx);
            }
            if operator.arg2.is_some() { 
                if needs_parentheses(priority, operator.arg1.as_ref().unwrap()) {
                    sum += parentheses_width;
                }

                sum += get_formula_width(operator.arg2.as_ref().unwrap(), &state, &gfx);
            }
            
            return sum;
        },
        Formula::Variable(id) => {
            return get_character_width(VARIABLE_LETTERS.chars().nth(*id as usize).unwrap(), &state, gfx);
        },
        Formula::NotCompleted(_) => {
            return FIELD_SIZE;
        },
    }
}

pub fn get_character_width(char: char, state: &State, gfx: &Graphics) -> f32 {
    let (_vw, vh) = gfx.size();
    
    match state.cached_sizes.get(&char) {
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
            get_operator_priority(operator.operator_type) <= parent_priority
        },
        Formula::Variable(_) => false,
        Formula::NotCompleted(_) => false,
    }
}


fn draw_text(text: &str, position: ScreenPosition, font: Font, gfx: &Graphics, draw: &mut Draw) -> f32 {
    draw.text(&font, text)
        .position(position.to_pixel(gfx).x as f32, position.to_pixel(gfx).y as f32)
        .size(TEXT_SCALE)
        .v_align_bottom()
        .h_align_left();

    return draw.last_text_bounds().width / gfx.size().1 as f32 * 2.0;
}

