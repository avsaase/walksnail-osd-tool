use serde::{Deserialize, Serialize};

use crate::util::Coordinates;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrtOptions {
    pub position: Coordinates<f32>,
    pub scale: f32,
    pub show_time: bool,
    pub show_sbat: bool,
    pub show_gbat: bool,
    pub show_signal: bool,
    pub show_latency: bool,
    pub show_bitrate: bool,
    pub show_distance: bool,
}

impl Default for SrtOptions {
    fn default() -> Self {
        Self {
            position: Coordinates::new(1.5, 95.0),
            scale: 35.0,
            show_time: false,
            show_sbat: false,
            show_gbat: false,
            show_signal: true,
            show_latency: true,
            show_bitrate: true,
            show_distance: true,
        }
    }
}
