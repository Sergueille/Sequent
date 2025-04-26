use notan::prelude::*;

/// Origin is the center of the screen
/// -r to r range for x, where r is the aspect ratio (w / h)
/// -1 to 1 range for y
///
/// ```
/// (-r, 1)         (r, 1)
///          
///          (0, 0)
///
/// (-r, -1)        (r, -1)
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

impl ScreenPosition {
    pub fn to_pixel(self, gfx: &Graphics) -> PixelPosition {
        let (w, h) = gfx.size();
        let ratio = w as f32 / h as f32;

        return PixelPosition {
            x: (h as f32 * 0.5 * (self.x + ratio)).round() as u32,
            y: (h as f32 * 0.5 * (1.0 - self.y)).round() as u32,
        }
    }

    pub fn new(x: f32, y: f32) -> ScreenPosition {
        return ScreenPosition { x, y };
    }

    pub fn add(self, rect: ScreenRect) -> ScreenPosition {
        return ScreenPosition { x: self.x + rect.x, y: self.y + rect.y };
    }

    pub fn subtract(self, rect: ScreenRect) -> ScreenPosition {
        return ScreenPosition { x: self.x - rect.x, y: self.y - rect.y };
    }
}

/// Represents two sizes, where 1 is half the height of the screen (to be coherent with ScreenPosition)
#[derive(Clone, Copy, Debug)]
pub struct ScreenRect {
    pub x: f32,
    pub y: f32,
}

impl ScreenRect {
    pub fn to_pixel(self, gfx: &Graphics) -> (i32, i32) {
        return ScreenPosition { x: self.x, y: self.y }.to_pixel(gfx).difference_with(ScreenPosition { x: 0.0, y: 0.0 }.to_pixel(gfx));
    }

    pub fn to_pixel_f32(self, gfx: &Graphics) -> (f32, f32) {
        let (rx, ry) = self.to_pixel(gfx);
        return (rx as f32, ry as f32);
    }

    pub fn scale(self, factor: f32) -> ScreenRect {
        return ScreenRect { x: self.x * factor, y: self.x * factor };
    }
}

/// Pixel position:
/// Origin is top left, corresponds to pixels
///
/// ```
/// (0, 0)          (w, 0)
///    
///       (w/2, h/2)
///
/// (0, w)          (w, h)
/// ```
#[derive(Clone, Copy, Debug)]
pub struct PixelPosition {
    pub x: u32,
    pub y: u32,
}

impl PixelPosition {
    pub fn from_couple((x, y): (u32, u32)) -> PixelPosition {
        return PixelPosition { x, y };
    }

    pub fn as_couple(&self) -> (u32, u32) {
        return (self.x, self.y);
    }

    pub fn as_f32_couple(&self) -> (f32, f32) {
        return (self.x as f32, self.y as f32);
    }

    pub fn to_screen(self, gfx: &Graphics) -> ScreenPosition {
        let (w, h) = gfx.size();

        return ScreenPosition {
            x: (self.x as f32 / h as f32) * 2.0 - w as f32,
            y: (self.y as f32 / h as f32) * 2.0 - h as f32,
        }
    }

    /// Returns `self` - `other`
    pub fn difference_with(&self, other: PixelPosition) -> (i32, i32) {
        let dx = self.x as i32 - other.x as i32;
        let dy = self.y as i32 - other.y as i32;

        return (dx, dy);
    }

    /// difference_with, but parsed to `f32` for convenience
    pub fn difference_with_f32(&self, other: PixelPosition) -> (f32, f32) {
        let (dx, dy) = self.difference_with(other);
        return (dx as f32, dy as f32);
    }

    pub fn new(x: u32, y: u32) -> PixelPosition {
        return PixelPosition { x, y };
    }
}

