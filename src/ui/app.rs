use std::{path::PathBuf, rc::Rc};

use crossbeam_channel::{Receiver, Sender};
use egui::{pos2, text::LayoutJob, vec2, Color32, TextFormat, TextStyle, TextureHandle, Visuals};

use crate::{
    ffmpeg::{Encoder, FromFfmpegMessage, RenderSettings, ToFfmpegMessage, VideoInfo},
    font, osd, srt,
    util::{Coordinates, Dimension},
};

use super::{
    osd_preview::create_osd_preview,
    utils::{set_custom_fonts, set_font_styles},
    RenderStatus,
};

#[derive(Default)]
pub struct WalksnailOsdTool {
    pub video_file: Option<PathBuf>,
    pub video_info: Option<VideoInfo>,
    pub osd_file: Option<osd::OsdFile>,
    pub font_file: Option<font::FontFile>,
    pub srt_file: Option<srt::SrtFile>,
    pub ui_dimensions: UiDimensions,
    pub to_ffmpeg_sender: Option<Sender<ToFfmpegMessage>>,
    pub from_ffmpeg_receiver: Option<Receiver<FromFfmpegMessage>>,
    pub render_status: RenderStatus,
    pub encoders: Vec<Rc<Encoder>>,
    pub show_undetected_encoders: bool,
    pub selected_encoder_idx: usize,
    pub dependencies: Dependencies,
    pub render_settings: RenderSettings,
    pub osd_preview: OsdPreview,
    pub osd_options: OsdOptions,
    pub about_window_open: bool,
    pub dark_mode: bool,
    pub srt_font: Option<rusttype::Font<'static>>,
}

