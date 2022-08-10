use crate::vector::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }
}