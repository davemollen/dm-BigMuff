const C1: f32 = 1e-5;
const C2: f32 = 1e-8;
const R1: f32 = 560000.;
const R2: f32 = 62000.;
const R3: f32 = 10000.;
const R5: f32 = 47.;
const R6: f32 = 47000.;

pub struct OpAmp2 {
  s: [f32; 2],
  z: [f32; 2],
}

impl OpAmp2 {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4.],
      z: [0.; 2],
    }
  }

  pub fn process(&mut self, input: f32, sustain: f32) -> f32 {
    let s_domain_coefficients = Self::get_s_domain_coefficients(sustain);
    let z_domain_coefficients = self.apply_bilinear_transform(s_domain_coefficients);
    self.filter(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(sustain: f32) -> ([f32; 3], [f32; 3]) {
    let r3_a = (1. - sustain) * R3;
    let r3_b = sustain * R3;

    let c1r1 = C1 * R1;
    let c1r2 = C1 * R2;
    let c2r6 = C2 * R6;
    let c1c2r2r6 = c1r2 * c2r6;

    let b1_a1 = c1r2 * R5 + c1r2 * r3_b;
    let b1 = b1_a1 + c1r1 * R5 + c1r1 * r3_b;
    let b2 = R5 + r3_b + r3_a + R2 + R1;
    let a0 = c1c2r2r6 * R5 + c1c2r2r6 * r3_b + c1c2r2r6 * r3_a;
    let a1 = b1_a1 + c2r6 * R5 + c2r6 * r3_b + c2r6 * r3_a + c2r6 * R2 + c2r6 * R1 + c1r2 * r3_a;

    ([0., b1, b2], [a0, a1, b2])
  }

  fn apply_bilinear_transform(&self, (mut b, mut a): ([f32; 3], [f32; 3])) -> ([f32; 3], [f32; 3]) {
    b[1] *= self.s[0];
    b[2] *= self.s[1];
    let b = [b[1] + b[2], 2. * b[2], b[1] + b[2]];

    a[1] *= self.s[0];
    a[2] *= self.s[1];
    let a = [
      a[0] + a[1] + a[2],
      -2. * a[0] + 2. * a[2],
      a[0] - a[1] + a[2],
    ];

    (b.map(|x| x / a[0]), a.map(|x| x / a[0]))
  }

  fn filter(&mut self, x: f32, (b, a): ([f32; 3], [f32; 3])) -> f32 {
    let y = x * b[0] + self.z[0];
    self.z[0] = x * b[1] - y * a[1] + self.z[1];
    self.z[1] = x * b[2] - y * a[2];

    y
  }
}
