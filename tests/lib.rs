#![allow(dead_code)]

extern crate rebind;
extern crate input;
extern crate window;

use input::Input;
use input::Button::Keyboard;
use input::keyboard::Key;
use rebind::{Action, Builder, ButtonTuple, InputRebind, InputTranslator, Translated};
use window::Size;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum TestAction {
    Action1,
    Action2,
    Action3,
    Action4,
    Action5,
    Action6,
    Action7,
    Action8,
    Action9,
    Action10
}

impl Action for TestAction { }

type TestBuilder = Builder<TestAction>;
type TestTranslator = InputTranslator<TestAction>;
type TestRebind = InputRebind<TestAction>;

fn create_prepopulated_builder_with_size(s: Size) -> TestBuilder {
    populate_builder(Builder::new(s))
}

fn create_prepopulated_builder() -> TestBuilder {
    populate_builder(Builder::default())
}

fn populate_builder(builder: TestBuilder) -> TestBuilder {
    builder.with_mapping(TestAction::Action1, Keyboard(Key::Up))
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
#[ignore]
fn test_conversion_from_rebind_to_translator() {
    let translator = create_prepopulated_builder().build_translator();

    let translator_clone = translator.clone();
    let converted_translator = Into::<TestTranslator>::into(Into::<TestRebind>::into(translator));

    drop(translator_clone);
    drop(converted_translator);
    //assert_eq!(translator_clone, converted_translator);
}

#[test]
fn test_add_button_to_translator_using_rebind() {
    use input::Button;
    const Q_KEY: Button = Keyboard(Key::Q);
    const E_KEY: Button = Keyboard(Key::E);

    let translator = create_prepopulated_builder().build_translator();
    let mut rebind = translator.into_rebind();
    rebind.insert_action_with_buttons(TestAction::Action5,
                                      ButtonTuple(Some(Q_KEY), Some(E_KEY), None));

    let translator = rebind.into_translator();

    assert_eq!(translator.translate(&Input::Press(Q_KEY)),
               Some(Translated::Press(TestAction::Action5)));
    assert_eq!(translator.translate(&Input::Press(E_KEY)),
               Some(Translated::Press(TestAction::Action5)));
}

const TEST_SIZE: Size = Size { width: 800, height: 600 };

#[test]
fn test_unmodified_mouse_input_works() {
    use input::Motion;
    let translator = create_prepopulated_builder_with_size(TEST_SIZE).build_translator();
    let mouse_motion = Input::Move(Motion::MouseCursor(45.0, 11.0));
    assert_eq!(translator.translate(&mouse_motion),
               Some(Translated::Move(Motion::MouseCursor(45.0, 11.0))));
}

#[test]
fn test_mirror_mouse_input_along_x_axis() {
    use input::Motion;
    let translator = create_prepopulated_builder_with_size(TEST_SIZE)
                         .x_motion_inverted(true)
                         .build_translator();
    let mouse_motion = Input::Move(Motion::MouseCursor(45.0, 11.0));
    assert_eq!(translator.translate(&mouse_motion),
               Some(Translated::Move(Motion::MouseCursor(755.0, 11.0)))); // 800 - 45 = 755
}

#[test]
fn test_mirror_mouse_input_along_y_axis() {
    use input::Motion;
    let translator = create_prepopulated_builder_with_size(TEST_SIZE)
                         .y_motion_inverted(true)
                         .build_translator();
    let mouse_motion = Input::Move(Motion::MouseCursor(45.0, 11.0));
    assert_eq!(translator.translate(&mouse_motion),
               Some(Translated::Move(Motion::MouseCursor(45.0, 589.0)))); // 800 - 45 = 755
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
