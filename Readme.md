[rebind](https://github.com/burtonageo/rebind)
==============================================

Description
-----------

A library for binding input keys to actions, and modifying mouse behaviour. Keys can be
bound to actions, and then translated during runtime. `Keys` are mapped to `Actions` using
a `HashMap`, so lookup time is constant.

    // Minimal api example

    use glutin_window::GlutinWindow;
    use piston::event_loop::{EventMap, Events};
    use piston::input::{Event, Input};
    use piston::window::WindowSettings;
    use rebind::{Action, InputTranslator, RebindBuilder, Translated};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum MyAction {
        Action1, Action2
    }

    impl Action for MyAction { }

    fn main() {
        let window = WindowSettings::new("rebind-example", (800, 600)).build().expect("Could not create window");
        let translator = InputRebind::<MyAction>::new((800, 600).into())
            .with_action_mapping(Keyboard(Key::1), MyAction::Action1)
            .with_action_mapping(Keyboard(Key::A), MyAction::Action1)
            .with_action_mapping(Keyboard(Key::2), MyAction::Action2)
            .with_action_mapping(Keyboard(Key::B), MyAction::Action2)
            .build_translator();

        for e in window.events() {
            if let Event::Input(i) = e {
                if let Some(a) = translator.translate(e) {
                    match a {
                        Translated::Pressed(MyAction::Action1) => {
                            println!("Action 1 pressed!");
                        },
                        Translated::Pressed(MyAction::Action2) => {
                            println!("Action 2 pressed!");
                        },
                        _ => { }
                    }
                }
            }
        }
    }

License
-------

Copyright (c) 2015 Rebind developers.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

Main improvements to be made:
-----------------------------

* Implement conversion from InputTranslator to InputRebind
* Improve the API: Is the distinction between InputRebind and InputTranslator necessary or useful?
* Add serialisation
* Add mouse sensitivity options
* Add tests/benchmarks

Contributions welcome.
