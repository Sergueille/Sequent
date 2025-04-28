
use crate::coord::*;
use crate::State;
use notan::prelude::*;
use notan::draw::*;

pub const ANGLE: f32 = std::f32::consts::PI * 0.25;
pub const SIZE: f32 = 80.0;
pub const LINE_HEIGHT: f32 = 0.18;
pub const NB_LINES: u32 = 33;
pub const NB_CHARS: u32 = 45;
pub const MOVE_SPEED: f32 = 0.2;
pub const SYMBOLS: &str = "¬→∧∨⊤⊥⊢";

pub struct BackgroundState {
    pub strings: Vec<String>,
    pub last_swap_time: f32,
}


pub fn init_background_state() -> BackgroundState {

    let mut res = Vec::with_capacity(NB_LINES as usize);

    for _ in 0..NB_LINES {
        let mut line = Vec::with_capacity((NB_CHARS * 2) as usize);
        for _ in 0..NB_CHARS {
            let rand = notan::random::rand::random::<usize>() % SYMBOLS.chars().count();

            line.push(SYMBOLS.chars().nth(rand).unwrap());
            line.push(' ');
        }

        res.push(line.iter().collect());
    }

    return BackgroundState { strings: res, last_swap_time: 0.0 };
}


pub fn draw_background(time: f32, draw: &mut Draw, gfx: &Graphics, state: &mut State) {
    if (time - state.background_state.last_swap_time) * MOVE_SPEED > 1.0  {
        state.background_state.last_swap_time = time;

        for i in 0..NB_LINES {
            if i % 2 == 0 {
                let last = state.background_state.strings[i as usize].pop().unwrap();
                state.background_state.strings[i as usize].insert(0, last);
                let last = state.background_state.strings[i as usize].pop().unwrap();
                state.background_state.strings[i as usize].insert(0, last);
            }
            else {
                let first = state.background_state.strings[i as usize].remove(0);
                state.background_state.strings[i as usize].push(first);
                let first = state.background_state.strings[i as usize].remove(0);
                state.background_state.strings[i as usize].push(first);
            }
        }
    }

    let base_position = ScreenPosition { x: state.screen_ratio, y: -1.0 - LINE_HEIGHT };

    for i in 0..NB_LINES {
        let width = state.cached_sizes.get(&'¬').unwrap();

        let shift = MOVE_SPEED * (time - state.background_state.last_swap_time) * 2.0 * width * SIZE / crate::proof::rendering::TEXT_SCALE;

        let pos = ScreenPosition {
            x: base_position.x - (LINE_HEIGHT / f32::sin(ANGLE)) * i as f32,
            y: base_position.y,
        }.to_pixel(gfx);

        let direction = if i % 2 == 0 { 1.0 } else { -1.0 };

        let x = pos.x as f32 + f32::cos(ANGLE) * shift * direction;
        let y = pos.y as f32 - f32::sin(ANGLE) * shift * direction;

        draw.text(&state.symbol_font, &state.background_state.strings[i as usize])
            .position(x, y) //pos.x as f32, pos.y as f32)
            .rotate_from((x, y), -ANGLE)
            .size(SIZE)
            .color(state.theme.bg_text)
            .h_align_left()
            .v_align_bottom();
    }

}

