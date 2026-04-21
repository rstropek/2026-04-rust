use std::sync::mpsc; // Multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rc) = mpsc::channel();
    let tx2 = tx.clone();
    
    thread::spawn(move || {
        let messages = ["hello", "world"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // tx goes out of scope -> dropped -> channel is closed
    });
    
    thread::spawn(move || {
        let messages = ["hello", "world"];
        for msg in messages {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // tx goes out of scope -> dropped -> channel is closed
    });
    
    for msg in rc {
        println!("Received: {}", msg);
    }
}
