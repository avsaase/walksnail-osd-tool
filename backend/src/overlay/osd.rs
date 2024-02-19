use image::{imageops::overlay, RgbaImage};

use crate::{
    font::{self, CharacterSize},
    osd::{self, OsdOptions},
};

pub fn get_character_size(lines: u32) -> CharacterSize
{
    match lines {
        //540 => CharacterSize::Race,
        720 => CharacterSize::Small,
        1080 => CharacterSize::Large,
        1440 => CharacterSize::XLarge,
        2160 => CharacterSize::Ultra,
        _ => CharacterSize:: Large
    }
}

#[inline]
pub fn overlay_osd(image: &mut RgbaImage, osd_frame: &osd::Frame, font: &font::FontFile, osd_options: &OsdOptions) {
    // TODO: check if this can be run in parallel
    let osd_character_size = get_character_size(image.height());
    for character in &osd_frame.glyphs {
        if character.index == 0 || osd_options.get_mask(&character.grid_position) {
            continue;
        }
        if let Some(character_image) = font.get_character(character.index as usize, &osd_character_size) {
            let grid_position = &character.grid_position;
            let (char_width, char_height) = character_image.dimensions();
            overlay(
                image,
                &character_image,
                (grid_position.x as i32 * char_width as i32 + osd_options.position.x).into(),
                (grid_position.y as i32 * char_height as i32 + osd_options.position.y).into(),
            )
        }
    }
}
