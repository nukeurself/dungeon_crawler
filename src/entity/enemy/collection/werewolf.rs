use crate::entity::enemy::Enemy;

pub struct Werewolf {}

impl Enemy for Werewolf {
    fn name(&self) -> String {
        String::from("Werewolf")
    }
}