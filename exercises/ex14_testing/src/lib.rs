
pub struct Item {
    pub name: String,
    pub amount: u32,
    pub type_: String,
    pub value: u32,
}

impl Item {
    pub fn new(name: String, amount: u32, type_: String, value: u32) -> Self {
        Item {
            name,
            amount,
            type_,
            value,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Item: {}, Amount: {}, Type: {}, Value: {}",
            self.name, self.amount, self.type_, self.value
        )
    }
}

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            items: Vec::new(),
        }
    }

    fn get_item_from_slot(&self, slot: usize) -> Option<&Item> {
        self.items.get(slot)
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn to_string(&self) -> String {
        let mut inventory_str = String::new();
        for (index, item) in self.items.iter().enumerate() {
            inventory_str.push_str(&format!("Slot {}: {}\n", index, item.to_string()));
        }
        inventory_str
    }
}

pub struct Player {
    pub name: String,
    pub hp: u32,
    pub level: u32,
    pub damage: u32,
    inventory: Inventory,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            hp: 100,
            level: 1,
            damage: 10,
            inventory: Inventory::new(),
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        if amount >= self.hp {
            self.hp = 0;
        } else {
            self.hp -= amount;
        }
    }

    pub fn pick_up_item(&mut self, item: Item) {
        self.inventory.add_item(item);
    }

    pub fn inventory_to_string(&self) -> String {
        self.inventory.to_string()
    }

    pub fn to_string(&self) -> String {
        format!(
            "Player: {}, HP: {}, Level: {}, Damage: {}",
            self.name, self.hp, self.level, self.damage
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_takes_damage() {
        let mut player = Player::new("TestHero".to_string());
        player.take_damage(50);
        assert_eq!(player.hp, 50);
        player.take_damage(60);
        assert_eq!(player.hp, 0);
    }

    #[test]
    fn test_inventory_add_item() {
        let mut inventory = Inventory::new();
        let item = Item::new("TestItem".to_string(), 1, "TestType".to_string(), 10);
        inventory.add_item(item);
        assert_eq!(inventory.get_item_from_slot(0).unwrap().name, "TestItem");
    }
}