use std::{path::PathBuf, time::Duration};

use egui::{Sense, Ui};

use super::WalksnailOsdTool;

impl WalksnailOsdTool {
    pub fn all_files_loaded(&self) -> bool {
        match (&self.video_file, &self.osd_file, &self.font_file) {
            (Some(_), Some(_), Some(_)) => true,
            (_, _, _) => false,
        }
    }
}

pub fn find_file_with_extention<'a>(files: &'a [PathBuf], extention: &'a str) -> Option<&'a PathBuf> {
    files
        .iter()
        .find(|f| f.extension().unwrap().to_str().unwrap() == extention)
}

pub fn separator_with_space(ui: &mut Ui, space: f32) {
    ui.add_space(space);
    ui.separator();
    ui.add_space(space);
}

pub fn format_minutes_seconds(mabe_duration: &Option<Duration>) -> String {
    match mabe_duration {
        Some(duration) => {
            let minutes = duration.as_secs() / 60;
            let seconds = duration.as_secs() % 60;
            format!("{}:{:0>2}", minutes, seconds)
        }
        None => "––:––".into(),
    }
}

pub fn clickable_if(condition: bool) -> Sense {
    if condition {
        Sense::click()
    } else {
        Sense::focusable_noninteractive()
    }
}
