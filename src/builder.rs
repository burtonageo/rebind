use {Action, InputTranslator, MouseTranslationData};
use input::Button;
use piston_window::Size;
use std::convert::Into;
use std::default::Default;

/// Convenience object for constructing an InputMap
pub struct InputTranslatorBuilder<A: Action> {
    input_remappings: Vec<(Button, A)>,
    mouse_data: MouseTranslationData
}

impl<A: Action> InputTranslatorBuilder<A> {
    pub fn new() -> Self { Default::default() }

    pub fn x_scroll_inverted(mut self, invert: bool) -> Self {
        self.mouse_data.x_axis_scroll_inverted = invert;
        self
    }

    pub fn get_x_scroll_inverted(&self) -> &bool {
        &self.mouse_data.x_axis_scroll_inverted
    }

    pub fn y_scroll_inverted(mut self, invert: bool) -> Self {
        self.mouse_data.y_axis_scroll_inverted = invert;
        self
    }

    pub fn get_y_scroll_inverted(&self) -> &bool {
        &self.mouse_data.y_axis_scroll_inverted
    }

    pub fn x_motion_inverted(mut self, invert: bool) -> Self {
        self.mouse_data.x_axis_motion_inverted = invert;
        self
    }

    pub fn get_x_motion_inverted(&self) -> &bool {
        &self.mouse_data.x_axis_motion_inverted
    }

    pub fn y_motion_inverted(mut self, invert: bool) -> Self {
        self.mouse_data.y_axis_motion_inverted = invert;
        self
    }

    pub fn get_y_motion_inverted(&self) -> &bool {
        &self.mouse_data.y_axis_motion_inverted
    }

    pub fn viewport_size(mut self, sz: Size) -> Self {
        self.mouse_data.viewport_size = sz;
        self
    }

    pub fn get_viewport_size(&self) -> &Size {
        &self.mouse_data.viewport_size
    }

    pub fn with_action_mapping(mut self, but: Button, act: A) -> Self {
        self.input_remappings.push((but, act));
        self
    }

    pub fn build(self) -> InputTranslator<A> {self.into()}
}

impl<A: Action> Default for InputTranslatorBuilder<A> {
    fn default() -> Self {
        InputTranslatorBuilder {
            input_remappings: vec![],
            mouse_data: MouseTranslationData::new((800, 600).into())
        }
    }
}

impl<A: Action> Into<InputTranslator<A>> for InputTranslatorBuilder<A> {
    fn into(self) -> InputTranslator<A> {
        let mut input_map = InputTranslator::new(self.mouse_data.viewport_size);

        input_map.mouse_translator.data.x_axis_motion_inverted = self.mouse_data.x_axis_motion_inverted;
        input_map.mouse_translator.data.y_axis_motion_inverted = self.mouse_data.y_axis_motion_inverted;
        input_map.mouse_translator.data.x_axis_scroll_inverted = self.mouse_data.x_axis_scroll_inverted;
        input_map.mouse_translator.data.y_axis_scroll_inverted = self.mouse_data.y_axis_scroll_inverted;

        //TODO: set key remappings
        input_map.keymap.btn_map.reserve(self.input_remappings.len());
        

        input_map
    }
}

