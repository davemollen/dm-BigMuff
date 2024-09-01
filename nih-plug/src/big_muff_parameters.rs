use nih_plug::{
  formatters::{s2v_f32_percentage, v2s_f32_percentage},
  prelude::{FloatParam, FloatRange, Params},
};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

use crate::editor;

#[derive(Params)]
pub struct BigMuffParameters {
  /// The editor state, saved together with the parameter state so the custom scaling can be
  /// restored.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "vol"]
  pub vol: FloatParam,

  #[id = "tone"]
  pub tone: FloatParam,

  #[id = "sustain"]
  pub sustain: FloatParam,
}

impl Default for BigMuffParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      vol: FloatParam::new("Vol", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      tone: FloatParam::new("Tone", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      sustain: FloatParam::new("Sustain", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
