pub struct OpAmp1 {
  s: f32,
  z: f32,
}

impl OpAmp1 {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self { s: t / 2., z: 0. }
  }

  pub fn process(&mut self, input: f32, s_domain_coefficients: (f32, [f32; 2])) -> f32 {
    let z_domain_coefficients = self.apply_bilinear_transform(s_domain_coefficients);
    self.filter(input, z_domain_coefficients)
  }

  fn apply_bilinear_transform(&self, (b0, mut a): (f32, [f32; 2])) -> ([f32; 2], [f32; 2]) {
    a[1] *= self.s;
    let a = [a[0] + a[1], a[0] - a[1]];

    ([b0 / a[0], b0 / a[0]], [1., a[1] / a[0]])
  }

  fn filter(&mut self, x: f32, (b, a): ([f32; 2], [f32; 2])) -> f32 {
    let y = x * b[0] + self.z;
    self.z = x * b[1] - y * a[1];

    y
  }
}
