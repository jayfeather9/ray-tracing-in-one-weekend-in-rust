use std::fs;

mod color;
mod vec3;
use vec3::Vec3;

fn main() -> Result<(), std::io::Error> {
    // String
    let mut s = String::new();

    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut percentage = 0;
    s.push_str(&format!("P3\n{image_width} {image_height}\n255\n"));
    for j in 0..image_height {
        // Log
        if j as f64 / image_height as f64 * 100f64 > percentage as f64 {
            if percentage % 5 == 0 {
                println!("{}% finished", percentage);
            }
            percentage += 1;
        }
        for i in 0..image_width {
            let pixel_color = Vec3::new(
                i as f64 / image_width as f64,
                j as f64 / image_height as f64,
                0.25,
            );
            color::write_color(&mut s, pixel_color);
        }
    }
    // Log
    println!("100% finished");

    // Write
    fs::write("image.ppm", s)?;

    // Log
    println!("Image saved");

    Ok(())
}
