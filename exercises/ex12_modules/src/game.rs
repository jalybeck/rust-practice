use crate::game::player::Player;

pub(crate) mod player;


pub(crate) struct Game {
    name: String,
    players: Vec<Player>,
}

impl Game {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            players: Vec::new(),
        }
    }

    pub(crate) fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub(crate)fn remove_player(&mut self, name: &str) -> Option<Player> {
        if let Some(index) = self.players.iter().position(|p| p.name == name) {
            Some(self.players.remove(index))
        } else {
            None
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game: {}, Players: {}", self.name, self.players.len())
    }
}