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

    #[error("Invalid fond file size")]
    InvalidFontFileDimensions { dimensions: Dimension<u32> },
}
