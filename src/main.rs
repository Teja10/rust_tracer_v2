use crate::util::write_color;


mod util;
mod vec3;
mod vec_util;
mod ray;

fn ray_color(r: ray::Ray) -> vec3::Color {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: vec3::Point3 = vec3::Point3::new(0.0, 0.0, 0.0);
    let horizontal: vec3::Vec3 = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: vec3::Vec3 = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: vec3::Point3 = origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new(0.0, 0.0, focal_length);

    // println!("P3");
    // println!("{} {}", image_width, image_height);
    // println!("{}", image_height - 1);
    let mut image: Vec<Vec<vec3::Color>> = vec![];
    // create a 2d array of RGB pixels
    for j in (0..image_height).rev() {
        let mut row: Vec<vec3::Color> = vec![];
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u: f64 = i as f64 / (image_width - 1) as f64;
            let v: f64 = j as f64 / (image_height - 1) as f64;
            let r: ray::Ray = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color: vec3::Color = ray_color(r);
            // write_color(pixel_color);
            row.push(pixel_color);
        }
        image.push(row);
    }

    util::write_image(&image);
    
}