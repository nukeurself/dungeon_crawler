use crate::inventory::item::*;

pub struct Axe {}

impl Item for Axe {
    fn name(&self) -> String {
        String::from("Axe")
    }
    
    fn damage(&self) -> Damage {
        Damage::Melee(Armor::Heavy(3))
    }

    fn armor(&self) -> Armor {
        Armor::Medium(2)
    }

    fn color(&self) -> (u8, u8, u8) {
        (176, 21, 4)
    }
}