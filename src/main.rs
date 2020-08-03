use std::fs;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;

mod vec3;
mod color;
mod point3;
mod ray;

use vec3::Vec3;
use point3::Point3;
use color::Color;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = (image_width / aspect_ratio) as i64;

        // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3(0, viewport_height, 0);
    let lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);


    let width = 256;
    let height = 256;

    let mut file : fs::File;

    match OpenOptions::new().write(true).create(true).open("./img.ppm") {
      Ok(f) => file = f,
      Err(error) => panic!(error.to_string()),
    };

    writeln!(file, "P3\n{} {}\n255\n", width, height);

    for y in (0..height).rev() {
      print!("\rRendered {}%", (((height - y) as f64/ height as f64) * 100.0) as i64); io::stdout().flush().unwrap();

      for x in 0..width {
        let color = color::Color::new(
          x as f64/(width-1) as f64,
          y as f64/(height-1) as f64,
          0.6,
        );

        writeln!(file, "{}\n", color::write_color(color));
      };
    };

    println!();
}
