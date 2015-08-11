use {Action, ButtonTuple};
use std::collections::HashMap;

pub struct InputRebind<A: Action> {
    keymap: HashMap<A, ButtonTuple>
}