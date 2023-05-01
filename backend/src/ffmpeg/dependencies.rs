use std::{path::PathBuf, process::Command};

use tracing::instrument;

#[instrument(ret)]
pub fn ffmpeg_available(ffmpeg_path: &PathBuf) -> bool {
    command_available(ffmpeg_path)
}

#[instrument(ret)]
pub fn ffprobe_available(ffprobe_path: &PathBuf) -> bool {
    command_available(ffprobe_path)
}

fn command_available(command: &PathBuf) -> bool {
    let mut command = Command::new(command);

    command
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    #[cfg(target_os = "windows")]
    std::os::windows::process::CommandExt::creation_flags(&mut command, crate::util::CREATE_NO_WINDOW);

    match command.status() {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
