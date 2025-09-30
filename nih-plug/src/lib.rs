use big_muff::{BigMuff, Params as ProcessParams};
use nih_plug::prelude::*;
use std::sync::Arc;
mod big_muff_parameters;
use big_muff_parameters::BigMuffParameters;
mod editor;

struct DmBigMuff {
  params: Arc<BigMuffParameters>,
  big_muff: BigMuff,
  process_params: ProcessParams,
}

impl Default for DmBigMuff {
  fn default() -> Self {
    let params = Arc::new(BigMuffParameters::default());
    Self {
      params: params.clone(),
      big_muff: BigMuff::new(44100.),
      process_params: ProcessParams::new(44100.),
    }
  }
}

impl Plugin for DmBigMuff {
  const NAME: &'static str = "BigMuff";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-BigMuff";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
    AudioIOLayout {
      main_input_channels: NonZeroU32::new(2),
      main_output_channels: NonZeroU32::new(2),
      ..AudioIOLayout::const_default()
    },
    AudioIOLayout {
      main_input_channels: NonZeroU32::new(1),
      main_output_channels: NonZeroU32::new(1),
      ..AudioIOLayout::const_default()
    },
  ];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.big_muff = BigMuff::new(buffer_config.sample_rate);
    self.process_params = ProcessParams::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    self.process_params.set(
      self.params.vol.value(),
      self.params.tone.value(),
      self.params.sustain.value(),
    );

    buffer.iter_samples().for_each(|mut channel_samples| {
      if channel_samples.len() == 2 {
        let channel_iterator = &mut channel_samples.iter_mut();
        let left_channel = channel_iterator.next().unwrap();
        let right_channel = channel_iterator.next().unwrap();
        let big_muff_out = self.big_muff.process(
          (*left_channel + *right_channel) * 0.5,
          &mut self.process_params,
        );
        *left_channel = big_muff_out;
        *right_channel = big_muff_out;
      } else {
        let sample = channel_samples.iter_mut().next().unwrap();
        *sample = self.big_muff.process(*sample, &mut self.process_params);
      };
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmBigMuff {
  const CLAP_ID: &'static str = "dm-BigMuff";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A fuzz plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::Distortion,
  ];
}

impl Vst3Plugin for DmBigMuff {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-BigMuff......";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Mono,
    Vst3SubCategory::Distortion,
  ];
}

nih_export_clap!(DmBigMuff);
nih_export_vst3!(DmBigMuff);
