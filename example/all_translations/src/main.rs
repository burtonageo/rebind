#![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature = "nightly", plugin(rebind_plugins))]

#[macro_use]
extern crate conrod;
extern crate find_folder;
extern crate graphics;
extern crate rebind;
extern crate piston;
extern crate piston_window;
extern crate viewport;

use conrod::{Canvas, Color, Colorable, Frameable, Labelable, Positionable, Sizeable, Theme, Toggle, Widget};
use conrod::color::{BLACK, grayscale, GREEN, RED};
use piston::input::{Event, Input, Motion, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::Window;
use piston_window::{EventLoop, Glyphs, PistonWindow, WindowSettings};
use rebind::{Action, Builder, InputTranslator, Translated};
use std::cell::RefCell;
use std::rc::Rc;

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;

struct App {
    ui: Ui,
    translator: InputTranslator<CharacterAction>,
    character: Character,
    cursor: VirtualCursor,
    bg_color: Color
}

impl App {
    fn input(&mut self, input: &Input) {
        if let Some(t) = self.translator.translate(input) {
            match t {
                Translated::Press(action) => {
                    const CHARACTER_WALK_SPEED_INCREMENT: f64 = 400.0;
                    match action {
                        CharacterAction::Jump => {
                            println!("You jumped! Yay!");
                        }
                        CharacterAction::MoveLeft => {
                            if self.character.current_velocity[0] > (self.character.max_velocity[0] * -1.0) {
                                self.character.current_velocity[0] -= CHARACTER_WALK_SPEED_INCREMENT;
                            }
                        }
                        CharacterAction::MoveRight => {
                            if self.character.current_velocity[0] < self.character.max_velocity[0] {
                                self.character.current_velocity[0] += CHARACTER_WALK_SPEED_INCREMENT;
                            }
                        }
                    }
                }
                Translated::Release(_) => {
                    self.character.current_velocity = [0.0, 0.0];
                }
                Translated::Move(Motion::MouseCursor(x, y)) => {
                    self.cursor.position = [x, y];
                }
                _ => {}
            }
        }
    }

    fn update(&mut self, window: &PistonWindow, args: &UpdateArgs) {
        // We need to pass the window to update (and set the size here) because using
        // the update event from the window events queue is currently broken.
        let size = window.size();
        self.translator.set_size((size.width, size.height));

        // Update the character's velocity
        let ctl = self.character.topleft;
        let v = self.character.current_velocity;

        self.character.topleft = [ctl[0] + (v[0] * args.dt), ctl[1] + (v[1] * args.dt)];

        // Update the Ui
        //
        // This demo translates and rebinds in a single screen, forcing a clone of the
        // translator. This limitation is a result of the necessity of providing a different
        // interface for rebinding. One way you could avoid a clone every update/render is to
        // structure your application to pass the translator between separate options and game
        // screens.
        let mut rebind = self.translator.clone().into_rebind();
        let bg_col = self.bg_color;
        self.ui.set_widgets(|mut ui| {
            widget_ids! {
                CANVAS,
                X_INVERT_TOGGLE,
                Y_INVERT_TOGGLE
            }
            Canvas::new().color(bg_col).set(CANVAS, &mut ui);

            Toggle::new(rebind.get_x_motion_inverted())
                .top_left_with_margins_on(CANVAS, 0.0, 20.0)
                .w_h(80.0, 40.0)
                .color(grayscale(0.4))
                .frame(1.0)
                .label("Invert X")
                .label_color(bg_col.plain_contrast())
                .label_font_size(16)
                .react(|b| rebind.set_x_motion_inverted(b))
                .set(X_INVERT_TOGGLE, &mut ui);
            
            Toggle::new(rebind.get_y_motion_inverted())
                .top_left_with_margins_on(CANVAS, 0.0, 20.0)
                .w_h(80.0, 40.0)
                .color(grayscale(0.4))
                .frame(1.0)
                .label("Invert Y")
                .label_color(bg_col.plain_contrast())
                .label_font_size(16)
                .react(|b| rebind.set_y_motion_inverted(b))
                .set(Y_INVERT_TOGGLE, &mut ui);
        });
        self.translator = rebind.into();
    }

    fn render(&mut self, window: &PistonWindow, args: &RenderArgs) {
        window.draw_2d(|c, gl| {
            use graphics::*;

            // Draw the ui
            self.ui.draw(c, gl);
/*
            // Draw the character
            let square = rectangle::square(self.character.topleft[0],
                                           self.character.topleft[1],
                                           self.character.size);
            rectangle(self.character.color.to_fsa(), square, c.transform, gl);

            // Draw the cursor dot
            let dot = ellipse::circle(self.cursor.position[0],
                                      self.cursor.position[1],
                                      self.cursor.size);
            ellipse(self.cursor.color.to_fsa(), dot, c.transform, gl)*/
        });
    }
}

struct Character {
    color: Color,
    topleft: [f64; 2],
    current_velocity: [f64; 2],
    max_velocity: [f64; 2],
    size: f64
}

impl Character {
    fn new(col: Color, tl: [f64; 2], sz: f64) -> Self {
        Character {
            color: col,
            topleft: tl,
            current_velocity: [0.0, 0.0],
            max_velocity: [800.0, 0.0],
            size: sz
        }
    }
}

struct VirtualCursor {
    position: [f64; 2],
    color: Color,
    size: f64
}

impl VirtualCursor {
    fn new() -> Self {
        VirtualCursor {
            position: [0.0, 0.0],
            color: GREEN,
            size: 5.0
        }
    }
}

#[cfg_attr(feature = "nightly", derive(Action))]
#[cfg_attr(not(feature = "nightly"), derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd))]
enum CharacterAction {
    Jump,
    MoveLeft,
    MoveRight
}

#[cfg(not(feature = "nightly"))]
impl Action for CharacterAction { }

fn main() {
    use piston::input::keyboard::Key;
    use piston::input::Button::Keyboard;

    const WINDOW_SIZE: (u32, u32) = (800, 600);

    let window = WindowSettings::new("rebind-example", WINDOW_SIZE)
                     .exit_on_esc(true)
                     .fullscreen(false)
                     .vsync(true)
                     .build::<PistonWindow>()
                     .expect("Could not create main window");

    let translator = Builder::new(WINDOW_SIZE)
                         .with_mapping(CharacterAction::Jump, Keyboard(Key::Space))
                         .with_mapping(CharacterAction::MoveLeft, Keyboard(Key::Left))
                         .with_mapping(CharacterAction::MoveLeft, Keyboard(Key::A))
                         .with_mapping(CharacterAction::MoveRight, Keyboard(Key::Right))
                         .with_mapping(CharacterAction::MoveRight, Keyboard(Key::D))
                         .build_translator();

    let character = {
        const INITIAL_CHARACTER_POS: [f64; 2] = [WINDOW_SIZE.0 as f64 / 20.0, WINDOW_SIZE.1 as f64 * 0.85];
        Character::new(RED, INITIAL_CHARACTER_POS, 50.0)
    };

    let ui = {
        let glyph_cache = {
            let font_path = find_folder::Search::ParentsThenKids(3, 3)
                                .for_folder("assets")
                                .expect("Could not find assets folder")
                                .join("fonts/NotoSans/NotoSans-Regular.ttf");

            Glyphs::new(&font_path, window.factory.borrow().clone())
                .expect("Could not find font file within assets folder")
        };

        Ui::new(glyph_cache, Theme::default())
    };

    let mut app = App {
        ui: ui,
        translator: translator,
        character: character,
        cursor: VirtualCursor::new(),
        bg_color: BLACK
    };

    for e in window {
        if let Some(ref event) = e.event {
            app.ui.handle_event(event);
            // e.input(|i| app.input(&i));
            e.update(|u| app.update(&u));
            e.render(|r| app.render(&r));
        }
    }
}
