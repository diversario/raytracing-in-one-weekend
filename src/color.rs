use crate::vec3;

pub type Color = vec3::Vec3;

pub fn write_color(pixel_color: Color) -> String {
  let ir = (255.99 * pixel_color.x()) as i64;
  let ig = (255.99 * pixel_color.y()) as i64;
  let ib = (255.99 * pixel_color.z()) as i64;

  format!("{} {} {}", ir, ig, ib)
}

