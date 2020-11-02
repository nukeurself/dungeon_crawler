use crate::entity::enemy::Enemy;

pub struct Vampire {}

impl Enemy for Vampire {
    fn name(&self) -> String {
        String::from("Vampire")
    }
}