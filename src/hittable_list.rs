use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use std::vec::Vec;

// #[derive(Debug, Clone, PartialEq)]
pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
  pub fn new(object: Box<dyn Hittable>) -> HittableList {
    return HittableList {
      objects: vec![object],
    };
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
            *hit_record = temp_rec;
        }
    }

    return hit_anything;
  }
}

