use std::fmt::Display;

use super::FontFileError;

pub(crate) const CHARACTER_WIDTH_4K: u32 = 72;
pub(crate) const CHARACTER_HEIGHT_4K: u32 = 108;
pub(crate) const CHARACTER_WIDTH_2K: u32 = 48;
pub(crate) const CHARACTER_HEIGHT_2K: u32 = 72;
pub(crate) const CHARACTER_WIDTH_LARGE: u32 = 36;
pub(crate) const CHARACTER_HEIGHT_LARGE: u32 = 54;
pub(crate) const CHARACTER_WIDTH_SMALL: u32 = 24;
pub(crate) const CHARACTER_HEIGHT_SMALL: u32 = 36;

#[derive(Debug, Clone, PartialEq)]
pub enum CharacterSize {
    Large,
    Small,
    Ultra,
    XLarge,
}

impl CharacterSize {
    pub fn width(&self) -> u32 {
        match self {
            CharacterSize::Large => CHARACTER_WIDTH_LARGE,
            CharacterSize::Small => CHARACTER_WIDTH_SMALL,
            CharacterSize::XLarge => CHARACTER_WIDTH_2K,
            CharacterSize::Ultra => CHARACTER_WIDTH_4K,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            CharacterSize::Large => CHARACTER_HEIGHT_LARGE,
            CharacterSize::Small => CHARACTER_HEIGHT_SMALL,
            CharacterSize::XLarge => CHARACTER_HEIGHT_2K,
            CharacterSize::Ultra => CHARACTER_HEIGHT_4K,
        }
    }
}

impl Display for CharacterSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CharacterSize::Large => "1080p",
                CharacterSize::Small => "720p",
                CharacterSize::XLarge => "2.7K",
                CharacterSize::Ultra => "4K",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontType {
    Standard,
    FourColor,
}

impl FontType {
    pub fn pages(&self) -> u32 {
        match self {
            FontType::Standard => 1,
            FontType::FourColor => 4,
        }
    }
}

pub fn detect_dimensions(width: u32, height: u32) -> Result<(CharacterSize, FontType, u32), FontFileError> {
    let (size, r#type) = if width == CHARACTER_WIDTH_SMALL {
        (CharacterSize::Small, FontType::Standard)
    } else if width == CHARACTER_WIDTH_LARGE {
        (CharacterSize::Large, FontType::Standard)
    } else if width == CHARACTER_WIDTH_2K {
        (CharacterSize::XLarge, FontType::Standard)
    } else if width == CHARACTER_WIDTH_4K {
        (CharacterSize::Ultra, FontType::Standard)
    } else if width == CHARACTER_WIDTH_SMALL * 4 {
        (CharacterSize::Small, FontType::FourColor)
    } else if width == CHARACTER_WIDTH_LARGE * 4 {
        (CharacterSize::Large, FontType::FourColor)
    } else if width == CHARACTER_WIDTH_2K * 4 {
        (CharacterSize::XLarge, FontType::FourColor)
    } else if width == CHARACTER_WIDTH_4K * 4 {
        (CharacterSize::Ultra, FontType::FourColor)
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
            (72, 27648, CharacterSize::Ultra, FontType::Standard, 256),
            (48, 18432, CharacterSize::XLarge, FontType::Standard, 256),
            (36, 13824, CharacterSize::Large, FontType::Standard, 256),
            (24, 18432, CharacterSize::Small, FontType::Standard, 512),
            (36, 27648, CharacterSize::Large, FontType::Standard, 512),
            (48, 36864, CharacterSize::XLarge, FontType::Standard, 512),
            (72, 55296, CharacterSize::Ultra, FontType::Standard, 512),
            (96, 9216, CharacterSize::Small, FontType::FourColor, 256),
            (144, 13824, CharacterSize::Large, FontType::FourColor, 256),
            (192, 18432, CharacterSize::XLarge, FontType::FourColor, 256),
            (288, 27648, CharacterSize::Ultra, FontType::FourColor, 256),
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
