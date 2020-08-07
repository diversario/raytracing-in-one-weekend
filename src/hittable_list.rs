use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use std::vec::Vec;

// #[derive(Debug, Clone, PartialEq)]
pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
  pub fn new() -> HittableList {
    return HittableList { objects: vec![] };
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }

  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
    let mut temp_rec: HitRecord = Default::default();

    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for object in &self.objects {
      if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        // temp_rec.t = 0.0;
        // temp_rec.front_face = true;
        // temp_rec.normal = crate::vec3::Vec3::new(0.0, 0.0, 0.0);
        // temp_rec.p = crate::vec3::Vec3::new(0.0, 0.0, 0.0);
        *hit_record = temp_rec;
      }
    }

    return hit_anything;
  }
}
