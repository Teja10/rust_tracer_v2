use crate::types::vec3::*;


// function takes in a 2d array as input
pub fn write_image(image: &Vec<Vec<Color>>) {
    let image_height = image.len();
    let image_width = image[0].len();
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("{}", 255);
    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j -1);
        for i in 0..image_width {
            image[j][i].write_color();
        }
    }
    eprintln!("Done.")
}
