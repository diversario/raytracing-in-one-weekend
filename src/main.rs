use std::fs;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;

mod vec3;

fn main() {
    let mut a = vec3::Point3::new(1.0, 2.0, 3.0);
    let mut b = vec3::Point3::new(0.1, 0.2, 0.3);
    println!("{:?}", a);
    a = -a;
    println!("{:?}", a);

    a[2] = 69.0;

    println!("{:?}", a + b);

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
        let r = x as f64/(width-1) as f64;
        let g = y as f64/(height-1) as f64;
        let b : f64 = 0.2;

        let ir = (255.99*r) as i64;
        let ig = (255.99*g) as i64;
        let ib = (255.99*b) as i64;

        writeln!(file, "{} {} {}", ir, ig, ib);
      };
    };

    println!();
}
