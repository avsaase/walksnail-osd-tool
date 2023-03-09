use thiserror::Error;

#[derive(Error, Debug)]
pub enum OsdFileError {
    // #[error("Unknown FC firmware: {0}")]
    // UnknownFcFirmware(String),

    // #[error("Incomplete frame with index {index}")]
    // IncompleteFrame { index: u32 },
    #[error("Malformed OSD file")]
    MalformedOsdFile {
        #[from]
        source: std::str::Utf8Error,
    },

    #[error("Unable to open OSD file")]
    UnableToOpenFile {
        #[from]
        source: std::io::Error,
    },
}
