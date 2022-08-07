use std::io;
use std::io::Write;
use std::sync::Arc;

use crate::camera::Camera;
use crate::common::random_f64;
use crate::hittable::{HitRecord, HittableList};
use crate::material::{Lambertian, Metal};
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
    };
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut record = HitRecord::empty();

    // Gather no more light if the ray bounce limit has been exceeded.
    if depth <= 0 {
        return Color::from(0.0);
    }

    if world.hit(r, 0.001, f64::INFINITY, &mut record) {
        let mut scattered = Ray { dir: Vec3::from(0.0), origin: Vec3::from(0.0) };
        let mut attenuation = Color::from(0.0);

        // TODO: can we do this without cloning? maybe pass the reference somehow?
        let material = record.material.clone();

        if material.scatter(r, &record, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::from(0.0); // black
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
    let max_depth = 50;

    // World
    let material_ground = Arc::new(Lambertian { albedo: Color { x: 0.8, y: 0.8, z: 0.0 } });
    let material_center = Arc::new(Lambertian { albedo: Color { x: 0.7, y: 0.3, z: 0.3 } });
    let material_left = Arc::new(Metal { albedo: Color { x: 0.8, y: 0.8, z: 0.8 } });
    let material_right = Arc::new(Metal { albedo: Color { x: 0.8, y: 0.6, z: 0.2 } });

    let world = HittableList {
        objects: vec![
            Box::new(Sphere { center: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0, material: material_ground }),
            Box::new(Sphere { center: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5, material: material_center }),
            Box::new(Sphere { center: Vec3 { x: -1.0, y: 0.0, z: -1.0 }, radius: 0.5, material: material_left }),
            Box::new(Sphere { center: Vec3 { x: 1.0, y: 0.0, z: -1.0 }, radius: 0.5, material: material_right }),
        ]
    };

    // Camera
    let camera = Camera::new(aspect_ratio);
    let samples_per_pixel = 100;

    // Render
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    io::stdout().write_all(header.as_bytes()).expect("error getting bytes from header");

    for j in (0..image_height).rev() {
        eprintln!("printing line {} / {}", image_height - j, image_height);
        for i in 0..image_width {
            let mut color = Color::from(0.0);
            for _s in 0..samples_per_pixel - 1 {
                let u = (i as f64 + random_f64(0.0, 1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64(0.0, 1.0)) / (image_height - 1) as f64;

                let ray = &camera.get_ray(u, v);
                color = color + ray_color(&ray, &world, max_depth)
            }
            color::write_color(io::stdout(), color, samples_per_pixel);
        }
    }
}
