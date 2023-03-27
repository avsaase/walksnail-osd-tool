use egui::{vec2, Checkbox, Color32, Image, Slider, Ui};

use super::{
    osd_preview::{calculate_horizontal_offset, calculate_vertical_offset},
    utils::separator_with_space,
    WalksnailOsdTool,
};

impl WalksnailOsdTool {
    pub fn render_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.osd_options(ui, ctx);

                separator_with_space(ui, 10.0);

                self.osd_preview(ui, ctx);

                separator_with_space(ui, 10.0);

                self.rendering_options(ui);
            });
        });
    }

    fn osd_options(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.heading("OSD Options");
        ui.style_mut().spacing.slider_width = self.ui_dimensions.osd_position_sliders_length;
        egui::Grid::new("position_sliders")
            .spacing(vec2(15.0, 10.0))
            .show(ui, |ui| {
                ui.label("Horizontal offset");
                ui.horizontal(|ui| {
                    if ui
                        .add(Slider::new(&mut self.osd_options.horizontal_offset, -200..=700).text("Pixels"))
                        .changed()
                    {
                        self.update_osd_preview(ctx);
                    }

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
                            self.update_osd_preview(ctx);
                        }
                    }

                    if ui.button("Reset").clicked() {
                        self.osd_options.horizontal_offset = 0;
                        self.update_osd_preview(ctx);
                    }
                });
                ui.end_row();

                //

                ui.label("Vertical offset");
                ui.horizontal(|ui| {
                    if ui
                        .add(Slider::new(&mut self.osd_options.vertical_offset, -200..=700).text("Pixels"))
                        .changed()
                    {
                        self.update_osd_preview(ctx);
                    }

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
                            self.update_osd_preview(ctx);
                        }
                    }

                    if ui.button("Reset").clicked() {
                        self.osd_options.vertical_offset = 0;
                        self.update_osd_preview(ctx);
                    }
                });
                ui.end_row();

                //

                ui.label("Show SRT data");
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(self.srt_file.is_some(), |ui| {
                        let options = &mut self.osd_options.srt_options;
                        let has_distance = self.srt_file.as_ref().map(|s| s.has_distance).unwrap_or(false);
                        if [
                            ui.checkbox(&mut options.show_time, "Time "),
                            ui.checkbox(&mut options.show_sbat, "SBat "),
                            ui.checkbox(&mut options.show_gbat, "GBat "),
                            ui.checkbox(&mut options.show_signal, "Signal "),
                            ui.checkbox(&mut options.show_latency, "Latency "),
                            ui.checkbox(&mut options.show_bitrate, "Bitrate "),
                            ui.add_enabled(has_distance, Checkbox::new(&mut options.show_distance, "Distance")),
                        ]
                        .iter()
                        .any(|r| r.changed())
                        {
                            self.update_osd_preview(ctx);
                        }
                    });
                });
                ui.end_row();
            });
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

        egui::Grid::new("render_options")
            .spacing(vec2(15.0, 10.0))
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
            });
    }
}
