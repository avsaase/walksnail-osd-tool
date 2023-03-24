use std::path::PathBuf;

use derivative::Derivative;
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

use crate::util::Dimension;

use super::{character_size::CharacterSize, error::FontFileError};

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct FontFile {
    pub file_path: PathBuf,
    pub character_count: u32,
    pub character_size: CharacterSize,
    #[derivative(Debug = "ignore")]
    pub characters: Vec<RgbaImage>,
}

impl FontFile {
    #[tracing::instrument(ret, err)]
    pub fn open(path: PathBuf) -> Result<Self, FontFileError> {
        let font_image = Reader::open(&path)?.decode()?;
        let (width, height) = font_image.dimensions();
        verify_dimensions(width, height)?;
        let character_size = CharacterSize::from_width(width);
        let character_count = height / character_size.height();

        let characters = split_characters(&font_image, &character_size, character_count);

        Ok(Self {
            file_path: path,
            character_count,
            character_size,
            characters,
        })
    }
}

fn verify_dimensions(width: u32, height: u32) -> Result<(), FontFileError> {
    if (width != CharacterSize::Large.width() && width != CharacterSize::Small.width()) || height % width != 0 {
        return Err(FontFileError::InvalidFontFileDimensions {
            dimensions: Dimension { width, height },
        });
    }

    Ok(())
}

fn split_characters(
    font_image: &DynamicImage,
    character_size: &CharacterSize,
    character_count: u32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image_height = font_image.height();
    let char_width = character_size.width();
    let char_height = character_size.height();

    let mut char_vec = Vec::with_capacity(character_count as usize);
    for y in (0..image_height).step_by(char_height as usize) {
        let char = font_image.view(0, y, char_width, char_height).to_image();
        char_vec.push(char);
    }
    char_vec
}
