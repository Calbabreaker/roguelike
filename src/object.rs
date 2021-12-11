use crate::map::Map;

#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    glyph: char,
    color: tcod::Color,
}

impl Object {
    pub fn new(x: i32, y: i32, glyph: char, color: tcod::Color) -> Self {
        return Object {
            x,
            y,
            prev_x: 0,
            prev_y: 0,
            glyph,
            color,
        };
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        let desired_x = self.x + dx;
        let desired_y = self.y + dy;
        if !map.tiles[desired_x as usize][desired_y as usize].solid {
            self.prev_x = self.x;
            self.prev_y = self.x;
            self.x = desired_x;
            self.y = desired_y;
        }
    }

    pub fn draw(&self, console: &mut dyn tcod::Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.glyph, tcod::BackgroundFlag::None);
    }
}
