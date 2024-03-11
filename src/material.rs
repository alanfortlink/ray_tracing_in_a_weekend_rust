use crate::{
    hit_record::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Clone)]
enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Clone)]
pub struct Material {
    pub albedo: Color,
    pub fuzz: f64,
    pub ir: f64,
    pub material_type: MaterialType,
}

impl Material {
    pub fn new_lambertian(color: Color) -> Self {
        Material {
            albedo: color,
            fuzz: 0.0,
            ir: 0.0,
            material_type: MaterialType::Lambertian,
        }
    }

    pub fn new_metal(color: Color, fuzz: f64) -> Self {
        Material {
            albedo: color,
            fuzz,
            ir: 0.0,
            material_type: MaterialType::Metal,
        }
    }

    pub fn new_dielectric(ir: f64) -> Self {
        Material {
            albedo: Color::new(1.0, 1.0, 1.0),
            fuzz: 0.0,
            ir,
            material_type: MaterialType::Dielectric,
        }
    }

    fn scatter_lambertian(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo.clone();

        Some((scattered, attenuation))
    }

    fn scatter_metal(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.dir.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Color::random_unit_vector());
        let attenuation = self.albedo;

        if scattered.dir.dot(rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        }

        None
    }

    fn scatter_dielectric(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
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

        if cannot_refract
            || Material::dielectric_reflectance(cos_theta, refraction_ratio) > rand::random()
        {
            let reflected = unit_direction.reflect(&rec.normal);
            return Some((Ray::new(rec.p, reflected), attenuation));
        }

        let refracted = unit_direction.refract(&rec.normal, refraction_ratio);
        Some((Ray::new(rec.p, refracted), attenuation))
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self.material_type {
            MaterialType::Lambertian => self.scatter_lambertian(r_in, rec),
            MaterialType::Metal => self.scatter_metal(r_in, rec),
            MaterialType::Dielectric => self.scatter_dielectric(r_in, rec),
        }
    }

    fn dielectric_reflectance(coside: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - coside).powi(5)
    }
}
