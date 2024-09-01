#![feature(portable_simd)]
mod clipper;
mod tone;
mod shared {
  pub mod float_ext;
  pub mod non_inverting_op_amp;
  pub mod one_pole_filter;
}
mod op_amp;
mod smooth_parameters;
use {
  clipper::Clipper, op_amp::OpAmp, shared::one_pole_filter::OnePoleFilter,
  smooth_parameters::SmoothParameters, tone::Tone,
};

pub struct BigMuff {
  one_pole_filter: OnePoleFilter,
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
  smooth_parameters: SmoothParameters,
}

impl BigMuff {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      one_pole_filter: OnePoleFilter::new(sample_rate, 88.419412828831),
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(),
      tone: Tone::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn map_vol_param(&self, vol: f32) -> f32 {
    vol * vol * vol
  }

  pub fn initialize_params(&mut self, vol: f32, tone: f32, sustain: f32) {
    self.smooth_parameters.initialize(vol, tone, sustain);
  }

  pub fn process(&mut self, input: f32, vol: f32, tone: f32, sustain: f32) -> f32 {
    let (vol, tone, sustain) = self.smooth_parameters.process(vol, tone, sustain);
    let highpass_output = input - self.one_pole_filter.process(input);
    let op_amp_output = self.op_amp.process(highpass_output, vol);
    let clip_output = self.clipper.process(op_amp_output) + input;
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * sustain
  }
}
