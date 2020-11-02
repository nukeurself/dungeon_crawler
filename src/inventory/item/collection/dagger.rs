use crate::inventory::item::*;

pub struct Dagger {}

impl Item for Dagger {
    fn name(&self) -> String {
        String::from("Dagger")
    }
    
    fn damage(&self) -> Damage {
        Damage::Melee(Armor::Light(1))
    }

    fn armor(&self) -> Armor {
        Armor::Light(1)
    }

    fn color(&self) -> (u8, u8, u8) {
        (176, 21, 4)
    }
}