extern crate big_muff;
extern crate lv2;
use big_muff::{BigMuff, Params};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  vol: InputPort<Control>,
  tone: InputPort<Control>,
  sustain: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-BigMuff")]
struct DmBigMuff {
  big_muff: BigMuff,
  params: Params,
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
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(*ports.vol, *ports.tone, *ports.sustain);

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.big_muff.process(*input, &mut self.params);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmBigMuff);
