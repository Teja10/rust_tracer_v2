mod util;
mod types;
mod hittable;

use std::rc::Rc;
use types::vec3::*;
use types::ray::*;

use hittable::hittable::*;
use hittable::hittable_list::*;
use hittable::sphere::*;

fn ray_color(r: Ray, world: &HittableList) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal() + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + 
        t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 512;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let mut world: HittableList = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Point3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    
    let mut image: Vec<Vec<Color>> = vec![];
    // create a 2d array of RGB pixels
    for j in (0..image_height).rev() {
        let mut row: Vec<Color> = vec![];
        for i in 0..image_width {
            let u: f64 = i as f64 / (image_width - 1) as f64;
            let v: f64 = j as f64 / (image_height - 1) as f64;
            let r: Ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color: Color = ray_color(r, &world);
            row.push(pixel_color);
        }
        image.push(row);
    }
    util::write_image(&image);
}