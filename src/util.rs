use std::{env::current_exe, fmt::Display, path::PathBuf};

use github_release_check::{GitHubReleaseItem, LookupError};
use serde::{Deserialize, Serialize};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer};

use crate::util::build_info::Build;

#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
pub struct Coordinates<T> {
    #[serde(rename = "x_position")]
    pub x: T,
    #[serde(rename = "y_position")]
    pub y: T,
}

impl<T> Coordinates<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Dimension<T> {
    pub width: T,
    pub height: T,
}

impl<T> Dimension<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
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
    directories::ProjectDirs::from("rs", "", "Walksnail OSD Tool").map(|dir| {
        let log_dir = dir.data_dir();

        std::fs::remove_file(log_dir.join("walksnail-osd-tool.log")).ok();

        let file_appender = tracing_appender::rolling::never(log_dir, "walksnail-osd-tool.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let stdout_log = tracing_subscriber::fmt::layer()
            .pretty()
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_filter(filter::LevelFilter::INFO);
        let file_log = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .compact()
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_writer(non_blocking)
            .with_filter(filter::LevelFilter::INFO);
        tracing_subscriber::registry().with(stdout_log).with(file_log).init();

        guard
    })
}

pub fn get_dependency_path(dependency: &str) -> PathBuf {
    let cur_exe = current_exe().unwrap();
    let exe_dir = cur_exe.parent().unwrap();

    if cfg!(all(target_os = "macos", feature = "macos-app-bundle")) {
        // Folder structure:
        // |
        // +-- MacOS
        //     +-- walksnail-osd-tool
        //     +-- ffmpeg
        //     +-- ffprobe
        exe_dir.join(dependency)
    } else if cfg!(all(target_os = "windows", feature = "windows-installer")) {
        // Folder structure:
        // |
        // +-- bin
        // |   +-- walksnail-osd-tool.exe
        // +-- ffmpeg
        //     +-- ffmpeg.exe
        //     +-- ffprobe.exe
        exe_dir.parent().unwrap().join("ffmpeg").join(dependency)
    } else {
        dependency.into()
    }
}

pub mod build_info {
    use std::fmt::Display;

    use semver::Version;

    pub enum Build {
        Release { version: Version, commit: String },
        Dev { commit: String },
        Unknown,
    }

    impl Display for Build {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Build::Release { version, commit } => write!(f, "{version} ({commit})"),
                Build::Dev { commit } => write!(f, "dev ({commit})"),
                Build::Unknown => write!(f, "Unknown"),
            }
        }
    }

    pub fn get_version() -> Build {
        let version: Option<Version> = option_env!("GIT_VERSION").and_then(|s| Version::parse(s).ok());
        let short_hash: Option<&'static str> = option_env!("GIT_COMMIT_HASH");

        match (version, short_hash.map(|s| s.to_string())) {
            (Some(version), Some(commit)) => Build::Release { version, commit },
            (None, Some(commit)) => Build::Dev { commit },
            _ => Build::Unknown,
        }
    }

    pub fn get_compiler() -> &'static str {
        env!("VERGEN_RUSTC_SEMVER")
    }

    pub fn get_target() -> &'static str {
        env!("VERGEN_CARGO_TARGET_TRIPLE")
    }
}

#[tracing::instrument(ret)]
pub fn check_updates() -> Result<Option<GitHubReleaseItem>, LookupError> {
    let github = github_release_check::GitHub::new().unwrap();
    const REPO: &str = "avsaase/walksnail-osd-tool";
    if let Build::Release { version, .. } = build_info::get_version() {
        let latest_version = github.get_latest_version(REPO)?;
        if latest_version > version {
            let releases = github.query(REPO)?;
            let latest_release = releases.first().cloned();
            return Ok(latest_release);
        }
    }
    Ok(None)
}
