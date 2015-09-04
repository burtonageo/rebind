#![allow(unused_variables)]

extern crate glutin_window;
extern crate graphics;
extern crate rebind;
extern crate piston;
extern crate opengl_graphics;
extern crate viewport;

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

struct App {
    window: RcWindow,
    graphics: Rc<RefCell<GlGraphics>>,
    translator: InputTranslator<CharacterAction>,
    character: Character,
    virtual_cursor_pos: [f64; 2],
    bg_color: [f32; 4]
}

impl App {
    fn input(&mut self, input: &Input) {
        if let Some(t) = self.translator.translate(input) {
            match t {
                Translated::Press(action) => {
                    const CHARACTER_WALK_SPEED_INCREMENT: f64 = 400.0f64;
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
                    self.virtual_cursor_pos = [x, y];
                },
                _ => { }
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs, window: RcWindow) {
        // we need to pass the window into update (and set the size here) because
        // using the update event from the window events is currently broken.
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
            gl_graphics.draw(args.viewport(), |_, gl| clear(self.bg_color, gl));
        }

        // draw the character
        {
            let square = rectangle::square(self.character.topleft[0],
                                           self.character.topleft[1],
                                           self.character.size);

            gl_graphics.draw(args.viewport(), |c, gl| rectangle(self.character.color, square, c.transform, gl));
        }

        // draw the cursor dot
        {
            let dot = ellipse::circle(self.virtual_cursor_pos[0], self.virtual_cursor_pos[1], 5.0);
            const CURSOR_DOT_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            gl_graphics.draw(args.viewport(), |c, gl| ellipse(CURSOR_DOT_COLOR, dot, c.transform, gl));
        }
    }
}

struct Character {
    color: [f32; 4],
    topleft: [f64; 2],
    current_velocity: [f64; 2],
    max_velocity: [f64; 2],
    size: f64
}

impl Character {
    fn new(col: [f32; 4], tl: [f64; 2], sz: f64) -> Self {
        Character {
            color: col,
            topleft: tl,
            current_velocity: [0.0f64, 0.0],
            max_velocity: [800.0, 0.0],
            size: sz
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
        .unwrap_or_else(|e| panic!("Could not create window: {}", e));

    let gl_graphics = GlGraphics::new(OPENGL);

    let translator = RebindBuilder::new(WINDOW_SIZE)
        .with_action_mapping(Keyboard(Key::Space), CharacterAction::Jump)
        .with_action_mapping(Keyboard(Key::Left),  CharacterAction::MoveLeft)
        .with_action_mapping(Keyboard(Key::A),     CharacterAction::MoveLeft)
        .with_action_mapping(Keyboard(Key::Right), CharacterAction::MoveRight)
        .with_action_mapping(Keyboard(Key::D),     CharacterAction::MoveRight)
        .build_translator();

    let character = Character::new([1.0, 0.0, 0.0, 1.0],
                                   [30.0, (WINDOW_SIZE.1 as f64) * 0.85],
                                   50.0);

    let mut app = App {
        window: Rc::new(RefCell::new(window)),
        graphics: Rc::new(RefCell::new(gl_graphics)),
        translator: translator,
        character: character,
        virtual_cursor_pos: [0.0, 0.0],
        bg_color: [0.0, 0.0, 0.0, 1.0] // black background
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
