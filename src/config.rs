use confy::ConfyError;
use serde::{Deserialize, Serialize};

use crate::ui::{OsdOptions, SrtOptions};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub osd_options: OsdOptions,
    pub srt_options: SrtOptions,
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
