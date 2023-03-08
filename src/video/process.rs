use std::{
    io::Write,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use ffmpeg_sidecar::{command::FfmpegCommand, event::FfmpegEvent};

use crate::{ffmpeg::VideoInfo, font, osd};

use super::{error::FfmpegError, frame_overlay_iter::FrameOverlayIter, render_progress::StopRenderMessage, Settings};

#[tracing::instrument(skip(osd_frames, font_file))]
pub fn process_video(
    ffmpeg_path: &PathBuf,
    input_video: &str,
    output_video: &str,
    osd_frames: Vec<osd::Frame>,
    font_file: font::FontFile,
    video_info: &VideoInfo,
    render_settings: &Settings,
    horizontal_offset: i32,
    vertical_offset: i32,
) -> Result<(Receiver<FfmpegEvent>, Sender<StopRenderMessage>), FfmpegError> {
    // Spawn the decoder ffmpeg instance
    let mut decoder = FfmpegCommand::new_with_exe(ffmpeg_path);
    #[cfg(target_os = "windows")]
    std::os::windows::process::CommandExt::creation_flags(decoder.as_inner_mut(), crate::CREATE_NO_WINDOW);
    let mut decoder = decoder.input(input_video).rawvideo().spawn()?;
    tracing::info!("Spawned ffmpeg decoder instance");

    // Spawn the encoder ffmpeg instance
    let mut encoder = FfmpegCommand::new_with_exe(ffmpeg_path);
    #[cfg(target_os = "windows")]
    std::os::windows::process::CommandExt::creation_flags(encoder.as_inner_mut(), crate::CREATE_NO_WINDOW);
    let mut encoder = encoder
        .args(["-f", "rawvideo"])
        .args(["-pix_fmt", "rgb24"])
        .size(video_info.width, video_info.height)
        .rate(video_info.frame_rate)
        .input("-")
        .codec_video(&render_settings.encoder.name)
        .args(["-b:v", &format!("{}M", render_settings.bitrate_mbps)])
        .args(["-y", output_video])
        .spawn()?;
    tracing::info!("Spawned ffmpeg encoder instance");

    // Create a channel to report progress back to the main (GUI) thread
    let (progress_tx, progress_rx) = mpsc::channel();
    let (stop_render_tx, stop_render_rx) = mpsc::channel();

    // Iterator over decoded video and OSD frames
    let frame_overlay_iter = FrameOverlayIter::new(
        decoder.iter().expect("Failed to create `FfmpegIterator` for decoder"),
        decoder,
        osd_frames,
        font_file,
        horizontal_offset,
        vertical_offset,
        progress_tx,
        stop_render_rx,
    );

    // On another thread run the iterator to completion and feed the output to the encoder's stdin
    let mut encoder_stdin = encoder.take_stdin().expect("Failed to get `stdin` for encoder");
    thread::spawn(move || {
        frame_overlay_iter.for_each(|f| {
            encoder_stdin.write(&f.data).ok();
        });
    });

    // On yet another thread run the encoder to completion
    thread::spawn(move || {
        encoder
            .iter()
            .expect("Failed to create `FfmpegIterator` for encoder")
            .for_each(|_| {});
    });

    Ok((progress_rx, stop_render_tx))
}
