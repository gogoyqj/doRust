use crate::garden::vegetables::vegetables;
use std::thread;

pub mod garden;

fn main() {
    let mut handles = vec![];

    for i in 1..6 {
        let handle = thread::spawn(move || {
            vegetables(i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
