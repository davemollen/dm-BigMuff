use std::f32::consts::TAU;

pub struct OnePoleFilter {
  z: f32,
  b1: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32, freq: f32) -> Self {
    Self {
      z: 0.,
      b1: (freq * sample_rate.recip() * -TAU).exp(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let a0 = 1.0 - self.b1;
    self.z = input * a0 + self.z * self.b1;
    self.z
  }
}
