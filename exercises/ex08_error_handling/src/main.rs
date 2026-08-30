struct Player {
    name: String,
    level: u32,
    hp: u32,
    gold: u32,
}

impl Player {
    fn read_from_file(path: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(path)?;
        let mut lines = contents.lines();
        let name = lines.next();
        let level = lines.next();
        let hp = lines.next();
        let gold = lines.next();

        println!(
            "Read lines: name={:?}, level={:?}, hp={:?}, gold={:?}",
            name, level, hp, gold
        );

        match (name, level, hp, gold) {
            (Some(name), Some(level), Some(hp), Some(gold)) => {
                let name = name
                    .split("=")
                    .nth(1)
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing name")
                    })?
                    .to_string();
                let level = level
                    .split("=")
                    .nth(1)
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing level")
                    })?
                    .parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                let hp = hp
                    .split("=")
                    .nth(1)
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing hp")
                    })?
                    .parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                let gold = gold
                    .split("=")
                    .nth(1)
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing gold")
                    })?
                    .parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                Ok(Player {
                    name,
                    level,
                    hp,
                    gold,
                })
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing player data",
            )),
        }
    }
}

fn main() {
    println!("Exercise 08: Error Handling");
    println!("---------------------------");

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/player.txt");
    match Player::read_from_file(path) {
        Ok(player) => println!(
            "Player loaded: {} (Level {}, HP {}, Gold {})",
            player.name, player.level, player.hp, player.gold
        ),
        Err(e) => eprintln!("Failed to load player: {}", e),
    }
}
