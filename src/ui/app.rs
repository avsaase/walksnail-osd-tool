use std::{
    collections::HashSet,
    path::PathBuf,
    time::{Duration, Instant},
};

use crossbeam_channel::{Receiver, Sender};
use derivative::Derivative;
use egui::{
    pos2, text::LayoutJob, vec2, Align2, Color32, Frame, Grid, TextFormat, TextStyle, TextureHandle, Visuals, Window,
};
use github_release_check::{GitHubReleaseItem, LookupError};
use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::{
    config::AppConfig,
    ffmpeg::{Encoder, FromFfmpegMessage, RenderSettings, ToFfmpegMessage, VideoInfo},
    font, osd, srt,
    util::{build_info, check_updates, Coordinates},
};

use super::{
    osd_preview::create_osd_preview,
    utils::{set_custom_fonts, set_style},
    RenderStatus,
};

#[derive(Default)]

pub struct WalksnailOsdTool {
    pub config_changed: Option<Instant>,
    pub video_file: Option<PathBuf>,
    pub video_info: Option<VideoInfo>,
    pub osd_file: Option<osd::OsdFile>,
    pub font_file: Option<font::FontFile>,
    pub srt_file: Option<srt::SrtFile>,
    pub ui_dimensions: UiDimensions,
    pub to_ffmpeg_sender: Option<Sender<ToFfmpegMessage>>,
    pub from_ffmpeg_receiver: Option<Receiver<FromFfmpegMessage>>,
    pub render_status: RenderStatus,
    pub encoders: Vec<Encoder>,
    pub dependencies: Dependencies,
    pub render_settings: RenderSettings,
    pub osd_preview: OsdPreview,
    pub osd_options: OsdOptions,
    pub srt_options: SrtOptions,
    pub srt_font: Option<rusttype::Font<'static>>,
    pub about_window_open: bool,
    pub dark_mode: bool,
    pub app_update: AppUpdate,
}

