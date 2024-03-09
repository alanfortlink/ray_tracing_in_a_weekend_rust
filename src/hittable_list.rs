use crate::{hit_record::HitRecord, hittable::Hittable, interval::Interval, ray::Ray};
use std::rc::Rc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.objects.push(hittable);
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            let hr = object.hit(r, &Interval::new(ray_t.min, closest_so_far));
            if let Some(_) = hr {
                hit_record = hr;
                closest_so_far = hit_record.as_ref().unwrap().t.clone();
            }
        }

        hit_record
    }
}
