pub struct OpAmp3 {
  s: [f32; 2],
  z: [f32; 2],
}

impl OpAmp3 {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4.],
      z: [0.; 2],
    }
  }

  pub fn process(&mut self, input: f32, s_domain_coefficients: (f32, [f32; 3])) -> f32 {
    let z_domain_coefficients = self.apply_bilinear_transform(s_domain_coefficients);
    self.filter(input, z_domain_coefficients)
  }

  fn apply_bilinear_transform(&self, (mut b1, mut a): (f32, [f32; 3])) -> ([f32; 3], [f32; 3]) {
    a[1] *= self.s[0];
    a[2] *= self.s[1];

    let a = [
      a[0] + a[1] + a[2],
      -2. * a[0] + 2. * a[2],
      a[0] - a[1] + a[2],
    ];

    b1 *= self.s[0];
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
