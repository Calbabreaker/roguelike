use crate::{
    map::{Map, Rect, MAP_HEIGHT, MAP_WIDTH},
    object::Object,
};
use tcod::console::Console;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub struct Game {
    root: tcod::console::Root,
    console: tcod::console::Offscreen,
    map: Map,
    objects: Vec<Object>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            root: tcod::RootConsole::initializer()
                .font("arial10x10.png", tcod::FontLayout::Tcod)
                .font_type(tcod::FontType::Greyscale)
                .size(SCREEN_WIDTH, SCREEN_HEIGHT)
                .title("Roguelike Game")
                .init(),
            console: tcod::OffscreenConsole::new(MAP_WIDTH, MAP_HEIGHT),
            map: Map::new(),
            objects: Vec::new(),
        };

        let player = create_object(&game.map.rooms[0], '@', tcod::colors::WHITE);
        let npc = create_object(&game.map.rooms[1], '@', tcod::colors::YELLOW);

        game.objects.push(player);
        game.objects.push(npc);
        game
    }

    pub fn run(&mut self) {
        while !self.root.window_closed() {
            self.render();
            self.handle_input();
        }
    }

    fn render(&mut self) {
        self.console.clear();

        self.map.recaculate_fov(&self.objects[0]);
        for object in &self.objects {
            if self.map.fov_map.is_in_fov(object.x, object.y) {
                object.draw(&mut self.console);
            }
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

fn create_object(room: &Rect, glyph: char, color: tcod::Color) -> Object {
    let (x, y) = room.get_center();
    Object::new(x, y, glyph, color)
}
