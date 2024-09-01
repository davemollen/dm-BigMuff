use crate::shared::non_inverting_op_amp::NonInvertingOpAmp;

const CS: f32 = 0.22e-6;
const RS: f32 = 1e3;
const RI: f32 = 10e3;
const CZ: f32 = 0.22e-6;
const RZ: f32 = 220.0;
const RF: f32 = 1e3;

pub struct Tone {
  non_inverting_op_amp: NonInvertingOpAmp,
}

impl Tone {
  const WP: f32 = 1.0 / (CS * RS * RI / (RS + RI));

  pub fn new(sample_rate: f32) -> Self {
    Self {
      non_inverting_op_amp: NonInvertingOpAmp::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let s_domain_coefficients = Self::get_s_domain_coefficients(tone);
    self
      .non_inverting_op_amp
      .process(input, s_domain_coefficients)
  }

  fn get_s_domain_coefficients(tone: f32) -> ([f32; 3], [f32; 3]) {
    let rl = tone;
    let rr: f32 = 1. - tone;
    let rl_and_rr = rl + rr;
    let a = rl * rr / rl_and_rr;
    let rz_and_a = RZ + a;
    let y = rl_and_rr * rz_and_a;
    let rl_rf_and_y = rl * RF + y;

    let wz = (CZ * rz_and_a).recip();

    let x = rr / rl_and_rr / rz_and_a * y;
    let w = y / rl_rf_and_y;

    let alpha = rl_rf_and_y / (y * RS * CS);

    (
      [0., alpha, alpha * w * wz],
      [1., Self::WP + wz + x, Self::WP * wz],
    )
  }
}
