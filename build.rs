fn main() {
    // Save details from build environment so they can be included in the binary
    built::write_built_file().expect("Failed to acquire build-time information");

    // Load icon data
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("icon_bytes");
    let icon = image::io::Reader::open("./_deploy/icons/256x256.png")
        .expect("Failed to load icon file")
        .decode()
        .expect("Failed to decode icon file");
    let icon_bytes = icon.as_bytes();
    std::fs::write(&dest_path, icon_bytes).expect("Failed to write icon bytes");
}
