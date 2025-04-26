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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

impl ScreenPosition {
    pub fn to_pixel(self, gfx: &Graphics) -> PixelPosition {
        let (w, h) = gfx.size();
        let ratio = w as f32 / h as f32;

        return PixelPosition {
            x: (h as f32 * 0.5 * (self.x + ratio)).round() as i32,
            y: (h as f32 * 0.5 * (1.0 - self.y)).round() as i32,
        }
    }

    pub fn new(x: f32, y: f32) -> ScreenPosition {
        return ScreenPosition { x, y };
    }

    pub fn add(self, rect: ScreenSize) -> ScreenPosition {
        return ScreenPosition { x: self.x + rect.x, y: self.y + rect.y };
    }

    pub fn subtract(self, rect: ScreenSize) -> ScreenPosition {
        return ScreenPosition { x: self.x - rect.x, y: self.y - rect.y };
    }

    pub fn difference_with(self, other: ScreenPosition) -> ScreenSize {
        return ScreenSize { x: self.x - other.x, y: self.y - other.y };
    }

    pub fn center() -> ScreenPosition {
        return ScreenPosition { x: 0.0, y: 0.0 };
    }
}

/// Represents two sizes, where 1 is half the height of the screen (to be coherent with ScreenPosition)
#[derive(Clone, Copy, Debug)]
pub struct ScreenSize {
    pub x: f32,
    pub y: f32,
}

impl ScreenSize {
    pub fn to_pixel(self, gfx: &Graphics) -> (i32, i32) {
        return ScreenPosition { x: self.x, y: self.y }.to_pixel(gfx).difference_with(ScreenPosition { x: 0.0, y: 0.0 }.to_pixel(gfx));
    }

    pub fn to_pixel_f32(self, gfx: &Graphics) -> (f32, f32) {
        let (rx, ry) = self.to_pixel(gfx);
        return (rx as f32, ry as f32);
    }

    pub fn scale(self, factor: f32) -> ScreenSize {
        return ScreenSize { x: self.x * factor, y: self.x * factor };
    }

    pub fn zero() -> ScreenSize {
        return ScreenSize { x: 0.0, y: 0.0 };
    }
}

/// Represents a rectangle on the screen, see ScreenPosition docs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenRect {
    pub bottom_left: ScreenPosition,
    pub top_right: ScreenPosition,
}

impl ScreenRect {
    /// Returns the smallest rect containing a and b
    pub fn merge(a: ScreenRect, b : ScreenRect) -> ScreenRect {
        let bl = ScreenPosition { x: f32::min(a.bottom_left.x, b.bottom_left.x), y: f32::min(a.bottom_left.y, b.bottom_left.y) };
        let tr = ScreenPosition { x: f32::max(a.top_right.x, b.top_right.x), y: f32::max(a.top_right.y, b.top_right.y) };

        return ScreenRect {
            bottom_left: bl, top_right: tr
        };
    }

    /// Special rect with infinities everywhere, sign of a very inelegant code...
    pub fn nothing() -> ScreenRect {
        return ScreenRect {
            bottom_left: ScreenPosition {
                x: f32::INFINITY,
                y: f32::INFINITY,
            },
            top_right: ScreenPosition {
                x: f32::NEG_INFINITY,
                y: f32::NEG_INFINITY,
            },
        }
    }

    pub fn center(self) -> ScreenPosition {
        return self.bottom_left.add(self.top_right.difference_with(self.bottom_left).scale(0.5));
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
    pub x: i32,
    pub y: i32,
}

impl PixelPosition {
    pub fn from_couple((x, y): (i32, i32)) -> PixelPosition {
        return PixelPosition { x, y };
    }

    pub fn as_couple(&self) -> (i32, i32) {
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
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        return (dx, dy);
    }

    /// difference_with, but parsed to `f32` for convenience
    pub fn difference_with_f32(&self, other: PixelPosition) -> (f32, f32) {
        let (dx, dy) = self.difference_with(other);
        return (dx as f32, dy as f32);
    }

    pub fn new(x: i32, y: i32) -> PixelPosition {
        return PixelPosition { x, y };
    }
}