impl WalksnailOsdTool {
    pub fn new(
        ctx: &egui::Context,
        dependencies_satisfied: bool,
        ffmpeg_path: PathBuf,
        ffprobe_path: PathBuf,
        encoders: Vec<Encoder>,
    ) -> Self {
        set_font_styles(ctx);
        let mut visuals = Visuals::light();
        visuals.indent_has_left_vline = false;
        set_custom_fonts(ctx);
        ctx.set_visuals(visuals);

        let srt_font: rusttype::Font<'static> =
            rusttype::Font::try_from_bytes(include_bytes!("../../resources/fonts/AzeretMono-Regular.ttf")).unwrap();

        Self {
            dependencies: Dependencies {
                dependencies_satisfied,
                ffmpeg_path,
                ffprobe_path,
            },
            encoders: encoders.into_iter().map(Rc::new).collect(),
            srt_font: Some(srt_font),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct Dependencies {
    pub dependencies_satisfied: bool,
    pub ffmpeg_path: PathBuf,
    pub ffprobe_path: PathBuf,
}

pub struct OsdPreview {
    pub texture_handle: Option<TextureHandle>,
    pub preview_frame: u32,
}

impl Default for OsdPreview {
    fn default() -> Self {
        Self {
            texture_handle: Default::default(),
            preview_frame: 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OsdOptions {
    pub horizontal_offset: i32,
    pub vertical_offset: i32,
    pub srt_options: SrtOptions,
}

#[derive(Debug, Clone)]
pub struct SrtOptions {
    pub position: Coordinates<i32>,
    pub scale: f32,
    pub show_time: bool,
    pub show_sbat: bool,
    pub show_gbat: bool,
    pub show_signal: bool,
    pub show_latency: bool,
    pub show_bitrate: bool,
    pub show_distance: bool,
}

impl Default for SrtOptions {
    fn default() -> Self {
        Self {
            position: Default::default(),
            scale: Default::default(),
            show_time: true,
            show_sbat: true,
            show_gbat: true,
            show_signal: true,
            show_latency: true,
            show_bitrate: true,
            show_distance: true,
        }
    }
}

impl SrtOptions {
    pub fn for_frame_size(&mut self, frame_size: Dimension<u32>) {
        let scale = frame_size.height / 30;
        let x_position = frame_size.width / 45;
        let y_position = frame_size.height - scale - x_position / 2;
        self.position = Coordinates::new(x_position as i32, y_position as i32);
        self.scale = scale as f32;
    }
}

pub struct UiDimensions {
    pub file_info_column1_width: f32,
    pub file_info_column2_width: f32,
    pub file_info_row_height: f32,
    pub osd_position_sliders_length: f32,
}

impl Default for UiDimensions {
    fn default() -> Self {
        Self {
            file_info_row_height: 17.0,
            file_info_column1_width: 100.0,
            file_info_column2_width: 135.0,
            osd_position_sliders_length: 200.0,
        }
    }
}

impl eframe::App for WalksnailOsdTool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.missing_dependencies_warning(ctx);

        // Keep updating the UI thread when rendering to make sure the indicated progress is up-to-date
        if self.render_status.is_in_progress() {
            ctx.request_repaint();
        }

        self.receive_ffmpeg_message();

        self.render_top_panel(ctx);

        self.render_bottom_panel(ctx);

        self.render_sidepanel(ctx);

        self.render_central_panel(ctx);
    }
}

impl WalksnailOsdTool {
    fn missing_dependencies_warning(&mut self, ctx: &egui::Context) {
        if !self.dependencies.dependencies_satisfied || self.encoders.is_empty() {
            egui::Window::new("Missing dependencies")
                .default_pos(pos2(175.0, 200.0))
                .fixed_size(vec2(350.0, 300.0))
                .collapsible(false)
                .show(ctx, |ui| {
                    let style = ctx.style();

                    let (default_color, strong_color) = if ui.visuals().dark_mode {
                        (Color32::LIGHT_GRAY, Color32::WHITE)
                    } else {
                        (Color32::DARK_GRAY, Color32::BLACK)
                    };
                    let default_font = TextFormat::simple(style.text_styles.get(&TextStyle::Body).unwrap().clone(), default_color);
                    let mono_font = TextFormat::simple(style.text_styles.get(&TextStyle::Monospace).unwrap().clone(), strong_color);

                    let mut job = LayoutJob::default();
                    job.append("ffmpeg", 0.0, mono_font.clone());
                    job.append(" and/or ", 0.0, default_font.clone());
                    job.append("ffprobe", 0.0, mono_font);
                    job.append(" could not be found. Nothing will work. They should have been installed together with this program. Please check your installation and report the problem on GitHub", 0.0, default_font);
                    ui.label(job);
                });
        }
    }

    pub fn update_osd_preview(&mut self, ctx: &egui::Context) {
        if let (Some(video_info), Some(osd_file), Some(font_file), Some(srt_file)) =
            (&self.video_info, &self.osd_file, &self.font_file, &self.srt_file)
        {
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [video_info.width as usize, video_info.height as usize],
                &create_osd_preview(
                    video_info.width,
                    video_info.height,
                    osd_file
                        .frames
                        .get(self.osd_preview.preview_frame as usize - 1)
                        .unwrap(),
                    srt_file.frames.last().unwrap(),
                    font_file,
                    self.srt_font.as_ref().unwrap(),
                    &self.osd_options,
                ),
            );
            let handle = ctx.load_texture("OSD preview", image, egui::TextureOptions::default());
            self.osd_preview.texture_handle = Some(handle);
        } else {
            self.osd_preview.texture_handle = None;
        }
    }

    pub fn receive_ffmpeg_message(&mut self) {
        if let (Some(tx), Some(rx), Some(video_info)) =
            (&self.to_ffmpeg_sender, &self.from_ffmpeg_receiver, &self.video_info)
        {
            while let Ok(message) = rx.try_recv() {
                if matches!(message, FromFfmpegMessage::EncoderFatalError(_))
                    || matches!(message, FromFfmpegMessage::EncoderFinished)
                {
                    tx.send(ToFfmpegMessage::AbortRender).ok();
                }
                self.render_status.update_from_ffmpeg_message(message, video_info)
            }
        }
    }
}
