use std::io::{Stdout, Write};

use crate::vector;

/// Write the translated [0,255] value of each color component to standard output.
pub(crate) fn write_color(mut out: Stdout, color: vector::Color, samples_per_pixel: usize) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    // Divide each color component by the number of samples. Also gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;

    let r_scaled = (r * scale).sqrt();
    let g_scaled = (g * scale).sqrt();
    let b_scaled = (b * scale).sqrt();

    // Write out the translated [0, 255] value of each color component.
    let row = format!("{} {} {}\n",
                      (256.0 * (r_scaled.max(0.0).min(1.0))) as i32,
                      (256.0 * (g_scaled.max(0.0).min(1.0))) as i32,
                      (256.0 * (b_scaled.max(0.0).min(1.0))) as i32
    );
    out.write_all(row.as_bytes()).expect("error getting bytes from row");
}
