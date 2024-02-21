use std::path::PathBuf;

use derivative::Derivative;
use image::{imageops::FilterType, io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

use super::{
    dimensions::{detect_dimensions, CharacterSize, FontType},
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
    characters: Vec<RgbaImage>,
}

impl FontFile {
    #[tracing::instrument(ret, err)]
    pub fn open(path: PathBuf) -> Result<Self, FontFileError> {
        let font_image = Reader::open(&path)?.decode()?;
        let (width, height) = font_image.dimensions();
        let (character_size, font_type, character_count) = detect_dimensions(width, height)?;

        let characters = split_characters(&font_image, &character_size, &font_type, character_count);

        Ok(Self {
            file_path: path,
            character_count,
            character_size,
            font_type,
            characters,
        })
    }

    pub fn get_character(&self, index: usize, size: &CharacterSize) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        if size.width() != self.character_size.width() || size.height() != self.character_size.height() {
            let original_image = self.characters.get(index).unwrap();
            let new_image = image::imageops::resize(original_image, size.width(), size.height(), FilterType::Lanczos3);
            return Some(new_image);
        }

        return self.characters.get(index).cloned();
    }
}

fn split_characters(
    font_image: &DynamicImage,
    character_size: &CharacterSize,
    font_type: &FontType,
    character_count: u32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let pages = font_type.pages();
    let char_width = character_size.width();
    let char_height = character_size.height();

    let mut char_vec = Vec::with_capacity((character_count * pages) as usize);

    for page_idx in 0..pages {
        let x = page_idx * char_width;
        for char_idx in 0..character_count {
            let y = char_idx * char_height;
            let char = font_image.view(x, y, char_width, char_height).to_image();
            char_vec.push(char);
        }
    }

    char_vec
}
