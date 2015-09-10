extern crate conrod;
extern crate glutin_window;
extern crate graphics;
extern crate rebind;
extern crate piston;
extern crate opengl_graphics;
extern crate viewport;

use conrod::Color;
use conrod::color::{black, green, red};
use glutin_window::GlutinWindow;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventMap, Events};
use piston::input::{
    Event,
    Input,
    Motion,
    RenderArgs,
    RenderEvent,
    UpdateArgs,
    UpdateEvent
};
use piston::input::keyboard::Key;
use piston::input::Button::Keyboard;
use piston::window::{Window, WindowSettings};
use rebind::{Action, InputTranslator, RebindBuilder, Translated};
use std::cell::RefCell;
use std::rc::Rc;

type RcWindow = Rc<RefCell<GlutinWindow>>;
type RcGraphics = Rc<RefCell<GlGraphics>>;

struct App {
    window: RcWindow,
    graphics: RcGraphics,
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
                        },
                        CharacterAction::MoveLeft => {
                            if self.character.current_velocity[0] > (self.character.max_velocity[0] * -1.0) {
                                self.character.current_velocity[0] -= CHARACTER_WALK_SPEED_INCREMENT;
                            }
                        },
                        CharacterAction::MoveRight => {
                            if self.character.current_velocity[0] < self.character.max_velocity[0] {
                                self.character.current_velocity[0] += CHARACTER_WALK_SPEED_INCREMENT;
                            }
                        }
                    }
                },
                Translated::Release(_) => {
                    self.character.current_velocity = [0.0, 0.0];
                },
                Translated::Move(Motion::MouseCursor(x, y)) => {
                    self.cursor.position = [x, y];
                },
                _ => { }
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs, window: RcWindow) {
        // we need to pass the window to update (and set the size here) because using
        // the update event from the window events queue is currently broken.
        self.translator.set_size(window.borrow().size());

        // update the character's velocity
        let ctl = self.character.topleft;
        let v = self.character.current_velocity;

        self.character.topleft = [ctl[0] + (v[0] * args.dt), ctl[1] + (v[1] * args.dt)];
    }

    fn render(&mut self, args: &RenderArgs) {
        let mut gl_graphics = self.graphics.borrow_mut();

        // draw the background color
        {
            gl_graphics.draw(args.viewport(), |_, gl| clear(self.bg_color.to_fsa(), gl));
        }

        // draw the character
        {
            let square = rectangle::square(self.character.topleft[0],
                                           self.character.topleft[1],
                                           self.character.size);

            gl_graphics.draw(args.viewport(), |c, gl| rectangle(self.character.color.to_fsa(),
                                                                square,
                                                                c.transform,
                                                                gl));
        }

        // draw the cursor dot
        {
            let dot = ellipse::circle(self.cursor.position[0],
                                      self.cursor.position[1],
                                      self.cursor.size);
            gl_graphics.draw(args.viewport(), |c, gl| ellipse(self.cursor.color.to_fsa(),
                                                              dot,
                                                              c.transform,
                                                              gl));
        }
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
            color: green(),
            size: 5.0
        }
    }
}

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CharacterAction {
    Jump,
    MoveLeft,
    MoveRight
}

impl Action for CharacterAction { }

fn main() {
    const OPENGL: OpenGL = OpenGL::V3_2;
    const WINDOW_SIZE: (u32, u32) = (800, 600);

    let window = WindowSettings::new("calaxite", WINDOW_SIZE)
        .exit_on_esc(true)
        .fullscreen(false)
        .opengl(OPENGL)
        .vsync(true)
        .build()
        .ok()
        .expect("Could not create window: {}");

    let gl_graphics = GlGraphics::new(OPENGL);

    let translator = RebindBuilder::new(WINDOW_SIZE)
        .with_action_mapping(Keyboard(Key::Space), CharacterAction::Jump)
        .with_action_mapping(Keyboard(Key::Left),  CharacterAction::MoveLeft)
        .with_action_mapping(Keyboard(Key::A),     CharacterAction::MoveLeft)
        .with_action_mapping(Keyboard(Key::Right), CharacterAction::MoveRight)
        .with_action_mapping(Keyboard(Key::D),     CharacterAction::MoveRight)
        .y_motion_inverted(true)
        .build_translator();

    let character = {
        const INITIAL_CHARACTER_POS: [f64; 2] = [WINDOW_SIZE.0 as f64 / 20.0,
                                                 WINDOW_SIZE.1 as f64 * 0.85];
        Character::new(red(), INITIAL_CHARACTER_POS, 50.0)
    };

    let mut app = App {
        window: Rc::new(RefCell::new(window)),
        graphics: Rc::new(RefCell::new(gl_graphics)),
        translator: translator,
        character: character,
        cursor: VirtualCursor::new(),
        bg_color: black()
    };

    for e in  app.window.clone().events() {
        let app_window = app.window.clone();
        match e {
            Event::Render(r) => { app.render(&r); },
            Event::Update(u) => { app.update(&u, app_window); },
            Event::Input(i) => { app.input(&i); },
            _ => { }
        }
    }
}
