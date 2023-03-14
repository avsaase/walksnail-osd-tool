use std::rc::Rc;

use crate::ffmpeg::{Codec, Encoder};

#[derive(Debug)]
pub struct EncoderSettings {
    pub encoder: Rc<Encoder>,
    pub bitrate_mbps: u32,
}

impl Default for EncoderSettings {
    fn default() -> Self {
        Self {
            encoder: Rc::new(Encoder {
                name: "libx264".to_string(),
                codec: Codec::H264,
                hardware: false,
            }),
            bitrate_mbps: 40,
        }
    }
}
