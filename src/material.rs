use crate::{Color, HitRecord, random_f64, Ray, Vec3};


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


pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Color { x: 1.0, y: 1.0, z: 1.0 };

        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.dir.unit_vector();

        let cos_theta = -unit_direction.dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = self.reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || reflectance > random_f64(0.0, 1.0) {
            // cannot refract
            Vec3::reflect(unit_direction, hit_record.normal)
        } else {
            Vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        *scattered = Ray { origin: hit_record.p, dir: direction };
        return true;
    }
}

impl Dielectric {
    /// Schlick's approximation for reflectance.
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}