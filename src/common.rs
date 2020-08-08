use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_float_range(from: f64, to: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(from..to);

    die.sample(&mut rng)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {return min;}
    if x > max {return max;}
    return x;
}
