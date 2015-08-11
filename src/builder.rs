use {Action, InputTranslator};
use input::Button;
use piston_window::Size;
use std::convert::Into;
use std::default::Default;

/// Convenience object for constructing an InputMap
pub struct InputTranslatorBuilder<A: Action> {
    input_remappings: Vec<(Button, A)>,
    x_axis_motion_inverted: bool,
    y_axis_motion_inverted: bool,
    x_axis_scroll_inverted: bool,
    y_axis_scroll_inverted: bool,
    viewport_size: Size
}

impl<A: Action> InputTranslatorBuilder<A> {
    pub fn new() -> Self { Default::default() }

    pub fn x_scroll_inverted(mut self, invert: bool) -> Self {
        self.x_axis_scroll_inverted = invert;
        self
    }

    pub fn get_x_scroll_inverted(&self) -> &bool {
        &self.x_axis_scroll_inverted
    }

    pub fn y_scroll_inverted(mut self, invert: bool) -> Self {
        self.y_axis_scroll_inverted = invert;
        self
    }

    pub fn get_y_scroll_inverted(&self) -> &bool {
        &self.y_axis_scroll_inverted
    }

    pub fn x_motion_inverted(mut self, invert: bool) -> Self {
        self.x_axis_motion_inverted = invert;
        self
    }
    pub fn get_x_motion_inverted(&self) -> &bool {
        &self.x_axis_motion_inverted
    }
    pub fn y_motion_inverted(mut self, invert: bool) -> Self {
        self.y_axis_motion_inverted = invert;
        self
    }
    pub fn get_y_motion_inverted(&self) -> &bool {
        &self.y_axis_motion_inverted
    }

    pub fn viewport_size(mut self, sz: Size) -> Self {
        self.viewport_size = sz;
        self
    }

    pub fn get_viewport_size(&self) -> &Size {
        &self.viewport_size
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
            x_axis_motion_inverted: false,
            y_axis_motion_inverted: false,
            x_axis_scroll_inverted: false,
            y_axis_scroll_inverted: false,
            viewport_size: (800, 600).into()
        }
    }
}

impl<A: Action> Into<InputTranslator<A>> for InputTranslatorBuilder<A> {
    fn into(self) -> InputTranslator<A> {
        let mut input_map = InputTranslator::new(self.viewport_size);

        input_map.mouse_translator.x_axis_motion_inverted = self.x_axis_motion_inverted;
        input_map.mouse_translator.y_axis_motion_inverted = self.y_axis_motion_inverted;
        input_map.mouse_translator.x_axis_scroll_inverted = self.x_axis_scroll_inverted;
        input_map.mouse_translator.y_axis_scroll_inverted = self.y_axis_scroll_inverted;

        //TODO: set key remappings
        input_map.keymap.btn_map.reserve(self.input_remappings.len());
        

        input_map
    }
}

