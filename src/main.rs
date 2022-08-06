use std::io;
use std::io::Write;

use crate::hittable::{HitRecord, HittableList};
use crate::sphere::Sphere;
use crate::vector::{Color, Point3, Vec3};

mod vector;
mod color;
mod ray;
mod hittable;
mod sphere;
mod common;

fn hit_sphere(center: Point3, radius: f64, ray: &ray::Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.dir.length_squared();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - f64::sqrt(discriminant)) / a;
    }
}

fn ray_color(r: &ray::Ray, world: &mut HittableList) -> Color {
    let mut record = HitRecord::empty();

    if world.hit(r, 0.0, f64::INFINITY, &mut record) {
        return 0.5 * (record.normal + Color { x: 1.0, y: 1.0, z: 1.0 });
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    return (1.0 - t) * vector::Color { x: 1.0, y: 1.0, z: 1.0 }
        + t * vector::Color { x: 0.5, y: 0.7, z: 1.0 };
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world = HittableList {
        objects: vec![
            Box::new(Sphere { center: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 }),
            Box::new(Sphere { center: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 }),
        ]
    };

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vector::Point3 { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = vector::Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = vector::Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0
        - vector::Vec3 { x: 0.0, y: 0.0, z: focal_length };


    // Render
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    io::stdout().write_all(header.as_bytes()).expect("error getting bytes from header");

    let mut j = image_height - 1;
    while j >= 0 {
        let mut i = 0;
        while i < image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = ray::Ray {
                origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            color::write_color(io::stdout(), ray_color(&ray, &mut world));
            i += 1;
        }
        j -= 1;
    }
}
