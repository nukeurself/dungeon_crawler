use colored::*;
use rand::Rng;

use crate::inventory;
mod collection;

pub fn get_random_enemy() -> Box<dyn Enemy> {
    let number = rand::thread_rng().gen_range(1, 6);
    match number {
        1 => Box::new(collection::skeleton::Skeleton {}),
        2 => Box::new(collection::zombie::Zombie {}),
        3 => Box::new(collection::slime::Slime {}),
        4 => Box::new(collection::vampire::Vampire {}),
        5 => Box::new(collection::werewolf::Werewolf {}),
        _ => Box::new(collection::skeleton::Skeleton {}),
    }
}

pub trait Enemy {
    fn name(&self) -> String;
    fn new_inventory(&self) {
        self.inventory = inventory::Inventory::new();
    }
}



