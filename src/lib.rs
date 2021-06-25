use std::process::Command;

pub async fn execute_danger() {
    loop {
        let mut child = Command::new("bash")
            .arg("-c")
            .arg("\"./fork\"")
            .spawn()
            .expect("failed to execute child");
        let ecode = child.wait()
            .expect("failed to wait on child");
        assert!(ecode.success());
    }
}
