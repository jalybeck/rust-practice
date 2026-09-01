use std::thread::{self, sleep};
use std::sync::mpsc::channel;

fn main() {
    println!("Exercise 16: Threads");
    println!("--------------------");
    
    let now = std::time::Instant::now();

    //This thread is spawned instantly
    let instantly_launching_thread = thread::spawn(move || {
        println!("{} secs: Hello from a thread!", now.elapsed().as_secs());
        "OK"
    });
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("{} secs: Hello from main thread!", now.elapsed().as_secs());

    std::thread::sleep(std::time::Duration::from_secs(5));

    let result = instantly_launching_thread.join().unwrap();
    println!("{:?} secs: Thread completed with result: {}", now.elapsed().as_secs(), result);

    //This thread is spawned via builder
    println!("{} secs: Spawning a builder thread...", now.elapsed().as_secs());
    let builder_thread = thread::Builder::new().spawn(move || {
        println!("{} secs: Hello from a builder thread!", now.elapsed().as_secs());
        "OK"
    }).unwrap();

    println!("{} secs: Builder thread spawned.", now.elapsed().as_secs());    
    std::thread::sleep(std::time::Duration::from_secs(1));

    let result = builder_thread.join().unwrap();
    println!("{:?} secs: Builder thread completed with result: {}", now.elapsed().as_secs(), result);

    let (tx, rx) = channel();
    let sender_thread = thread::spawn(move || {
        sleep(std::time::Duration::from_secs(5));
        tx.send("Message from sender thread").unwrap();
    });
    println!("{} secs: Waiting for message...", now.elapsed().as_secs());
    let message = rx.recv().unwrap();
    println!("{} secs: Received message: {}", now.elapsed().as_secs(), message);
    sender_thread.join().unwrap();
}