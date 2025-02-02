use super::*;
use std::ops;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
  e: [f64; 3],
}

impl Vec3 {
  pub fn new_zero() -> Vec3 {
    Vec3 { e: [0.0, 0.0, 0.0] }
  }

  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { e: [x, y, z] }
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
    self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
  }

  pub fn length(self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
  }

  pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
      u.e[1] * v.e[2] - u.e[2] * v.e[1],
      u.e[2] * v.e[0] - u.e[0] * v.e[2],
      u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
  }

  pub fn unit_vector(v: Vec3) -> Vec3 {
    let len = v.length();
    v / len
  }

  pub fn random() -> Vec3 {
    Vec3::new(common::random_float(), random_float(), random_float())
  }

  pub fn random_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
      common::random_float_range(min, max),
      random_float_range(min, max),
      random_float_range(min, max),
    )
  }

  pub fn random_unit_vector() -> Vec3 {
    let a = random_float_range(0.0, 2.0 * std::f64::consts::PI);
    let z = random_float_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    return Vec3::new(r * a.cos(), r * a.sin(), z);
  }
}

impl ops::Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Self::Output {
    Vec3 {
      e: [-self.e[0], -self.e[1], -self.e[2]],
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
      e: [
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
      e: [
        self.e[0] + other.e[0],
        self.e[1] + other.e[1],
        self.e[2] + other.e[2],
      ],
    };
  }
}

impl ops::Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      e: [
        self.e[0] - other.e[0],
        self.e[1] - other.e[1],
        self.e[2] - other.e[2],
      ],
    }
  }
}

impl ops::SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Self) {
    *self = Self {
      e: [
        self.e[0] - other.e[0],
        self.e[1] - other.e[1],
        self.e[2] - other.e[2],
      ],
    };
  }
}

impl ops::Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, other: f64) -> Self::Output {
    Self {
      e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
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

impl ops::Mul<f64> for Vec3 {
  type Output = Self;

  fn mul(self, other: f64) -> Self::Output {
    Self {
      e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
    }
  }
}

impl ops::MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, other: f64) {
    self.e[0] *= other;
    self.e[1] *= other;
    self.e[2] *= other;
  }
}
