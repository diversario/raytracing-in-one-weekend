use crate::point3::Point3;
use crate::vec3::Vec3;
use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
  orig: Point3,
  dir: Vec3,
}

impl Ray {
  pub fn new(origin :Point3, direction: Vec3) -> Ray {
    Ray {
      orig: origin,
      dir: direction,
    }
  }

  pub fn origin(self) -> Point3 {
    self.orig
  }

  pub fn direction(self) -> Vec3 {
    self.dir
  }

  pub fn at(self, t: f64) -> Point3 {
    self.orig + self.dir * t
  }
}

pub fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t;
}
