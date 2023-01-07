use std::time::Duration;

use ffmpeg_sidecar::event::FfmpegEvent;

use crate::ffmpeg::VideoInfo;

#[derive(PartialEq, Default)]
pub enum RenderStatus {
    #[default]
    Idle,
    InProgress {
        time_remaining: Option<Duration>,
        fps: f32,
        speed: f32,
        progress_pct: f32,
    },
    Completed,
}

impl RenderStatus {
    pub fn from_ffmpeg_event(value: FfmpegEvent, video_info: &VideoInfo) -> Self {
        match value {
            FfmpegEvent::Progress(p) => {
                let frame = p.frame as f32;
                let total_frames = video_info.total_frames as f32;
                let progress_pct = frame / total_frames;
                let frames_remaining = total_frames - frame;
                let time_remaining_secs = frames_remaining / p.fps;
                RenderStatus::InProgress {
                    time_remaining: if time_remaining_secs.is_finite() && time_remaining_secs.is_sign_positive() {
                        Some(Duration::from_secs_f32(time_remaining_secs))
                    } else {
                        None
                    },
                    fps: p.fps,
                    speed: p.speed,
                    progress_pct,
                }
            }
            FfmpegEvent::LogEOF => RenderStatus::Idle,
            FfmpegEvent::Done => RenderStatus::Completed,
            _ => RenderStatus::Idle,
        }
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress { .. })
    }

    pub fn is_not_in_progress(&self) -> bool {
        !self.is_in_progress()
    }
}
