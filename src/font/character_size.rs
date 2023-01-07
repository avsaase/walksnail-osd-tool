use std::fmt::Display;

const CHARACTER_WIDTH_LARGE: u32 = 36;
const CHARACTER_HEIGHT_LARGE: u32 = 54;
const CHARACTER_WIDTH_SMALL: u32 = 24;
const CHARACTER_HEIGHT_SMALL: u32 = 36;

#[derive(Debug, Clone)]
pub enum CharacterSize {
    Large,
    Small,
}

impl CharacterSize {
    pub fn from_width(width: u32) -> Self {
        match width {
            CHARACTER_WIDTH_LARGE => Self::Large,
            CHARACTER_WIDTH_SMALL => Self::Small,
            _ => panic!("Invalid OSD character width"),
        }
    }

    pub fn width(&self) -> u32 {
        match self {
            CharacterSize::Large => CHARACTER_WIDTH_LARGE,
            CharacterSize::Small => CHARACTER_WIDTH_SMALL,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            CharacterSize::Large => CHARACTER_HEIGHT_LARGE,
            CharacterSize::Small => CHARACTER_HEIGHT_SMALL,
        }
    }
}

impl Display for CharacterSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CharacterSize::Large => "Large",
                CharacterSize::Small => "Small",
            }
        )
    }
}
