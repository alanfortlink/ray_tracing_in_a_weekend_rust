use crate::{hit_record::HitRecord, material::Material, ray::Ray, vec3::Color};

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Metal {
            albedo: color,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.dir.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Color::random_unit_vector());
        let attenuation = self.albedo;

        if scattered.dir.dot(rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        }

        None
    }
}
