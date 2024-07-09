use std::env;
use std::path::Path;
use std::process::Command;

/// The main function that drives the build script.
///
/// It verifies the existence of the frontend directory, watches for changes in the frontend and
/// server code, and initiates the frontend build process.
fn main() {
    let frontend_dir = Path::new("../frontend");

    if !frontend_dir.exists() {
        panic!("❌ Frontend directory does not exist: {:?}", frontend_dir);
    }

    // Watch the frontend code
    println!("cargo:rerun-if-changed={}", frontend_dir.display());

    // Watch the server code
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");

    match run_bun_build(frontend_dir) {
        Ok(_) => println!("✅ Successfully built Astro frontend"),
        Err(e) => panic!("❌ Build failed: {}", e),
    }
}

/// Runs the `bun` build command in the specified frontend directory.
///
/// # Arguments
///
/// * `frontend_dir` - The path to the frontend directory.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the build process. If successful, returns `Ok(())`.
/// If the build fails, returns an `Err` with a description of the error.
fn run_bun_build(frontend_dir: &Path) -> Result<(), String> {
    // Check if bun is available in the environment
    if !is_bun_available() {
        return Err("`bun` is not installed or not found in the PATH".into());
    }

    let output = Command::new("bun")
        .current_dir(frontend_dir)
        .args(&["run", "build"])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("📝 Command stdout:\n{}", stdout);
        println!("📝 Command stderr:\n{}", stderr);

        return Err("Failed to build Astro frontend".into());
    }

    Ok(())
}

/// Checks if the `bun` command is available in the current environment.
///
/// # Returns
///
/// `true` if the `bun` command is found in the PATH, `false` otherwise.
fn is_bun_available() -> bool {
    if let Ok(path_var) = env::var("PATH") {
        for path in env::split_paths(&path_var) {
            let bun_path = path.join(if cfg!(windows) { "bun.exe" } else { "bun" });
            if bun_path.exists() {
                return true;
            }
        }
    }
    false
}