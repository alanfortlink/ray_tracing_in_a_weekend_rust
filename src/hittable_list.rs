use crate::{hit_record::HitRecord, interval::Interval, ray::Ray, sphere::Sphere};

#[derive(Clone)]
pub struct HittableList {
    pub spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            spheres: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in self.spheres.iter() {
            let hr = object.hit(r, &Interval::new(ray_t.min, closest_so_far));
            if let Some(_) = hr {
                hit_record = hr;
                closest_so_far = hit_record.as_ref().unwrap().t.clone();
            }
        }

        hit_record
    }
}
