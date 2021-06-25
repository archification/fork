extern crate async_std;

mod lib;

use std::thread::spawn;
use lib::execute_danger;

#[tokio::main]
async fn main() {
    loop {
        spawn(execute_danger).join().expect("thread panicked").await;
    }
}
