use crate::{
    map::{Map, MAP_HEIGHT, MAP_WIDTH},
    object::{Ai, Fighter, Object},
};
use rand::Rng;
use tcod::console::Console;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

const PLAYER_INDEX: usize = 0;
const MAX_MONSTERS_IN_ROOM: i32 = 5;

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
        let mut player = Object::new(start_x, start_y, '@', tcod::colors::WHITE, "Player", true);
        player.fighter = Some(Fighter {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        });

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
            self.object_update();
        }
    }

    fn render(&mut self) {
        self.console.clear();

        let player = &self.objects[PLAYER_INDEX];
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

            Key { code: Up, .. } => self.move_obj_by(PLAYER_INDEX, 0, -1),
            Key { code: Down, .. } => self.move_obj_by(PLAYER_INDEX, 0, 1),
            Key { code: Left, .. } => self.move_obj_by(PLAYER_INDEX, -1, 0),
            Key { code: Right, .. } => self.move_obj_by(PLAYER_INDEX, 1, 0),

            _ => {}
        }
    }

    fn object_update(&mut self) {
        let player = &self.objects[PLAYER_INDEX];
        if player.alive {
            for i in 0..self.objects.len() {
                if self.objects[i].ai.is_some() {
                    self.ai_basic_turn(i);
                }
            }
        }
    }

    fn ai_basic_turn(&mut self, monster_index: usize) {
        let monster = &self.objects[monster_index];
        let player = &self.objects[PLAYER_INDEX];
        if self.map.fov_map.is_in_fov(monster.x, monster.y) {
            // move towards player if too far away
            if monster.distance(&player) >= 2.0 {
                // cannot borrow position since function mut borrows self
                let (player_x, player_y) = player.get_pos();
                self.move_obj_towards(monster_index, player_x, player_y);
            } else if player.fighter.as_ref().map_or(false, |f| f.hp > 0) {
                let monster = &self.objects[monster_index];
                println!(
                    "The attack of the {} bounces off your shiny metal armor!",
                    monster.name
                );
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
        let num_monsters = rand::thread_rng().gen_range(0..MAX_MONSTERS_IN_ROOM + 1);
        let room = &self.map.rooms[room_index];

        for _ in 0..num_monsters {
            let x = rand::thread_rng().gen_range(room.x1 + 1..room.x2);
            let y = rand::thread_rng().gen_range(room.y1 + 1..room.y2);

            if !self.is_blocked(x, y) {
                let monster = match rand::random::<f32>() {
                    p if p < 0.8 => Object::new(x, y, 'o', tcod::colors::DARKER_GREEN, "Orc", true)
                        .add_fighter(10, 10, 0, 3)
                        .add_ai(Ai::Basic),
                    _ => Object::new(x, y, 'T', tcod::colors::DARK_GREEN, "Troll", true)
                        .add_fighter(16, 16, 1, 4)
                        .add_ai(Ai::Basic),
                };

                self.objects.push(monster);
            }
        }
    }

    fn move_obj_by(&mut self, obj_index: usize, diff_x: i32, diff_y: i32) {
        let (x, y) = &self.objects[obj_index].get_pos();
        let (target_x, target_y) = (x + diff_x, y + diff_y);

        if obj_index == PLAYER_INDEX {
            let target_index = self
                .objects
                .iter()
                .position(|object| object.get_pos() == (target_x, target_y));

            target_index.map(|target_index| {
                println!(
                    "The {} laughs at your puny efforts to attack him!",
                    self.objects[target_index].name
                );
            });
        }

        if !self.is_blocked(target_x, target_y) {
            self.objects[obj_index].set_pos(target_x, target_y);
        }
    }

    fn move_obj_towards(&mut self, obj_index: usize, target_x: i32, target_y: i32) {
        let (x, y) = &self.objects[obj_index].get_pos();
        let (diff_x, diff_y) = (target_x - x, target_y - y);
        let distance = ((diff_x.pow(2) + diff_y.pow(2)) as f32).sqrt();

        let step_x = (diff_x as f32 / distance).round() as i32;
        let step_y = (diff_y as f32 / distance).round() as i32;
        self.move_obj_by(obj_index, step_x, step_y);
    }
}
