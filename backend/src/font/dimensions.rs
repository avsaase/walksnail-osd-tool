use std::fmt::Display;

use super::FontFileError;

pub(crate) const CHARACTER_WIDTH_LARGE: u32 = 36;
pub(crate) const CHARACTER_HEIGHT_LARGE: u32 = 54;
pub(crate) const CHARACTER_WIDTH_SMALL: u32 = 24;
pub(crate) const CHARACTER_HEIGHT_SMALL: u32 = 36;

#[derive(Debug, Clone, PartialEq)]
pub enum CharacterSize {
    Large,
    Small,
}

impl CharacterSize {
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

#[derive(Debug, Clone, PartialEq)]
pub enum FontType {
    SinglePage,
    FourPage,
}

impl FontType {
    pub fn pages(&self) -> u32 {
        match self {
            FontType::SinglePage => 1,
            FontType::FourPage => 4,
        }
    }
}

pub fn detect_dimensions(width: u32, height: u32) -> Result<(CharacterSize, FontType, u32), FontFileError> {
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

    let characters_count = height / size.height();

    Ok((size, r#type, characters_count))
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok_eq};

    use super::*;

    #[test]
    fn detect_valid_font_sizes() {
        let test_cases = [
            (36, 13824, CharacterSize::Large, FontType::SinglePage, 256),
            (24, 18432, CharacterSize::Small, FontType::SinglePage, 512),
            (36, 27648, CharacterSize::Large, FontType::SinglePage, 512),
            (96, 9216, CharacterSize::Small, FontType::FourPage, 256),
            (144, 13824, CharacterSize::Large, FontType::FourPage, 256),
        ];
        for test in test_cases {
            assert_ok_eq!(detect_dimensions(test.0, test.1), (test.2, test.3, test.4));
        }
    }

    #[test]
    fn reject_invalid_font_sizes() {
        let test_cases = [(36, 13824 + 1), (24 + 1, 18432), (36 - 12, 27648 - 10)];
        for test in test_cases {
            assert_err!(detect_dimensions(test.0, test.1));
        }
    }
}
