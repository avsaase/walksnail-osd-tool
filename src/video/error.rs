use thiserror::Error;

#[derive(Debug, Error)]
pub enum FfmpegError {
    #[error("Failed to spawn FFMPEG process")]
    FailedToSpawnFfmpeg {
        #[from]
        source: std::io::Error,
    },
}
