use crate::types::vec3::*;
use crate::types::ray::*;
use crate::hittable::hittable::*;


pub struct Sphere {
    center: Point3,
    radius: f64
}

pub trait SphereTraits {
    fn new(center: Point3, radius: f64) -> Sphere;
    fn center(&self) -> Point3;
    fn radius(&self) -> f64;
}

impl SphereTraits for Sphere {
    fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    fn center(&self) -> Point3 {
        self.center
    }

    fn radius(&self) -> f64 {
        self.radius
    }
}

impl HittableTraits for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center();
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius() * self.radius();
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.set_t(root);
        rec.set_p(r.at(rec.t()));
        let outward_normal = (rec.p() - self.center()) / self.radius();
        rec.set_face_normal(r, outward_normal);

        true
    }
}