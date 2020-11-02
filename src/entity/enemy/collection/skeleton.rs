use crate::entity::enemy::Enemy;

pub struct Skeleton {}

impl Enemy for Skeleton {
    fn name(&self) -> String {
        String::from("Skeleton")
    }
}