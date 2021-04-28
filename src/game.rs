use crate::{
    map::{Map, MAP_HEIGHT, MAP_WIDTH},
    object::Object,
};
use tcod::console::Console;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const LIMIT_FPS: i32 = 20;

pub struct Game {
    root: tcod::console::Root,
    console: tcod::console::Offscreen,
    map: Map,
    objects: Vec<Object>,
}

impl Game {
    pub fn new() -> Self {
        tcod::system::set_fps(LIMIT_FPS);

        let root = tcod::console::Root::initializer()
            .font("arial10x10.png", tcod::FontLayout::Tcod)
            .font_type(tcod::FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Roguelike Game")
            .init();

        let console = tcod::console::Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

        let player = Object::new(25, 23, '@', tcod::colors::WHITE);

        // assumes 0 is the player
        let mut objects = Vec::new();
        objects.push(player);

        return Game {
            root,
            console,
            map: Map::new(&mut objects[0]),
            objects,
        };
    }

    pub fn run(&mut self) {
        while !self.root.window_closed() {
            self.render();
            self.update();
        }
    }

    fn update(&mut self) {
        self.handle_input();
    }

    fn render(&mut self) {
        self.console.clear();

        for object in &self.objects {
            object.draw(&mut self.console);
        }

        self.map.draw(&mut self.console);

        tcod::console::blit(
            &self.console,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );

        self.root.flush();
    }

    fn handle_input(&mut self) {
        use tcod::input::Key;
        use tcod::input::KeyCode::*;

        let player = &mut self.objects[0];
        let key = self.root.wait_for_keypress(true);

        match key {
            Key {
                printable: 'f',
                ctrl: true,
                ..
            } => self.root.set_fullscreen(!self.root.is_fullscreen()),

            Key { code: Escape, .. } => std::process::exit(0),

            Key { code: Up, .. } => player.move_by(0, -1, &self.map),
            Key { code: Down, .. } => player.move_by(0, 1, &self.map),
            Key { code: Left, .. } => player.move_by(-1, 0, &self.map),
            Key { code: Right, .. } => player.move_by(1, 0, &self.map),

            _ => {}
        }
    }
}
