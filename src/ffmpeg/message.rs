use ffmpeg_sidecar::event::FfmpegProgress;

pub enum FfmpegMessage {
    DecoderError(String),
    EncoderError(String),
    Progress(FfmpegProgress),
    DecoderFinished,
}

pub struct StopRenderMessage;
