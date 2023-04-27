use confy::ConfyError;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::{
    ffmpeg::RenderSettings,
    ui::{AppUpdate, OsdOptions, SrtOptions, WalksnailOsdTool},
};

#[derive(Debug, Deserialize, Serialize, Derivative)]
#[derivative(Default)]
pub struct AppConfig {
    pub osd_options: OsdOptions,
    pub srt_options: SrtOptions,
    pub render_options: RenderSettings,
    pub app_update: AppUpdate,
    pub font_path: String,
}

const CONFIG_NAMESPACE: &str = "walksnail-osd-tool";
const CONFIG_NAME: &str = "saved_settings";

impl AppConfig {
    #[tracing::instrument(ret)]
    pub fn load_or_create() -> Self {
        let config: Result<Self, _> = confy::load(CONFIG_NAMESPACE, CONFIG_NAME);
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
        confy::store(CONFIG_NAMESPACE, CONFIG_NAME, self)
            .map_err(|e| tracing::error!("Failed to save config file, {}", e))
            .ok();
    }
}

impl From<&mut WalksnailOsdTool> for AppConfig {
    fn from(app_state: &mut WalksnailOsdTool) -> Self {
        Self {
            osd_options: app_state.osd_options.clone(),
            srt_options: app_state.srt_options.clone(),
            render_options: app_state.render_settings.clone(),
            app_update: app_state.app_update.clone(),
            font_path: app_state
                .font_file
                .as_ref()
                .map(|f| f.file_path.clone())
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        }
    }
}
