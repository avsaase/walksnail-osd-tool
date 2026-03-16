use confy::ConfyError;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::{ffmpeg::RenderSettings, osd::OsdOptions, srt::SrtOptions, util::AppUpdate, NAMESPACE};

#[derive(Debug, Deserialize, Serialize, Derivative)]
#[derivative(Default)]
pub struct AppConfig {
    pub osd_options: OsdOptions,
    pub srt_options: SrtOptions,
    pub render_options: RenderSettings,
    pub app_update: AppUpdate,
    pub font_path: String,
    pub out_path: String,
}

const CONFIG_NAME: &str = "saved_settings";

impl AppConfig {
    #[tracing::instrument(ret)]
    pub fn load_or_create() -> Self {
        let config: Result<Self, _> = confy::load(NAMESPACE, CONFIG_NAME);
        if let Err(ConfyError::BadRonData(_)) = config {
            tracing::warn!("Invalid config found, resetting to default");
            let default_config = AppConfig::default();
            tracing::debug!("Default config: {:?}", default_config);
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
        confy::store(NAMESPACE, CONFIG_NAME, self)
            .map_err(|e| tracing::error!("Failed to save config file, {}", e))
            .ok();
    }
}
