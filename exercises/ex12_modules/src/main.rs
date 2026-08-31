mod game;

fn main() {
    println!("Exercise 12: Modules");
    println!("-----------------");

    let mut game = game::Game::new("Adventure".to_string());
    let player1 = game::player::Player::new("Alice".to_string());
    let player2 = game::player::Player::new("Bob".to_string());

    game.add_player(player1);
    game.add_player(player2);

    println!("{}", game);
    
    if let Some(removed) = game.remove_player("Alice") {
        println!("Removed player: {}", removed.get_name());
    }

    println!("{}", game);
    
    // Level up Bob
    if let Some(bob) = game.remove_player("Bob") {
        let mut bob = bob;
        bob.level_up();
        println!("{} leveled up to level {}", bob.get_name(), bob.get_level());
        game.add_player(bob);
    }
    
    println!("{}", game);
}