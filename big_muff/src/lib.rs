#![feature(portable_simd)]
mod clipper;
mod tone;
mod shared {
  pub mod float_ext;
}
mod op_amp1;
mod op_amp2;
mod op_amp3;
mod params;
use {
  clipper::Clipper, op_amp1::OpAmp1, op_amp2::OpAmp2, op_amp3::OpAmp3, params::Smoother, tone::Tone
};
pub use params::Params;

pub struct BigMuff {
  op_amp1: OpAmp1,
  op_amp2: OpAmp2,
  op_amp3: OpAmp3,
  clipper: Clipper,
  tone: Tone
}

impl BigMuff {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp1: OpAmp1::new(sample_rate),
      op_amp2: OpAmp2::new(sample_rate),
      op_amp3: OpAmp3::new(sample_rate),
      clipper: Clipper::new(),
      tone: Tone::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let vol = params.vol.next();
    let tone = params.tone.next();
    let sustain = params.sustain.next();

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
