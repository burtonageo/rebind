#![allow(dead_code, unused_variables)]

use {Action, InputTranslator, RebindBuilder, InputRebind};
use window::Size;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
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

fn create_prepopulated_builder() -> TestBuilder {
    use input::keyboard::Key;
    use input::Button::Keyboard;

    RebindBuilder::new(TEST_SIZE)
        .with_action_mapping(Keyboard(Key::Up),    TestAction::Action1)
        .with_action_mapping(Keyboard(Key::W),     TestAction::Action1)
        .with_action_mapping(Keyboard(Key::Down),  TestAction::Action2)
        .with_action_mapping(Keyboard(Key::S),     TestAction::Action2)
        .with_action_mapping(Keyboard(Key::Left),  TestAction::Action3)
        .with_action_mapping(Keyboard(Key::A),     TestAction::Action3)
        .with_action_mapping(Keyboard(Key::Right), TestAction::Action4)
        .with_action_mapping(Keyboard(Key::D),     TestAction::Action4)
}

#[test]
fn test_empty() {
    let builder = TestBuilder::new(TEST_SIZE).build_translator();
}

#[test]
fn test_conversion_from_rebind_to_translator() {
    let translator = create_prepopulated_builder().build_translator();

    let translator_clone = translator.clone();
    let converted_translator = Into::<TestTranslator>::into(
            Into::<TestRebind>::into(translator));

    assert_eq!(translator_clone, converted_translator);
}
