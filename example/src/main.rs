#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate rebind;
extern crate piston;
extern crate opengl_graphics;
extern crate viewport;

use conrod::{
    Background,
    Color,
    Colorable,
    Label,
    Labelable,
    Positionable,
    Theme,
    Toggle,
    Ui,
    Widget
};
use conrod::color::{black, green, red};
use glutin_window::GlutinWindow;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
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
type RcUi = Rc<RefCell<Ui<GlyphCache<'static>>>>;

struct App {
    window: RcWindow,
    graphics: RcGraphics,
    ui: RcUi,
    translator: InputTranslator<CharacterAction>,
    character: Character,
    cursor: VirtualCursor,
    bg_color: Color
}

widget_ids! {
    TITLE
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

    fn update(&mut self, args: &UpdateArgs) {
        // we need to pass the window to update (and set the size here) because using
        // the update event from the window events queue is currently broken.
        self.translator.set_size(self.window.borrow().size());

        // update the character's velocity
        let ctl = self.character.topleft;
        let v = self.character.current_velocity;

        self.character.topleft = [ctl[0] + (v[0] * args.dt), ctl[1] + (v[1] * args.dt)];
    }

    fn render(&mut self, args: &RenderArgs) {
        let mut gl_graphics = self.graphics.borrow_mut();
        let mut ui = &mut *self.ui.borrow_mut();

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

        // draw the ui
        {
            Background::new().color(self.bg_color).set(ui);

            Label::new("Hello")
                .xy(-150.0, -150.0)
                .font_size(32)
                .color(self.bg_color.plain_contrast())
                .set(TITLE, ui);
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

    let ui = {
        let glyph_cache = {
            let font_path = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .ok()
                .expect("Could not find assets folder")
                .join("fonts/NotoSans/NotoSans-Regular.ttf");

            GlyphCache::new(&font_path)
                .ok()
                .expect("Could not find font file within assets folder")
        };

        Ui::new(glyph_cache, Theme::default())
    };

    let mut app = App {
        window: Rc::new(RefCell::new(window)),
        graphics: Rc::new(RefCell::new(gl_graphics)),
        ui: Rc::new(RefCell::new(ui)),
        translator: translator,
        character: character,
        cursor: VirtualCursor::new(),
        bg_color: black()
    };

    for e in app.window.clone().events() {
        match e {
            Event::Input(i) => { app.input(&i); },
            Event::Update(u) => { app.update(&u); },
            Event::Render(r) => { app.render(&r); },
            _ => { }
        }
    }
}