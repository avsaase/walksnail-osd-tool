use image::{imageops::overlay, RgbaImage};

use crate::{
    font::{self, CharacterSize},
    osd::{self, OsdOptions},
};

pub fn get_character_size(lines: u32) -> CharacterSize {
    match lines {
        540 => CharacterSize::Race,
        720 => CharacterSize::Small,
        1080 => CharacterSize::Large,
        1440 => CharacterSize::XLarge,
        2160 => CharacterSize::Ultra,
        _ => CharacterSize::Large,
    }
}

#[inline]
pub fn overlay_osd(image: &mut RgbaImage, osd_frame: &osd::Frame, font: &font::FontFile, osd_options: &OsdOptions) {
    // TODO: check if this can be run in parallel
    let osd_character_size = osd_options
        .character_size
        .clone()
        .unwrap_or(get_character_size(image.height()));
    for character in &osd_frame.glyphs {
        if character.index == 0 || osd_options.get_mask(&character.grid_position) {
            continue;
        }
        if let Some(character_image) = font.get_character(character.index as usize, &osd_character_size) {
            let grid_position = &character.grid_position;

            // According to https://betaflight.com/docs/wiki/configurator/osd-tab
            // INFO
            // HD OSD defaults to a 53 column x 20 row grid of OSD elements.
            // When the VTX is online BetaFlight will query via MSP Displayport to determine the optimum grid size and may update the grid to match what is supported by the digital VTX system
            const ROW_COUNT: f32 = 20.0;
            const COL_COUNT: f32 = 53.0;

            let scale_height = image.height() as f32 / ROW_COUNT;
            let scale_width = image.width() as f32 / COL_COUNT;

            overlay(
                image,
                &character_image,
                (grid_position.x as f32 * scale_width + osd_options.position.x as f32) as i64,
                (grid_position.y as f32 * scale_height + osd_options.position.y as f32) as i64,
            )
        }
    }
}
