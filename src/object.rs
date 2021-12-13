#[derive(Debug, PartialEq)]
pub enum DeathType {
    Player,
    Monster,
}

// combat related stuff (player, monsters, NPCs)
#[derive(Debug)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub death_type: DeathType,
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
    pub blocks: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
    pub alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, glyph: char, color: tcod::Color, name: &str, blocks: bool) -> Self {
        return Object {
            x,
            y,
            glyph,
            color,
            name: name.into(),
            blocks,
            fighter: None,
            ai: None,
            alive: true,
        };
    }

    pub fn add_fighter(
        mut self,
        max_hp: i32,
        hp: i32,
        defense: i32,
        power: i32,
        death_type: DeathType,
    ) -> Self {
        self.fighter = Some(Fighter {
            max_hp,
            hp,
            defense,
            power,
            death_type,
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

    pub fn take_damage(&mut self, damage: i32) {
        if let Some(fighter) = &mut self.fighter {
            fighter.hp -= damage;
            if fighter.hp <= 0 {
                self.die();
            }
        }
    }

    pub fn attack(&self, target: &mut Object) {
        let damage = self.fighter.as_ref().map_or(0, |f| f.power)
            - target.fighter.as_ref().map_or(0, |f| f.defense);

        if damage > 0 {
            println!(
                "{} attacks {} for {} hit points!",
                self.name, target.name, damage
            );
            target.take_damage(damage);
        } else {
            println!(
                "{} attacks {} but it has no effect!",
                self.name, target.name
            );
        }
    }

    pub fn die(&mut self) {
        if let Some(fighter) = &self.fighter {
            match fighter.death_type {
                DeathType::Player => {
                    println!("You died!");
                    self.color = tcod::colors::RED;
                }
                DeathType::Monster => {
                    println!("{} is dead!", self.name);
                    self.fighter = None;
                    self.color = tcod::colors::DARK_RED;
                }
            }

            self.glyph = '%';
            self.blocks = false;
            self.ai = None;
            self.alive = false;
            self.name = format!("Remains of {}", self.name);
        }
    }
}
