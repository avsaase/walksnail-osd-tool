#![windows_subsystem = "windows"] // Hide console windows on Windows in release builds
#![allow(clippy::too_many_arguments)]

use ffmpeg::{dependencies::dependencies_statisfied, Encoder};
use ui::WalksnailOsdTool;

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
    // On startup check if ffmpeg and ffprove are available on the user's system
    // Then check which encoders are available
    let dependencies_satisfied = dependencies_statisfied();
    let available_ecoders = if dependencies_satisfied {
        Encoder::get_available_encoders()
    } else {
        vec![]
    };

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(780.0, 900.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Walksnail OSD Overlay Tool",
        options,
        Box::new(move |_cc| Box::new(WalksnailOsdTool::new(dependencies_satisfied, available_ecoders))),
    )
}
