use super::*;

#[derive(Default)]
pub struct Camera {
  pub origin: Point3,
  pub lower_left_corner: Point3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let mut c = Camera {
      origin: Point3::new(0.0, 0.0, 0.0),
      horizontal: Vec3::new(viewport_width, 0.0, 0.0),
      vertical: Vec3::new(0.0, viewport_height, 0.0),
      lower_left_corner: Vec3::new_zero(),
    };

    c.lower_left_corner = c.origin - c.horizontal / 2.0 - c.vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    return c;
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
    )
  }
}
