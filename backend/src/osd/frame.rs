use std::fmt::Debug;

use super::{
    error::OsdFileError,
    glyph::{Glyph, GridPosition},
};

const TIMESTAMP_BYTES: usize = 4;
const BYTES_PER_GLYPH: usize = 2;
const GRID_WIDTH: usize = 53;
const _GRID_HEIGHT: usize = 20;

#[derive(Debug, Clone, Default)]
pub struct Frame {
    pub time_millis: u32,
    pub glyphs: Vec<Glyph>,
}

impl TryFrom<&[u8]> for Frame {
    type Error = OsdFileError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let time_millis = u32::from_le_bytes(value[..TIMESTAMP_BYTES].try_into().unwrap());
        let glyphs = value[TIMESTAMP_BYTES..]
            .chunks(BYTES_PER_GLYPH)
            .enumerate()
            .filter_map(|(idx, glyph_bytes)| {
                let x = idx % GRID_WIDTH;
                let y = idx / GRID_WIDTH;
                let bytes = [glyph_bytes[0], glyph_bytes[1]];
                let index = u16::from_le_bytes(bytes);
                if index == 0x00 || index == 0x20 {
                    None
                } else {
                    let glyph = Glyph {
                        index,
                        grid_position: GridPosition {
                            x: x as u32,
                            y: y as u32,
                        },
                    };
                    Some(glyph)
                }
            })
            .collect();
        Ok(Self { time_millis, glyphs })
    }
}
