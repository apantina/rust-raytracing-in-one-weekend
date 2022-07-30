use std::io;
use std::io::Write;

mod vector;
mod color;

fn main() {

    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    io::stdout().write_all(header.as_bytes()).expect("error getting bytes from header");

    let mut j = image_height - 1;
    while j >= 0 {
        let mut i = 0;
        while i < image_width {
            let rgb = vector::Color {
                x: i as f64 / (image_width - 1) as f64,
                y: j as f64 / (image_height - 1) as f64,
                z: 0.25,
            };
            color::write_color(io::stdout(), rgb);
            i += 1;
        }
        j -= 1;
    }
}
