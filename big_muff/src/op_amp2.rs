const R1: f32 = 10000.;
const R2: f32 = 47000.;
const R3: f32 = 560000.;
const R4: f32 = 62000.;
const R5: f32 = 47.;
const R_SUS: f32 = 10000.;
const C1: f32 = 4.7e-9;
const C2: f32 = 1e-8;
const C3: f32 = 1e-5;

pub struct OpAmp2 {
  s: [f32; 3],
  z: [f32; 3],
}

impl OpAmp2 {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4., t * t * t / 8.],
      z: [0.; 3],
    }
  }

  pub fn process(&mut self, input: f32, sustain: f32) -> f32 {
    let s_domain_coefficients = Self::get_s_domain_coefficients(sustain);
    let z_domain_coefficients = self.apply_bilinear_transform(s_domain_coefficients);
    self.filter(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(sustain: f32) -> ([f32; 4], [f32; 4]) {
    let r_sus_a = sustain * R_SUS;
    let r_sus_b = (1. - sustain) * R_SUS;

    let c3r4 = C3 * R4;
    let c3r3 = C3 * R3;
    let c2r2 = C2 * R2;
    let c2r1 = C2 * R1;
    let c1c2r1r2 = C1 * c2r1 * R2;
    let c1c2c3r1r2r4 = c1c2r1r2 * c3r4;
    let c1c3r1r3 = C1 * R1 * c3r3;
    let c2c3r1r4 = c2r1 * c3r4;
    let c2c3r2r4 = c2r2 * c3r4;

    let b0 = c3r4 * R5 + c3r4 * r_sus_a + c3r3 * R5 + c3r3 * r_sus_a;
    let b1 = R5 + r_sus_a + r_sus_b + R4 + R3;

    let a0 = c1c2c3r1r2r4 * R5 + c1c2c3r1r2r4 * r_sus_a + c1c2c3r1r2r4 * r_sus_b;
    let a1 = c1c2r1r2 * R5
      + c1c2r1r2 * r_sus_a
      + c1c2r1r2 * r_sus_b
      + c1c2r1r2 * R4
      + c2c3r1r4 * R5
      + c2c3r2r4 * R5
      + r_sus_a * c2c3r1r4
      + r_sus_a * c2c3r2r4
      + c2c3r1r4 * r_sus_b
      + c2c3r2r4 * r_sus_b
      + c1c2r1r2 * R3
      - c1c3r1r3 * R5
      - c1c3r1r3 * r_sus_a
      - c1c3r1r3 * r_sus_b;
    let a2 = c2r1 * R5
      + c2r2 * R5
      + c2r1 * r_sus_a
      + c2r2 * r_sus_a
      + c2r1 * r_sus_b
      + c2r2 * r_sus_b
      + c2r1 * R4
      + c2r2 * R4
      + c2r1 * R3
      + c2r2 * R3
      + c3r4 * R5
      + r_sus_a * c3r4
      + c3r4 * r_sus_b;

    ([0., 0., b0, b1], [a0, a1, a2, b1])
  }

  fn apply_bilinear_transform(&self, (mut b, mut a): ([f32; 4], [f32; 4])) -> ([f32; 4], [f32; 4]) {
    b[1] *= self.s[0];
    b[2] *= self.s[1];
    b[3] *= self.s[2];

    b = [
      b[0] + b[1] + b[2] + b[3],
      -3. * b[0] - b[1] + b[2] + 3. * b[3],
      3. * b[0] - b[1] - b[2] + 3. * b[3],
      -b[0] + b[1] - b[2] + b[3],
    ];

    a[1] *= self.s[0];
    a[2] *= self.s[1];
    a[3] *= self.s[2];

    a = [
      a[0] + a[1] + a[2] + a[3],
      -3. * a[0] - a[1] + a[2] + 3. * a[3],
      3. * a[0] - a[1] - a[2] + 3. * a[3],
      -a[0] + a[1] - a[2] + a[3],
    ];

    (b.map(|x| x / a[0]), a.map(|x| x / a[0]))
  }

  fn filter(&mut self, x: f32, (b, a): ([f32; 4], [f32; 4])) -> f32 {
    let y = x * b[0] + self.z[0];
    self.z[0] = x * b[1] - y * a[1] + self.z[1];
    self.z[1] = x * b[2] - y * a[2] + self.z[2];
    self.z[2] = x * b[3] - y * a[3];

    y
  }
}
