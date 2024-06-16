use std::path::Path;
use std::process::Command;

fn main() {
    let frontend_dir = Path::new("../frontend");

    if !frontend_dir.exists() {
        panic!("Frontend directory does not exist: {:?}", frontend_dir);
    }

    let output = Command::new("bun")
        .current_dir(frontend_dir)
        .args(&["run", "build"])
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("Command stdout: {}", stdout);
        println!("Command stderr: {}", stderr);

        panic!("Failed to build Astro frontend");
    } else {
        println!("Successfully built Astro frontend");
    }

    println!("cargo:rerun-if-changed=../frontend/**/*");
    println!("cargo:rerun-if-changed=build.rs");
}