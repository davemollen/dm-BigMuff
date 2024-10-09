mod coefficients;
use coefficients::Coefficients;
use std::simd::{f32x8, num::SimdFloat};

const OVERSAMPLE_FACTOR: f32 = 8.;

pub struct FirFilter {
  buffer: Vec<f32x8>,
  coefficients: Vec<f32x8>,
  index: usize,
  mask: usize,
}

impl FirFilter {
  pub fn new() -> Self {
    let coefficients = Coefficients::new();
    let length = coefficients.len();
    debug_assert!(length.is_power_of_two());

    Self {
      buffer: vec![f32x8::splat(0.); length],
      coefficients,
      index: 0,
      mask: length - 1,
    }
  }

  pub fn upsample(&mut self, input: f32) -> f32x8 {
    self.write(f32x8::splat(input * OVERSAMPLE_FACTOR));
    (0..self.buffer.len())
      .map(|i| self.buffer[(self.index + self.buffer.len() - i) & self.mask] * self.coefficients[i])
      .sum()
  }

  pub fn downsample(&mut self, input: f32x8) -> f32 {
    self.write(input.reverse());
    (0..self.buffer.len())
      .map(|i| self.buffer[(self.index + self.buffer.len() - i) & self.mask] * self.coefficients[i])
      .sum::<f32x8>()
      .reduce_sum()
  }

  fn write(&mut self, input: f32x8) {
    self.index = self.index + 1 & self.mask;
    self.buffer[self.index] = input;
  }
}
