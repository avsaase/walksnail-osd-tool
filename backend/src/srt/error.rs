use parse_display::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SrtFileError {
    #[error("Failed to parse data from STR file, source: {source}")]
    FailedToParseData {
        #[from]
        source: ParseError,
    },

    #[error("Unable to open SRT file, source: {source}")]
    UnableToOpenFile {
        #[from]
        source: srtparse::ReaderError,
    },
}
