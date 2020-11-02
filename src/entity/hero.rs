use colored::*;
use crate::inventory::item;

pub struct Hero {
    name: String,
    items: Vec<Box<dyn item::Item>>,
}

impl Hero {
    pub fn new() -> Hero {
        let random_item = item::get_random_item();
        let items: Vec<Box<dyn item::Item>> = vec![random_item];
        
        Hero {
            name: String::from("Noob"),
            items,
        }
    }

    pub fn summary(&self) {
        let item = &self.items[0];
        println!("{} wears {}", self.name.truecolor(0, 115, 209), item.summary());
    }
}