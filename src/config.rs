use confy::ConfyError;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::ui::{AppUpdate, OsdOptions, SrtOptions, WalksnailOsdTool};

#[derive(Debug, Deserialize, Serialize, Derivative)]
#[derivative(Default)]
pub struct AppConfig {
    pub osd_options: OsdOptions,
    pub srt_options: SrtOptions,
    pub app_update: AppUpdate,
}

impl AppConfig {
    #[tracing::instrument(ret)]
    pub fn load_or_create() -> Self {
        let config: Result<Self, _> = confy::load("Walksnail OSD Tool", "saved_settings");
        if let Err(ConfyError::BadRonData(_)) = config {
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
        confy::store("Walksnail OSD Tool", "saved_settings", self)
            .map_err(|e| tracing::error!("Failed to save config file, {}", e))
            .ok();
    }
}

impl From<&mut WalksnailOsdTool> for AppConfig {
    fn from(app_state: &mut WalksnailOsdTool) -> Self {
        Self {
            osd_options: app_state.osd_options.clone(),
            srt_options: app_state.srt_options.clone(),
            app_update: app_state.app_update.clone(),
        }
    }
}
