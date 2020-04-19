use std::thread::spawn;

mod lib;

use lib::make_danger;
use lib::execute_danger;

fn main() {
    make_danger();
    loop {
        spawn(execute_danger).join().expect("fork thread panicked");
    }
}
