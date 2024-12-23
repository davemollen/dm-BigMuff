mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub vol: LinearSmooth,
  pub tone: LinearSmooth,
  pub sustain: LinearSmooth,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      vol: LinearSmooth::new(20., sample_rate),
      tone: LinearSmooth::new(20., sample_rate),
      sustain: LinearSmooth::new(20., sample_rate),
    }
  }

  pub fn set(&mut self, vol: f32, tone: f32, sustain: f32) {
    self.vol.set_target(vol * vol * vol);
    self.tone.set_target(tone);
    self.sustain.set_target(sustain);
  }
}
