use ffmpeg_sidecar::event::FfmpegProgress;

pub enum FromFfmpegMessage {
    DecoderFatalError(String),
    EncoderFatalError(String),
    Progress(FfmpegProgress),
    DecoderFinished,
    EncoderFinished,
}

pub enum ToFfmpegMessage {
    AbortRender,
}
