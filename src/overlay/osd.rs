use ffmpeg_sidecar::event::OutputVideoFrame;
use image::{imageops::overlay, RgbaImage};

use crate::{
    font, osd,
    srt::{self},
    ui::OsdOptions,
};

use super::srt::draw_srt_data;

#[inline]
pub fn overlay_osd_on_image(
    osd_frame: &osd::Frame,
    srt_frame: &srt::SrtFrame,
    font: &font::FontFile,
    srt_font: &rusttype::Font,
    rgba_image: &mut RgbaImage,
    osd_options: &OsdOptions,
) {
    // TODO: check if this can be run in parallel
    for character in &osd_frame.glyphs {
        if character.index == 0 {
            continue;
        }
        if let Some(character_image) = font.characters.get(character.index as usize) {
            let grid_position = &character.grid_position;
            let (char_width, char_height) = character_image.dimensions();
            overlay(
                rgba_image,
                character_image,
                (grid_position.x as i32 * char_width as i32 + osd_options.horizontal_offset).into(),
                (grid_position.y as i32 * char_height as i32 + osd_options.vertical_offset).into(),
            )
        }
    }

    draw_srt_data(rgba_image, &srt_frame.data, srt_font, &osd_options.srt_options);
}

#[inline]
pub fn overlay_osd_on_video(
    mut video_frame: OutputVideoFrame,
    osd_frame: &osd::Frame,
    srt_frame: &srt::SrtFrame,
    font: &font::FontFile,
    srt_font: &rusttype::Font,
    osd_options: &OsdOptions,
) -> OutputVideoFrame {
    let mut image = RgbaImage::from_raw(video_frame.width, video_frame.height, video_frame.data).unwrap();
    overlay_osd_on_image(osd_frame, srt_frame, font, srt_font, &mut image, osd_options);
    video_frame.data = image.as_raw().to_vec();
    video_frame
}
