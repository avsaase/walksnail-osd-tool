use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use image::{ImageBuffer, ImageError, Rgba};
use image::error::{DecodingError, ImageFormatHint};

use crate::font::{CharacterSize, FontFileError};

const CHAR_HEIGHT: usize = 18;
const CHAR_WIDTH: usize = 12;
const GARBAGE: usize = 10;
const NIBBLE_LEN: usize = 2;

pub fn read_mcm<P: AsRef<Path>>(path: P, size: CharacterSize) -> Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, FontFileError> {
    let width_factor = size.width() as usize / CHAR_WIDTH;
    let height_factor = size.height() as usize / CHAR_HEIGHT;
    let file = BufReader::new(File::open(path).map_err(|e| FontFileError::FailedToOpen { source: e })?);

    let mut lines = file.lines();
    assert_eq!("MAX7456", lines.next().unwrap()
        .map_err(|e| FontFileError::FailedToDecode { source: ImageError::Decoding(DecodingError::new(ImageFormatHint::Unknown, e)) })?.as_str()
    );

    let result = lines.collect::<io::Result<Vec<String>>>().map(|v| v.join(""))?;


    let mcm_chars = result.chars().collect::<Vec<_>>();
    let char_images = mcm_chars.chunks(CHAR_HEIGHT * CHAR_WIDTH * NIBBLE_LEN + GARBAGE * 8).map(|x| {
        let pixels = x.iter().take(CHAR_HEIGHT * CHAR_WIDTH * NIBBLE_LEN).collect::<Vec<_>>()
            .chunks(NIBBLE_LEN)
            .map(|pixel| {
                match pixel {
                    &['0', '0'] => [0u8, 0, 0, 255].repeat(width_factor),
                    &['0', '1'] => [0, 0, 0, 0].repeat(width_factor),
                    &['1', '0'] => [255, 255, 255, 255].repeat(width_factor),
                    &['1', '1'] => [255, 255, 255, 0].repeat(width_factor),
                    _ => panic!()
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        let pixels_resized = pixels.chunks(CHAR_WIDTH * width_factor * 4).map(|chunk| chunk.repeat(height_factor)).flatten().collect::<Vec<_>>();

        ImageBuffer::from_raw(CHAR_WIDTH as u32 * width_factor as u32, CHAR_HEIGHT as u32 * height_factor as u32, pixels_resized).unwrap()
    })
        .collect::<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>>();
    Ok(char_images)
}
