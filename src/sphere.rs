use crate::{interval::Interval, material::Material, vec3::Point3};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &Interval,
    ) -> Option<crate::hit_record::HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);

        let outward_normal = (p - self.center) / self.radius;
        Some(crate::hit_record::HitRecord::build(
            root,
            r,
            &outward_normal,
            self.material.clone(),
        ))
    }
}
