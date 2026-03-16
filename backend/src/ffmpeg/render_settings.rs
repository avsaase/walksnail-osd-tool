use serde::{Deserialize, Serialize};

use crate::ffmpeg::{Codec, Encoder};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderSettings {
    pub encoder: Encoder,
    pub selected_encoder_idx: usize,
    pub show_undetected_encoders: bool,
    pub bitrate_mbps: u32,
    pub upscale: bool,
    pub rescale_to_4x3_aspect: bool,
    pub use_chroma_key: bool,
    pub chroma_key: [f32; 3],
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            encoder: Encoder {
                name: "libx264".to_string(),
                codec: Codec::H264,
                hardware: false,
                detected: false,
                extra_args: Vec::new(),
            },
            selected_encoder_idx: 0,
            show_undetected_encoders: false,
            bitrate_mbps: 40,
            upscale: false,
            rescale_to_4x3_aspect: false,
            use_chroma_key: false,
            chroma_key: [1.0 / 255.0, 177.0 / 255.0, 64.0 / 255.0],
        }
    }
}
