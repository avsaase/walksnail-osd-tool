use std::time::Instant;

use egui::{vec2, CentralPanel, Checkbox, Color32, Grid, Image, ScrollArea, Slider, Ui};

use super::{
    osd_preview::{calculate_horizontal_offset, calculate_vertical_offset},
    utils::separator_with_space,
    WalksnailOsdTool,
};

impl WalksnailOsdTool {
    pub fn render_central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.style_mut().spacing.slider_width = self.ui_dimensions.osd_position_sliders_length;

                self.osd_options(ui, ctx);

                separator_with_space(ui, 10.0);

                self.srt_options(ui, ctx);

                separator_with_space(ui, 10.0);

                self.osd_preview(ui, ctx);

                separator_with_space(ui, 10.0);

                self.rendering_options(ui);
            });
        });
    }

    fn osd_options(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.heading("OSD Options");
        let mut changed = false;

        Grid::new("osd_options")
            // .spacing(vec2(15.0, 10.0))
            .min_col_width(140.0)
            .show(ui, |ui| {
                ui.label("Horizontal offset");
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(Slider::new(&mut self.osd_options.horizontal_offset, -200..=700).text("Pixels"))
                        .changed();

                    if ui.button("Center").clicked() {
                        if let (Some(video_info), Some(osd_file), Some(font_file)) =
                            (&self.video_info, &self.osd_file, &self.font_file)
                        {
                            self.osd_options.horizontal_offset = calculate_horizontal_offset(
                                video_info.width,
                                osd_file
                                    .frames
                                    .get(self.osd_preview.preview_frame as usize - 1)
                                    .unwrap(),
                                &font_file.character_size,
                            );
                            changed |= true;
                        }
                    }

                    if ui.button("Reset").clicked() {
                        self.osd_options.horizontal_offset = 0;
                        changed |= true;
                    }
                });
                ui.end_row();

                //

                ui.label("Vertical offset");
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(Slider::new(&mut self.osd_options.vertical_offset, -200..=700).text("Pixels"))
                        .changed();

                    if ui.button("Center").clicked() {
                        if let (Some(video_info), Some(osd_file), Some(font_file)) =
                            (&self.video_info, &self.osd_file, &self.font_file)
                        {
                            self.osd_options.vertical_offset = calculate_vertical_offset(
                                video_info.height,
                                osd_file
                                    .frames
                                    .get(self.osd_preview.preview_frame as usize - 1)
                                    .unwrap(),
                                &font_file.character_size,
                            );
                            changed |= true
                        }
                    }

                    if ui.button("Reset").clicked() {
                        self.osd_options.vertical_offset = 0;
                        changed |= true
                    }
                });
                ui.end_row();
            });
        if changed {
            self.update_osd_preview(ctx);
            self.config_changed = Some(Instant::now());
        }
    }

    fn srt_options(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.heading("SRT Options");
        let mut changed = false;

        Grid::new("srt_options")
            // .spacing(vec2(15.0, 10.0))
            .min_col_width(140.0)
            .show(ui, |ui| {
                ui.label("Elements");
                ui.horizontal(|ui| {
                    let options = &mut self.srt_options;
                    let has_distance = self.srt_file.as_ref().map(|s| s.has_distance).unwrap_or(true);
                    changed |= [
                        ui.checkbox(&mut options.show_time, "Time "),
                        ui.checkbox(&mut options.show_sbat, "SBat "),
                        ui.checkbox(&mut options.show_gbat, "GBat "),
                        ui.checkbox(&mut options.show_signal, "Signal "),
                        ui.checkbox(&mut options.show_latency, "Latency "),
                        ui.checkbox(&mut options.show_bitrate, "Bitrate "),
                        ui.add_enabled(has_distance, Checkbox::new(&mut options.show_distance, "Distance")),
                    ]
                    .iter()
                    .any(|r| r.changed());
                });
                ui.end_row();

                ui.label("Horizontal position");
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(Slider::new(&mut self.srt_options.position.x, 0.0..=1.0).fixed_decimals(3))
                        .changed();

                    if ui.button("Reset").clicked() {
                        self.srt_options.position.x = 0.015;
                        changed |= true;
                    }
                });
                ui.end_row();

                ui.label("Vertical position");
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(Slider::new(&mut self.srt_options.position.y, 0.0..=1.0).fixed_decimals(3))
                        .changed();

                    if ui.button("Reset").clicked() {
                        self.srt_options.position.y = 0.95;
                        changed |= true;
                    }
                });
                ui.end_row();

                ui.label("Size");
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(Slider::new(&mut self.srt_options.scale, 0.0..=100.0).fixed_decimals(2))
                        .changed();

                    if ui.button("Reset").clicked() {
                        self.srt_options.scale = 35.0;
                        changed |= true;
                    }
                });
                ui.end_row();
            });

        if changed {
            self.update_osd_preview(ctx);
            self.config_changed = Some(Instant::now());
        }
    }

    fn osd_preview(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.heading("Preview");
        if let (Some(handle), Some(video_info)) = (&self.osd_preview.texture_handle, &self.video_info) {
            let preview_width = ui.available_width();
            let aspect_ratio = video_info.width as f32 / video_info.height as f32;
            let preview_height = preview_width / aspect_ratio;
            let image = Image::new(handle, vec2(preview_width, preview_height));
            ui.add(image.bg_fill(Color32::LIGHT_GRAY));

            ui.horizontal(|ui| {
                ui.label("Preview frame");
                let preview_frame_slider = ui.add(
                    Slider::new(
                        &mut self.osd_preview.preview_frame,
                        1..=self.osd_file.as_ref().map(|f| f.frame_count).unwrap_or(1),
                    )
                    .smart_aim(false),
                );
                if preview_frame_slider.changed() {
                    self.update_osd_preview(ctx);
                }
            });
        }
    }

    fn rendering_options(&mut self, ui: &mut Ui) {
        ui.heading("Rendering Options");
        let selectable_encoders = self
            .encoders
            .iter()
            .filter(|e| self.show_undetected_encoders || e.detected)
            .collect::<Vec<_>>();

        Grid::new("render_options")
            // .spacing(vec2(15.0, 10.0))
            .min_col_width(140.0)
            .show(ui, |ui| {
                ui.label("Encoder");
                ui.horizontal(|ui| {
                    let selection = egui::ComboBox::from_id_source("encoder").width(350.0).show_index(
                        ui,
                        &mut self.selected_encoder_idx,
                        selectable_encoders.len(),
                        |i| {
                            selectable_encoders
                                .get(i)
                                .map(|e| e.to_string())
                                .unwrap_or("None".to_string())
                        },
                    );
                    if selection.changed() {
                        // This is a little hacky but it's nice to have a single struct that keeps track of all render settings
                        self.render_settings.encoder =
                            (*selectable_encoders.get(self.selected_encoder_idx).unwrap()).clone()
                    }
                    if ui
                        .checkbox(&mut self.show_undetected_encoders, "Show undeteced encoders")
                        .changed()
                    {
                        self.selected_encoder_idx = 0;
                        tracing::info!("Toggled show undetected encoders: {}", self.show_undetected_encoders);
                    };
                });
                ui.end_row();

                ui.label("Encoding bitrate");
                ui.add(Slider::new(&mut self.render_settings.bitrate_mbps, 0..=100).text("Mbps"));
                ui.end_row();

                ui.label("Upscale to 1440p");
                ui.add(Checkbox::without_text(&mut self.render_settings.upscale));
                ui.end_row();
            });
    }
}
