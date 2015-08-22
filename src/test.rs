#![allow(dead_code, unused_variables)]

use {Action, InputTranslator, RebindBuilder, InputRebind};
use window::Size;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TestAction {
    Action1, Action2, Action3, Action4, Action5,
    Action6, Action7, Action8, Action9, Action10
}

impl Action for TestAction { }

type TestBuilder = RebindBuilder<TestAction>;
type TestTranslator = InputTranslator<TestAction>;
type TestRebind = InputRebind<TestAction>;

const TEST_SIZE: Size = Size {
    width:  800,
    height: 600
};

#[test]
fn test_empty() {
    let builder = TestBuilder::new(TEST_SIZE).build();
}

#[test]
fn test_conversion_from_rebind_to_translator() {
    assert!(true)
}
