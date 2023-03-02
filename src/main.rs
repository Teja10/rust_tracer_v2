mod util;
mod types;

use types::vec3::*;
use types::ray::*;

// check if a ray hits a sphere with specified center and radius
fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    // compute the discriminant of the quadratic equation
    let oc: Vec3 = r.origin() - center;
    let a = r.direction().length_squared();
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    // if the discriminant is negative, the ray misses the sphere
    // else there is an intersection and can evaluate the quadratic equation for t
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}


fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
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
            let pixel_color: Color = ray_color(r);
            row.push(pixel_color);
        }
        image.push(row);
    }
    util::write_image(&image);
}