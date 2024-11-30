use notan::prelude::*;

// Screen position:
// Origin is the center of the screen
// -r to r range for x, where r is the aspect ratio (w / h)
// -1 to 1 range for y
//
// (-r, 1)         (r, 1)
//          
//          (0, 0)
//
// (-r, -1)        (r, -1)
//

// Pixel position:
// Origin is top left, corresponds to pixels
//
// (0, 0)          (w, 0)
//    
//       (w/2, h/2)
//
// (0, w)          (w, h)
//

#[derive(Clone, Copy, Debug)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

impl ScreenPosition {
    pub fn to_pixel(self, state: &Graphics) -> PixelPosition {
        let (w, h) = state.size();
        let ratio = w as f32 / h as f32;

        return PixelPosition {
            x: (h as f32 * 0.5 * (self.x + ratio)).round() as u32,
            y: (h as f32 * 0.5 * (1.0 - self.y)).round() as u32,
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct PixelPosition {
    pub x: u32,
    pub y: u32,
}

impl PixelPosition {
    pub fn from_couple((x, y): (u32, u32)) -> Self {
        return PixelPosition { x, y };
    }

    pub fn as_couple(&self) -> (u32, u32) {
        return (self.x, self.y);
    }

    pub fn to_screen(self, state: &Graphics) -> ScreenPosition {
        let (w, h) = state.size();

        return ScreenPosition {
            x: (self.x as f32 / h as f32) * 2.0 - w as f32,
            y: (self.y as f32 / h as f32) * 2.0 - h as f32,
        }
    }
}

