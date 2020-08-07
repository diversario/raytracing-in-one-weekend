use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
  center: Point3,
  radius: f64,
}

impl Sphere {
  pub fn new(ctr: Point3, r: f64) -> Sphere {
    Sphere {
      center: ctr,
      radius: r,
    }
  }

  pub fn new_boxed(ctr: Point3, r: f64) -> Box<Sphere> {
    Box::new(Sphere {
      center: ctr,
      radius: r,
    })
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
    let oc = r.origin() - self.center;
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(oc, r.direction());
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant > 0.0 {
      let root = discriminant.sqrt();

      let temp = (-half_b - root) / a;

      if temp < t_max && temp > t_min {
        hit_record.t = temp;
        hit_record.p = r.at(hit_record.t);
        // hit_record.normal = (hit_record.p - self.center) / self.radius;

        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, outward_normal);

        return true;
      }

      let temp = (-half_b + root) / a;

      if temp < t_max && temp > t_min {
        hit_record.t = temp;
        hit_record.p = r.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        return true;
      }
    }

    return false;
  }
}
