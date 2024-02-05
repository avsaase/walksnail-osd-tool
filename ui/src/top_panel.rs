use std::thread::sleep;
use std::time::Duration;
use egui::{Align2, Button, Frame, Label, RichText, Sense, Ui, vec2, Visuals, Window};
use backend::ffmpeg::start_video_render;
use crate::render_status::Status;
use crate::util::get_output_video_path;

use super::WalksnailOsdTool;

impl WalksnailOsdTool {
    pub fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                self.import_files(ui, ctx);
                self.reset_files(ui);
                self.multi_files(ui, ctx);
                self.process_multi();
                ui.add_space(ui.available_width() - 55.0);
                self.toggle_light_dark_theme(ui, ctx);
                self.about_window(ui, ctx);
            });
            ui.add_space(3.0);
        });
    }

    fn process_multi(&mut self) {
        if self.multi_file_window == false {
            return;
        }
        if self.render_status.is_in_progress() {
            return;
        }
        if self.videos.is_empty() {
            return;
        }
        let video = self.videos.pop().unwrap();
        &self.import_video_file(&[video.into()]);
        &self.render_status.start_render();
        if let (Some(video_path), Some(osd_file), Some(font_file), Some(video_info), Some(srt_file)) = (
            &self.video_file,
            &self.osd_file,
            &self.font_file,
            &self.video_info,
            &self.srt_file,
        ) {
            self.osd_options.osd_playback_speed_factor = if self.osd_options.adjust_playback_speed {
                let video_duration = video_info.duration;
                let osd_duration = osd_file.duration;
                video_duration.as_secs_f32() / osd_duration.as_secs_f32()
            } else {
                1.0
            };
            match start_video_render(
                &self.dependencies.ffmpeg_path,
                video_path,
                &get_output_video_path(video_path),
                osd_file.frames.clone(),
                srt_file.frames.clone(),
                font_file.clone(),
                self.srt_font.as_ref().unwrap().clone(),
                &self.osd_options,
                &self.srt_options,
                video_info,
                &self.render_settings,
            ) {
                Ok((to_ffmpeg_sender, from_ffmpeg_receiver)) => {
                    self.to_ffmpeg_sender = Some(to_ffmpeg_sender);
                    self.from_ffmpeg_receiver = Some(from_ffmpeg_receiver);
                }
                Err(_) => {
                    self.render_status.status = Status::Error {
                        progress_pct: 0.0,
                        error: "Failed to start video render".to_string(),
                    }
                }
            };
        }
    }
    fn multi_files(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        if ui
            .add_enabled(self.all_files_loaded() && self.render_status.is_not_in_progress(), Button::new("Multiple Files"))
            .clicked()
        {
            self.multi_file_window = !self.multi_file_window;
        }

        if self.multi_file_window {
            let frame = Frame::window(&ctx.style());
            Window::new("Multiple Files")
                .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                .frame(frame)
                .open(&mut self.multi_file_window)
                .auto_sized()
                .collapsible(false)
                .show(ctx, |ui| {
                    let videos = std::fs::read_dir(self.video_file.as_ref().unwrap().parent().unwrap())
                        .unwrap()
                        .filter_map(|res| res.ok())
                        .map(|dir_entry| dir_entry.path())
                        .filter(|path| path.extension().map(|a| a == "mp4").unwrap_or(false))
                        .filter(|path| !path.file_name().map(|file_name| file_name.to_str().unwrap().ends_with("with_osd.mp4")).unwrap_or(false))
                        .collect::<Vec<_>>();
                    egui::Grid::new("multi").spacing(vec2(10.0, 5.0)).show(ui, |ui| {
                        ui.label("File folder:");
                        ui.label(self.video_file.as_ref().unwrap().parent().unwrap().to_string_lossy());
                        ui.end_row();
                        ui.label("Files count:");
                        ui.label(videos.len().to_string());
                        ui.end_row();
                        ui.label("Font file:");
                        ui.label(self.font_file.as_ref().unwrap()
                                     .file_path
                                     .file_name()
                                     .map(|f| f.to_string_lossy())
                                     .unwrap_or("-".into()), );
                        ui.end_row();
                        let button = ui.add_enabled(self.render_status.is_not_in_progress(), Button::new("Start Bulk process"));
                        if button.clicked() {
                            tracing::info!("Start multiple render button clicked");
                            self.videos = videos;
                        }
                    });
                });
        }
    }

    fn import_files(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        if ui
            .add_enabled(self.render_status.is_not_in_progress(), Button::new("Open files"))
            .clicked()
        {
            if let Some(file_handles) = rfd::FileDialog::new()
                .add_filter("Avatar files", &["mp4", "osd", "png", "srt"])
                .pick_files()
            {
                tracing::info!("Opened files {:?}", file_handles);
                self.import_video_file(&file_handles);
                self.import_osd_file(&file_handles);
                self.import_font_file(&file_handles);
                self.import_srt_file(&file_handles);

                self.update_osd_preview(ctx);
                self.render_status.reset();
            }
        }

        // Collect dropped files
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let file_handles = i
                    .raw
                    .dropped_files
                    .iter()
                    .flat_map(|f| f.path.clone())
                    .collect::<Vec<_>>();
                tracing::info!("Dropped files {:?}", file_handles);
                self.import_video_file(&file_handles);
                self.import_osd_file(&file_handles);
                self.import_font_file(&file_handles);
                self.import_srt_file(&file_handles);

                self.update_osd_preview(ctx);
                self.render_status.reset();
            }
        });
    }

    fn reset_files(&mut self, ui: &mut Ui) {
        if ui
            .add_enabled(self.render_status.is_not_in_progress(), Button::new("Reset files"))
            .clicked()
        {
            self.video_file = None;
            self.video_info = None;
            self.osd_file = None;
            self.font_file = None;
            self.srt_file = None;
            self.osd_preview.texture_handle = None;
            self.osd_preview.preview_frame = 1;
            self.render_status.reset();
            tracing::info!("Reset files");
        }
    }

    fn toggle_light_dark_theme(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let icon = if self.dark_mode { "☀" } else { "🌙" };
        if ui.add(Button::new(icon).frame(false)).clicked() {
            let mut visuals = if self.dark_mode {
                Visuals::light()
            } else {
                Visuals::dark()
            };
            visuals.indent_has_left_vline = false;
            ctx.set_visuals(visuals);
            self.dark_mode = !self.dark_mode;
        }
    }

    fn about_window(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        if ui.add(Button::new(RichText::new("ℹ")).frame(false)).clicked() {
            self.about_window_open = !self.about_window_open;
        }

        let frame = Frame::window(&ctx.style());
        if self.about_window_open {
            Window::new("About")
                .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                .frame(frame)
                .open(&mut self.about_window_open)
                .auto_sized()
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.add_space(10.0);

                    egui::Grid::new("about").spacing(vec2(10.0, 5.0)).show(ui, |ui| {
                        ui.label("Author:");
                        ui.label("Alexander van Saase");
                        ui.end_row();

                        ui.label("Version:");
                        let version = &self.app_version;
                        if ui
                            .add(Label::new(version).sense(Sense::click()))
                            .on_hover_text_at_pointer("Double-click to copy to clipboard")
                            .double_clicked()
                        {
                            ui.output_mut(|o| o.copied_text = version.clone());
                        }
                        ui.end_row();

                        ui.label("Target:");
                        ui.label(&self.target);
                        ui.end_row();

                        ui.label("License:");
                        ui.hyperlink_to(
                            "General Public License v3.0",
                            "https://github.com/avsaase/walksnail-osd-tool/blob/master/LICENSE.md",
                        );
                        ui.end_row();
                    });

                    ui.add_space(10.0);

                    ui.hyperlink_to("Buy me a coffee", "https://www.buymeacoffee.com/avsaase");
                });
        }
    }
}
