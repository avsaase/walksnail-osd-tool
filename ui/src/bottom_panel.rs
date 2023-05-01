use backend::ffmpeg::{start_video_render, ToFfmpegMessage};
use egui::{vec2, Align, Button, Color32, Layout, ProgressBar, RichText, Ui};

use crate::{render_status::Status, util::get_output_video_path};

use super::{util::format_minutes_seconds, WalksnailOsdTool};

impl WalksnailOsdTool {
    pub fn render_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                self.start_stop_render_button(ui);
                self.render_progress(ui);
            });
            ui.add_space(2.0);
        });
    }

    fn start_stop_render_button(&mut self, ui: &mut Ui) {
        let button_size = vec2(110.0, 40.0);
        if self.render_status.is_not_in_progress() {
            if ui
                .add_enabled(
                    self.all_files_loaded(),
                    Button::new("Start render").min_size(button_size),
                )
                .on_disabled_hover_text("First load video, OSD, SRT and font files")
                .clicked()
            {
                tracing::info!("Start render button clicked");
                self.render_status.start_render();
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
        } else {
            if ui.add(Button::new("Stop render").min_size(button_size)).clicked() {
                tracing::info!("Stop render button clicked");
                if let Some(sender) = &self.to_ffmpeg_sender {
                    sender
                        .send(ToFfmpegMessage::AbortRender)
                        .map_err(|_| tracing::warn!("Failed to send abort render message"))
                        .unwrap();
                    self.render_status.stop_render();
                }
            }
        }
    }

    fn render_progress(&mut self, ui: &mut Ui) {
        match &self.render_status.status {
            Status::Idle => {}
            Status::InProgress {
                time_remaining,
                fps,
                speed,
                progress_pct,
            } => {
                ui.vertical(|ui| {
                    ui.add(ProgressBar::new(*progress_pct).show_percentage());
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.add_space(3.0);
                        let time_remaining_string = if let Some(time_remaining) = time_remaining {
                            format_minutes_seconds(time_remaining)
                        } else {
                            "––:––".into()
                        };
                        ui.label(format!(
                            "Time remaining: {}, fps: {:.1}, speed: {:.3}x",
                            time_remaining_string, fps, speed
                        ));
                    });
                });
            }
            Status::Completed => {
                ui.vertical(|ui| {
                    ui.add(ProgressBar::new(1.0).text("Done"));
                });
            }
            Status::Cancelled { progress_pct } => {
                ui.vertical(|ui| {
                    ui.add(ProgressBar::new(*progress_pct).text("Cancelled"));
                });
            }
            Status::Error { progress_pct, error } => {
                ui.vertical(|ui| {
                    ui.add(ProgressBar::new(*progress_pct));
                    ui.label(RichText::new(error.clone()).color(Color32::RED));
                });
            }
        }
    }
}
