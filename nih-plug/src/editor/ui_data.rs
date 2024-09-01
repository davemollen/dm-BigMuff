use crate::sd1_parameters::SD1Parameters;
use nih_plug::{params::internals::ParamPtr, prelude::GuiContext};
use nih_plug_vizia::vizia::prelude::*;
use std::sync::Arc;
pub enum ParamChangeEvent {
  SetParam(ParamPtr, f32),
}

#[derive(Lens)]
pub struct UiData {
  pub params: Arc<SD1Parameters>,
  pub gui_context: Arc<dyn GuiContext>,
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetParam(param_ptr, value) => {
        unsafe {
          self
            .gui_context
            .raw_set_parameter_normalized(*param_ptr, *value)
        };
      }
    });
  }
}
