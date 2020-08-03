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
use ray::Ray;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = (image_width / aspect_ratio) as i64 as f64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut file : fs::File;

    match OpenOptions::new().write(true).create(true).open("./img.ppm") {
      Ok(f) => file = f,
      Err(error) => panic!(error.to_string()),
    };

    writeln!(file, "P3\n{} {}\n255\n", image_width, image_height);

    for y in (0..image_height as i64).rev() {
      print!("\rRendered {}%", (((image_height - y as f64) as f64/ image_height as f64) * 100.0) as i64); io::stdout().flush().unwrap();

      for x in 0..image_width as i64 {
        let u = x as f64/(image_width-1.0) as f64;
        let v = y as f64/(image_height-1.0) as f64;

        let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
        let pixel_color = ray::ray_color(&ray);

        writeln!(file, "{}\n", color::write_color(pixel_color));
      };
    };

    println!();
}
