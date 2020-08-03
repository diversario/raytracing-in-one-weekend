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
      if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
      }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t;
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius*radius;
    let discriminant = b * b - 4.0*a*c;
    discriminant > 0.0
}
