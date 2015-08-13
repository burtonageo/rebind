#![warn(missing_docs)]

extern crate input;
extern crate piston_window;
extern crate rustc_serialize;
extern crate viewport;

mod builder;
mod rebind;

use input::{Input, Button, Motion};
use piston_window::Size;
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;
use viewport::Viewport;

pub use rebind::InputRebind;
pub use builder::InputTranslatorBuilder;

/// Represents a logical action to be bound to a particular button press, e.g.
/// jump, attack, or move forward
pub trait Action: Copy + PartialEq + Eq + Hash { }

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

/// A three-element tuple of Option<Button>. Used as the key of an InputTranslator
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ButtonTuple(pub Option<Button>, pub Option<Button>, pub Option<Button>);

impl ButtonTuple {
    /// Creates a new tuple with no buttons in it (equivalent to
    /// ```Default::default()```
    pub fn new() -> Self { Default::default() }

    /// Check if the button is in the tuple.
    pub fn contains(&self, button: Button) -> bool {
        let sbtn = Some(button);
        self.0 == sbtn || self.1 == sbtn || self.2 == sbtn
    }

    #[allow(missing_docs)]
    pub fn remove_inplace(&mut self, btn: Button) {
        let sbtn = Some(btn);
        if self.0 == sbtn {self.0 = None}
        if self.1 == sbtn {self.1 = None}
        if self.2 == sbtn {self.2 = None}
    }

    #[allow(missing_docs)]
    pub fn replace_inplace(&mut self, btn_idx: u32, btn: Button) -> bool {
        match btn_idx {
            0 => {self.0 = Some(btn); true},
            1 => {self.1 = Some(btn); true},
            2 => {self.2 = Some(btn); true},
            _ => false
        }
    }

    #[allow(missing_docs)]
    pub fn insert_inplace(&mut self, btn: Button) -> bool {
        match self {
            &mut ButtonTuple(a, b, c) if a.is_none() => {*self = ButtonTuple(Some(btn), b, c); true},
            &mut ButtonTuple(a, b, c) if b.is_none() => {*self = ButtonTuple(a, Some(btn), c); true},
            &mut ButtonTuple(a, b, c) if c.is_none() => {*self = ButtonTuple(a, b, Some(btn)); true}
            _ => false
        }
    }
}

impl Default for ButtonTuple {
    /// Creates a new tuple with no buttons in it
    fn default() -> Self { ButtonTuple(None, None, None) }
}

/// An object which translates piston::input::Input events into input_map::Translated<A> events
#[derive(Clone)]
pub struct InputTranslator<A: Action> {
    keymap: HashMap<Button, A>,
    mouse_translator: MouseTranslator
}

impl<A: Action> InputTranslator<A> {

    /// Creates an empty InputTranslator.
    pub fn new(size: Size) -> Self {
        InputTranslator {
            keymap: HashMap::new(),
            mouse_translator: MouseTranslator::new(size)
        }
    }

    /// Translate an Input into a Translated<A> event
    pub fn translate(&self, input: &Input) -> Option<Translated<A>> {
        macro_rules! translate_button(($but_state:ident, $but_var:ident) => (
            match self.keymap.get(&$but_var).map(|x| *x) {
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

    /// Re-set the mouse bounds size used for calculating mouse events
    pub fn set_size(&mut self, size: Size) {
        self.mouse_translator.data.viewport_size = size
    }

    /// Re-set the mouse bounds size from a viewport
    pub fn set_size_from_viewport(&mut self, vp: Viewport) {
        self.set_size(Size::from(vp.draw_size));
    }
}

#[derive(Clone)]
struct MouseTranslationData {
    x_axis_motion_inverted: bool,
    y_axis_motion_inverted: bool,
    x_axis_scroll_inverted: bool,
    y_axis_scroll_inverted: bool,
    viewport_size: Size
}

impl MouseTranslationData {
    fn new(size: Size) -> Self {
        MouseTranslationData {
            x_axis_motion_inverted: false,
            y_axis_motion_inverted: false,
            x_axis_scroll_inverted: false,
            y_axis_scroll_inverted: false,
            viewport_size: size
        }
    }
}

#[derive(Clone)]
struct MouseTranslator {
    data: MouseTranslationData
}

impl MouseTranslator {
    fn new(size: Size) -> Self {
        MouseTranslator {
            data: MouseTranslationData::new(size)
        }
    }

    fn translate(&self, motion: Motion) -> Motion {
        match motion {
            Motion::MouseCursor(x, y) => {
                let (sw, sh) = {
                    let Size {width, height} = self.data.viewport_size;
                    (width as f64, height as f64)
                };

                let cx = if self.data.x_axis_motion_inverted { sw - x } else { x };
                let cy = if self.data.y_axis_motion_inverted { sh - y } else { y };

                Motion::MouseCursor(cx, cy)
            },
            Motion::MouseScroll(x, y) => {
                let mx = if self.data.x_axis_scroll_inverted { -1.0f64 } else { 1.0 };
                let my = if self.data.y_axis_scroll_inverted { -1.0f64 } else { 1.0 };
                Motion::MouseScroll(x * mx, y * my)
            },
            relative => relative
        }
    }
}
