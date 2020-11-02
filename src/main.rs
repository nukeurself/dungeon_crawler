use dungeon_crawler::entity::*;

fn main() {
    let enemy = enemy::get_random_enemy();
    let hero = hero::Hero::new();
    enemy.summary();
    hero.summary();
}
