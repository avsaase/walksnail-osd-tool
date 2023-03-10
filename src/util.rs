use std::{env::current_exe, fmt::Display, path::PathBuf};

use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug, Clone)]
pub struct Coordinates<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
pub struct Dimension<T> {
    pub width: T,
    pub height: T,
}

impl Display for Dimension<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl From<Dimension<u32>> for String {
    fn from(value: Dimension<u32>) -> Self {
        format!("{}x{}", value.width, value.height)
    }
}

pub fn init_tracing() -> Option<WorkerGuard> {
    directories::ProjectDirs::from("", "", "Walksnail OSD Tool").map(|dir| {
        let log_dir = dir.data_dir();

        std::fs::remove_file(log_dir.join("walksnail-osd-tool.log")).ok();

        let file_appender = tracing_appender::rolling::never(log_dir, "walksnail-osd-tool.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        tracing_subscriber::fmt()
            .with_ansi(false)
            .compact()
            .with_writer(non_blocking)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .init();
        guard
    })
}

pub fn ffmpeg_path() -> PathBuf {
    let cur_exe = current_exe().unwrap();
    let exe_dir = cur_exe.parent().unwrap();

    if cfg!(all(target_os = "macos", feature = "macos-app-bundle")) {
        exe_dir.join("ffmpeg")
    } else if cfg!(all(target_os = "windows", feature = "windows-installer")) {
        exe_dir.parent().unwrap().join("ffmpeg").join("ffmpeg")
    } else {
        "ffmpeg".into()
    }
}

pub fn ffprobe_path() -> PathBuf {
    let cur_exe = current_exe().unwrap();
    let exe_dir = cur_exe.parent().unwrap();

    if cfg!(all(target_os = "macos", feature = "macos-app-bundle")) {
        exe_dir.join("ffprobe")
    } else if cfg!(all(target_os = "windows", feature = "windows-installer")) {
        exe_dir.parent().unwrap().join("ffmpeg").join("ffprobe")
    } else {
        "ffprobe".into()
    }
}

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));

    pub fn get_version() -> Option<String> {
        let version = GIT_VERSION.map(|s| s.to_string());
        let sha = GIT_COMMIT_HASH_SHORT.map(|s| s.to_string());

        match (version, sha) {
            (None, None) => None,
            (None, Some(sha)) => Some(sha),
            (Some(version), None) => Some(version),
            (Some(version), Some(sha)) if version == sha => Some(version),
            (Some(version), Some(sha)) => Some(format!("{}-{}", version, sha)),
        }
    }

    pub fn get_compiler() -> String {
        RUSTC_VERSION.to_string()
    }

    pub fn get_target() -> String {
        TARGET.to_string()
    }
}
