use confy::ConfyError;
use serde::{Deserialize, Serialize};

use crate::{
    ui::{OsdOptions, SrtOptions},
    util::Coordinates,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub osd_config: OsdOptions,
    pub srt_config: SrtConfig,
}

impl AppConfig {
    #[tracing::instrument(ret)]
    pub fn load_or_create() -> Self {
        let config: Result<Self, _> = confy::load("Walksnail OSD Tool", "saved_settings");
        if let Err(ConfyError::BadTomlData(_)) = config {
            tracing::warn!("Invalid config found, resetting to default");
            let default_config = AppConfig::default();
            default_config.save();
            default_config
        } else {
            config
                .map_err(|e| tracing::error!("Failed to load or create new config, caused by {e}"))
                .unwrap()
        }
    }

    #[tracing::instrument]
    pub fn save(&self) {
        confy::store("Walksnail OSD Tool", "saved_settings", self).ok();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SrtConfig {
    pub x_position: f32,
    pub y_position: f32,
    pub scale: f32,
    pub show_time: bool,
    pub show_sbat: bool,
    pub show_gbat: bool,
    pub show_signal: bool,
    pub show_latency: bool,
    pub show_bitrate: bool,
    pub show_distance: bool,
}

impl Default for SrtConfig {
    fn default() -> Self {
        SrtOptions::default().into()
    }
}

impl From<SrtOptions> for SrtConfig {
    fn from(value: SrtOptions) -> Self {
        Self {
            x_position: value.position.x,
            y_position: value.position.y,
            scale: value.scale,
            show_time: value.show_time,
            show_sbat: value.show_sbat,
            show_gbat: value.show_gbat,
            show_signal: value.show_signal,
            show_latency: value.show_latency,
            show_bitrate: value.show_bitrate,
            show_distance: value.show_distance,
        }
    }
}

impl From<SrtConfig> for SrtOptions {
    fn from(value: SrtConfig) -> Self {
        Self {
            position: Coordinates {
                x: value.x_position,
                y: value.y_position,
            },
            scale: value.scale,
            show_time: value.show_time,
            show_sbat: value.show_sbat,
            show_gbat: value.show_gbat,
            show_signal: value.show_signal,
            show_latency: value.show_latency,
            show_bitrate: value.show_bitrate,
            show_distance: value.show_distance,
        }
    }
}
