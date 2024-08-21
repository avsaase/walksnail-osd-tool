#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console on Windows in release builds
#![allow(clippy::too_many_arguments)]
#![allow(clippy::collapsible_else_if)]

use app::WalksnailOsdTool;
use backend::{
    config::AppConfig,
    ffmpeg::{ffmpeg_available, ffprobe_available, Encoder},
};
use egui::{IconData, ViewportBuilder};
use poll_promise::Promise;

use crate::util::check_updates;

mod app;
mod bottom_panel;
mod central_panel;
mod osd_preview;
mod render_status;
mod side_panel;
mod top_panel;
mod util;

fn main() -> Result<(), eframe::Error> {
    let _guard = util::init_tracing();

    use util::build_info;
    tracing::info!(
        "{}",
        format!(
            "App started (version: {}, target: {}, compiled with: rustc {})",
            build_info::get_version(),
            build_info::get_target(),
            build_info::get_compiler()
        )
    );

    // On startup check if ffmpeg and ffprove are available on the user's system
    // Then check which encoders are available
    let ffmpeg_path = util::get_dependency_path("ffmpeg");
    let ffprobe_path = util::get_dependency_path("ffprobe");
    let dependencies_satisfied = ffmpeg_available(&ffmpeg_path) && ffprobe_available(&ffprobe_path);
    let encoders = if dependencies_satisfied {
        Encoder::get_available_encoders(&ffmpeg_path)
    } else {
        vec![]
    };

    let config = AppConfig::load_or_create();
    let promise = if config.app_update.check_on_startup {
        Promise::spawn_thread("check_updates", check_updates).into()
    } else {
        None
    };

    let icon_data = IconData {
        rgba: include_bytes!(concat!(env!("OUT_DIR"), "/icon_bytes")).to_vec(),
        width: 256,
        height: 256,
    };

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_drag_and_drop(true)
            .with_icon(icon_data)
            .with_min_inner_size([600.0, 300.0])
            .with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };
    tracing::info!("Starting GUI");
    eframe::run_native(
        "Walksnail OSD Tool",
        options,
        Box::new(move |cc| {
            Ok(Box::new(WalksnailOsdTool::new(
                &cc.egui_ctx,
                dependencies_satisfied,
                ffmpeg_path,
                ffprobe_path,
                encoders,
                config,
                build_info::get_version().to_string(),
                build_info::get_target().to_string(),
                promise,
            )))
        }),
    )
}
