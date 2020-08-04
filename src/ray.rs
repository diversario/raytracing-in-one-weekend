use crate::color::Color;
use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
  orig: Point3,
  dir: Vec3,
}

impl Ray {
  pub fn new(origin: Point3, direction: Vec3) -> Ray {
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
  let mut t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);

  if t > 0.0 {
    let normal = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
    return Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
  }

  let unit_direction = Vec3::unit_vector(r.direction());

  t = 0.5 * (unit_direction.y() + 1.0);

  return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
  let oc = r.origin() - *center;
  let a = r.direction().length_squared();
  let half_b = Vec3::dot(oc, r.direction());
  let c = oc.length_squared() - radius * radius;
  let discriminant = half_b * half_b - a * c;
  if discriminant < 0.0 {
    -1.0
  } else {
    (-half_b - discriminant.sqrt()) / a
  }
}
