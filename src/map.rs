use crate::object::Object;
use rand::Rng;
use tcod::console::Console;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

const COLOR_DARK_WALL: tcod::Color = tcod::Color { r: 0, g: 0, b: 100 };

const COLOR_LIGHT_WALL: tcod::Color = tcod::Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: tcod::Color = tcod::Color {
    r: 50,
    g: 50,
    b: 150,
};

const COLOR_LIGHT_GROUND: tcod::Color = tcod::Color {
    r: 200,
    g: 180,
    b: 50,
};

pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        return Rect {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        };
    }

    pub fn get_center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        return (center_x, center_y);
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        return (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub solid: bool,
    pub transparent: bool,
    pub explored: bool,
}

impl Tile {
    pub fn new_empty() -> Self {
        return Tile {
            solid: false,
            transparent: true,
            explored: false,
        };
    }

    pub fn new_wall() -> Self {
        return Tile {
            solid: true,
            transparent: false,
            explored: false,
        };
    }
}

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Rect>,
    pub fov_map: tcod::map::Map,
}

impl Map {
    pub fn new() -> Self {
        let mut map = Map {
            tiles: vec![vec![Tile::new_wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            fov_map: tcod::map::Map::new(MAP_WIDTH, MAP_HEIGHT),
            rooms: Vec::new(),
        };

        for _ in 0..MAX_ROOMS {
            let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);
            let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);
            let x = rand::thread_rng().gen_range(0..MAP_WIDTH - w);
            let y = rand::thread_rng().gen_range(0..MAP_HEIGHT - h);

            let new_room = Rect::new(x, y, w, h);

            let invalid = map
                .rooms
                .iter()
                .any(|other_room| new_room.intersects_with(other_room));

            if !invalid {
                if !map.rooms.is_empty() {
                    let (center_x, center_y) = new_room.get_center();
                    let (prev_center_x, prev_center_y) = map.rooms.last().unwrap().get_center();

                    if rand::random() {
                        map.create_htunnel(prev_center_x, center_x, prev_center_y);
                        map.create_vtunnel(prev_center_y, center_y, center_x);
                    } else {
                        map.create_vtunnel(prev_center_y, center_y, prev_center_x);
                        map.create_htunnel(prev_center_x, center_x, center_y);
                    }
                }

                map.create_room(&new_room);
                map.rooms.push(new_room);
            }
        }

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let tile = map.tiles[x as usize][y as usize];
                map.fov_map.set(x, y, tile.transparent, !tile.solid);
            }
        }

        map
    }

    pub fn draw(&mut self, console: &mut tcod::console::Offscreen) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let transparent = self.tiles[x as usize][y as usize].transparent;
                let visible = self.fov_map.is_in_fov(x, y);
                let color = match (visible, transparent) {
                    (false, true) => COLOR_DARK_WALL,
                    (false, false) => COLOR_DARK_GROUND,

                    (true, true) => COLOR_LIGHT_WALL,
                    (true, false) => COLOR_LIGHT_GROUND,
                };

                let explored = &mut self.tiles[x as usize][y as usize].explored;
                if visible {
                    *explored = true;
                }

                if *explored {
                    console.set_char_background(x, y, color, tcod::BackgroundFlag::Set);
                }
            }
        }
    }

    pub fn recaculate_fov(&mut self, player: &Object) {
        let should_recalculate = player.x != player.prev_x || player.y != player.prev_y;
        if should_recalculate {
            self.fov_map
                .compute_fov(player.x, player.y, 10, true, tcod::map::FovAlgorithm::Basic);
        }
    }

    fn create_room(&mut self, room: &Rect) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                self.tiles[x as usize][y as usize] = Tile::new_empty();
            }
        }
    }

    fn create_htunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in std::cmp::min(x1, x2)..(std::cmp::max(x1, x2) + 1) {
            self.tiles[x as usize][y as usize] = Tile::new_empty();
        }
    }

    fn create_vtunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in std::cmp::min(y1, y2)..(std::cmp::max(y1, y2) + 1) {
            self.tiles[x as usize][y as usize] = Tile::new_empty();
        }
    }
}
