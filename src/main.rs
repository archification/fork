extern crate rand;

use std::thread::spawn;

mod lib;

use lib::make_danger;
//use lib::execute_danger;
use lib::what_are_those;

fn main() {
    make_danger();
//    execute_danger();
    loop {
        spawn(what_are_those).join().expect("file write thread panicked");
    }
}
