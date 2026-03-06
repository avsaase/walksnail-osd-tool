use std::process::Command;
use vergen::{vergen, Config};

fn main() {
    println!("cargo:rerun-if-changed=./src");

    // Configure vergen to skip Git-derived info (avoids safe.directory issues inside Docker)
    let mut config = Config::default();
    *config.git_mut().enabled_mut() = false;

    vergen(config).expect("vergen failed");

    // Capture git tag and short commit (non-fatal)
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        if let Ok(short_commit) = String::from_utf8(output.stdout) {
            if !short_commit.trim().is_empty() {
                println!("cargo:rustc-env=GIT_COMMIT_HASH={}", short_commit.trim());
            }
        }
    }

    // Load icon data
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("icon_bytes");
    let icon = image::io::Reader::open("../resources/icons/256x256.png")
        .expect("Failed to load icon file")
        .decode()
        .expect("Failed to decode icon file");
    let icon_bytes = icon.as_bytes();
    std::fs::write(dest_path, icon_bytes).expect("Failed to write icon bytes");
}
