use crate::{
    map::{Map, MAP_HEIGHT, MAP_WIDTH},
    object::Object,
};
use rand::Rng;
use tcod::console::Console;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

const PLAYER: usize = 0;
const MAX_ROOM_MONSTERS: i32 = 3;

pub struct Game {
    root: tcod::console::Root,
    console: tcod::console::Offscreen,
    map: Map,
    objects: Vec<Object>,
    prev_player_pos: (i32, i32),
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
            prev_player_pos: (-1, -1),
        };

        let (start_x, start_y) = game.map.rooms[0].get_center();
        let player = Object::new(start_x, start_y, '@', tcod::colors::WHITE, "Player", true);
        game.objects.push(player);

        for i in 0..game.map.rooms.len() {
            game.place_random_objects(i);
        }

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

        let player = &self.objects[PLAYER];
        if player.get_pos() != self.prev_player_pos {
            self.map.recaculate_fov(player);
        }

        self.prev_player_pos = player.get_pos();

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

        let key = self.root.wait_for_keypress(true);

        match key {
            Key {
                printable: 'f',
                ctrl: true,
                ..
            } => self.root.set_fullscreen(!self.root.is_fullscreen()),

            Key { code: Escape, .. } => std::process::exit(0),

            Key { code: Up, .. } => self.move_obj_by(PLAYER, 0, -1),
            Key { code: Down, .. } => self.move_obj_by(PLAYER, 0, 1),
            Key { code: Left, .. } => self.move_obj_by(PLAYER, -1, 0),
            Key { code: Right, .. } => self.move_obj_by(PLAYER, 1, 0),

            _ => {
                let player = &self.objects[PLAYER];
                if player.alive {
                    for object in &self.objects {
                        // make sure object not player
                        if (object as *const _) != (player as *const _) {
                            println!("The {} growls!", object.name);
                        }
                    }
                }
            }
        }
    }

    fn is_blocked(&self, x: i32, y: i32) -> bool {
        if self.map.tiles[x as usize][y as usize].solid {
            return true;
        }

        self.objects
            .iter()
            .any(|object| object.solid && object.get_pos() == (x, y))
    }

    fn place_random_objects(&mut self, room_index: usize) {
        let num_monsters = rand::thread_rng().gen_range(0..MAX_ROOM_MONSTERS + 1);
        let room = &self.map.rooms[room_index];

        for _ in 0..num_monsters {
            let x = rand::thread_rng().gen_range(room.x1 + 1..room.x2);
            let y = rand::thread_rng().gen_range(room.y1 + 1..room.y2);

            if !self.is_blocked(x, y) {
                let monster = match rand::random::<f32>() {
                    p if p < 0.8 => {
                        Object::new(x, y, 'o', tcod::colors::DESATURATED_GREEN, "Orc", true)
                    }
                    _ => Object::new(x, y, 'T', tcod::colors::DARK_GREEN, "Troll", true),
                };

                self.objects.push(monster);
            }
        }
    }

    fn move_obj_by(&mut self, obj_index: usize, dx: i32, dy: i32) {
        let (x, y) = &self.objects[obj_index].get_pos();
        let (desired_x, desired_y) = (x + dx, y + dy);

        if obj_index == PLAYER {
            let target_index = self
                .objects
                .iter()
                .position(|object| object.get_pos() == (desired_x, desired_y));

            match target_index {
                Some(target_index) => {
                    println!(
                        "The {} laughs at your puny efforts to attack him!",
                        self.objects[target_index].name
                    );
                }
                None => {}
            }
        }

        if !self.is_blocked(desired_x, desired_y) {
            self.objects[obj_index].set_pos(desired_x, desired_y);
        }
    }
}
