
use crate::proof::*;
use crate::coord::*;
use crate::State;

use notan::draw::DrawCustomPipeline;
use notan::draw::DrawImages;
use notan::glyph;
use notan::glyph::DefaultGlyphPipeline;
use notan::glyph::GlyphBrush;
use notan::glyph::Section;
use notan::prelude::*;

pub const PROOF_MARGIN: f32 = 2e-3;
pub const FIELD_SIZE: f32 = 5e-3;
pub const TEXT_SCALE: f32 = 20.0;
pub const LINE_HEIGHT: f32 = 10e-3;

// OPTI: getting the width of the proofs many times before rendering (in the dumbest possible way)
//       maybe it is slow (idk if it is cached by notan)

pub fn draw_proof(proof: &Proof, position: ScreenPosition, gfx: &mut Graphics, draw: &mut notan::draw::Draw, state: &State) { 
    let mut brush: glyph::GlyphBrush<&glyph::ab_glyph::FontVec> = 
        glyph::GlyphBrushBuilder::using_fonts(vec![
            &state.text_font,
            &state.symbol_font,
        ]).build(gfx);

    _queue_proof_text(proof, position, &mut brush, gfx);

    // TEST
    brush.queue(
        Section::new().add_text(
            glyph::Text::new("Test")
                .with_color((10.0, 1.0, 1.0, 1.0))
                .with_scale(200.0)
                .with_font_id(glyph::FontId(1))
        )
            .with_screen_position((1.0, 1.0))
    );

    let mut pipeline = DefaultGlyphPipeline::new(gfx).unwrap();
    brush.render_queue(&mut gfx.device, &mut pipeline);
}

fn _queue_proof_text(proof: &Proof, position: ScreenPosition, brush: &mut GlyphBrush<&glyph::ab_glyph::FontVec>, gfx: &mut Graphics) {
    let root_section = 
        proof.root.cached_text_section.as_ref()
        .expect("Missing text section on sequent!")
        .clone() // OPTI
        .with_screen_position(position.to_pixel(gfx).as_f32_couple());

    brush.queue(root_section);
}

pub fn get_proof_width(proof: &Proof, graphics: &mut Graphics, state: &mut State) -> f32 {
    let mut top_len_sum = 0.0;
    for (i, p) in proof.branches.iter().enumerate() {
        if i != 0 {
            top_len_sum = PROOF_MARGIN;
        }

        top_len_sum += get_proof_width(p, graphics, state);
    }

    let bottom_sum = state.text_calculator.bounds(
        &proof.root.cached_text_section.as_ref()
            .expect("Missing text section on sequent!")
    ).width;

    return f32::max(top_len_sum, bottom_sum);
}

pub fn compute_sequent_text_section<'a>(seq: &'a mut Sequent, state: &State) -> &'a glyph::Section<'static> {
    let mut section = glyph::Section::new().with_layout(
        notan::glyph::Layout::default()
            .v_align(glyph::VerticalAlign::Top)
            .h_align(glyph::HorizontalAlign::Center)
    );

    for (i, formula) in seq.before.iter().enumerate() {
        if i != 0 {
            section = section.add_text(
                glyph::Text::new(", ")
                    // .with_font_id(state.text_font)
                    .with_scale(TEXT_SCALE)
            );
        }

        section = _add_formula_text(&formula, section, state);
    }

    section = section.add_text(
        glyph::Text::new(" ⊢ ")
            // .with_font_id(state.symbol_font)
            .with_scale(TEXT_SCALE)
    );

    for (i, formula) in seq.after.iter().enumerate() {
        if i != 0 {
            section = section.add_text(
                glyph::Text::new(", ")
                    // .with_font_id(state.text_font)
                    .with_scale(TEXT_SCALE)
            );
        }

        section = _add_formula_text(&formula, section, state);
    }

    seq.cached_text_section = Some(section);

    return &seq.cached_text_section.as_ref().unwrap();
}

fn _add_formula_text<'a>(f: &Formula, section: glyph::Section<'a>, state: &State) -> glyph::Section<'a> {
    return match f {
        Formula::Operator(operator) => {
            let symbol = get_operator_symbol(operator.operator_type);
            let arity = get_operator_arity(operator.operator_type);
            let mut new_sect = section;

            if arity == 2 {
                new_sect = _add_formula_text(operator.arg1.as_ref().unwrap(), new_sect, state);
            }

            new_sect = new_sect.add_text(
                glyph::Text::new(symbol)
                    // .with_font_id(state.symbol_font)
                    .with_scale(TEXT_SCALE)
            );

            if arity > 0 {
                let arg = if arity == 2 { &operator.arg2 } else { &operator.arg1 };
                new_sect = _add_formula_text(arg.as_ref().unwrap(), new_sect, state);
            }

            new_sect
        },
        Formula::Variable(_) => {
            let s = "A"; // TEST
            section.add_text(glyph::Text::new(s)/*.with_font_id(state.text_font)*/.with_scale(TEXT_SCALE))
        },
        Formula::NotCompleted(_) => {
            let s = "_"; // HACK: find way to insert other thing
            section.add_text(glyph::Text::new(s)/*.with_font_id(state.text_font)*/.with_scale(TEXT_SCALE))
        },
    };
}

