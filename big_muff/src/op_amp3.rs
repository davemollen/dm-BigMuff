pub struct OpAmp3 {
  z: [f32; 2],
  z_domain_coefficients: ([f32; 3], [f32; 3]),
}

impl OpAmp3 {
  pub fn new(sample_rate: f32) -> Self {
    let s_domain_coefficients = (-813008.1300813, [1., 14210.344231102, 368043.51746551]);
    let z_domain_coefficients = Self::apply_bilinear_transform(s_domain_coefficients, sample_rate);

    Self {
      z: [0.; 2],
      z_domain_coefficients,
    }
  }

  pub fn reset(&mut self) {
    self.z = [0.; 2];
  }

  pub fn process(&mut self, input: f32) -> f32 {
    self.filter(input, self.z_domain_coefficients)
  }

  fn apply_bilinear_transform(
    (mut b1, mut a): (f32, [f32; 3]),
    sample_rate: f32,
  ) -> ([f32; 3], [f32; 3]) {
    let t = sample_rate.recip();
    let s = [t / 2., t * t / 4.];

    a[1] *= s[0];
    a[2] *= s[1];

    let a = [
      a[0] + a[1] + a[2],
      -2. * a[0] + 2. * a[2],
      a[0] - a[1] + a[2],
    ];

    b1 *= s[0];
    let b0 = b1 / a[0];

    ([b0, 0., -b0], [1., a[1] / a[0], a[2] / a[0]])
  }

  fn filter(&mut self, x: f32, (b, a): ([f32; 3], [f32; 3])) -> f32 {
    let y = x * b[0] + self.z[0];
    self.z[0] = x * b[1] - y * a[1] + self.z[1];
    self.z[1] = x * b[2] - y * a[2];

    y
  }
}
