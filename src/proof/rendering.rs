
use std::collections::HashMap;

use crate::proof::*;
use crate::State;

// Screen units
pub const PROOF_MARGIN: f32 = 2e-3;
pub const FIELD_SIZE: f32 = 5e-3;
pub const TEXT_SCALE: f32 = 20.0;
pub const LINE_HEIGHT: f32 = 10e-3;
pub const OPERATOR_MARGIN: f32 = 2e-3;

pub const SYMBOLS: &str = "¬→∧∨⊤⊥⊢";

/// Letters used for variables, in order
pub const VARIABLE_LETTERS: &str = "ABCDEFGHIJ";


pub fn get_proof_width(p: &Proof, state: &State) -> f32 {
    let mut top_sum = if p.branches.len() > 0 { (p.branches.len() - 1) as f32 * PROOF_MARGIN } else { 0.0 };

    for proof in p.branches.iter() {
        top_sum += get_proof_width(&proof, &state);
    }

    return f32::max(top_sum, get_sequent_width(&p.root, &state));
}


pub fn get_sequent_width(s: &Sequent, state: &State) -> f32 {
    let mut sum = get_character_width('⊢', &state);
    sum += 2.0 * OPERATOR_MARGIN;

    for f in s.before.iter().chain(s.after.iter()) {
        sum += get_formula_width(&f, &state);
    }

    return sum;
}


pub fn get_formula_width(f: &Formula, state: &State) -> f32 {
    match f {
        Formula::Operator(operator) => {
            let mut sum = get_character_width(get_operator_symbol(operator.operator_type).chars().next().unwrap(), &state);
            sum += get_operator_arity(operator.operator_type) as f32 * OPERATOR_MARGIN;

            if operator.arg1.is_some() { sum += get_formula_width(operator.arg1.as_ref().unwrap(), &state); }
            if operator.arg2.is_some() { sum += get_formula_width(operator.arg2.as_ref().unwrap(), &state); }
            
            return sum;
        },
        Formula::Variable(id) => {
            return get_character_width(VARIABLE_LETTERS.chars().nth(*id as usize).unwrap(), &state);
        },
        Formula::NotCompleted(_) => {
            return FIELD_SIZE;
        },
    }
}

pub fn get_character_width(char: char, state: &State) -> f32 {
    match state.cached_sizes.get(&char) {
        Some(w) => *w,
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

    for c in 'A'..'Z' {
        insert_char(c, text_font, &mut res, &mut calculator)
    }

    for c in SYMBOLS.chars() {
        insert_char(c, symbol_font, &mut res, &mut calculator)
    }

    return res;
}

