use crate::vec3;


pub fn write_color(pixel_color: vec3::Color) {
    let ir = (255.999 * pixel_color.x()) as u32;
    let ig = (255.999 * pixel_color.y()) as u32;
    let ib = (255.999 * pixel_color.z()) as u32;
    println!("{} {} {}", ir, ig, ib);
}
// function takes in a 2d array as input
pub fn write_image(image: &Vec<Vec<vec3::Color>>) {
    let image_height = image.len();
    let image_width = image[0].len();
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("{}", 255);
    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j - 1);
        for i in 0..image_width {
            write_color(image[j][i]);
        }
    }
    eprintln!("Done.")
}
