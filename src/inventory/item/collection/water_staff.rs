use crate::inventory::item::*;

pub struct WaterStaff {}

impl Item for WaterStaff {
    fn name(&self) -> String {
        String::from("Water Staff")
    }

    fn damage(&self) -> Damage {
        Damage::Magic(Element::Water(2))
    }

    fn armor(&self) -> Armor {
        Armor::Light(1)
    }

    fn color(&self) -> (u8, u8, u8) {
        (0, 85, 255)
    }
}