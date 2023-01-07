use std::process::Command;

pub fn ffmpeg_available() -> bool {
    command_available("ffmpeg")
}

pub fn ffprobe_available() -> bool {
    command_available("ffprobe")
}

fn command_available(command: &str) -> bool {
    Command::new(command)
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok()
}
