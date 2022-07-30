use std::io;
use std::io::{Stdout, Write};
use crate::vector;

/// Write the translated [0,255] value of each color component to standard output.
pub(crate) fn write_color(mut out: Stdout, color: vector::Color) {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    let row = format!("{} {} {}\n", ir, ig, ib);
    out.write_all(row.as_bytes()).expect("error getting bytes from row");
}