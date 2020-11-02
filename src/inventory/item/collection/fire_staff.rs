use crate::inventory::item::*;

pub struct FireStaff {}

impl Item for FireStaff {
    fn name(&self) -> String {
        String::from("Fire Staff")
    }

    fn damage(&self) -> Damage {
        Damage::Magic(Element::Fire(2))
    }

    fn armor(&self) -> Armor {
        Armor::Light(1)
    }

    fn color(&self) -> (u8, u8, u8) {
        (255, 31, 53)
    }
}