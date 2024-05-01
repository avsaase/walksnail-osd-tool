use std::time::Instant;

use backend::util::Coordinates;
use egui::{
    vec2, Button, CentralPanel, Checkbox, CollapsingHeader, Color32, CursorIcon, Grid, Image, Rect, RichText,
    ScrollArea, Sense, Slider, Stroke, Ui,
};

use crate::{
    osd_preview::{calculate_horizontal_offset, calculate_vertical_offset},
    util::{separator_with_space, tooltip_text},
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
        let mut changed = false;

        CollapsingHeader::new(RichText::new("OSD Options").heading())
            .default_open(true)
            .show_unindented(ui, |ui| {
                Grid::new("osd_options")
                    .min_col_width(self.ui_dimensions.options_column1_width)
                    .show(ui, |ui| {
                        ui.label("Horizontal position")
                            .on_hover_text(tooltip_text("Horizontal position of the flight controller OSD (pixels from the left edge of the video)."));
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(Slider::new(&mut self.osd_options.position.x, -200..=700).text("Pixels"))
                                .changed();

                            if ui.button("Center").clicked() {
                                if let (Some(video_info), Some(osd_file), Some(font_file)) =
                                    (&self.video_info, &self.osd_file, &self.font_file)
                                {
                                    self.osd_options.position.x = calculate_horizontal_offset(
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
                                self.osd_options.position.x = 0;
                                changed |= true;
                            }
                        });
                        ui.end_row();

                        //

                        ui.label("Vertical position")
                            .on_hover_text(tooltip_text("Vertical position of the flight controller OSD (pixels from the top of the video).").small());
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(Slider::new(&mut self.osd_options.position.y, -200..=700).text("Pixels"))
                                .changed();

                            if ui.button("Center").clicked() {
                                if let (Some(video_info), Some(osd_file), Some(font_file)) =
                                    (&self.video_info, &self.osd_file, &self.font_file)
                                {
                                    self.osd_options.position.y = calculate_vertical_offset(
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
                                self.osd_options.position.y = 0;
                                changed |= true
                            }
                        });
                        ui.end_row();

                        ui.label("Mask")
                            .on_hover_text(tooltip_text("Click edit to select OSD elements on the preview that should not be rendered on the video. This can be useful to hide GPS coordinates, etc."));
                        ui.horizontal(|ui| {
                            let txt = if !self.osd_preview.mask_edit_mode_enabled || !self.all_files_loaded() {"Edit"} else {"Save"};
                            if ui.add_enabled(self.all_files_loaded(), Button::new(txt))
                                .on_disabled_hover_text(tooltip_text("First load the input files")).clicked() {
                                self.osd_preview.mask_edit_mode_enabled = !self.osd_preview.mask_edit_mode_enabled;
                            }
                            if ui.button("Reset").clicked() {
                                self.osd_options.reset_mask();
                                self.config_changed = Instant::now().into();
                                self.update_osd_preview(ctx);
                            }
                            let masked_positions = self.osd_options.masked_grid_positions.len();
                            ui.label(&format!("{} positions masked", masked_positions));
                        });
                        ui.end_row();

                        ui.label("Adjust playback speed")
                            .on_hover_text(tooltip_text("Attempt to correct for wrong OSD timestamps in <=32.37.10 firmwares that causes video and OSD to get out of sync."));
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(Checkbox::without_text(&mut self.osd_options.adjust_playback_speed))
                                .changed()
                        });
                    });
            });

        if changed {
            self.update_osd_preview(ctx);
            self.config_changed = Some(Instant::now());
        }
    }

    fn srt_options(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let mut changed = false;

        CollapsingHeader::new(RichText::new("SRT Options").heading())
            .default_open(true)
            .show_unindented(ui, |ui| {
                Grid::new("srt_options")
                    .min_col_width(self.ui_dimensions.options_column1_width)
                    .show(ui, |ui| {
                        ui.label("Horizontal position").on_hover_text(tooltip_text(
                            "Horizontal position of the SRT data (% of the total video width from the left edge).",
                        ));
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(Slider::new(&mut self.srt_options.position.x, 0.0..=100.0).fixed_decimals(1))
                                .changed();

                            if ui.button("Reset").clicked() {
                                self.srt_options.position.x = 1.5;
                                changed |= true;
                            }
                        });
                        ui.end_row();

                        ui.label("Vertical position").on_hover_text(tooltip_text(
                            "Vertical position of the SR data (% of video height from the top edge).",
                        ));
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(Slider::new(&mut self.srt_options.position.y, 0.0..=100.0).fixed_decimals(1))
                                .changed();

                            if ui.button("Reset").clicked() {
                                self.srt_options.position.y = 95.0;
                                changed |= true;
                            }
                        });
                        ui.end_row();

                        ui.label("Size")
                            .on_hover_text(tooltip_text("Font size of the SRT data."));
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(Slider::new(&mut self.srt_options.scale, 10.0..=60.0).fixed_decimals(1))
                                .changed();

                            if ui.button("Reset").clicked() {
                                self.srt_options.scale = 35.0;
                                changed |= true;
                            }
                        });
                        ui.end_row();

                        ui.label("SRT data").on_hover_text(tooltip_text(
                            "Select data from the SRT file to be rendered on the video.",
                        ));
                        let options = &mut self.srt_options;
                        let has_distance = self.srt_file.as_ref().map(|s| s.has_distance).unwrap_or(true);
                        Grid::new("srt_selection").show(ui, |ui| {
                            changed |= ui.checkbox(&mut options.show_time, "Time").changed();
                            changed |= ui.checkbox(&mut options.show_sbat, "SBat").changed();
                            changed |= ui.checkbox(&mut options.show_gbat, "GBat").changed();
                            changed |= ui.checkbox(&mut options.show_signal, "Signal").changed();
                            ui.end_row();

                            changed |= ui.checkbox(&mut options.show_latency, "Latency").changed();
                            changed |= ui.checkbox(&mut options.show_bitrate, "Bitrate").changed();
                            changed |= ui
                                .add_enabled(has_distance, Checkbox::new(&mut options.show_distance, "Distance"))
                                .changed();
                            ui.end_row();
                        });
                        ui.end_row();
                    });
            });

        if changed {
            self.update_osd_preview(ctx);
            self.config_changed = Some(Instant::now());
        }
    }

    fn osd_preview(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        CollapsingHeader::new(RichText::new("Preview").heading())
            .default_open(true)
            .show_unindented(ui, |ui| {
                if let (Some(handle), Some(video_info)) = (&self.osd_preview.texture_handle, &self.video_info) {
                    let preview_width = ui.available_width();
                    let aspect_ratio = video_info.width as f32 / video_info.height as f32;
                    let preview_height = preview_width / aspect_ratio;
                    let image = Image::new(handle, vec2(preview_width, preview_height));
                    let rect = ui.add(image.bg_fill(Color32::LIGHT_GRAY)).rect;

                    if self.osd_preview.mask_edit_mode_enabled {
                        self.draw_grid(ui, ctx, rect);
                    }

                    ui.horizontal(|ui| {
                        ui.label("Preview frame").on_hover_text(tooltip_text(
                            "The selected frame is also used for centering the OSD under OSD Options.",
                        ));
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
            });
    }

    fn draw_grid(&mut self, ui: &mut Ui, ctx: &egui::Context, image_rect: Rect) {
        let video_width = self.video_info.as_ref().unwrap().width as f32;
        let video_height = self.video_info.as_ref().unwrap().height as f32;

        let top_left = image_rect.left_top();
        let preview_width = image_rect.width();
        let preview_height = image_rect.height();

        let grid_width = preview_width * 0.99375;
        let grid_height = preview_height;
        let cell_width = grid_width / 53.0;
        let cell_height = grid_height / 20.0;

        let painter = ui.painter_at(image_rect);

        let horizontal_offset = self.osd_options.position.x as f32 / video_width * preview_width;
        let vertical_offset = self.osd_options.position.y as f32 / video_height * preview_height;

        let response = ui
            .allocate_rect(image_rect, Sense::click())
            .on_hover_cursor(CursorIcon::Crosshair);

        for i in 0..53 {
            for j in 0..20 {
                let rect = Rect::from_min_size(
                    top_left
                        + vec2(i as f32 * cell_width, j as f32 * cell_height)
                        + vec2(horizontal_offset, vertical_offset),
                    vec2(cell_width, cell_height),
                );

                let grid_position = Coordinates::new(i, j);
                let masked = self.osd_options.get_mask(&grid_position);
                if masked {
                    painter.rect_filled(rect, 0.0, Color32::RED.gamma_multiply(0.5));
                }

                if let Some(hover_pos) = ctx.pointer_hover_pos() {
                    if rect.contains(hover_pos) {
                        painter.rect_filled(rect, 0.0, Color32::RED.gamma_multiply(0.2));
                    }
                }

                if response.clicked() {
                    if let Some(click_pos) = ctx.pointer_interact_pos() {
                        if rect.contains(click_pos) {
                            self.osd_options.toggle_mask(grid_position);
                            self.update_osd_preview(ctx);
                            self.config_changed = Instant::now().into();
                        }
                    }
                }
            }
        }

        let line_stroke = Stroke::new(1.0, Color32::GRAY.gamma_multiply(0.5));

        for i in 0..=53 {
            let x = top_left.x + i as f32 * cell_width + horizontal_offset;
            let y_min = image_rect.y_range().start() + vertical_offset;
            let y_max = image_rect.y_range().end() + vertical_offset;
            painter.vline(x, y_min..=y_max, line_stroke);
        }
        for i in 0..=20 {
            let x_min = image_rect.x_range().start() + horizontal_offset;
            let x_max = image_rect.x_range().end() + horizontal_offset;
            let y = top_left.y + i as f32 * cell_height + vertical_offset;
            painter.hline(x_min..=x_max, y, line_stroke);
        }
    }

    fn rendering_options(&mut self, ui: &mut Ui) {
        let mut changed = false;
        CollapsingHeader::new(RichText::new("Rendering Options").heading())
            .default_open(true)
            .show_unindented(ui, |ui| {
                let selectable_encoders = self
                    .encoders
                    .iter()
                    .filter(|e| self.render_settings.show_undetected_encoders || e.detected)
                    .collect::<Vec<_>>();

                Grid::new("render_options")
                    .min_col_width(self.ui_dimensions.options_column1_width)
                    .show(ui, |ui| {
                        ui.label("Encoder")
                            .on_hover_text(tooltip_text("Encoder used for rendering. In some cases not all available encoders are detected. Check the box to also show these."));
                        ui.horizontal(|ui| {
                            let selection = egui::ComboBox::from_id_source("encoder").width(350.0).show_index(
                                ui,
                                &mut self.render_settings.selected_encoder_idx,
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
                                    (*selectable_encoders.get(self.render_settings.selected_encoder_idx).unwrap()).clone();
                                changed |= true;
                            }

                            if ui
                                .add(Checkbox::without_text(&mut self.render_settings.show_undetected_encoders))
                                .changed() {
                                    self.render_settings.selected_encoder_idx = 0;
                                    self.render_settings.encoder =
                                        (*selectable_encoders.first().unwrap()).clone();
                                    changed |= true;
                            }
                        });
                        ui.end_row();

                        ui.label("Encoding bitrate").on_hover_text(tooltip_text("Target bitrate of the rendered video."));
                        changed |= ui.add(Slider::new(&mut self.render_settings.bitrate_mbps, 0..=160).text("Mbps")).changed();
                        ui.end_row();

                        ui.label("Upscale to 1440p").on_hover_text(tooltip_text("Upscale the output video to 1440p to get better quality after uplaoding to YouTube."));
                        changed |= ui.add(Checkbox::without_text(&mut self.render_settings.upscale)).changed();
                        ui.end_row();

                        ui.label("Rescale to 4:3 aspect ratio").on_hover_text(tooltip_text("Rescale the output video to 4:3 aspect ratio, useful when you have 4:3 camera and recording is done by VRX in \"4:3 Fullscreen\" mode."));
                        changed |= ui.add(Checkbox::without_text(&mut self.render_settings.rescale_to_4x3_aspect)).changed();
                        ui.end_row();

                        ui.label("Chroma key").on_hover_text(tooltip_text("Render the video with a chroma key instead of the input video so the OSD can be overlay in video editing software."));
                        ui.horizontal(|ui| {
                            changed |= ui.add(Checkbox::without_text(&mut self.render_settings.use_chroma_key)).changed();
                            changed |= ui.color_edit_button_rgb(&mut self.render_settings.chroma_key).changed();
                        });
                        ui.end_row();
                    });
            });

        if changed {
            self.config_changed = Some(Instant::now());
        }
    }
}
