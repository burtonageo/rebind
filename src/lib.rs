extern crate input;
extern crate piston_window;
extern crate rustc_serialize;
extern crate viewport;

use input::{Input, Button, Motion};
use piston_window::Size;
use std::collections::HashMap;
use viewport::Viewport;

/// Represents a logical action to be bound to a particular button press, e.g.
/// jump, attack, or move forward
pub trait Action: Copy + PartialEq + Eq { }

/// A translated action.
#[derive(Debug, Copy, Clone)]
pub enum Translated<A: Action> {
    /// A keypress event which was bound to an action
    Press(A),

    /// A key release event which was bound to an action
    Release(A),

    /// A translated mouse motion. The logical origin of a translated MouseCursor event
    /// is in the top left corner of the window, and the logical scroll is non-natural.
    /// Relative events are unchanged for now.
    Move(Motion)
}

#[derive(Clone)]
pub struct InputMap<A: Action> {
    keymap: KeyMap<A>,
    mouse_translator: MouseTranslator
}

impl<A: Action> InputMap<A> {
    pub fn new(size: Size) -> Self {
        InputMap {
            keymap: KeyMap::new(),
            mouse_translator: MouseTranslator::new(size)
        }
    }

    pub fn translate(&self, input: &Input) -> Option<Translated<A>> {
        macro_rules! translate_button(($but_state:ident, $but_var:ident) => (
            match self.keymap.translate($but_var) {
                Some(act) => Some(Translated::$but_state(act)),
                None => None
            });
        );

        match input {
            &Input::Press(button) => translate_button!(Press, button),
            &Input::Release(button) => translate_button!(Release, button),
            &Input::Move(motion) =>
                Some(Translated::Move(self.mouse_translator.translate(motion))),
            _ => None
        }
    }

    pub fn rebind_button(&mut self, _but: Button, _act: A) {
        // TODO implement
    }

    pub fn add_binding(&mut self, but: Button, act: A) {
        self.keymap.add_mapping(but, act);
    }

    pub fn get_bindings_for_action(&self, _act: A) -> ButtonTuple {
        ButtonTuple(None, None, None) // TODO implement
    }

    pub fn set_size(&mut self, size: Size) {
        self.mouse_translator.viewport_size = size
    }

    pub fn set_size_from_viewport(&mut self, vp: Viewport) {
        self.set_size(Size::from(vp.draw_size));
    }
}

#[derive(Clone)]
struct MouseTranslator {
    pub x_axis_movement_inverted: bool,
    pub y_axis_movement_inverted: bool,
    pub x_axis_scroll_inverted: bool,
    pub y_axis_scroll_inverted: bool,
    pub viewport_size: Size
}

impl MouseTranslator {
    fn new(size: Size) -> Self {
        MouseTranslator {
            x_axis_movement_inverted: false,
            y_axis_movement_inverted: false,
            x_axis_scroll_inverted: false,
            y_axis_scroll_inverted: false,
            viewport_size: size
        }
    }

    fn translate(&self, motion: Motion) -> Motion {
        match motion {
            Motion::MouseCursor(x, y) => {
                let (_sw, _sh) = {
                    let Size {width, height} = self.viewport_size;
                    (width as f64, height as f64)
                };

                Motion::MouseCursor(x, y) // TODO implement
            },
            Motion::MouseScroll(x, y) => {
                let mx = if self.x_axis_scroll_inverted { -1.0f64 } else { 1.0 };
                let my = if self.y_axis_scroll_inverted { -1.0f64 } else { 1.0 };
                Motion::MouseScroll(x * mx, y * my)
            },
            relative => relative
        }
    }
}

#[derive(Clone, Debug)]
struct KeyMap<A: Action> {
    btn_map: HashMap<ButtonTuple, A>
}

impl<A: Action> KeyMap<A> {
    fn new() -> Self {
        KeyMap {
            btn_map: HashMap::new()
        }
    }

    fn add_mapping(&mut self, button: Button, action: A) {
        let mut bt = self.get_bindings_for_action(action).unwrap_or(ButtonTuple::new());
        let bt = if bt.insert_inplace(button) {bt} else {ButtonTuple::new()};
        self.btn_map.insert(bt, action);
    }

    fn _with_mapping(mut self, button: Button, action: A) -> Self {
        self.add_mapping(button, action);
        self
    }

    fn get_bindings_for_action(&self, action: A) -> Option<ButtonTuple> {
        self.btn_map.iter().find(|&(_, &a)| a == action).map(|(&bt, _)| bt)
    }

    fn translate(&self, button: Button) -> Option<A> {
        self.btn_map.iter().find(|&(&bt, _)| bt.contains(button)).map(|(_, &a)| a)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ButtonTuple(Option<Button>, Option<Button>, Option<Button>);

impl ButtonTuple {
    fn new() -> Self {
        ButtonTuple(None, None, None)
    }

    fn contains(&self, btn: Button) -> bool {
        let sbtn = Some(btn);
        self.0 == sbtn || self.1 == sbtn || self.2 == sbtn
    }

    fn _remove_inplace(&mut self, btn: Button) {
        let sbtn = Some(btn);
        if self.0 == sbtn {self.0 = None}
        if self.1 == sbtn {self.1 = None}
        if self.2 == sbtn {self.2 = None}
    }

    fn _replace_inplace(&mut self, btn_idx: u32, btn: Button) -> bool {
        match btn_idx {
            0 => {self.0 = Some(btn); true},
            1 => {self.1 = Some(btn); true},
            2 => {self.2 = Some(btn); true},
            _ => false
        }
    }

    fn insert_inplace(&mut self, btn: Button) -> bool {
        let &mut ButtonTuple(a, b, c) = self;
        if a.is_none() {
            *self = ButtonTuple(Some(btn), b, c)
        } else if b.is_none() {
            *self = ButtonTuple(a, Some(btn), c)
        } else if c.is_none() {
            *self = ButtonTuple(a, b, Some(btn))
        } else {
            return false;
        }
        true
    }
}
