use serde::{Deserialize, Serialize};

use crate::ffmpeg::{Codec, Encoder};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderSettings {
    pub encoder: Encoder,
    pub selected_encoder_idx: usize,
    pub show_undetected_encoders: bool,
    pub bitrate_mbps: u32,
    pub upscale: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            encoder: Encoder {
                name: "libx264".to_string(),
                codec: Codec::H264,
                hardware: false,
                detected: false,
            },
            selected_encoder_idx: 0,
            show_undetected_encoders: false,
            bitrate_mbps: 40,
            upscale: false,
        }
    }
}
