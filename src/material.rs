use crate::{
    color::Color,
    geometry::{Ray, Vec3},
    hittable::HitRecord,
    util::rand_f64,
};

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = rec.normal + Vec3::rand_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            orig: rec.p,
            dir: scatter_direction,
        };
        let attenuation = self.albedo;

        Some(ScatterResult {
            attenuation,
            scattered,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut reflected = Vec3::reflect(r_in.dir, rec.normal);
        reflected =
            Vec3::unit_vector(reflected) + (f64::min(self.fuzz, 1.0) * Vec3::rand_unit_vector());
        let scattered = Ray {
            orig: rec.p,
            dir: reflected,
        };
        let attenuation = self.albedo;

        if Vec3::dot(scattered.dir, rec.normal) > 0.0 {
            return Some(ScatterResult {
                attenuation,
                scattered,
            });
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = Vec3::unit_vector(r_in.dir);
        let cos_theta = f64::min(Vec3::dot(-unit_dir, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Self::schlick_reflectance(cos_theta, ri) > rand_f64() {
            direction = Vec3::reflect(unit_dir, rec.normal)
        } else {
            direction = Vec3::refract(unit_dir, rec.normal, ri);
        }

        let scattered = Ray {
            orig: rec.p,
            dir: direction,
        };

        Some(ScatterResult {
            attenuation,
            scattered,
        })
    }
}

impl Dielectric {
    fn schlick_reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
