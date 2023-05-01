use std::fmt::{Debug, Display};

use crate::util::Coordinates;

pub type GridPosition = Coordinates<u32>;

#[derive(Debug, Clone)]
pub struct Glyph {
    pub index: u16,
    pub grid_position: GridPosition,
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.index < 128 {
            if let Some(char) = char::from_u32(self.index as u32) {
                if char.is_ascii() && !char.is_ascii_control() {
                    write!(f, "{}", char)?;
                } else {
                    write!(f, " ")?;
                }
            }
        } else {
            write!(f, "*")?;
        }
        Ok(())
    }
}
