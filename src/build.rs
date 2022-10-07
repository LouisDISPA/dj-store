use std::{process::Command, path::Path};

fn main() {
    if !Path::new("ui/build").exists() {
        println!("Building UI");
        let output = Command::new("yarn")
            .arg("build")
            .current_dir("ui")
            .output()
            .expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
