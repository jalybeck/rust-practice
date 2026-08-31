
use ex14_testing::{Item, Player};  

fn main() {
    println!("Exercise 14: Testing");
    println!("-------------------");

    println!("Creating items...");
    let sword = Item::new("Sword".to_string(), 1, "Weapon".to_string(), 10);
    let potion = Item::new("Health Potion".to_string(), 3, "Consumable".to_string(),5);
    let body_armor = Item::new("Body Armor".to_string(), 1, "Armor".to_string(), 5);

    println!("Creating player...");
    let mut player = Player::new("Hero".to_string());
    player.pick_up_item(sword);
    player.pick_up_item(potion);
    player.pick_up_item(body_armor);
    println!("Player Inventory:\n{}", player.inventory_to_string());

    println!("Player takes 20 damage...");
    player.take_damage(20);
    println!("Player Status: {}", player.to_string());

}