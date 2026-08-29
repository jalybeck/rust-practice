
struct Item {
    name: String,
    amount: u32,
    type_: String,
    value: u32,
}

impl Item {
    fn new(name: String, amount: u32, type_: String, value: u32) -> Self {
        Item {
            name,
            amount,
            type_,
            value,
        }
    }

    fn to_string(&self) -> String {
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

struct Player {
    name: String,
    hp: u32,
    level: u32,
    damage: u32,
    inventory: Inventory,
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            hp: 100,
            level: 1,
            damage: 10,
            inventory: Inventory::new(),
        }
    }

    fn take_damage(&mut self, amount: u32) {
        if amount >= self.hp {
            self.hp = 0;
        } else {
            self.hp -= amount;
        }
    }

    fn pick_up_item(&mut self, item: Item) {
        self.inventory.add_item(item);
    }

    fn inventory_to_string(&self) -> String {
        self.inventory.to_string()
    }

    fn to_string(&self) -> String {
        format!(
            "Player: {}, HP: {}, Level: {}, Damage: {}",
            self.name, self.hp, self.level, self.damage
        )
    }
}



fn main() {
    println!("Exercise 03: Structs");
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