use crate::types::vec3::*;
use crate::types::ray::*;

// we use this struct to record when a ray intersects with any of our hittable objects
// A hit includes the point of intersection, the normal at the intersection
// the location of the intersection along the ray (t), and whether the ray hit the front or back
// of the object
#[derive(Clone, Copy)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool
}

pub trait HitRecordTraits {
    fn new() -> HitRecord;
    fn p(&self) -> Point3;
    fn normal(&self) -> Vec3;
    fn t(&self) -> f64;
    fn front_face(&self) -> bool;

    fn set_p(&mut self, p: Point3);
    fn set_t(&mut self, t: f64);

    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3);
}

impl HitRecordTraits for HitRecord {
    fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false
        }
    }

    fn p(&self) -> Point3 {
        self.p
    }

    fn normal(&self) -> Vec3 {
        self.normal
    }

    fn t(&self) -> f64 {
        self.t
    }

    fn front_face(&self) -> bool {
        self.front_face
    }

    fn set_p(&mut self, p: Point3) {
        self.p = p;
    }

    fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait HittableTraits {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}