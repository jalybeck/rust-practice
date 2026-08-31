mod inventory;
use crate::game::player::inventory::Inventory;

pub(crate) struct Player {
    pub(crate) name: String,
    level: u32,
    inventory: Inventory,
}

impl Player {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            level: 1,
            inventory: Inventory::new(),
        }
    }

    pub(crate) fn level_up(&mut self) {
        self.level += 1;
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_level(&self) -> u32 {
        self.level
    }

    pub(crate) fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player: {}, Level: {}, {}", self.name, self.level, self.inventory)
    }
}