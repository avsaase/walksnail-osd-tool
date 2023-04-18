use std::{env::current_exe, fmt::Display, path::PathBuf};

use github_release_check::{GitHubReleaseItem, LookupError};
use semver::Version;
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
                Build::Release { version, .. } => write!(f, "{version}"),
                Build::Dev { commit } => write!(f, "dev ({commit})"),
                Build::Unknown => write!(f, "Unknown"),
            }
        }
    }

    pub fn get_version() -> Build {
        let version: Option<Version> = option_env!("GIT_VERSION").and_then(|s| Version::parse(s).ok());
        let short_hash: Option<&'static str> = option_env!("GIT_COMMIT_HASH");

        let version = Version::parse("0.0.1").unwrap().into();

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
    if let Build::Release {
        version: current_version,
        ..
    } = build_info::get_version()
    {
        let github = github_release_check::GitHub::new().unwrap();
        let releases = github.query("avsaase/walksnail-osd-tool")?;
        let update_target = releases
            .iter()
            .find(|release| {
                Version::parse(release.tag_name.trim_start_matches('v'))
                    .map_or(false, |version| should_update_to_version(&current_version, &version))
            })
            .cloned();
        Ok(update_target)
    } else {
        Ok(None)
    }
}

fn should_update_to_version(current_version: &Version, to_version: &Version) -> bool {
    if to_version <= current_version {
        return false;
    }

    let version_is_full_release = to_version.pre.is_empty();
    if version_is_full_release {
        return true;
    }

    let current_version_is_prerelease = !current_version.pre.is_empty();
    if current_version_is_prerelease {
        return to_version.major == current_version.major && to_version.minor == current_version.minor;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version(version: &str) -> Version {
        Version::parse(version).unwrap()
    }

    #[test]
    fn update_to_new_release() {
        let current_version = version("0.1.0");
        let new_version = version("0.2.0");
        assert!(should_update_to_version(&current_version, &new_version));
    }

    #[test]
    fn not_update_to_older_release() {
        let current_version = version("0.2.0");
        let new_version = version("0.1.0");
        assert!(!should_update_to_version(&current_version, &new_version));
    }

    #[test]
    fn update_from_prerelease_to_full_release() {
        let current_version = version("0.1.0-beta.2");
        let new_version = version("0.1.0");
        assert!(should_update_to_version(&current_version, &new_version));
    }

    #[test]
    fn update_from_prerelease_to_new_prerelease() {
        let current_version = version("0.1.0-beta.1");
        let new_version = version("0.1.0-beta.3");
        assert!(should_update_to_version(&current_version, &new_version));
    }

    #[test]
    fn not_update_from_prerelease_to_older_prerelease() {
        let current_version = version("0.1.0-beta.3");
        let new_version = version("0.1.0-beta.2");
        assert!(!should_update_to_version(&current_version, &new_version));
    }

    #[test]
    fn not_update_from_prerelease_to_prerelease_in_new_cyce() {
        let current_version = version("0.1.0-beta.3");
        let new_version = version("0.2.0-beta.2");
        assert!(!should_update_to_version(&current_version, &new_version));
    }

    #[test]
    fn not_update_from_release_to_prerelease_of_new_release() {
        let current_version = version("0.1.0");
        let new_version = version("0.2.0-beta.2");
        assert!(!should_update_to_version(&current_version, &new_version));
    }

    #[test]
    fn not_update_to_same_release() {
        let current_version = version("0.1.0");
        assert!(!should_update_to_version(&current_version, &current_version));
    }
}
