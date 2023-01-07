use std::{fs, path::PathBuf};

use crate::osd::frame::Frame;

use super::{error::OsdFileError, fc_firmware::FcFirmware};

const HEADER_BYTES: usize = 40;
const FC_TYPE_BYTES: usize = 4;
const FRAME_BYTES: usize = 2124;

pub struct OsdFile {
    pub file_path: PathBuf,
    pub fc_firmware: FcFirmware,
    pub frame_count: u32,
    pub frames: Vec<Frame>,
}

impl OsdFile {
    pub fn open(path: PathBuf) -> Result<Self, OsdFileError> {
        let mut bytes = fs::read(&path)?;
        let header_bytes = bytes.drain(0..HEADER_BYTES).collect::<Vec<u8>>();
        let fc_firmware = FcFirmware::try_from(&header_bytes[..FC_TYPE_BYTES])?;

        let frames = bytes
            .chunks(FRAME_BYTES)
            .map(|frame_bytes| frame_bytes.try_into().unwrap())
            .collect::<Vec<Frame>>();

        Ok(Self {
            file_path: path,
            fc_firmware,
            frame_count: frames.len() as u32,
            frames,
        })
    }
}
