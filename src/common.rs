use rand::{thread_rng, Rng};

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}