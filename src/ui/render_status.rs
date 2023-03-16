use std::time::Duration;

use crate::ffmpeg::{FfmpegMessage, VideoInfo};

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
        match self.status {
            Status::Idle => {}
            Status::InProgress { progress_pct, .. } => self.status = Status::Cancelled { progress_pct },
            Status::Completed => {}
            Status::Cancelled { .. } => {}
            Status::Error { .. } => {}
        }
    }

    pub fn reset(&mut self) {
        self.status = Status::Idle;
    }

    fn finished(&mut self) {
        self.status = Status::Completed;
    }

    pub fn update_from_ffmpeg_message(&mut self, message: FfmpegMessage, video_info: &VideoInfo) {
        match (&self.status, &message) {
            (Status::Idle, _) => {}

            (
                Status::InProgress { progress_pct, .. },
                FfmpegMessage::DecoderFatalError(e) | FfmpegMessage::EncoderFatalError(e),
            ) => {
                self.status = Status::Error {
                    progress_pct: *progress_pct,
                    error: e.clone(),
                }
            }

            (Status::InProgress { .. }, FfmpegMessage::Progress(p)) => {
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

            (Status::InProgress { .. }, FfmpegMessage::DecoderFinished) => self.finished(),

            (Status::Completed, _) => {}

            (Status::Cancelled { .. }, _) => {}

            (Status::Error { .. }, _) => {}
        }
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, Status::InProgress { .. })
    }

    pub fn is_not_in_progress(&self) -> bool {
        !self.is_in_progress()
    }
}
