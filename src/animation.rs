
/// Function used to animate things based on time.
/// They will create a smooth movement between 0 and 1, with a duration proportional to tau


pub fn ease_out_exp(time: f32, tau: f32) -> f32 {
    return 1.0 - f32::exp(-time / tau); 
}

/// Second order response
/// If c < -1, will anticipate movement
/// If c = -1, the derivative at 0 is null
/// If c = 1, is the same as ease_out_exp
/// If c > 1, overshoots
pub fn ease_out_exp_second(time: f32, tau: f32, c: f32) -> f32 {
    return 1.0 - c * f32::exp(-time / tau) + (c - 1.0) * f32::exp(-time / tau / 2.0);
}

