use crate::types::ray::*;
use crate::hittable::hittable::*;


use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn HittableTraits>>
}

pub trait HittableListTraits {
    fn new() -> HittableList;
    fn clear(&mut self);
    fn add(&mut self, object: Rc<dyn HittableTraits>);
}

impl HittableListTraits for HittableList {
    fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, object: Rc<dyn HittableTraits>) {
        self.objects.push(object);
    }
}

impl HittableTraits for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        // iterate through all hittables and check for hits
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}