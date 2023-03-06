use std::path::PathBuf;

use ffprobe::FfProbe;

use super::error::VideoInfoError;

#[derive(Debug)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub frame_rate: f32,
    pub bitrate: u32,
    pub duration_seconds: u32,
    pub total_frames: u32,
}

impl VideoInfo {
    #[tracing::instrument(ret)]
    pub fn get(path: &PathBuf) -> Result<Self, VideoInfoError> {
        let info = ffprobe::ffprobe(path)?;
        info.try_into()
    }
}

impl TryFrom<FfProbe> for VideoInfo {
    type Error = VideoInfoError;

    fn try_from(value: FfProbe) -> Result<Self, Self::Error> {
        let stream = value.streams.get(0).ok_or(VideoInfoError::NoStream)?;

        let width = stream.width.ok_or(VideoInfoError::NoFrameWidth)? as u32;
        let height = stream.height.ok_or(VideoInfoError::NoFrameHeight)? as u32;
        let frame_rate = {
            let frame_rate_string = &stream.avg_frame_rate;
            let mut split = frame_rate_string.split('/');
            let num = split
                .next()
                .and_then(|num| num.parse::<f32>().ok())
                .ok_or(VideoInfoError::NoFrameRate)?;
            let den = split
                .next()
                .and_then(|num| num.parse::<f32>().ok())
                .ok_or(VideoInfoError::NoFrameRate)?;
            num / den
        };
        let bitrate = stream
            .bit_rate
            .as_ref()
            .and_then(|b| b.parse::<u32>().ok())
            .ok_or(VideoInfoError::NoBitrate)?;

        let duration_seconds = stream
            .duration
            .as_ref()
            .and_then(|s| s.parse::<f32>().ok())
            .ok_or(VideoInfoError::NoDuration)? as u32;

        let total_frames = (frame_rate * duration_seconds as f32) as u32;

        Ok(Self {
            width,
            height,
            frame_rate,
            bitrate,
            duration_seconds,
            total_frames,
        })
    }
}
