use std::ops;
use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
  e: Vec<f64>,
}

impl Vec3 {
  pub fn new_zero() -> Vec3 {
    Vec3 {
      e: vec![0.0, 0.0, 0.0],
    }
  }

  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { e: vec![x, y, z] }
  }

  pub fn x(&self) -> f64 {
    self.e[0]
  }
  pub fn y(&self) -> f64 {
    self.e[1]
  }
  pub fn z(&self) -> f64 {
    self.e[2]
  }

  pub fn length_squared(self) -> f64 {
    self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
  }

  pub fn length(self) -> f64 {
    self.length_squared().sqrt()
  }
}

impl ops::Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Self::Output {
    Vec3 {
      e: vec![-self.e[0], -self.e[1], -self.e[2]],
    }
  }
}

impl ops::Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, idx: usize) -> &Self::Output {
    &self.e[idx]
  }
}

impl ops::IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
    &mut self.e[idx]
  }
}

impl ops::Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      e: vec![
        self.e[0] + other.e[0],
        self.e[1] + other.e[1],
        self.e[2] + other.e[2],
      ],
    }
  }
}

impl ops::AddAssign for Vec3 {
  fn add_assign(&mut self, other: Self) {
    *self = Self {
      e: vec![
        self.e[0] + other.e[0],
        self.e[1] + other.e[1],
        self.e[2] + other.e[2],
      ],
    };
  }
}

impl ops::Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, other: f64) -> Self::Output {
    Self {
      e: vec![self.e[0] / other, self.e[1] / other, self.e[2] / other],
    }
  }
}

impl ops::DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, other: f64) {
      self.e[0] /= other;
      self.e[1] /= other;
      self.e[2] /= other;
  }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
