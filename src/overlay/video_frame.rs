use ffmpeg_sidecar::event::OutputVideoFrame;
use image::RgbaImage;

use crate::{font, osd};

use super::overlay_osd_on_image;

pub fn overlay_osd_on_video(
    mut video_frame: OutputVideoFrame,
    osd_frame: &osd::Frame,
    font: &font::FontFile,
    horizontal_offset: i32,
    vertical_offset: i32,
) -> OutputVideoFrame {
    let mut image = RgbaImage::from_raw(video_frame.width, video_frame.height, video_frame.data).unwrap();
    overlay_osd_on_image(osd_frame, font, &mut image, horizontal_offset, vertical_offset);
    video_frame.data = image.as_raw().to_vec();
    video_frame
}
