use crate::inventory::item::*;

pub struct WindStaff {}

impl Item for WindStaff {
    fn name(&self) -> String {
        String::from("Wind Staff")
    }

    fn damage(&self) -> Damage {
        Damage::Magic(Element::Wind(2))
    }

    fn armor(&self) -> Armor {
        Armor::Light(1)
    }

    fn color(&self) -> (u8, u8, u8) {
        (207, 228, 255)
    }
}
