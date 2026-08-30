trait Updateable {
    fn update(&self, delta_time: f32);
}

trait Drawable {
    fn draw(&self);

    fn default_draw(&self) {
        println!("Drawing default object");
    }
}

struct Player {
    name: String,
    level: u32,
    hp: u32,
    gold: u32,
}

impl Updateable for Player {
    fn update(&self, delta_time: f32) {
        println!(
            "Updating player: {} (Level {}, HP {}, Gold {}) with delta_time: {}",
            self.name, self.level, self.hp, self.gold, delta_time
        );
    }
}

impl Drawable for Player {
    fn draw(&self) {
        Drawable::default_draw(self);
        println!(
            "Drawing player: {} ", self.name
        );
    }
}

fn draw_object(obj: &impl Drawable) {
    obj.draw();
}

fn update_object(obj: &impl Updateable, delta_time: f32) {
    obj.update(delta_time);
}

fn draw_and_update(obj: &(impl Drawable + Updateable), delta_time: f32) {
    draw_object(obj);
    update_object(obj, delta_time);
}

fn main() {
    println!("Exercise 09: Traits");
    println!("-------------------");
    let player = Player {
        name: String::from("Alice"),
        level: 12,
        hp: 85,
        gold: 420,
    };
    
    draw_and_update(&player, 0.016);
}