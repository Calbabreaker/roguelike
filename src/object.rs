#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    glyph: char,
    color: tcod::Color,
    pub name: String,
    pub solid: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, glyph: char, color: tcod::Color, name: &str, solid: bool) -> Self {
        return Object {
            x,
            y,
            glyph,
            color,
            name: name.into(),
            solid,
            alive: true,
        };
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_pos(&self) -> (i32, i32) {
        return (self.x, self.y);
    }

    pub fn draw(&self, console: &mut dyn tcod::Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.glyph, tcod::BackgroundFlag::None);
    }
}
