use crate::entity::enemy::Enemy;

pub struct Slime {}

impl Enemy for Slime {
    fn name(&self) -> String {
        String::from("Slime")
    }
}