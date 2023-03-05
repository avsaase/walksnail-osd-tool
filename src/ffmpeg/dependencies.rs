use std::process::Command;

use tracing::instrument;

#[instrument(ret)]
pub fn ffmpeg_available() -> bool {
    command_available("ffmpeg")
}

#[instrument(ret)]
pub fn ffprobe_available() -> bool {
    command_available("ffprobe")
}

pub fn dependencies_statisfied() -> bool {
    ffmpeg_available() && ffprobe_available()
}

fn command_available(command: &str) -> bool {
    let mut command = Command::new(command);

    command
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    #[cfg(target_os = "windows")]
    std::os::windows::process::CommandExt::creation_flags(&mut command, crate::CREATE_NO_WINDOW);

    command.status().is_ok()
}
