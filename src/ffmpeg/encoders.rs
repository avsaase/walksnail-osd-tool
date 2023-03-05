use rayon::prelude::*;

use std::{fmt::Display, process::Command, vec};

#[derive(Debug, PartialEq, Clone)]
pub enum Codec {
    H264,
    H265,
}

impl Display for Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Codec::H264 => write!(f, "h264"),
            Codec::H265 => write!(f, "h265"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Encoder {
    pub name: String,
    pub codec: Codec,
    pub hardware: bool,
}

impl Encoder {
    fn new(name: &str, codec: Codec, hardware: bool) -> Self {
        Self {
            name: name.to_string(),
            codec,
            hardware,
        }
    }

    #[tracing::instrument(ret)]
    pub fn get_available_encoders() -> Vec<Self> {
        let all_encoders = vec![
            Encoder::new("libx264", Codec::H264, false),
            Encoder::new("libx265", Codec::H265, false),
            #[cfg(target_os = "windows")]
            Encoder::new("h264_amf", Codec::H264, true),
            #[cfg(target_os = "windows")]
            Encoder::new("h264_mf", Codec::H264, true),
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Encoder::new("h264_nvenc", Codec::H264, true),
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Encoder::new("h264_qsv", Codec::H264, true),
            #[cfg(target_os = "linux")]
            Encoder::new("h264_vaapi", Codec::H264, true),
            #[cfg(target_os = "linux")]
            Encoder::new("h264_v4l2m2m", Codec::H264, true),
            #[cfg(target_os = "macos")]
            Encoder::new("h264_videotoolbox", Codec::H264, true),
            #[cfg(target_os = "windows")]
            Encoder::new("hevc_amf", Codec::H265, true),
            #[cfg(target_os = "windows")]
            Encoder::new("hevc_mf", Codec::H265, true),
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Encoder::new("hevc_nvenc", Codec::H265, true),
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Encoder::new("hevc_qsv", Codec::H265, true),
            #[cfg(target_os = "linux")]
            Encoder::new("hevc_vaapi", Codec::H265, true),
            #[cfg(target_os = "linux")]
            Encoder::new("hevc_v4l2m2m", Codec::H265, true),
        ];

        all_encoders
            .into_par_iter()
            .filter(Self::ffmpeg_encoder_available)
            .collect::<Vec<_>>()
    }

    fn ffmpeg_encoder_available(encoder: &Encoder) -> bool {
        let mut command = Command::new("ffmpeg");

        command
            .args([
                "-hide_banner",
                "-f",
                "lavfi",
                "-i",
                "nullsrc",
                "-c:v",
                &encoder.name,
                "-frames:v",
                "1",
                "-f",
                "null",
                "-",
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        #[cfg(target_os = "windows")]
        std::os::windows::process::CommandExt::creation_flags(&mut command, crate::CREATE_NO_WINDOW);

        let status = command
            .status()
            .expect("Failed to execute ffmpeg command to check encoder compatibility");
        status.code().expect("Failed to get status code from ffmpeg command") == 0
    }
}

impl Display for Encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} — {} — {}",
            self.name,
            self.codec,
            if self.hardware { "hardware" } else { "software" }
        )
    }
}
