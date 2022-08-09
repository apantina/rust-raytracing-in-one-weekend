use std::sync::Arc;

use rand::{Rng, thread_rng};

use crate::{Color, Dielectric, Hittable, HittableList, Lambertian, Metal, Point3, Sphere};

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}

pub fn random_scene() -> HittableList {
    let mut world_objects: Vec<Arc<dyn Hittable + Send + Sync>> = Vec::new();

    let material_ground = Arc::new(Lambertian { albedo: Color { x: 0.5, y: 0.5, z: 0.5 } });
    world_objects.push(Arc::new(Sphere { center: Point3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: material_ground }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64(0.0, 1.0);
            let center = Point3 { x: a as f64 + 0.9 * random_f64(0.0, 1.0), y: 0.2, z: 0.9 * random_f64(0.0, 1.0) + b as f64 };

            if (center - Point3 { x: 4.0, y: 0.2, z: 0.0 }).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_unit_vector() * Color::random_unit_vector();
                    let sphere_material = Arc::new(Lambertian { albedo });
                    world_objects.push(Arc::new(Sphere { center, radius: 0.2, material: sphere_material }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64(0.0, 0.5);
                    let sphere_material = Arc::new(Metal { albedo, fuzz });
                    world_objects.push(Arc::new(
                        Sphere { center, radius: 0.2, material: sphere_material }
                    ));
                } else {
                    // glass
                    let sphere_material = Arc::new(
                        Dielectric { refraction_index: 1.5 }
                    );
                    world_objects.push(Arc::new(
                        Sphere { center, radius: 0.2, material: sphere_material }
                    ));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric { refraction_index: 1.5 });
    world_objects.push(Arc::new(Sphere { center: Point3 { x: 0.0, y: 1.0, z: 0.0 }, radius: 1.0, material: material1 }));

    let material2 = Arc::new(Lambertian { albedo: Color { x: 0.4, y: 0.2, z: 0.1 } });
    world_objects.push(Arc::new(Sphere { center: Point3 { x: -4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: material2 }));

    let material3 = Arc::new(Metal { albedo: Color { x: 0.7, y: 0.6, z: 0.5 }, fuzz: 0.0 });
    world_objects.push(Arc::new(Sphere { center: Point3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: material3 }));

    return HittableList { objects: world_objects };
}