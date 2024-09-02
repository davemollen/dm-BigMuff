#![feature(portable_simd)]
mod clipper;
mod tone;
mod shared {
  pub mod float_ext;
}
mod op_amp1;
mod op_amp2;
mod op_amp3;
mod smooth_parameters;
use {
  clipper::Clipper, op_amp1::OpAmp1, op_amp2::OpAmp2, op_amp3::OpAmp3,
  smooth_parameters::SmoothParameters, tone::Tone,
};

pub struct BigMuff {
  op_amp1: OpAmp1,
  op_amp2: OpAmp2,
  op_amp3: OpAmp3,
  clipper: Clipper,
  tone: Tone,
  smooth_parameters: SmoothParameters,
}

impl BigMuff {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp1: OpAmp1::new(sample_rate),
      op_amp2: OpAmp2::new(sample_rate),
      op_amp3: OpAmp3::new(sample_rate),
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
    let op_amp1_output = self
      .op_amp1
      .process(input, (-5.8928571428571, [1., 119.04761904762]));
    let op_amp2_output = self.op_amp2.process(op_amp1_output, sustain);
    let op_amp3_output = self.op_amp3.process(
      op_amp2_output,
      (-813008.1300813, [1., 14210.344231102, 368043.51746551]),
    );
    let clip_output = self.clipper.process(op_amp3_output);
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * vol
  }
}
