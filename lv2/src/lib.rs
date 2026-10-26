extern crate big_muff;
extern crate lv2;
use big_muff::{BigMuff, Bypass, Params};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  vol: InputPort<InPlaceControl>,
  tone: InputPort<InPlaceControl>,
  sustain: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
  enabled: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-BigMuff")]
struct DmBigMuff {
  big_muff: BigMuff,
  params: Params,
  bypass: Bypass,
}

impl Plugin for DmBigMuff {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      big_muff: BigMuff::new(sample_rate),
      params: Params::new(sample_rate),
      bypass: Bypass::new(sample_rate),
    })
  }

  /// Reset the internal state of the plugin.
  fn activate(&mut self, _features: &mut Self::InitFeatures) {
    self.big_muff.reset();
    self.params.reset();
    self.bypass.reset();
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), sample_count: u32) {
    self.bypass.set_enabled(ports.enabled.get() == 0., || {
      self.big_muff.reset();
      self.params.reset();
    });

    if self.bypass.is_done_activating() {
      ports.output.copy_from(&ports.input, sample_count);
      return;
    }

    self
      .params
      .set(ports.vol.get(), ports.tone.get(), ports.sustain.get());

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let dry = input.get();
      let wet = self.big_muff.process(dry, &mut self.params);
      let big_muff_output = self.bypass.process(dry, wet);
      output.set(big_muff_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmBigMuff);
