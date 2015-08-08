use Action;
use input::Button;
use piston_window::Size;

pub struct InputMapBuilder<A: Action> {
    input_remappings: Vec<(Button, A)>,
    x_axis_movement_inverted: bool,
    y_axis_movement_inverted: bool,
    x_axis_scroll_inverted: bool,
    y_axis_scroll_inverted: bool,
    viewport_size: Size
}

impl<A: Action> InputMapBuilder<A> {
    pub fn x_scroll_inverted(mut self, invert: bool) -> Self {
        self.x_axis_scroll_inverted = invert;
        self
    }

    pub fn get_x_scroll_inverted(mut self, invert: bool) -> Self {
        &self.x_axis_scroll_inverted
    }

    pub fn y_scroll_inverted(mut self, invert: bool) -> Self {
        self.y_axis_scroll_inverted = invert;
        self
    }

    pub fn get_y_scroll_inverted(&self) -> bool {
        &self.y_axis_scroll_inverted
    }

    pub fn viewport_size(mut self, sz: Size) -> Self {
        self.viewport_size = sz;
        self
    }

    pub fn get_viewport_size(&self) -> &Size {
        &self.viewport_size
    }
}

impl<A: Action> Default for InputMapBuilder<A> {
    fn default() -> Self {
        InputMapBuilder {
            input_remappings: vec![],
            x_axis_movement_inverted: false,
            y_axis_movement_inverted: false,
            x_axis_scroll_inverted: false,
            y_axis_scroll_inverted: false,
            viewport_size: Size {width: 800, height: 600}
        }
    }
}

impl<A> Into<InputMap<A>> for InputMapBuilder<A> {
    fn into(self) -> InputMap<A> {
        InputMap::new() // TODO implement
    }
}

