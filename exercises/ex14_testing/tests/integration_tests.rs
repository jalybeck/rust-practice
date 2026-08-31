// tests/integration_test.rs
use ex14_testing::{Item, Player};

#[test]
fn test_item_creation() {
    let sword = Item::new("Sword".into(), 1, "Weapon".into(), 10);
    assert_eq!(sword.to_string(), "Item: Sword, Amount: 1, Type: Weapon, Value: 10");
}

#[test]
fn test_player_creation() {
    let player = Player::new("TestHero".into());
    assert_eq!(player.name, "TestHero");
    assert_eq!(player.hp, 100);
    assert_eq!(player.level, 1);
    assert_eq!(player.damage, 10);
}

#[test]
fn test_player_takes_damage() {
    let mut player = Player::new("TestHero".into());
    player.take_damage(50);
    assert_eq!(player.hp, 50);
    player.take_damage(60);
    assert_eq!(player.hp, 0);
}