use std::path::PathBuf;

use derivative::Derivative;
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use crate::font::mcm_reader::read_mcm;

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
    pub characters: Vec<RgbaImage>,
}

impl FontFile {
    #[tracing::instrument(ret, err)]
    pub fn open(path: PathBuf) -> Result<Self, FontFileError> {
        match path.extension().map(|e| e.to_str().unwrap()) {
            None => panic!(),
            Some("mcm") => {
                let characters = read_mcm(&path, CharacterSize::Large)?;
                Ok(Self {
                    file_path: path,
                    character_count: characters.len() as u32,
                    character_size: CharacterSize::Large,
                    font_type: FontType::Standard,
                    characters,
                })
            }
            Some(_) => {
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
        }
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
