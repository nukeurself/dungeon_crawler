use colored::*;
use rand::Rng;

pub mod collection;

pub fn get_random_item() -> Box<dyn Item> {
    let number = rand::thread_rng().gen_range(1, 9);
    match number {
        1 => Box::new(collection::dagger::Dagger {}),
        2 => Box::new(collection::sword::Sword {}),
        3 => Box::new(collection::axe::Axe {}),
        4 => Box::new(collection::fire_staff::FireStaff {}),
        5 => Box::new(collection::water_staff::WaterStaff {}),
        6 => Box::new(collection::earth_staff::EarthStaff {}),
        7 => Box::new(collection::wind_staff::WindStaff {}),
        8 => Box::new(collection::shield::Shield {}),
        _ => Box::new(collection::sword::Sword {}),
    }
}

pub enum Armor {
    Light(i32),
    Medium(i32),
    Heavy(i32),
}

pub enum Element {
    Fire(i32),
    Water(i32),
    Earth(i32),
    Wind(i32),
}

pub enum Damage {
    Melee(Armor),
    Range(Armor),
    Magic(Element),
}

pub trait Item {
    fn name(&self) -> String;
    fn damage(&self) -> Damage;
    fn armor(&self) -> Armor;
    fn color(&self) -> (u8, u8, u8);

    fn level(&self) -> i32 {
        1
    }

    fn summary(&self) -> String {
        let damage = damage_to_string(self.damage());
        let armor = armor_to_string(self.armor());
        let (r, g, b) = self.color();
        
        format!(
            "{} (lvl {}, {} dmg, {} def)", 
            self.name().truecolor(r, g, b), 
            self.level().to_string().truecolor(255, 184, 31),
            damage,
            armor,
        )
    }
}

fn damage_to_string(damage: Damage) -> String {
    match damage {
        Damage::Melee(armor) => armor_to_string(armor),
        Damage::Range(armor) => armor_to_string(armor),
        Damage::Magic(element) => magic_to_string(element),
    }
}

fn magic_to_string(element: Element) -> String {
    let color_value = (255, 184, 31);
    let color_name = (176, 4, 44);
    match element {
        Element::Fire(value) => format_string(value, "Fire", color_value, color_name),
        Element::Water(value) => format_string(value, "Water", color_value, color_name),
        Element::Earth(value) => format_string(value, "Earth", color_value, color_name),
        Element::Wind(value) => format_string(value, "Wind", color_value, color_name),
    }
}

fn armor_to_string(armor: Armor) -> String {
    let color_value = (255, 184, 31);
    let color_name = (176, 4, 44);
    match armor {
        Armor::Light(value) => format_string(value, "Light", color_value, color_name),
        Armor::Medium(value) => format_string(value, "Medium", color_value, color_name),
        Armor::Heavy(value) => format_string(value, "Heavy", color_value, color_name),
    }
}

fn format_string(value: i32, name: &str, color_1: (u8, u8, u8), color_2: (u8, u8, u8)) -> String {
    let (r1, g1, b1) = color_1;
    let (r2, g2, b2) = color_2;
    format!("{} {}", value.to_string().truecolor(r1,g1,b1), name.truecolor(r2,g2,b2))
}