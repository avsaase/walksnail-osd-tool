use image::RgbaImage;

use crate::{
    font, osd,
    overlay::{overlay_osd, overlay_srt_data},
    srt,
};

use super::{OsdOptions, SrtOptions};

#[tracing::instrument(skip(osd_frame, srt_frame, font), level = "debug")]
pub fn create_osd_preview(
    width: u32,
    height: u32,
    osd_frame: &osd::Frame,
    srt_frame: &srt::SrtFrame,
    font: &font::FontFile,
    srt_font: &rusttype::Font,
    osd_options: &OsdOptions,
    srt_options: &SrtOptions,
) -> RgbaImage {
    let mut image = RgbaImage::new(width, height);

    overlay_osd(&mut image, osd_frame, font, osd_options);
    overlay_srt_data(&mut image, &srt_frame.data, srt_font, srt_options);

    image
}

#[tracing::instrument(level = "debug")]
pub fn calculate_horizontal_offset(width: u32, osd_frame: &osd::Frame, character_size: &font::CharacterSize) -> i32 {
    let min_x_grid = osd_frame.glyphs.iter().map(|g| g.grid_position.x).min().unwrap();
    let max_x_grid = osd_frame.glyphs.iter().map(|g| g.grid_position.x).max().unwrap();
    let pixel_range = (max_x_grid - min_x_grid + 1) * character_size.width();
    let offset = (width - pixel_range) / 2 - min_x_grid * character_size.width();
    offset as i32
}

#[tracing::instrument(level = "debug")]
pub fn calculate_vertical_offset(height: u32, osd_frame: &osd::Frame, character_size: &font::CharacterSize) -> i32 {
    let min_y_grid = osd_frame.glyphs.iter().map(|g| g.grid_position.y).min().unwrap();
    let max_y_grid = osd_frame.glyphs.iter().map(|g| g.grid_position.y).max().unwrap();
    let pixel_range = (max_y_grid - min_y_grid + 1) * character_size.height();
    let offset = (height - pixel_range) / 2 - min_y_grid * character_size.height();
    offset as i32
}
