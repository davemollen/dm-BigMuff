mod fir_filter;
use {
  fir_filter::FirFilter,
  std::simd::{f32x8, num::SimdFloat, StdFloat},
};

const OVERSAMPLE_FACTOR: f32 = 8.;

pub struct Clipper {
  upsample_fir: FirFilter,
  downsample_fir: FirFilter,
}

impl Clipper {
  pub fn new() -> Self {
    Self {
      upsample_fir: FirFilter::new(),
      downsample_fir: FirFilter::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let upsampled = self
      .upsample_fir
      .process(f32x8::splat(offset * OVERSAMPLE_FACTOR));
    let clipped = Self::clip(upsampled);
    self.downsample_fir.process(clipped).reduce_sum() * 0.9731
  }

  fn clip(x: f32x8) -> f32x8 {
    let x_abs = x.abs();
    let x2 = x_abs * x_abs;
    let x6 = x2 * x2 * x2;
    let a = f32x8::splat(1.) + x6;

    x / a.sqrt().sqrt().sqrt()
  }
}
