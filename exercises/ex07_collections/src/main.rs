use std::collections::HashMap;

struct Player {
    name: String,
    score: u32,
}

impl Player {
    fn new(name: String, score: u32) -> Self {
        Player { name, score }
    }
}

struct Leaderboard;

impl Leaderboard{

    fn display(players_map: &HashMap<String, Player>) {
        let mut players: Vec<&Player> = players_map.values().collect();
        players.sort_by(|a, b| b.score.cmp(&a.score));

        println!("Leaderboard:");
        for (i, player) in players.iter().enumerate() {
            println!("{}. {} - {}", i + 1, player.name, player.score);
        }
    }
}

fn main() {
    println!("Exercise 07: Collections");
    println!("---------------------");

    let mut player_map: HashMap<String, Player> = HashMap::new();
    player_map.insert("Alice".to_string(), Player::new("Alice".to_string(), 1200));
    player_map.insert("Bob".to_string(), Player::new("Bob".to_string(), 950));
    player_map.insert("Charlie".to_string(), Player::new("Charlie".to_string(), 1500));
    
    println!("HashMap contents:");
    for (name, player) in &player_map {
        println!("{}: {} - {}", name, player.name, player.score);
    }
    println!();

    Leaderboard::display(&player_map);

    if let Some(player) = player_map.get_mut("Alice") {
        println!("Found Alice with score: {}", player.score);
        player.score += 1000;
    }

    println!();
    Leaderboard::display(&player_map);
}