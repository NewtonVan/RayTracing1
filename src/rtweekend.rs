#![allow(dead_code)]

use rand::Rng;
pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double() -> f32 {
    // Returns a random real in [0, 1)
    let mut rng = rand::thread_rng();
    rng.gen::<f32>() // Generates a random value in the range [0, 1)
}

pub fn random_double_in_range(min: f32, max: f32) -> f32 {
    // Returns a random real in [min, max)
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max) // Generates a random value in the range [min, max)
}
