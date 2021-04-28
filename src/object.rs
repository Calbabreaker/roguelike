use crate::map::Map;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    character: char,
    color: tcod::Color,
}

impl Object {
    pub fn new(x: i32, y: i32, character: char, color: tcod::Color) -> Self {
        return Object {
            x,
            y,
            character,
            color,
        };
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        let desired_x = self.x + dx;
        let desired_y = self.y + dy;
        if !map[desired_x as usize][desired_y as usize].solid {
            self.x = desired_x;
            self.y = desired_y;
        }
    }

    pub fn draw(&self, console: &mut dyn tcod::Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.character, tcod::BackgroundFlag::None);
    }
}
