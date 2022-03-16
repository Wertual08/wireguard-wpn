use std::time;
use std::thread;

fn main() {
    println!("Hello, world!");

    while (true) {
        thread::sleep(time::Duration::from_secs(1))
    }
}
