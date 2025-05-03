
/// Returns a * (1-t) + b * t
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a * (1.0 - t) + b * t
}

/// Returns a * (1-t) + b * t
/// TODO: make this interpolate hsv instead of rgb to prevent surprising interpolations between different hues 
pub fn color_lerp(a: notan::app::Color, b: notan::app::Color, t: f32) -> notan::app::Color {
    return notan::app::Color::new(
        a.r * (1.0 - t) + b.r * t, 
        a.g * (1.0 - t) + b.g * t, 
        a.b * (1.0 - t) + b.b * t, 
        a.a * (1.0 - t) + b.a * t, 
    );
}

