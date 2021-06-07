use std::thread::spawn;

mod lib;

use lib::execute_danger;

fn main() {
    loop {
        spawn(execute_danger).join().expect("file write thread panicked");
    }
}
