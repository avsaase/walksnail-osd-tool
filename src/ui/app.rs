use std::{
    path::PathBuf,
    rc::Rc,
    sync::mpsc::{Receiver, Sender},
};

use egui::{
    pos2, vec2, Align, Button, Color32, FontFamily, FontId, Image, Layout, ProgressBar, RichText, TextStyle,
    TextureHandle, Ui, Visuals,
};
use egui_extras::{Column, TableBuilder};
use ffmpeg_sidecar::event::FfmpegEvent;

use crate::{
    ffmpeg::{
        dependencies::{ffmpeg_available, ffprobe_available},
        Encoder, VideoInfo,
    },
    font::{self, FontFile},
    osd::{self, calculate_horizontal_offset, calculate_vertical_offset, osd_preview, OsdFile},
    video::{process_video, Settings, StopRenderMessage},
};

use super::{
    utils::{clickable_if, find_file_with_extention, format_minutes_seconds, separator_with_space},
    RenderStatus,
};

#[derive(Default)]
pub struct WalksnailOsdTool {
    pub video_file: Option<PathBuf>,
    pub video_info: Option<VideoInfo>,
    pub osd_file: Option<osd::OsdFile>,
    pub font_file: Option<font::FontFile>,
    ui_dimensions: UiDimensions,
    progress_receiver: Option<Receiver<FfmpegEvent>>,
    stop_render_sender: Option<Sender<StopRenderMessage>>,
    render_status: RenderStatus,
    available_encoders: Vec<Rc<Encoder>>,
    selected_encoder_idx: usize,
    dependencies_statisfied: Option<bool>,
    render_settings: Settings,
    osd_preview: OsdPreview,
}

struct OsdPreview {
    texture_handle: Option<TextureHandle>,
    horizontal_offset: i32,
    vertical_offset: i32,
    preview_frame: u32,
}

impl Default for OsdPreview {
    fn default() -> Self {
        Self {
            texture_handle: Default::default(),
            horizontal_offset: Default::default(),
            vertical_offset: Default::default(),
            preview_frame: 1,
        }
    }
}

struct UiDimensions {
    file_info_column1_width: f32,
    file_info_column2_width: f32,
    file_info_row_height: f32,
    osd_position_sliders_length: f32,
}

impl Default for UiDimensions {
    fn default() -> Self {
        Self {
            file_info_row_height: 16.0,
            file_info_column1_width: 90.0,
            file_info_column2_width: 135.0,
            osd_position_sliders_length: 200.0,
        }
    }
}

impl eframe::App for WalksnailOsdTool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.change_text_style(ctx);
        ctx.set_visuals(Visuals::light());

        // On startup check if the runtime dependencies are available. Show a warning if not.
        self.check_dependencies(ctx);

        // Keep updating the UI thread when rendering to make sure the indicated progress is up-to-date
        if self.render_status.is_in_progress() {
            ctx.request_repaint();
        }

        // Receive render progress from the ffmpeg thread (if it is running)
        self.receive_render_progress();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                self.import_files(ui, ctx);
                self.reset_files(ui);
            });
            ui.add_space(5.0);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                self.start_stop_render_button(ui);
                self.render_progress(ui);
            });
            ui.add_space(2.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Input Files");
                ui.horizontal(|ui| {
                    self.video_info(ui);
                    self.osd_info(ui);
                    self.font_info(ui);
                });

                separator_with_space(ui, 10.0);

                ui.heading("OSD Position");
                self.osd_position(ui, ctx);

                separator_with_space(ui, 10.0);

                ui.heading("Rendering Options");
                self.rendering_options(ui);
            });
        });
    }
}

impl WalksnailOsdTool {
    fn change_text_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        use FontFamily::{Monospace, Proportional};

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

    fn import_files(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let button = Button::new("Open files").sense(clickable_if(self.render_status.is_not_in_progress()));
        if ui.add(button).clicked() {
            if let Some(file_handles) = rfd::FileDialog::new()
                .add_filter("Goggle DVR & Font Files", &["mp4", "osd", "png"])
                .pick_files()
            {
                if let Some(video_file) = find_file_with_extention(&file_handles, "mp4") {
                    self.video_file = Some(video_file.clone());
                    self.video_info = VideoInfo::get(video_file).ok();
                }
                if let Some(osd_file_path) = find_file_with_extention(&file_handles, "osd") {
                    self.osd_file = OsdFile::open(osd_file_path.clone()).ok();
                    self.osd_preview.preview_frame = 1;
                }
                if let Some(font_file_path) = find_file_with_extention(&file_handles, "png") {
                    self.font_file = FontFile::open(font_file_path.clone()).ok();
                }
                self.update_osd_preview(ctx);
            }
        }
    }

