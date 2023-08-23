use std::process::Command;

use vergen::{vergen, Config};

fn main() {
    println!("cargo:rerun-if-changed=./src");

    // Save details from build environment so they can be included in the binary
    vergen(Config::default()).unwrap();

    if let Some(git_tag) = Command::new("git")
        .args(["describe", "--exact-match", "--tags", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
    {
        if !git_tag.is_empty() {
            println!("cargo:rustc-env=GIT_VERSION={}", git_tag);
        }
    }

    if let Some(short_commit) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
    {
        println!("cargo:rustc-env=GIT_COMMIT_HASH={}", short_commit);
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
