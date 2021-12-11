// combat related stuff (player, monsters, NPCs)
#[derive(Debug)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Debug, PartialEq)]
pub enum Ai {
    Basic,
}

#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    glyph: char,
    color: tcod::Color,
    pub name: String,
    pub solid: bool,
    pub alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
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
            fighter: None,
            ai: None,
        };
    }

    pub fn add_fighter(mut self, max_hp: i32, hp: i32, defense: i32, power: i32) -> Self {
        self.fighter = Some(Fighter {
            max_hp,
            hp,
            defense,
            power,
        });
        self
    }

    pub fn add_ai(mut self, ai: Ai) -> Self {
        self.ai = Some(ai);
        self
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_pos(&self) -> (i32, i32) {
        return (self.x, self.y);
    }

    pub fn distance(&self, other: &Object) -> f32 {
        let (diff_x, diff_y) = (other.x - self.x, other.y - self.y);
        ((diff_x.pow(2) + diff_y.pow(2)) as f32).sqrt()
    }

    pub fn draw(&self, console: &mut dyn tcod::Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.glyph, tcod::BackgroundFlag::None);
    }
}
