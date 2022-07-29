use std::io;
use std::io::Write;

fn main() {

    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    io::stdout().write_all(header.as_bytes()).expect("error getting bytes from header");

    let mut j = image_height - 1;
    let mut write_cnt = 0;
    while j >= 0 {
        let mut i = 0;
        while i < image_width {
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let row = format!("{} {} {}\n", ir, ig, ib);

            io::stdout().write_all(row.as_bytes()).expect("error getting bytes from header");
            write_cnt += 1;
            i += 1;
        }
        j -= 1;
    }
}
