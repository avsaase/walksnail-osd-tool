use std::{os::windows::process::CommandExt, process::Command};

pub fn ffmpeg_available() -> bool {
    command_available("ffmpeg")
}

pub fn ffprobe_available() -> bool {
    command_available("ffprobe")
}

fn command_available(command: &str) -> bool {
    let mut command = Command::new(command);

    command
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    #[cfg(target_os = "windows")]
    command.creation_flags(crate::CREATE_NO_WINDOW);

    command.status().is_ok()
}
