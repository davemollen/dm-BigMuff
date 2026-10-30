pub struct OpAmp1 {
  z: f32,
  z_domain_coefficients: ([f32; 2], [f32; 2]),
}

impl OpAmp1 {
  pub fn new(sample_rate: f32) -> Self {
    let s_domain_coefficients = (-5.8928571428571, [1., 119.04761904762]);
    let z_domain_coefficients = Self::apply_bilinear_transform(s_domain_coefficients, sample_rate);

    Self {
      z: 0.,
      z_domain_coefficients,
    }
  }

  pub fn reset(&mut self) {
    self.z = 0.;
  }

  pub fn process(&mut self, input: f32) -> f32 {
    self.filter(input, self.z_domain_coefficients)
  }

  fn filter(&mut self, x: f32, (b, a): ([f32; 2], [f32; 2])) -> f32 {
    let y = x * b[0] + self.z;
    self.z = x * b[1] - y * a[1];

    y
  }

  fn apply_bilinear_transform(
    (b0, mut a): (f32, [f32; 2]),
    sample_rate: f32,
  ) -> ([f32; 2], [f32; 2]) {
    let t = sample_rate.recip();
    let s = t / 2.;

    a[1] *= s;
    let a = [a[0] + a[1], a[0] - a[1]];

    ([b0 / a[0], b0 / a[0]], [1., a[1] / a[0]])
  }
}
