use crate::vector::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            origin: orig.clone(),
            dir: dir.clone(),
        }
    }

    fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }
}