pub mod item;

pub struct Inventory {
    items: Vec<Box<dyn item::Item>>
}

impl Inventory {
    pub fn new() -> Inventory {
        let items = vec![];
        
        Inventory {
            items
        }
    }

    pub fn add_random_items(&mut self, num: i32) {
        for i in 0..num {
            let item = item::get_random_item();
            self.items.push(item);
        }
    } 
}