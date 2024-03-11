use crate::{material::Material, ray::Ray, vec3::*};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, material: Material) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face: false,
            material,
        }
    }

    pub fn build(t: f64, r: &Ray, outward_normal: &Vec3, material: Material) -> Self {
        let front_face = r.dir.dot(*outward_normal) < 0.0;
        let normal = match front_face {
            true => *outward_normal,
            false => -*outward_normal,
        };

        HitRecord {
            p: r.at(t),
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(*outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}
