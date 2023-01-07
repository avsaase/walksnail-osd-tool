use ffmpeg_sidecar::event::OutputVideoFrame;
use image::{DynamicImage, RgbImage};

use crate::{font, osd};

use super::overlay_osd_on_image;

pub fn overlay_osd_on_video(
    mut video_frame: OutputVideoFrame,
    osd_frame: &osd::Frame,
    font: &font::FontFile,
    horizontal_offset: i32,
    vertical_offset: i32,
) -> OutputVideoFrame {
    let image = RgbImage::from_raw(video_frame.width, video_frame.height, video_frame.data).unwrap();
    let mut rgba_image = DynamicImage::ImageRgb8(image).into_rgba8();
    overlay_osd_on_image(osd_frame, font, &mut rgba_image, horizontal_offset, vertical_offset);
    let rgb_image = DynamicImage::ImageRgba8(rgba_image).into_rgb8();
    video_frame.data = rgb_image.as_raw().to_vec();
    video_frame
}
