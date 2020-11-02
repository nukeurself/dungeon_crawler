use crate::inventory::item::*;

pub struct EarthStaff {}

impl Item for EarthStaff {
    fn name(&self) -> String {
        String::from("Earth Staff")
    }

    fn damage(&self) -> Damage {
        Damage::Magic(Element::Earth(2))
    }

    fn armor(&self) -> Armor {
        Armor::Light(1)
    }

    fn color(&self) -> (u8, u8, u8) {
        (138, 85, 0)
    }
}