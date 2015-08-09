use {Action, InputMap};
use input::Button;
use piston_window::Size;
use std::convert::Into;
use std::default::Default;

/// Convenience object for constructing an InputMap
pub struct InputMapBuilder<A: Action> {
    input_remappings: Vec<(Button, A)>,
    x_axis_motion_inverted: bool,
    y_axis_motion_inverted: bool,
    x_axis_scroll_inverted: bool,
    y_axis_scroll_inverted: bool,
    viewport_size: Size
}

impl<A: Action> InputMapBuilder<A> {
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
}

impl<A: Action> Default for InputMapBuilder<A> {
    fn default() -> Self {
        InputMapBuilder {
            input_remappings: vec![],
            x_axis_motion_inverted: false,
            y_axis_motion_inverted: false,
            x_axis_scroll_inverted: false,
            y_axis_scroll_inverted: false,
            viewport_size: (800, 600).into()
        }
    }
}

impl<A: Action> Into<InputMap<A>> for InputMapBuilder<A> {
    fn into(self) -> InputMap<A> {
        let mut imap = InputMap::new(self.viewport_size);

        imap.mouse_translator.x_axis_motion_inverted = self.x_axis_motion_inverted;
        imap.mouse_translator.y_axis_motion_inverted = self.y_axis_motion_inverted;
        imap.mouse_translator.x_axis_scroll_inverted = self.x_axis_scroll_inverted;
        imap.mouse_translator.y_axis_scroll_inverted = self.y_axis_scroll_inverted;

        //TODO: set key remappings
        imap.keymap.btn_map.reserve(self.input_remappings.len());

        imap
    }
}

