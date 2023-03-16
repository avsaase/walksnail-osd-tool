use ffmpeg_sidecar::event::FfmpegProgress;

pub enum FfmpegMessage {
    DecoderFatalError(String),
    EncoderFatalError(String),
    Progress(FfmpegProgress),
    DecoderFinished,
}

pub struct StopRenderMessage;
