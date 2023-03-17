use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use egui::Ui;

use crate::{ffmpeg::VideoInfo, font::FontFile, osd::OsdFile};

use super::WalksnailOsdTool;

impl WalksnailOsdTool {
    pub fn all_files_loaded(&self) -> bool {
        match (&self.video_file, &self.video_info, &self.osd_file, &self.font_file) {
            (Some(_), Some(_), Some(_), Some(_)) => true,
            (_, _, _, _) => false,
        }
    }

    pub fn import_font_file(&mut self, file_handles: &[PathBuf]) {
        if let Some(font_file_path) = find_file_with_extention(file_handles, "png") {
            self.font_file = FontFile::open(font_file_path.clone()).ok();
        }
    }

    pub fn import_osd_file(&mut self, file_handles: &[PathBuf]) {
        if let Some(osd_file_path) = find_file_with_extention(file_handles, "osd") {
            self.osd_file = OsdFile::open(osd_file_path.clone()).ok();
            self.osd_preview.preview_frame = 1;
        }
    }

    pub fn import_video_file(&mut self, file_handles: &[PathBuf]) {
        if let Some(video_file) = find_file_with_extention(file_handles, "mp4") {
            self.video_file = Some(video_file.clone());
            self.video_info = VideoInfo::get(video_file, &self.dependencies.ffprobe_path).ok();
        }
    }
}

pub fn find_file_with_extention<'a>(files: &'a [PathBuf], extention: &'a str) -> Option<&'a PathBuf> {
    files.iter().find_map(|f| {
        f.extension().and_then(|e| {
            if e.to_string_lossy() == extention {
                Some(f)
            } else {
                None
            }
        })
    })
}

pub fn separator_with_space(ui: &mut Ui, space: f32) {
    ui.add_space(space);
    ui.separator();
    ui.add_space(space);
}

pub fn format_minutes_seconds(mabe_duration: &Option<Duration>) -> String {
    match mabe_duration {
        Some(duration) => {
            let minutes = duration.as_secs() / 60;
            let seconds = duration.as_secs() % 60;
            format!("{}:{:0>2}", minutes, seconds)
        }
        None => "––:––".into(),
    }
}

pub fn get_output_video_path(input_video_path: &Path) -> PathBuf {
    let input_video_file_name = input_video_path.file_stem().unwrap().to_string_lossy();
    let output_video_file_name = format!("{}_with_osd.mp4", input_video_file_name);
    let mut output_video_path = input_video_path.parent().unwrap().to_path_buf();
    output_video_path.push(output_video_file_name);
    output_video_path
}

pub fn set_font_styles(ctx: &egui::Context) {
    use egui::{
        FontFamily::{Monospace, Proportional},
        FontId, Style, TextStyle,
    };
    let mut style = Style::clone(&ctx.style());
    style.text_styles = [
        (TextStyle::Small, FontId::new(9.0, Proportional)),
        (TextStyle::Body, FontId::new(15.0, Proportional)),
        (TextStyle::Button, FontId::new(15.0, Proportional)),
        (TextStyle::Heading, FontId::new(17.0, Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, Monospace)),
    ]
    .into();
    ctx.set_style(style);
}
