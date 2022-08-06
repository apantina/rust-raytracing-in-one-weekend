use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(mut self, ray: &Ray, outward_normal: Point3) {
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
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp = HitRecord::empty();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp) {
                hit_anything = true;
                closest_so_far = temp.t;
                *hit_record = temp;
            }
        }

        return hit_anything;
    }
}