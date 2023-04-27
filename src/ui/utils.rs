use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use egui::{FontFamily, FontId, Margin, RichText, Separator, TextStyle, Ui};

use crate::{ffmpeg::VideoInfo, font::FontFile, osd::OsdFile, srt::SrtFile};

use super::WalksnailOsdTool;

impl WalksnailOsdTool {
    pub fn all_files_loaded(&self) -> bool {
        self.video_loaded() && self.osd_loaded() && self.srt_loaded() && self.font_loaded()
    }

    pub fn video_loaded(&self) -> bool {
        self.video_file.is_some() && self.video_info.is_some()
    }

    pub fn osd_loaded(&self) -> bool {
        self.osd_file.is_some()
    }

    pub fn srt_loaded(&self) -> bool {
        self.srt_file.is_some()
    }

    pub fn font_loaded(&self) -> bool {
        self.font_file.is_some()
    }

    pub fn import_video_file(&mut self, file_handles: &[PathBuf]) {
        if let Some(video_file) = filter_file_with_extention(file_handles, "mp4") {
            self.video_file = Some(video_file.clone());
            self.video_info = VideoInfo::get(video_file, &self.dependencies.ffprobe_path).ok();

            // Try to load the matching OSD and SRT files
            self.import_osd_file(&[matching_file_with_extension(video_file, "osd")]);
            self.import_srt_file(&[matching_file_with_extension(video_file, "srt")]);
        }
    }

    pub fn import_osd_file(&mut self, file_handles: &[PathBuf]) {
        if let Some(osd_file_path) = filter_file_with_extention(file_handles, "osd") {
            self.osd_file = OsdFile::open(osd_file_path.clone()).ok();
            self.osd_preview.preview_frame = 1;
        }
    }

    pub fn import_srt_file(&mut self, file_handles: &[PathBuf]) {
        if let Some(str_file_path) = filter_file_with_extention(file_handles, "srt") {
            self.srt_file = SrtFile::open(str_file_path.clone()).ok();
            self.srt_options.show_distance &= self.srt_file.as_ref().map(|s| s.has_distance).unwrap_or(true);
            self.config_changed = Some(Instant::now());
        }
    }

    pub fn import_font_file(&mut self, file_handles: &[PathBuf]) {
        if let Some(font_file_path) = filter_file_with_extention(file_handles, "png") {
            self.font_file = FontFile::open(font_file_path.clone()).ok();
            self.config_changed = Some(Instant::now());
        }
    }
}

pub fn filter_file_with_extention<'a>(files: &'a [PathBuf], extention: &'a str) -> Option<&'a PathBuf> {
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

#[tracing::instrument(ret, level = "info")]
pub fn matching_file_with_extension(path: &PathBuf, extention: &str) -> PathBuf {
    let file_name = path.file_stem().unwrap();
    let parent = path.parent().unwrap();
    parent.join(file_name).with_extension(extention)
}

pub fn separator_with_space(ui: &mut Ui, space: f32) {
    ui.scope(|ui| {
        ui.visuals_mut().widgets.noninteractive.bg_stroke.width = 0.5;
        ui.add(Separator::default().spacing(space));
    });
}

pub fn format_minutes_seconds(duration: &Duration) -> String {
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    format!("{}:{:0>2}", minutes, seconds)
}

pub fn get_output_video_path(input_video_path: &Path) -> PathBuf {
    let input_video_file_name = input_video_path.file_stem().unwrap().to_string_lossy();
    let output_video_file_name = format!("{}_with_osd.mp4", input_video_file_name);
    let mut output_video_path = input_video_path.parent().unwrap().to_path_buf();
    output_video_path.push(output_video_file_name);
    output_video_path
}

pub fn set_style(ctx: &egui::Context) {
    use egui::{
        FontFamily::{Monospace, Proportional},
        Style,
    };
    let mut style = Style::clone(&ctx.style());
    style.text_styles = [
        (TextStyle::Small, FontId::new(9.0, Proportional)),
        (TextStyle::Body, FontId::new(15.0, Proportional)),
        (TextStyle::Button, FontId::new(15.0, Proportional)),
        (TextStyle::Heading, FontId::new(17.0, Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, Monospace)),
        (TextStyle::Name("Tooltip".into()), FontId::new(14.0, Proportional)),
    ]
    .into();
    style.spacing.window_margin = Margin {
        left: 20.0,
        right: 20.0,
        top: 6.0,
        bottom: 20.0,
    };
    ctx.set_style(style);
}

pub fn tooltip_text(text: &str) -> RichText {
    RichText::new(text).font(FontId::new(14.0, FontFamily::Proportional))
}

pub fn set_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "inter-regular".to_owned(),
        egui::FontData::from_static(include_bytes!("../../resources/fonts/Inter-Regular.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "inter-regular".to_owned());

    ctx.set_fonts(fonts);
}
