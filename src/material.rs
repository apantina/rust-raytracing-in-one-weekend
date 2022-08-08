use crate::{Color, HitRecord, Ray, Vec3};


pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray { origin: hit_record.p, dir: scatter_direction };
        *attenuation = self.albedo;

        return true;
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(r_in.dir.unit_vector(), hit_record.normal);
        // keep fuzz parameter in bounds of [0.0, 1.0]
        *scattered = Ray {
            origin: hit_record.p,
            dir: reflected + self.fuzz.min(1.0).max(0.0) * Vec3::random_in_unit_sphere(),
        };
        *attenuation = self.albedo;

        return scattered.dir.dot(hit_record.normal) > 0.0;
    }
}
