use std::sync::Arc;

use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Point3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;

        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }

    /// Initializes an empty HitRecord to be used (and populated) by other functions.
    pub fn empty() -> HitRecord {
        HitRecord {
            p: Vec3::from(0.0),
            normal: Vec3::from(0.0),
            t: 0.0,
            front_face: false,
            material: Arc::new(Lambertian { albedo: Vec3::from(0.0) }),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            match object.hit(r, t_min, closest_so_far) {
                Some(record) => {
                    closest_so_far = record.t;
                    temp = Option::from(record)
                }
                None => continue
            }
        }

        return temp;
    }
}