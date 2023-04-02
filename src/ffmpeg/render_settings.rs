use std::rc::Rc;

use crate::ffmpeg::{Codec, Encoder};

#[derive(Debug)]
pub struct RenderSettings {
    pub encoder: Rc<Encoder>,
    pub bitrate_mbps: u32,
    pub upscale: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            encoder: Rc::new(Encoder {
                name: "libx264".to_string(),
                codec: Codec::H264,
                hardware: false,
                detected: false,
            }),
            bitrate_mbps: 40,
            upscale: false,
        }
    }
}
