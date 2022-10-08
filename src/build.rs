use std::{path::Path, process::Command};

fn main() {
    // Detect if the ui is built, if not we build it
    // Don't check for changes on the ui code
    if !Path::new("./ui/build/index.html").exists() {
        println!("cargo:warning=Building the UI");
        let output = Command::new("yarn").arg("build").current_dir("ui").output();

        match output {
            Err(_) => {
                println!("cargo:warning=Failed to run yarn");
            }
            Ok(output) if !output.status.success() => {
                println!("cargo:warning=Failed to build the UI");
            }
            Ok(_) => {
                println!("cargo:warning=UI built successfully");
            }
        }
    }

    // If the build directory doesn't exist, we create it empty
    // so that the build with EmbedRust doesn't fail.
    // TODO: see if we can avoid this by not using EmbedRust when the frontend doen't build.
    // can we use a feature flag?
    // use an env var?
    if !Path::new("./ui/build").exists() {
        std::fs::create_dir("ui/build").unwrap();
    }
}
