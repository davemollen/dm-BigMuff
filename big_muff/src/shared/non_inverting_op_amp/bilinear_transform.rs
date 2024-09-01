pub struct BilinearTransform {
  s: [f32; 2],
}

impl BilinearTransform {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4.],
    }
  }

  pub fn process(&self, (mut b, mut a): ([f32; 3], [f32; 3])) -> ([f32; 3], [f32; 3]) {
    b[1] *= self.s[0];
    b[2] *= self.s[1];
    a[1] *= self.s[0];
    a[2] *= self.s[1];

    let b = [
      b[0] + b[1] + b[2],
      -2. * b[0] + 2. * b[2],
      b[0] - b[1] + b[2],
    ];

    let a = [
      a[0] + a[1] + a[2],
      -2. * a[0] + 2. * a[2],
      a[0] - a[1] + a[2],
    ];

    (b.map(|x| x / a[0]), a.map(|x| x / a[0]))
  }
}

#[cfg(test)]
mod tests {
  use super::BilinearTransform;

  #[test]
  fn bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: ([f32; 3], [f32; 3]) = (
      [1.0, 3136750.4835589933, 644745325.5963894],
      [1.0, 106447.45325596389, 644745325.5963894],
    );
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [16.004657741990535, -0.8010592913376103, -15.05881476037464],
        [1.0, -0.8010592913376086, -0.05415701838410163]
      )
    );
  }
}
