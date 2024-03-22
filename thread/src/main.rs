use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    let v = vec![1,2,3,4];

    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    handle.join().unwrap();


    sleep(Duration::from_secs(10));
}
