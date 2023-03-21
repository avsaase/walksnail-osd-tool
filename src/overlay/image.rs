use image::{imageops::overlay, RgbaImage};

use crate::{font, osd};

pub fn overlay_osd_on_image(
    osd_frame: &osd::Frame,
    font: &font::FontFile,
    rgba_image: &mut RgbaImage,
    horizontal_offset: i32,
    vertical_offset: i32,
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
                (grid_position.x as i32 * char_width as i32 + horizontal_offset).into(),
                (grid_position.y as i32 * char_height as i32 + vertical_offset).into(),
            )
        }
    }
}
