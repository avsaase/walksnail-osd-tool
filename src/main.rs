#![allow(clippy::too_many_arguments)]

use ui::WalksnailOsdTool;

mod ffmpeg;
mod font;
mod osd;
mod overlay;
mod ui;
mod util;
mod video;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(780.0, 900.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Walksnail OSD Overlay Tool",
        options,
        Box::new(|_cc| Box::<WalksnailOsdTool>::default()),
    )
}