impl WalksnailOsdTool {
    pub fn new(
        ctx: &egui::Context,
        dependencies_satisfied: bool,
        ffmpeg_path: PathBuf,
        ffprobe_path: PathBuf,
        encoders: Vec<Encoder>,
    ) -> Self {
        set_style(ctx);
        let mut visuals = Visuals::light();
        visuals.indent_has_left_vline = false;
        set_custom_fonts(ctx);
        ctx.set_visuals(visuals);

        let srt_font: rusttype::Font<'static> =
            rusttype::Font::try_from_bytes(include_bytes!("../../resources/fonts/AzeretMono-Regular.ttf")).unwrap();

        let config = AppConfig::load_or_create();
        let srt_options = config.srt_options;
        let osd_options = config.osd_options;

        // Load last used font file
        let font_path = PathBuf::from(config.font_path);
        let font_file = font::FontFile::open(font_path).ok();

        // Check for app updates
        let promise = if config.app_update.check_on_startup {
            Promise::spawn_thread("check_updates", check_updates).into()
        } else {
            None
        };
        let app_update = AppUpdate {
            promise,
            ..Default::default()
        };

        Self {
            dependencies: Dependencies {
                dependencies_satisfied,
                ffmpeg_path,
                ffprobe_path,
            },
            encoders,
            srt_font: Some(srt_font),
            osd_options,
            srt_options,
            font_file,
            app_update,
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

#[derive(Derivative)]
#[derivative(Default)]
pub struct OsdPreview {
    pub texture_handle: Option<TextureHandle>,
    #[derivative(Default(value = "1"))]
    pub preview_frame: u32,
    pub mask_edit_mode_enabled: bool,
}

#[derive(Clone, Serialize, Deserialize, Derivative)]
#[derivative(Default, Debug)]
pub struct OsdOptions {
    pub position: Coordinates<i32>,
    #[derivative(Default(value = "true"))]
    pub adjust_playback_speed: bool,
    #[derivative(Default(value = "1.0"))]
    #[serde(skip)]
    pub osd_playback_speed_factor: f32,
    pub masked_grid_positions: HashSet<Coordinates<u32>>,
}

impl OsdOptions {
    pub fn get_mask(&self, position: &Coordinates<u32>) -> bool {
        self.masked_grid_positions.contains(position)
    }

    pub fn toggle_mask(&mut self, position: Coordinates<u32>) {
        if self.masked_grid_positions.contains(&position) {
            self.masked_grid_positions.remove(&position);
        } else {
            self.masked_grid_positions.insert(position);
        }
    }

    pub fn reset_mask(&mut self) {
        self.masked_grid_positions.clear();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrtOptions {
    pub position: Coordinates<f32>,
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
            position: Coordinates::new(1.5, 95.0),
            scale: 35.0,
            show_time: false,
            show_sbat: false,
            show_gbat: false,
            show_signal: true,
            show_latency: true,
            show_bitrate: true,
            show_distance: true,
        }
    }
}

pub struct UiDimensions {
    pub file_info_column1_width: f32,
    pub file_info_column2_width: f32,
    pub file_info_row_height: f32,
    pub options_column1_width: f32,
    pub osd_position_sliders_length: f32,
}

impl Default for UiDimensions {
    fn default() -> Self {
        Self {
            file_info_row_height: 17.0,
            file_info_column1_width: 100.0,
            file_info_column2_width: 135.0,
            options_column1_width: 180.0,
            osd_position_sliders_length: 200.0,
        }
    }
}

#[derive(Derivative, Deserialize, Serialize)]
#[derivative(Default, Debug)]
pub struct AppUpdate {
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub promise: Option<Promise<Result<Option<GitHubReleaseItem>, LookupError>>>,
    #[serde(skip)]
    pub new_release: Option<GitHubReleaseItem>,
    #[serde(skip)]
    pub window_open: bool,
    #[serde(skip)]
    pub check_finished: bool,
    #[derivative(Default(value = "true"))]
    pub check_on_startup: bool,
}

impl Clone for AppUpdate {
    fn clone(&self) -> Self {
        Self {
            promise: None,
            new_release: self.new_release.clone(),
            window_open: self.window_open,
            check_finished: self.check_finished,
            check_on_startup: self.check_on_startup,
        }
    }
}

impl eframe::App for WalksnailOsdTool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.missing_dependencies_warning(ctx);

        self.update_window(ctx);

        // Keep updating the UI thread when rendering to make sure the indicated progress is up-to-date
        if self.render_status.is_in_progress() {
            ctx.request_repaint();
        }

        self.receive_ffmpeg_message();
        self.poll_update_check();

        self.render_top_panel(ctx);

        self.render_bottom_panel(ctx);

        self.render_sidepanel(ctx);

        self.render_central_panel(ctx);

        self.save_config_if_changed();
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
                    &self.srt_options,
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

    fn save_config_if_changed(&mut self) {
        if self
            .config_changed
            .map_or(false, |t| t.elapsed() > Duration::from_millis(2000))
        {
            let config: AppConfig = self.into();
            config.save();
            self.config_changed = None;
        }
    }

    fn poll_update_check(&mut self) {
        if !self.app_update.check_finished {
            if let Some(promise) = &self.app_update.promise {
                if let Some(result) = promise.ready() {
                    self.app_update.check_finished = true;
                    if let Ok(Some(latest_release)) = result {
                        self.app_update.new_release = Some(latest_release.clone());
                        self.app_update.window_open = true;
                    };
                }
            }
        }
    }

    fn update_window(&mut self, ctx: &egui::Context) {
        if self.app_update.window_open {
            if let Some(latest_release) = &self.app_update.new_release {
                let frame = Frame::window(&ctx.style());
                Window::new("App update!")
                    .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                    .frame(frame)
                    .open(&mut self.app_update.window_open)
                    .collapsible(false)
                    .auto_sized()
                    .show(ctx, |ui| {
                        ui.add_space(10.0);

                        Grid::new("update").spacing(vec2(10.0, 5.0)).show(ui, |ui| {
                            ui.label("Current version:");
                            ui.label(build_info::get_version().to_string());
                            ui.end_row();

                            ui.label("New version:");
                            ui.label(latest_release.tag_name.trim_start_matches('v'));
                            ui.end_row();
                        });
                        ui.add_space(5.0);
                        ui.hyperlink_to("View release on GitHub", &latest_release.html_url);

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui
                                .checkbox(
                                    &mut self.app_update.check_on_startup,
                                    "Check for updates on when program starts",
                                )
                                .changed()
                            {
                                self.config_changed = Instant::now().into();
                            };
                        });
                    });
            }
        }
    }
}
