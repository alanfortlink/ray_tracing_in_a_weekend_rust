use crate::{hit_record::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    pub fn reflectance(coside: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - coside).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.dir.unit_vector();

        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random() {
            let reflected = unit_direction.reflect(&rec.normal);
            return Some((Ray::new(rec.p, reflected), attenuation));
        }

        let refracted = unit_direction.refract(&rec.normal, refraction_ratio);
        Some((Ray::new(rec.p, refracted), attenuation))
    }
}
