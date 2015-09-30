#![allow(dead_code, unused_variables)]

extern crate rebind;
extern crate input;

use rebind::{Action, ButtonTuple, InputTranslator, RebindBuilder, InputRebind, Translated};
use input::Input;
use input::Button::Keyboard;
use input::keyboard::Key;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum TestAction {
    Action1, Action2, Action3, Action4, Action5,
    Action6, Action7, Action8, Action9, Action10
}

impl Action for TestAction { }

type TestBuilder = RebindBuilder<TestAction>;
type TestTranslator = InputTranslator<TestAction>;
type TestRebind = InputRebind<TestAction>;

fn create_prepopulated_builder() -> TestBuilder {
    RebindBuilder::default()
        .with_mapping(TestAction::Action1, Keyboard(Key::Up))
        .with_mapping(TestAction::Action1, Keyboard(Key::W))
        .with_mapping(TestAction::Action2, Keyboard(Key::Down))
        .with_mapping(TestAction::Action2, Keyboard(Key::S))
        .with_mapping(TestAction::Action3, Keyboard(Key::Left))
        .with_mapping(TestAction::Action3, Keyboard(Key::A))
        .with_mapping(TestAction::Action4, Keyboard(Key::Right))
        .with_mapping(TestAction::Action4, Keyboard(Key::D))
}

#[test]
fn test_translator_get_action_from_buttonpress() {
    let translator = create_prepopulated_builder().build_translator();

    assert_eq!(translator.translate(&Input::Press(Keyboard(Key::Down))).unwrap(),
               Translated::Press(TestAction::Action2));

    assert_eq!(translator.translate(&Input::Press(Keyboard(Key::D))).unwrap(),
               Translated::Press(TestAction::Action4));

    assert_eq!(translator.translate(&Input::Press(Keyboard(Key::Left))).unwrap(),
               Translated::Press(TestAction::Action3));

    assert_eq!(translator.translate(&Input::Press(Keyboard(Key::W))).unwrap(),
               Translated::Press(TestAction::Action1));
}

#[test]
fn test_conversion_from_rebind_to_translator() {
    let translator = create_prepopulated_builder().build_translator();

    let translator_clone = translator.clone();
    let converted_translator = Into::<TestTranslator>::into(
            Into::<TestRebind>::into(translator));

    assert_eq!(translator_clone, converted_translator);
}

#[test]
fn test_add_button_to_translator_using_rebind() {
    use input::Button;
    const Q_KEY: Button = Keyboard(Key::Q);
    const E_KEY: Button = Keyboard(Key::E);

    let translator = create_prepopulated_builder().build_translator();
    let mut rebind = translator.into_rebind();
    rebind.insert_action_with_buttons(TestAction::Action5, ButtonTuple(Some(Q_KEY), Some(E_KEY), None));

    let translator = rebind.into_translator();

    assert_eq!(translator.translate(&Input::Press(Q_KEY)), Some(Translated::Press(TestAction::Action5)));
    assert_eq!(translator.translate(&Input::Press(E_KEY)), Some(Translated::Press(TestAction::Action5)));
}

#[test]
fn test_get_num_buttons_set() {
    let mut bt = ButtonTuple::new();
    assert_eq!(bt.num_buttons_set(), 0);

    bt.0 = Some(Keyboard(Key::Z));
    assert_eq!(bt.num_buttons_set(), 1);

    bt.1 = Some(Keyboard(Key::Q));
    assert_eq!(bt.num_buttons_set(), 2);

    bt.2 = Some(Keyboard(Key::D0));
    assert_eq!(bt.num_buttons_set(), 3);
}

#[test]
fn test_get_button_iter_len() {
    let bt = ButtonTuple(Some(Keyboard(Key::B)), None, None);
    let mut bti = bt.iter();
    assert_eq!(bti.len(), 3);
    let _ = bti.next();
    assert_eq!(bti.len(), 2);
    let _ = bti.next();
    assert_eq!(bti.len(), 1);
    let _ = bti.next();
    assert_eq!(bti.len(), 0);
    let _ = bti.next();
    assert_eq!(bti.len(), 0);
}