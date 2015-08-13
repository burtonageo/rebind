use {Action, ButtonTuple, InputTranslator, MouseTranslationData};
use input::Button;
use piston_window::Size;
use std::collections::HashMap;
use std::convert::Into;
use std::default::Default;

#[allow(missing_docs)]
pub struct InputRebind<A: Action> {
    keymap: HashMap<A, ButtonTuple>,
    mouse_data: MouseTranslationData
}

impl<A: Action> InputRebind<A> {
    #[allow(missing_docs)]
    pub fn new(size: Size) -> Self {
        InputRebind {
            keymap: HashMap::new(),
            mouse_data: MouseTranslationData::new(size)
        }
    }

    #[allow(missing_docs)]
    pub fn insert_action(&mut self, action: A) -> Option<ButtonTuple> {
        self.keymap.insert(action, ButtonTuple::new())
    }

    #[allow(missing_docs)]
    pub fn insert_action_with_buttons(&mut self, action: A, buttons: ButtonTuple) -> Option<ButtonTuple> {
        self.keymap.insert(action, buttons)
    }

    #[allow(missing_docs)]
    pub fn get_bindings(&self, action: &A) -> Option<&ButtonTuple> {
        self.keymap.get(action)
    }

    #[allow(missing_docs)]
    pub fn get_bindings_mut(&mut self, action: &mut A) -> Option<&mut ButtonTuple> {
        self.keymap.get_mut(action)
    }

    #[allow(missing_docs)]
    pub fn get_x_scroll_inverted(&self) -> &bool {
        &self.mouse_data.x_axis_scroll_inverted
    }

    #[allow(missing_docs)]
    pub fn get_x_scroll_inverted_mut(&mut self) -> &mut bool {
        &mut self.mouse_data.x_axis_scroll_inverted
    }

    #[allow(missing_docs)]
    pub fn get_y_scroll_inverted(&self) -> &bool {
        &self.mouse_data.y_axis_scroll_inverted
    }

    #[allow(missing_docs)]
    pub fn get_y_scroll_inverted_mut(&mut self) -> &mut bool {
        &mut self.mouse_data.y_axis_scroll_inverted
    }

    #[allow(missing_docs)]
    pub fn get_x_motion_inverted(&self) -> &bool {
        &self.mouse_data.x_axis_motion_inverted
    }

    #[allow(missing_docs)]
    pub fn get_x_motion_inverted_mut(&mut self) -> &mut bool {
        &mut self.mouse_data.x_axis_motion_inverted
    }

    #[allow(missing_docs)]
    pub fn get_y_motion_inverted(&self) -> &bool {
        &self.mouse_data.y_axis_motion_inverted
    }

    #[allow(missing_docs)]
    pub fn get_y_motion_inverted_mut(&mut self) -> &mut bool {
        &mut self.mouse_data.y_axis_motion_inverted
    }

    #[allow(missing_docs)]
    pub fn get_viewport_size(&self) -> &Size {
        &self.mouse_data.viewport_size
    }

    #[allow(missing_docs)]
    pub fn get_viewport_size_mut(&mut self) -> &mut Size {
        &mut self.mouse_data.viewport_size
    }
}

impl<A: Action> Default for InputRebind<A> {
    #[allow(missing_docs)]
    fn default() -> Self {
        InputRebind::new((800, 600).into())
    }
}

impl<A: Action> Into<InputTranslator<A>> for InputRebind<A> {
    #[allow(missing_docs)]
    fn into(self) -> InputTranslator<A> {
        let mut input_translator = InputTranslator::new(self.mouse_data.viewport_size);
        input_translator.mouse_translator.data = self.mouse_data;
        input_translator.keymap = self.keymap.values()
                                             .flat_map(|&bt| bt.into_iter())
                                             .filter_map(|x| x)
                                             .zip(self.keymap.keys().map(|x| x.clone()))
                                             .collect();

        input_translator
    }
}

impl<A: Action> Into<InputRebind<A>> for InputTranslator<A> {
    #[allow(missing_docs)]
    fn into(self) -> InputRebind<A> {
        let mut input_rebind = InputRebind::new(self.mouse_translator.data.viewport_size);
        input_rebind.mouse_data = self.mouse_translator.data;
        //input_rebind.keymap = self.keymap.btn_map.
        //input_rebind.keymap = self.keymap.btn_map.drain().map(|(k, v)| (v, k)).collect();
        input_rebind
    }
}

impl ButtonTuple {
    fn into_iter(self) -> ButtonTupleIterator {
        ButtonTupleIterator {
            button_tuple: self,
            i: 0
        }
    }
}

struct ButtonTupleIterator {
    button_tuple: ButtonTuple,
    i: usize
}

impl Iterator for ButtonTupleIterator {
    type Item = Option<Button>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        match i {
            0 => Some(self.button_tuple.0),
            1 => Some(self.button_tuple.1),
            2 => Some(self.button_tuple.2),
            _ => None
        }
    }
}
