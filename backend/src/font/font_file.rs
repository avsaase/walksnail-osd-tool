use std::path::PathBuf;

use derivative::Derivative;
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

use super::{
    dimensions::{CharacterSize, FontType, CHARACTER_WIDTH_LARGE, CHARACTER_WIDTH_SMALL},
    error::FontFileError,
};

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct FontFile {
    pub file_path: PathBuf,
    pub character_count: u32,
    pub character_size: CharacterSize,
    pub font_type: FontType,
    #[derivative(Debug = "ignore")]
    pub characters: Vec<RgbaImage>,
}

impl FontFile {
    #[tracing::instrument(ret, err)]
    pub fn open(path: PathBuf) -> Result<Self, FontFileError> {
        let font_image = Reader::open(&path)?.decode()?;
        let (width, height) = font_image.dimensions();
        let (character_size, font_type, character_count) = verify_dimensions(width, height)?;

        let characters = split_characters(&font_image, &character_size, character_count);

        Ok(Self {
            file_path: path,
            character_count,
            character_size,
            font_type,
            characters,
        })
    }
}

fn verify_dimensions(width: u32, height: u32) -> Result<(CharacterSize, FontType, u32), FontFileError> {
    let (size, r#type) = if width == CHARACTER_WIDTH_SMALL {
        (CharacterSize::Small, FontType::SinglePage)
    } else if width == CHARACTER_WIDTH_LARGE {
        (CharacterSize::Large, FontType::SinglePage)
    } else if width == CHARACTER_WIDTH_SMALL * 4 {
        (CharacterSize::Small, FontType::FourPage)
    } else if width == CHARACTER_WIDTH_LARGE * 4 {
        (CharacterSize::Large, FontType::FourPage)
    } else {
        return Err(FontFileError::InvalidFontFileWidth { width });
    };

    if height % size.height() != 0 {
        return Err(FontFileError::InvalidFontFileHeight { height });
    }

    let characters_count = height / size.height() * r#type.pages();

    Ok((size, r#type, characters_count))
}

fn split_characters(
    font_image: &DynamicImage,
    character_size: &CharacterSize,
    character_count: u32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image_height = font_image.height();
    let image_width = font_image.width();

    let char_width = character_size.width();
    let char_height = character_size.height();

    let mut char_vec = Vec::with_capacity(character_count as usize);

    for x in (0..image_width).step_by(char_width as usize) {
        for y in (0..image_height).step_by(char_height as usize) {
            let char = font_image.view(x, y, char_width, char_height).to_image();
            char_vec.push(char);
        }
    }

    char_vec
}
