use crate::{
    hit_record::HitRecord,
    material::Material,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Self {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo.clone();

        Some((scattered, attenuation))
    }
}
