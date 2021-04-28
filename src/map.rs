pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub const COLOR_DARK_WALL: tcod::Color = tcod::Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: tcod::Color = tcod::Color {
    r: 50,
    g: 50,
    b: 100,
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
        let center_y = (self.y2 + self.y2) / 2;
        return (center_x, center_y);
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        return (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 <= other.y1);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub solid: bool,
    pub transparent: bool,
}

impl Tile {
    pub fn new_empty() -> Self {
        return Tile {
            solid: false,
            transparent: true,
        };
    }

    pub fn new_wall() -> Self {
        return Tile {
            solid: true,
            transparent: false,
        };
    }
}

pub type Map = Vec<Vec<Tile>>;

pub fn make_map() -> Map {
    let mut map = vec![vec![Tile::new_wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);
    create_room(room1, &mut map);
    create_room(room2, &mut map);
    create_htunnel(25, 50, 23, &mut map);
    return map;
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::new_empty();
        }
    }
}

fn create_htunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in std::cmp::min(x1, x2)..(std::cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::new_empty();
    }
}

fn create_vtunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in std::cmp::min(y1, y2)..(std::cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::new_empty();
    }
}
