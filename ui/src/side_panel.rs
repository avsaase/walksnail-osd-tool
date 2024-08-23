use backend::font::FontType;
use egui::{CollapsingHeader, Color32, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use super::WalksnailOsdTool;
use crate::util::{format_minutes_seconds, separator_with_space};

impl WalksnailOsdTool {
    pub fn render_sidepanel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .default_width(270.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(10.0);
                    self.video_info(ui);
                    separator_with_space(ui, 15.0);
                    self.osd_info(ui);
                    separator_with_space(ui, 15.0);
                    self.srt_info(ui);
                    separator_with_space(ui, 15.0);
                    self.font_info(ui);
                });
            });
    }

    fn video_info(&self, ui: &mut Ui) {
        let video_info = self.video_info.as_ref();
        let file_loaded = video_info.is_some();

        CollapsingHeader::new(RichText::new("Video file").heading())
            .icon(move |ui, opennes, response| circle_icon(ui, opennes, response, file_loaded))
            .default_open(true)
            .show(ui, |ui| {
                ui.push_id("video_info", |ui| {
                    TableBuilder::new(ui)
                        .column(Column::exact(self.ui_dimensions.file_info_column1_width))
                        .column(
                            Column::remainder()
                                .at_least(self.ui_dimensions.file_info_column2_width)
                                .clip(true),
                        )
                        .auto_shrink([false, true])
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
                                    if let Some(duration) = video_info.map(|i| i.duration) {
                                        ui.label(format_minutes_seconds(&duration));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });
                        });
                });
            });
    }

    fn osd_info(&self, ui: &mut Ui) {
        let osd_file = self.osd_file.as_ref();
        let file_loaded = osd_file.is_some();
        let video_file_loaded = self.video_info.is_some();

        CollapsingHeader::new(RichText::new("OSD file").heading())
            .icon(move |ui, opennes, response| circle_icon(ui, opennes, response, file_loaded))
            .default_open(true)
            .show(ui, |ui| {
                ui.push_id("osd_info", |ui| {
                    TableBuilder::new(ui)
                        .column(Column::exact(self.ui_dimensions.file_info_column1_width))
                        .column(
                            Column::remainder()
                                .at_least(self.ui_dimensions.file_info_column2_width)
                                .clip(true),
                        )
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

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("Duration:");
                                });
                                row.col(|ui| {
                                    ui.horizontal(|ui| {
                                        if let Some(duration) = osd_file.map(|i| i.duration) {
                                            if video_file_loaded {
                                                let time_difference =
                                                    self.video_info.as_ref().unwrap().duration.as_secs_f32()
                                                        - duration.as_secs_f32();
                                                let abs_diff_seconds = time_difference.abs();

                                                if abs_diff_seconds > 0.2 {
                                                    ui.label(
                                                        RichText::new("⚠")
                                                            .color(Color32::from_rgb(255, 200, 0))
                                                            .strong(),
                                                    )
                                                    .on_hover_text(format!(
                                                        "OSD and Video files duration mismatch: {0}",
                                                        abs_diff_seconds
                                                    ));
                                                }
                                            }

                                            ui.label(format_minutes_seconds(&duration));
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

    pub fn srt_info(&self, ui: &mut Ui) {
        let srt_file = self.srt_file.as_ref();
        let file_loaded = srt_file.is_some();

        CollapsingHeader::new(RichText::new("SRT file").heading())
            .icon(move |ui, opennes, response| circle_icon(ui, opennes, response, file_loaded))
            .default_open(true)
            .show(ui, |ui| {
                ui.push_id("srt_info", |ui| {
                    TableBuilder::new(ui)
                        .column(Column::exact(self.ui_dimensions.file_info_column1_width))
                        .column(
                            Column::remainder()
                                .at_least(self.ui_dimensions.file_info_column2_width)
                                .clip(true),
                        )
                        .body(|mut body| {
                            let row_height = self.ui_dimensions.file_info_row_height;
                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label("File name:");
                                });
                                row.col(|ui| {
                                    if let Some(srt_file) = srt_file {
                                        ui.label(
                                            srt_file
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
                                    ui.label("Duration:");
                                });
                                row.col(|ui| {
                                    if let Some(duration) = srt_file.map(|i| i.duration) {
                                        ui.label(format_minutes_seconds(&duration));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });
                        });
                });
            });
    }

    fn font_info(&self, ui: &mut Ui) {
        let font_file = self.font_file.as_ref();
        let file_loaded = font_file.is_some();

        CollapsingHeader::new(RichText::new("Font file").heading())
            .icon(move |ui, opennes, response| circle_icon(ui, opennes, response, file_loaded))
            .default_open(true)
            .show(ui, |ui| {
                ui.push_id("font_info", |ui| {
                    TableBuilder::new(ui)
                        .column(Column::exact(self.ui_dimensions.file_info_column1_width))
                        .column(
                            Column::remainder()
                                .at_least(self.ui_dimensions.file_info_column2_width)
                                .clip(true),
                        )
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
                                        ui.label(format!(
                                            "{}{}",
                                            font_file.character_count,
                                            if font_file.font_type == FontType::FourColor {
                                                " (4 colors)"
                                            } else {
                                                ""
                                            }
                                        ));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });
                        });
                });
            });
    }
}

fn circle_icon(ui: &egui::Ui, _openness: f32, response: &egui::Response, loaded: bool) {
    let stroke = ui.style().interact(response).fg_stroke;
    let radius = 3.0;
    if loaded {
        ui.painter().circle_filled(response.rect.center(), radius, stroke.color);
    } else {
        ui.painter().circle_stroke(response.rect.center(), radius - 0.5, stroke);
    }
}
