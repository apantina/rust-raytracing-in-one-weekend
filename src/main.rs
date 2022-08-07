use std::io;
use std::io::Write;

use rand::random;

use crate::camera::Camera;
use crate::common::random_f64;
use crate::hittable::{HitRecord, HittableList};
use crate::sphere::Sphere;
use crate::vector::{Color, Point3, Vec3};

mod vector;
mod color;
mod ray;
mod hittable;
mod sphere;
mod common;
mod camera;

fn hit_sphere(center: Point3, radius: f64, ray: &ray::Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.dir.length_squared();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;

    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(r: &ray::Ray, world: &HittableList) -> Color {
    let mut record = HitRecord::empty();

    if world.hit(r, 0.0, f64::INFINITY, &mut record) {
        return 0.5 * (record.normal + Color { x: 1.0, y: 1.0, z: 1.0 });
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    return (1.0 - t) * Color { x: 1.0, y: 1.0, z: 1.0 }
        + t * Color { x: 0.5, y: 0.7, z: 1.0 };
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let world = HittableList {
        objects: vec![
            Box::new(Sphere { center: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 }),
            Box::new(Sphere { center: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 }),
        ]
    };

    // Camera
    let camera = Camera::new(aspect_ratio);
    let samples_per_pixel = 10;

    // Render
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    io::stdout().write_all(header.as_bytes()).expect("error getting bytes from header");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Color::from(0.0);
            for _s in 0..samples_per_pixel - 1 {
                let u = (i as f64 + random_f64(0.0, 1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64(0.0, 1.0)) / (image_height - 1) as f64;

                let ray = &camera.get_ray(u, v);
                color = color + ray_color(&ray, &world)
            }
            color::write_color(io::stdout(), color, samples_per_pixel);
        }
    }
}
