use {Action, InputRebind, InputTranslator, MouseTranslationData, to_act_bt_hashmap};
use input::Button;
use window::Size;
use std::convert::Into;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::default::Default;
use std::marker::PhantomData;

/// Convenience object for constructing an InputMap.
#[derive(Debug)]
pub struct Builder<A: Action, S: BuildHasher = RandomState> {
    input_remappings: Vec<(Button, A)>,
    mouse_data: MouseTranslationData,
    _hasher: PhantomData<S>
}

impl<A: Action, S: BuildHasher + Default> Builder<A, S> {
    /// Creates a new `Builder` with the specified viewport size.
    pub fn new<Sz: Into<Size>>(size: Sz) -> Self {
        Builder {
            input_remappings: vec![],
            mouse_data: MouseTranslationData::new(size),
            _hasher: PhantomData
        }
    }

    /// Set whether the x scroll is inverted on the builder.
    pub fn x_scroll_inverted(mut self, invert: bool) -> Self {
        self.mouse_data.x_axis_scroll_inverted = invert;
        self
    }

    /// Returns true if the x scroll is inverted on the builder.
    pub fn get_x_scroll_inverted(&self) -> bool {
        self.mouse_data.x_axis_scroll_inverted
    }

    /// Set whether the y scroll is inverted on the builder.
    pub fn y_scroll_inverted(mut self, invert: bool) -> Self {
        self.mouse_data.y_axis_scroll_inverted = invert;
        self
    }

    /// Returns true if the y scroll is inverted on the builder.
    pub fn get_y_scroll_inverted(&self) -> bool {
        self.mouse_data.y_axis_scroll_inverted
    }

    /// Set whether the x axis motion is inverted on the builder.
    pub fn x_motion_inverted(mut self, invert: bool) -> Self {
        self.mouse_data.x_axis_motion_inverted = invert;
        self
    }

    /// Returns true if the x axis motion is inverted on the builder.
    pub fn get_x_motion_inverted(&self) -> bool {
        self.mouse_data.x_axis_motion_inverted
    }

    /// Set whether the y axis motion is inverted on the builder.
    pub fn y_motion_inverted(mut self, invert: bool) -> Self {
        self.mouse_data.y_axis_motion_inverted = invert;
        self
    }

    /// Returns true if the y axis motion is inverted on the builder.
    pub fn get_y_motion_inverted(&self) -> bool {
        self.mouse_data.y_axis_motion_inverted
    }

    /// Set the mouse sensitivity.
    pub fn mouse_sensitivity(mut self, sensitivity: f64) -> Self {
        self.mouse_data.sensitivity = sensitivity;
        self
    }

    /// Returns the mouse sensitivity.
    pub fn get_mouse_sensitivity(&self) -> f64 {
        self.mouse_data.sensitivity
    }

    /// Sets the viewport size used for mouse position calculations.
    pub fn viewport_size(mut self, size: Size) -> Self {
        self.mouse_data.viewport_size = size;
        self
    }

    /// Returns the currently set viewport size.
    pub fn get_viewport_size(&self) -> Size {
        self.mouse_data.viewport_size
    }

    /// Add an association between the Button and Action.
    pub fn with_mapping(mut self, action: A, button: Button) -> Self {
        self.input_remappings.push((button, action));
        self
    }

    /// Creates an `InputTranslator` from this builder object.
    pub fn build_translator(self) -> InputTranslator<A, S> {
        self.into()
    }

    /// Creates an `InputRebind` from this builder object.
    pub fn build_rebind(self) -> InputRebind<A, S> {
        self.into()
    }
}

impl<A: Action, S: BuildHasher + Default> Default for Builder<A, S> {
    fn default() -> Self {
        Self::new((800, 600))
    }
}


impl<A: Action, S: BuildHasher + Default> Into<InputTranslator<A, S>> for Builder<A, S> {
    fn into(self) -> InputTranslator<A, S> {
        let mut translator = InputTranslator::new(self.mouse_data.viewport_size);

        translator.mouse_translator.data = self.mouse_data;
        translator.keymap = self.input_remappings.iter().cloned().collect();

        translator
    }
}

impl<A: Action, S: BuildHasher + Default> Into<InputRebind<A, S>> for Builder<A, S> {
    fn into(self) -> InputRebind<A, S> {
        let mut rebind = InputRebind::new(self.mouse_data.viewport_size);

        rebind.mouse_data = self.mouse_data;
        rebind.keymap = to_act_bt_hashmap(self.input_remappings.iter().cloned());

        rebind
    }
}
