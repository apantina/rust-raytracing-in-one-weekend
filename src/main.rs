use std::io;
use std::io::Write;
use std::time::Instant;

use rayon::prelude::*;

use crate::camera::Camera;
use crate::color::color_to_rgb;
use crate::common::{random_f64, random_scene};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::{Color, Point3, Vec3};

mod vector;
mod color;
mod ray;
mod hittable;
mod sphere;
mod common;
mod camera;
mod material;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    // Gather no more light if the ray bounce limit has been exceeded.
    if depth <= 0 {
        return Color::from(0.0);
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        Some(record) => {
            let mut scattered = Ray { dir: Vec3::from(0.0), origin: Vec3::from(0.0) };
            let mut attenuation = Color::from(0.0);

            if record.material.scatter(r, &record, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }

            return Color::from(0.0); // black
        }
        None => Color::from(0.0)
    };

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    return (1.0 - t) * Color { x: 1.0, y: 1.0, z: 1.0 }
        + t * Color { x: 0.5, y: 0.7, z: 1.0 };
}

fn main() {

    // Image
    let aspect_ratio = 4.0 / 3.0;
    let image_width = 800 as usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let max_depth = 50;
    let samples_per_pixel = 64;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3 { x: 13.0, y: 2.0, z: 3.0 };
    let lookat = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    let camera = Camera::new(
        lookfrom,
        lookat,
        Point3 { x: 0.0, y: 1.0, z: 0.0 },
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    // Render
    let now = Instant::now();
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    io::stdout().write_all(header.as_bytes()).expect("error getting bytes from header");

// create image data structure, chunk into total of <image_width> rows/chunks
    let mut image = vec![0; (image_width * image_height * 3) as usize];
    let rows: Vec<(usize, &mut [u8])> = image
        .chunks_mut((image_width * 3 as usize) as usize).enumerate().collect();

    // process the rows in parallel
    rows.into_par_iter().for_each(|(j, row)| {
        for i in 0..image_width {
            let mut color = Color::from(0.0);
            for _s in 0..samples_per_pixel - 1 {
                let u = (i as f64 + random_f64(0.0, 1.0)) / (image_width - 1) as f64;
                let v = (image_height as f64 - (j as f64 + random_f64(0.0, 1.0))) / (image_height - 1) as f64;

                let ray = &camera.get_ray(u, v);
                color = color + ray_color(&ray, &world, max_depth)
            }
            let rgb = color_to_rgb(color, samples_per_pixel);
            row[i * 3] = rgb.0;
            row[i * 3 + 1] = rgb.1;
            row[i * 3 + 2] = rgb.2;
        }
    });

    let mut idx = 0;
    for _ in 0..image.len() / 3 {
        let row = format!(
            "{} {} {}\n",
            image[idx], image[idx + 1], image[idx + 2]);
        io::stdout().write_all(row.as_bytes()).expect("error getting bytes from row");
        idx = idx + 3;
    }

    let elapsed_time = now.elapsed();
    eprintln!("Rendering took {} ms.", elapsed_time.as_millis());
}