    fn reset_files(&mut self, ui: &mut Ui) {
        let button = Button::new("Reset files").sense(clickable_if(self.render_status.is_not_in_progress()));
        if ui.add(button).clicked() {
            self.video_file = None;
            self.video_info = None;
            self.osd_file = None;
            self.font_file = None;
            self.osd_preview.texture_handle = None;
            self.osd_preview.preview_frame = 1;
            self.render_status = RenderStatus::Idle;
        }
    }

    fn video_info(&self, ui: &mut Ui) {
        let video_info = self.video_info.as_ref();

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Video file").strong());
                ui.push_id("video_info", |ui| {
                    TableBuilder::new(ui)
                        .column(Column::exact(self.ui_dimensions.file_info_column1_width))
                        .column(Column::exact(self.ui_dimensions.file_info_column2_width))
                        .body(|mut body| {
                            let row_height = self.ui_dimensions.file_info_row_height;
                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("File name:");
                                });
                                row.col(|ui| {
                                    if let Some(video_file) = &self.video_file {
                                        ui.label(video_file.file_name().unwrap().to_string_lossy());
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("Resolution:");
                                });
                                row.col(|ui| {
                                    if let (Some(width), Some(height)) =
                                        (video_info.map(|i| i.width), video_info.map(|i| i.height))
                                    {
                                        ui.label(format!("{}x{}", width, height));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("Frame rate:");
                                });
                                row.col(|ui| {
                                    if let Some(frame_rate) = video_info.map(|i| i.frame_rate) {
                                        ui.label(format!("{:.2} fps", frame_rate));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("Bitrate:");
                                });
                                row.col(|ui| {
                                    if let Some(bitrate) = video_info.map(|i| i.bitrate) {
                                        let bitrate_mbps = bitrate as f32 / 1_000_000.0;
                                        ui.label(format!("{:.2} Mbps", bitrate_mbps));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("Duration:");
                                });
                                row.col(|ui| {
                                    if let Some(duration_secs) = video_info.map(|i| i.duration_seconds) {
                                        let minutes = duration_secs / 60;
                                        let seconds = duration_secs % 60;
                                        ui.label(format!("{}:{:0>2}", minutes, seconds));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });
                        });
                });
            });
        });
    }

    fn osd_info(&self, ui: &mut Ui) {
        let osd_file = self.osd_file.as_ref();

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("OSD file").strong());
                ui.push_id("osd_info", |ui| {
                    TableBuilder::new(ui)
                        .column(Column::exact(self.ui_dimensions.file_info_column1_width))
                        .column(Column::exact(self.ui_dimensions.file_info_column2_width))
                        .body(|mut body| {
                            let row_height = self.ui_dimensions.file_info_row_height;
                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("File name:");
                                });
                                row.col(|ui| {
                                    if let Some(osd_file) = osd_file {
                                        ui.label(
                                            osd_file
                                                .file_path
                                                .file_name()
                                                .map(|f| f.to_string_lossy())
                                                .unwrap_or("-".into()),
                                        );
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("FC firmware:");
                                });
                                row.col(|ui| {
                                    if let Some(osd_file) = osd_file {
                                        ui.label(osd_file.fc_firmware.to_string());
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("Frames:");
                                });
                                row.col(|ui| {
                                    if let Some(osd_file) = osd_file {
                                        ui.label(osd_file.frame_count.to_string());
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            // Add two empty rows so the `ui.group()`s are the same height
                            body.rows(row_height, 2, |_, mut row| {
                                row.col(|_| {});
                            });
                        });
                });
            });
        });
    }

    fn font_info(&self, ui: &mut Ui) {
        let font_file = self.font_file.as_ref();

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Font file").strong());
                ui.push_id("font_info", |ui| {
                    TableBuilder::new(ui)
                        .column(Column::exact(self.ui_dimensions.file_info_column1_width))
                        .column(Column::exact(self.ui_dimensions.file_info_column2_width))
                        .body(|mut body| {
                            let row_height = self.ui_dimensions.file_info_row_height;
                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("File name:");
                                });
                                row.col(|ui| {
                                    if let Some(font_file) = font_file {
                                        ui.label(
                                            font_file
                                                .file_path
                                                .file_name()
                                                .map(|f| f.to_string_lossy())
                                                .unwrap_or("-".into()),
                                        );
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("Font size:");
                                });
                                row.col(|ui| {
                                    if let Some(font_file) = font_file {
                                        ui.label(font_file.character_size.to_string());
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("Characters:");
                                });
                                row.col(|ui| {
                                    if let Some(font_file) = font_file {
                                        ui.label(font_file.character_count.to_string());
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });

                            // Add two empty rows so the `ui.group()`s are the same height
                            body.rows(row_height, 2, |_, mut row| {
                                row.col(|_| {});
                            });
                        });
                });
            });
        });
    }

    fn osd_position(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.style_mut().spacing.slider_width = self.ui_dimensions.osd_position_sliders_length;
        egui::Grid::new("position_sliders")
            .spacing(vec2(15.0, 10.0))
            .show(ui, |ui| {
                ui.label("Horizontal offset");
                let horizontal_offset_slider =
                    ui.add(egui::Slider::new(&mut self.osd_preview.horizontal_offset, -200..=700).text("Pixels"));
                ui.add_space(3.0);

                if ui.button("Center").clicked() {
                    if let (Some(video_info), Some(osd_file), Some(font_file)) =
                        (&self.video_info, &self.osd_file, &self.font_file)
                    {
                        self.osd_preview.horizontal_offset = calculate_horizontal_offset(
                            video_info.width,
                            osd_file
                                .frames
                                .get(self.osd_preview.preview_frame as usize - 1)
                                .unwrap(),
                            &font_file.character_size,
                        );
                        self.update_osd_preview(ctx);
                    }
                }

                if ui.button("Reset").clicked() {
                    self.osd_preview.horizontal_offset = 0;
                    self.update_osd_preview(ctx);
                }
                ui.end_row();

                ui.label("Vertical offset");
                let vertical_offset_slider =
                    ui.add(egui::Slider::new(&mut self.osd_preview.vertical_offset, -200..=700).text("Pixels"));
                ui.add_space(3.0);

                if ui.button("Center").clicked() {
                    if let (Some(video_info), Some(osd_file), Some(font_file)) =
                        (&self.video_info, &self.osd_file, &self.font_file)
                    {
                        self.osd_preview.vertical_offset = calculate_vertical_offset(
                            video_info.height,
                            osd_file
                                .frames
                                .get(self.osd_preview.preview_frame as usize - 1)
                                .unwrap(),
                            &font_file.character_size,
                        );
                        self.update_osd_preview(ctx);
                    }
                }

                if ui.button("Reset").clicked() {
                    self.osd_preview.vertical_offset = 0;
                    self.update_osd_preview(ctx);
                }
                ui.end_row();

                if horizontal_offset_slider.changed() || vertical_offset_slider.changed() {
                    self.update_osd_preview(ctx);
                }
            });

        ui.collapsing("Preview", |ui| {
            ui.horizontal(|ui| {
                ui.label("Preview frame");
                let preview_frame_slider = ui.add(
                    egui::Slider::new(
                        &mut self.osd_preview.preview_frame,
                        1..=self.osd_file.as_ref().map(|f| f.frame_count).unwrap_or(1),
                    )
                    .smart_aim(false),
                );
                if preview_frame_slider.changed() {
                    self.update_osd_preview(ctx);
                }
            });

            if let Some(handle) = &self.osd_preview.texture_handle {
                let width = 725.0;
                let widescreen_height = width * 9.0 / 16.0;
                let image = Image::new(handle, vec2(width, widescreen_height));
                ui.add(image.bg_fill(Color32::LIGHT_GRAY));
            }
        });
    }

    fn rendering_options(&mut self, ui: &mut Ui) {
        egui::Grid::new("render_options")
            .spacing(vec2(15.0, 10.0))
            .show(ui, |ui| {
                ui.label("Encoder");
                let resp = egui::ComboBox::from_id_source("encoder").width(250.0).show_index(
                    ui,
                    &mut self.selected_encoder_idx,
                    self.available_encoders.len(),
                    |i| {
                        self.available_encoders
                            .get(i)
                            .map(|e| e.to_string())
                            .unwrap_or("None".to_string())
                    },
                );
                if resp.changed() {
                    // This is a little hacky but it's nice to have a single struct that keeps track of all render settings
                    self.render_settings.encoder =
                        self.available_encoders.get(self.selected_encoder_idx).unwrap().clone();
                }
                ui.end_row();

                ui.label("Encoding bitrate");
                ui.add(egui::Slider::new(&mut self.render_settings.bitrate_mbps, 0..=100).text("Mbit/s"));
                ui.end_row();
            });
    }

    fn start_stop_render_button(&mut self, ui: &mut Ui) {
        let button_size = vec2(110.0, 40.0);
        match self.render_status {
            RenderStatus::Idle | RenderStatus::Completed => {
                if ui
                    .add(
                        egui::Button::new("Start render")
                            .sense(clickable_if(self.all_files_loaded()))
                            .min_size(button_size),
                    )
                    .clicked()
                {
                    if let (Some(video_path), Some(osd_file), Some(font_file), Some(video_info)) =
                        (&self.video_file, &self.osd_file, &self.font_file, &self.video_info)
                    {
                        process_video(
                            video_path.to_str().unwrap(),
                            "output_video.mp4",
                            osd_file.frames.clone(),
                            font_file.clone(),
                            video_info,
                            &self.render_settings,
                            self.osd_preview.horizontal_offset,
                            self.osd_preview.vertical_offset,
                        )
                        .map(|(progress_rx, stop_render_tx)| {
                            self.progress_receiver = Some(progress_rx);
                            self.stop_render_sender = Some(stop_render_tx);
                        })
                        .ok();
                    }
                }
            }
            RenderStatus::InProgress {
                time_remaining: _,
                fps: _,
                speed: _,
                progress_pct: _,
            } => {
                if ui.add(Button::new("Stop render").min_size(button_size)).clicked() {
                    if let Some(sender) = &self.stop_render_sender {
                        sender.send(StopRenderMessage).ok();
                    }
                }
            }
        }
    }

    fn render_progress(&mut self, ui: &mut Ui) {
        match self.render_status {
            RenderStatus::Idle => {}
            RenderStatus::InProgress {
                time_remaining,
                fps,
                speed,
                progress_pct,
            } => {
                ui.vertical(|ui| {
                    ui.add(ProgressBar::new(progress_pct).show_percentage());
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.add_space(3.0);
                        ui.label(format!(
                            "Time remaining: {}, fps: {:.1}, speed: {:.3}x",
                            format_minutes_seconds(&time_remaining),
                            fps,
                            speed
                        ));
                    });
                });
            }
            RenderStatus::Completed => {
                ui.vertical(|ui| {
                    ui.add(ProgressBar::new(1.0).text("Done"));
                });
            }
        }
    }

    fn receive_render_progress(&mut self) {
        if let (Some(rx), Some(video_info)) = (&self.progress_receiver, &self.video_info) {
            rx.try_recv()
                .map(|message| self.render_status = RenderStatus::from_ffmpeg_event(message, video_info))
                .ok();
        }
    }

    fn check_dependencies(&mut self, ctx: &egui::Context) {
        match &self.dependencies_statisfied {
            None => {
                let ffmpeg_available = ffmpeg_available();
                let ffprobe_available = ffprobe_available();
                let available_encoders = if ffmpeg_available {
                    Encoder::get_available_encoders()
                } else {
                    vec![]
                };
                self.dependencies_statisfied =
                    Some(ffmpeg_available && ffprobe_available && !available_encoders.is_empty());
                self.available_encoders = available_encoders.into_iter().map(Rc::new).collect();
            }
            Some(dependencies_statisfied) => {
                if !dependencies_statisfied {
                    egui::Window::new("Missing dependencies")
                        .default_pos(pos2(175.0, 200.0))
                        .fixed_size(vec2(350.0, 300.0))
                        .collapsible(false)
                        .show(ctx, |ui| {
                            let text = indoc::indoc! {"
                                ffmpeg and/or ffprobe could not be found on your system. Nothing will work.
                                
                                You have two options:
                                  1. Make sure both ffmpeg and ffprobe are installed and in your path.
                                  2. Download a version of this tool which includes the dependencies.
                            "};
                            ui.label(text);
                        });
                }
            }
        }
    }

    fn update_osd_preview(&mut self, ctx: &egui::Context) {
        if let (Some(video_info), Some(osd_file), Some(font_file)) = (&self.video_info, &self.osd_file, &self.font_file)
        {
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [video_info.width as usize, video_info.height as usize],
                &osd_preview(
                    video_info.width,
                    video_info.height,
                    osd_file
                        .frames
                        .get(self.osd_preview.preview_frame as usize - 1)
                        .unwrap(),
                    font_file,
                    self.osd_preview.horizontal_offset,
                    self.osd_preview.vertical_offset,
                ),
            );
            let handle = ctx.load_texture("OSD preview", image, egui::TextureOptions::default());
            self.osd_preview.texture_handle = Some(handle);
        }
    }
}
