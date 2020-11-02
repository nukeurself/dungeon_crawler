use crate::entity::enemy::Enemy;

pub struct Zombie {}

impl Enemy for Zombie {
    fn name(&self) -> String {
        String::from("Zombie")
    }
}