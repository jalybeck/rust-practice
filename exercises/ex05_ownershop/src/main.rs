struct Owner {
    name: String,
    item: Option<String>,
}

impl Owner {
    fn new(name: String) -> Self {
        Owner { name, item: None }
    }
}

struct Borrower {
    name: String,
    borrowed_item: Option<String>,
}

impl Borrower {
    fn new(name: String) -> Self {
        Borrower { name, borrowed_item: None }
    }
}

fn main() {
    println!("Exercise 05: Ownership & Borrowing");
    println!("-------------------------------");

    let mut owner = Owner::new("Alice".to_string());
    let mut borrower = Borrower::new("Bob".to_string());
    
    println!("Owner: {}, Borrower: {}", owner.name, borrower.name);

    // Owner lends item to borrower
    owner.item = Some("Laptop".to_string());
    borrower.borrowed_item = owner.item.take();

    println!("Owner's item: {:?}, Borrower's item: {:?}\n", owner.item, borrower.borrowed_item);

    println!("After lending:");
    println!("Owner's item: {:?}, Borrower's item: {:?}\n", owner.item, borrower.borrowed_item);
    
    // Borrower returns item to owner
    owner.item = borrower.borrowed_item.take();
    
    println!("After returning:");
    println!("Owner's item: {:?}, Borrower's item: {:?}\n", owner.item, borrower.borrowed_item);
}