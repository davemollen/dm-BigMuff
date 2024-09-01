mod ramp_smooth;
use ramp_smooth::RampSmooth;

pub struct SmoothParameters {
  smooth_drive: RampSmooth,
  smooth_tone: RampSmooth,
  smooth_level: RampSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_drive: RampSmooth::new(sample_rate, 20.),
      smooth_tone: RampSmooth::new(sample_rate, 20.),
      smooth_level: RampSmooth::new(sample_rate, 20.),
    }
  }

  pub fn initialize(&mut self, drive: f32, tone: f32, level: f32) {
    self.smooth_drive.initialize(drive);
    self.smooth_tone.initialize(tone);
    self.smooth_level.initialize(level);
  }

  pub fn process(&mut self, drive: f32, tone: f32, level: f32) -> (f32, f32, f32) {
    (
      self.smooth_drive.process(drive),
      self.smooth_tone.process(tone),
      self.smooth_level.process(level),
    )
  }
}
