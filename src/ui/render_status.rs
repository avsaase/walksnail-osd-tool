use std::time::Duration;

use crate::ffmpeg::{FromFfmpegMessage, VideoInfo};

#[derive(Default)]
pub struct RenderStatus {
    pub status: Status,
}

#[derive(PartialEq, Default)]
pub enum Status {
    #[default]
    Idle,
    InProgress {
        time_remaining: Option<Duration>,
        fps: f32,
        speed: f32,
        progress_pct: f32,
    },
    Completed,
    Cancelled {
        progress_pct: f32,
    },
    Error {
        progress_pct: f32,
        error: String,
    },
}

impl RenderStatus {
    pub fn start_render(&mut self) {
        self.status = Status::InProgress {
            time_remaining: None,
            fps: 0.0,
            speed: 0.0,
            progress_pct: 0.0,
        };
    }

    pub fn stop_render(&mut self) {
        if let Status::InProgress { progress_pct, .. } = self.status {
            self.status = Status::Cancelled { progress_pct }
        }
    }

    pub fn reset(&mut self) {
        self.status = Status::Idle;
    }

    fn finished(&mut self) {
        self.status = Status::Completed;
    }

    fn error(&mut self, error: &str) {
        self.status = Status::Error {
            progress_pct: 0.0,
            error: error.into(),
        }
    }

    pub fn update_from_ffmpeg_message(&mut self, message: FromFfmpegMessage, video_info: &VideoInfo) {
        match (&self.status, &message) {
            (
                Status::InProgress { progress_pct, .. },
                FromFfmpegMessage::DecoderFatalError(e) | FromFfmpegMessage::EncoderFatalError(e),
            ) => {
                self.status = Status::Error {
                    progress_pct: *progress_pct,
                    error: e.clone(),
                }
            }

            (Status::InProgress { .. }, FromFfmpegMessage::Progress(p)) => {
                let frame = p.frame as f32;
                let total_frames = video_info.total_frames as f32;
                let progress_pct = frame / total_frames;
                let frames_remaining = total_frames - frame;
                let time_remaining_secs = frames_remaining / p.fps;
                self.status = Status::InProgress {
                    time_remaining: if time_remaining_secs.is_finite() && time_remaining_secs.is_sign_positive() {
                        Some(Duration::from_secs_f32(time_remaining_secs))
                    } else {
                        None
                    },
                    fps: p.fps,
                    speed: p.speed,
                    progress_pct,
                };
            }

            (Status::InProgress { .. }, FromFfmpegMessage::DecoderFinished) => self.finished(),

            // The decoder should always finish first so if the encoder finished when the render is in progress it must be an error
            (Status::InProgress { progress_pct, .. }, FromFfmpegMessage::EncoderFinished) if *progress_pct < 0.001 => {
                self.error("Encoder unexpectedly finished")
            }

            _ => {}
        }
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, Status::InProgress { .. })
    }

    pub fn is_not_in_progress(&self) -> bool {
        !self.is_in_progress()
    }
}
