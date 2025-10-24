use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create two transmitters and the consumer
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    // Spawn first thread to send messages from first transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Spawn second thread to send messages from second transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Print all messages received
    for received in rx {
        println!("Got: {}", received);
    }
}
