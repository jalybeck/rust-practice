#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Action {
    Walking(Direction),
    Running(Direction),
    Still
}

#[derive(Debug)]
enum Weapon {
    Sword,
    Bow,
    Staff,
}

struct Player {
    name: String,
    action: Action,
    weapon: Option<Weapon>,
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            action: Action::Still,
            weapon: None
        }
    }

    fn print_status(&self) {
        match &self.action {
            Action::Walking(dir) => println!("Player: {}, Action: Walking, Direction: {:?}", self.name, dir),
            Action::Running(dir) => println!("Player: {}, Action: Running, Direction: {:?}", self.name, dir),
            Action::Still => println!("Player: {}, Action: Still", self.name),
        }
        
        match &self.weapon {
            Some(weapon) => println!("Weapon: {:?}", weapon),
            None => println!("No weapon"),
        }
    }

    fn walk(&mut self, direction: Direction) {
        self.action = Action::Walking(direction);
    }

    fn run(&mut self, direction: Direction) {
        self.action = Action::Running(direction);
    }

    fn stop(&mut self) {
        self.action = Action::Still;
    }

    fn pick_weapon(&mut self, weapon: Weapon) {
        self.weapon = Some(weapon);
    }
    
    fn drop_weapon(&mut self) {
        self.weapon = None;
    }
    
}

fn main() {
    println!("Exercise 4: Enums");
    println!("------------------");

    let mut player = Player::new(String::from("Player1"));
    player.print_status();

    player.walk(Direction::Right);
    player.print_status();

    player.run(Direction::Up);
    player.print_status();

    player.stop();
    player.print_status();

    player.pick_weapon(Weapon::Sword);
    player.print_status();

    player.drop_weapon();
    player.print_status();

}
