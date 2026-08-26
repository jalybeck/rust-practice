fn main() {
    println!("Exercise 1: Basics");
    println!("------------------");

    let player_name = "Alice";
    let player_max_hp = 100.0;
    let mut player_current_hp = player_max_hp;    
    let mut player_position = (20, 20);

    println!("Player {} has {} HP", player_name, player_current_hp);
    println!("Player {} is at position ({}, {})", player_name, player_position.0, player_position.1);

    player_position.0 += 5;
    player_position.1 -= 3;
    println!("Player {} moved to position ({}, {})", player_name, player_position.0, player_position.1);

    let damage = 30.9;
    player_current_hp -= damage;
    println!("Player {} took {} damage and now has {} HP", player_name, damage, player_current_hp);

    let damage = damage - 20.1;
    player_current_hp -= damage;

    println!("Player {} dropped to hole and took {} damage and now has {} HP", player_name, damage, player_current_hp);
}