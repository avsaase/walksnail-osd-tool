use thiserror::Error;

use crate::util::Dimension;

#[derive(Error, Debug)]
pub enum FontFileError {
    #[error("Failed to open font file")]
    FailedToOpen {
        #[from]
        source: std::io::Error,
    },

    #[error("Failed to decode font file")]
    FailedToDecode {
        #[from]
        source: image::ImageError,
    },

    #[error("Invalid fond file dimensions {dimensions}")]
    InvalidFontFileDimensions { dimensions: Dimension<u32> },

    #[error("Invalid fond file width {width}")]
    InvalidFontFileWidth { width: u32 },

    #[error("Invalid fond file height {height}")]
    InvalidFontFileHeight { height: u32 },
}
