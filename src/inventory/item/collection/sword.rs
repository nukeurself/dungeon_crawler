use crate::inventory::item::*;

pub struct Sword {}

impl Item for Sword {
    fn name(&self) -> String {
        String::from("Sword")
    }

    fn damage(&self) -> Damage {
        Damage::Melee(Armor::Medium(2))
    }

    fn armor(&self) -> Armor {
        Armor::Medium(2)
    }

    fn color(&self) -> (u8, u8, u8) {
        (140, 219, 255)
    }
}