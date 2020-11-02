use crate::inventory::item::*;

pub struct Shield {}

impl Item for Shield {
    fn name(&self) -> String {
        String::from("Shield")
    }

    fn damage(&self) -> Damage {
        Damage::Melee(Armor::Light(1))
    }

    fn armor(&self) -> Armor {
        Armor::Heavy(1)
    }

    fn color(&self) -> (u8, u8, u8) {
        (153, 176, 4)
    }
}