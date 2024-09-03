use big_muff::BigMuff;
use nih_plug::prelude::*;
use std::sync::Arc;
mod big_muff_parameters;
use big_muff_parameters::BigMuffParameters;
mod editor;

struct DmBigMuff {
  params: Arc<BigMuffParameters>,
  big_muff: BigMuff,
}

impl DmBigMuff {
  pub fn get_params(&self) -> (f32, f32, f32) {
    let vol = self.params.vol.value();
    let tone = self.params.tone.value();
    let sustain = self.params.sustain.value();

    self.big_muff.map_params(vol, tone, sustain)
  }
}

impl Default for DmBigMuff {
  fn default() -> Self {
    let params = Arc::new(BigMuffParameters::default());
    Self {
      params: params.clone(),
      big_muff: BigMuff::new(44100.),
    }
  }
}

impl Plugin for DmBigMuff {
  const NAME: &'static str = "dm-BigMuff";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-BigMuff";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
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
    let (vol, tone, sustain) = self.get_params();
    self.big_muff.initialize_params(vol, tone, sustain);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let (vol, tone, sustain) = self.get_params();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let sample = channel_samples.iter_mut().next().unwrap();
      *sample = self.big_muff.process(*sample, vol, tone, sustain);
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
    ClapFeature::Utility,
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
