mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub vol: LinearSmooth,
  pub tone: LinearSmooth,
  pub sustain: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      vol: LinearSmooth::new(sample_rate, 20.),
      tone: LinearSmooth::new(sample_rate, 20.),
      sustain: LinearSmooth::new(sample_rate, 20.),
      is_initialized: false,
    }
  }

  pub fn set(&mut self, vol: f32, tone: f32, sustain: f32) {
    if self.is_initialized {
      self.vol.set_target(vol * vol * vol);
      self.tone.set_target(tone);
      self.sustain.set_target(sustain);
    } else {
      self.vol.reset(vol * vol * vol);
      self.tone.reset(tone);
      self.sustain.reset(sustain);
      self.is_initialized = true;
    }
  }
}
