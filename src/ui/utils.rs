use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use egui::{Sense, Ui};

use super::WalksnailOsdTool;

impl WalksnailOsdTool {
    pub fn all_files_loaded(&self) -> bool {
        match (&self.video_file, &self.video_info, &self.osd_file, &self.font_file) {
            (Some(_), Some(_), Some(_), Some(_)) => true,
            (_, _, _, _) => false,
        }
    }
}

pub fn find_file_with_extention<'a>(files: &'a [PathBuf], extention: &'a str) -> Option<&'a PathBuf> {
    files.iter().find_map(|f| {
        f.extension().and_then(|e| {
            if e.to_string_lossy() == extention {
                Some(f)
            } else {
                None
            }
        })
    })
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

pub fn get_output_video_path(input_video_path: &Path) -> PathBuf {
    let input_video_file_name = input_video_path.file_stem().unwrap().to_string_lossy();
    let output_video_file_name = format!("{}_with_osd.mp4", input_video_file_name);
    let mut output_video_path = input_video_path.parent().unwrap().to_path_buf();
    output_video_path.push(output_video_file_name);
    output_video_path
}
