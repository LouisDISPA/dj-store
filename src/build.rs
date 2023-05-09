fn main() {
    #[cfg(feature = "embed-ui")]
    build_ui();
}

#[cfg(feature = "embed-ui")]
fn build_ui() {
    use std::{path::Path, process::Command};

    // Detect if the ui is built, if not we build it
    // Don't check for changes on the ui code
    if !Path::new("./ui/build/index.html").exists() {
        println!("cargo:warning=Building the UI");
        let output = Command::new("yarn").arg("build").current_dir("ui").output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    println!("cargo:warning=Failed to build the UI");
                    println!(
                        "cargo:warning=-> {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to build the UI");
                println!("cargo:warning=-> {}", e);
            }
        }
    } else {
        println!("cargo:warning=UI already built");
        println!("cargo:warning=-> The UI might not be up to date. Run `yarn build` in the ui folder to update it");
    }

    // If the build directory doesn't exist, we create it empty
    // so that the build with EmbedRust doesn't fail.
    if !Path::new("./ui/build").exists() {
        std::fs::create_dir("ui/build").unwrap();
    }
}
