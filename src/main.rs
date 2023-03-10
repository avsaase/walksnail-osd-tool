#![windows_subsystem = "windows"] // Hide console on Windows
#![allow(clippy::too_many_arguments)]

use eframe::IconData;
use ffmpeg::Encoder;
use ui::WalksnailOsdTool;

use crate::ffmpeg::dependencies::{ffmpeg_available, ffprobe_available};

mod ffmpeg;
mod font;
mod osd;
mod overlay;
mod ui;
mod util;
mod video;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() -> Result<(), eframe::Error> {
    let _guard = util::init_tracing();

    use util::build_info;
    tracing::info!(
        "{}",
        format!(
            "App started (version: {}, compiled with: {}, target: {})",
            build_info::get_version().unwrap_or("Unknwon".into()),
            build_info::get_compiler(),
            build_info::get_target()
        )
    );

    // On startup check if ffmpeg and ffprove are available on the user's system
    // Then check which encoders are available
    let ffmpeg_path = util::ffmpeg_path();
    let ffprobe_path = util::ffprobe_path();
    let dependencies_satisfied = ffmpeg_available(&ffmpeg_path) && ffprobe_available(&ffprobe_path);
    let detected_encoders = if dependencies_satisfied {
        Encoder::get_available_encoders(&ffmpeg_path)
    } else {
        vec![]
    };

    let icon_data = IconData {
        rgba: include_bytes!(concat!(env!("OUT_DIR"), "/icon_bytes")).to_vec(),
        width: 256,
        height: 256,
    };

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(780.0, 900.0)),
        resizable: false,
        icon_data: Some(icon_data),
        ..Default::default()
    };
    tracing::info!("Starting GUI");
    eframe::run_native(
        "Walksnail OSD Tool",
        options,
        Box::new(move |_cc| {
            Box::new(WalksnailOsdTool::new(
                dependencies_satisfied,
                ffmpeg_path,
                ffprobe_path,
                detected_encoders,
            ))
        }),
    )
}
