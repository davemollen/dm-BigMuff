const C1: f32 = 1e-6;
const C2: f32 = 1e-7;
const C3: f32 = 1.2e-7;
const R1: f32 = 1200.;
const R2: f32 = 10000.;
const R4: f32 = 5600.;

pub struct Tone {
  s: [f32; 3],
  z: [f32; 3],
}

impl Tone {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4., t * t * t / 8.],
      z: [0.0; 3],
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let s_domain_coefficients = Self::get_s_domain_coefficients(tone);
    let z_domain_coefficients = self.apply_bilinear_transform(s_domain_coefficients);
    self.filter(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(tone: f32) -> ([f32; 3], [f32; 3]) {
    let r2_a = (1. - tone) * R2;
    let r2_b = tone * R2;

    let c1r1 = C1 * R1;
    let c1c2r1 = c1r1 * C2;
    let c3r4 = C3 * R4;
    let c3r1 = C3 * R1;
    let c1c3r4 = C1 * c3r4;
    let c1c2c3r1r4 = c1c2r1 * c3r4;
    let c1c2c3r1r4r2_a = c1c2c3r1r4 * r2_a;
    let b0 = c1c2c3r1r4 * r2_b;

    let b1 = c1c2r1 * R4 + c1c2r1 * r2_b + c1c2r1 * r2_a + r2_b * C2 * c3r1;
    let b2 = C1 * r2_a + c1r1 + C2 * R1;

    let a0 = b0 + c1c2c3r1r4r2_a;
    let a1 = b1 + c1c3r4 * r2_b + c1c3r4 * r2_a + c1c3r4 * R1 + C2 * c3r1 * r2_a;
    let a2 = b2 + C1 * R4 + C1 * r2_b + C3 * r2_b + C3 * r2_a + c3r1;

    ([b0, b1, b2], [a0, a1, a2])
  }

  fn apply_bilinear_transform(&self, (mut b, mut a): ([f32; 3], [f32; 3])) -> ([f32; 4], [f32; 4]) {
    b[1] *= self.s[0];
    b[2] *= self.s[1];

    let b0 = b[0] + b[1] + b[2];
    let b1 = -3. * b[0] - b[1] + b[2];
    let b2 = 3. * b[0] - b[1] - b[2];
    let b3 = -b[0] + b[1] - b[2];

    a[1] *= self.s[0];
    a[2] *= self.s[1];

    let a0 = a[0] + a[1] + a[2] + self.s[2];
    let a1 = -3. * a[0] - a[1] + a[2] + 3. * self.s[2];
    let a2 = 3. * a[0] - a[1] - a[2] + 3. * self.s[2];
    let a3 = -a[0] + a[1] - a[2] + self.s[2];

    (
      [b0 / a0, b1 / a0, b2 / a0, b3 / a0],
      [1., a1 / a0, a2 / a0, a3 / a0],
    )
  }

  fn filter(&mut self, x: f32, (b, a): ([f32; 4], [f32; 4])) -> f32 {
    let y = x * b[0] + self.z[0];
    self.z[0] = x * b[1] - y * a[1] + self.z[1];
    self.z[1] = x * b[2] - y * a[2] + self.z[2];
    self.z[2] = x * b[3] - y * a[3];

    y
  }
}
