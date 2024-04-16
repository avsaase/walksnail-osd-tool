use thiserror::Error;

#[derive(Debug, Error)]
pub enum VideoInfoError {
    #[error("Failed to read frame width from video")]
    NoFrameWidth,
    #[error("Failed to read frame height from video")]
    NoFrameHeight,
    #[error("Failed to read frame rate from video")]
    NoFrameRate,
    #[error("Failed to read bitrate from video")]
    NoBitrate,
    #[error("Failed to read video duration from video")]
    NoTimeScale,
    #[error("Failed to read time scale value from video")]
    NoDuration,
    #[error("No stream in video file")]
    NoStream,
    #[error("Failed to probe video file")]
    FfprobeFailed {
        #[from]
        source: ffprobe::FfProbeError,
    },
}

#[derive(Debug, Error)]
pub enum FfmpegError {
    #[error("Failed to spawn FFMPEG process")]
    FailedToSpawnFfmpeg {
        #[from]
        source: std::io::Error,
    },
}
