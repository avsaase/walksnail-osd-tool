use std::{
    io::{self, Write},
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use ffmpeg_sidecar::{
    child::FfmpegChild,
    command::FfmpegCommand,
    event::{FfmpegEvent, LogLevel},
};

use crate::{font, osd, overlay::FrameOverlayIter};

use super::{encoder_settings::EncoderSettings, Encoder, FfmpegMessage, StopRenderMessage, VideoInfo};

#[tracing::instrument(skip(osd_frames, font_file), err)]
pub fn render_video(
    ffmpeg_path: &PathBuf,
    input_video: &PathBuf,
    output_video: &PathBuf,
    osd_frames: Vec<osd::Frame>,
    font_file: font::FontFile,
    video_info: &VideoInfo,
    render_settings: &EncoderSettings,
    horizontal_offset: i32,
    vertical_offset: i32,
) -> Result<(Receiver<FfmpegMessage>, Sender<StopRenderMessage>), io::Error> {
    let mut decoder_process = spawn_decoder(ffmpeg_path, input_video)?;

    let mut encoder_process = spawn_encoder(
        ffmpeg_path,
        video_info.width,
        video_info.height,
        video_info.frame_rate,
        render_settings.bitrate_mbps,
        &render_settings.encoder,
        output_video,
    )?;

    // Channel to report ffmpeg status back to the main (GUI) thread
    let (ffmpeg_sender, ffmpeg_receiver) = mpsc::channel();

    // Channel to stop the render process
    let (stop_render_sender, stop_render_receiver) = mpsc::channel();

    // Iterator over decoded video and OSD frames
    let frame_overlay_iter = FrameOverlayIter::new(
        decoder_process
            .iter()
            .expect("Failed to create `FfmpegIterator` for decoder"),
        decoder_process,
        osd_frames,
        font_file,
        horizontal_offset,
        vertical_offset,
        ffmpeg_sender.clone(),
        stop_render_receiver,
    );

    // On another thread run the iterator to completion and feed the output to the encoder's stdin
    let mut encoder_stdin = encoder_process.take_stdin().expect("Failed to get `stdin` for encoder");
    thread::spawn(move || {
        tracing::info_span!("Decoder thread").in_scope(|| {
            frame_overlay_iter.for_each(|f| {
                encoder_stdin.write(&f.data).ok();
            });
        });
    });

    // On yet another thread run the encoder to completion
    thread::spawn(move || {
        tracing::info_span!("Encoder thread").in_scope(|| {
            encoder_process.iter().map_or_else(
                |e| tracing::error!("Failed to create encoder iterator with error {}", e),
                |v| v.for_each(|event| handle_encoder_events(event, &ffmpeg_sender)),
            );
        });
    });

    Ok((ffmpeg_receiver, stop_render_sender))
}

#[tracing::instrument(skip(ffmpeg_path))]
pub fn spawn_decoder(ffmpeg_path: &PathBuf, input_video: &PathBuf) -> Result<FfmpegChild, io::Error> {
    let decoder = FfmpegCommand::new_with_path(ffmpeg_path)
        .create_no_window()
        .input(input_video.to_str().unwrap())
        .args(["-f", "rawvideo", "-pix_fmt", "rgba", "-"])
        .spawn()?;
    Ok(decoder)
}

#[tracing::instrument(skip(ffmpeg_path))]
pub fn spawn_encoder(
    ffmpeg_path: &PathBuf,
    width: u32,
    height: u32,
    frame_rate: f32,
    bitrate_mbps: u32,
    video_encoder: &Encoder,
    output_video: &PathBuf,
) -> Result<FfmpegChild, io::Error> {
    let encoder = FfmpegCommand::new_with_path(ffmpeg_path)
        .create_no_window()
        .format("rawvideo")
        .pix_fmt("rgba")
        .size(width, height)
        .rate(frame_rate)
        .input("-")
        .pix_fmt("yuv420p")
        .codec_video(&video_encoder.name)
        .args(["-b:v", &format!("{}M", bitrate_mbps)])
        .overwrite()
        .output(output_video.to_str().unwrap())
        .spawn()?;
    Ok(encoder)
}

fn handle_encoder_events(ffmpeg_event: FfmpegEvent, ffmpeg_sender: &Sender<FfmpegMessage>) {
    match ffmpeg_event {
        FfmpegEvent::Log(level, e) => {
            if level == LogLevel::Fatal
            // there are some fatal errors that ffmpeg considers normal errors
                || e.contains("Error initializing output")
                || e.contains("[error] Cannot load")
            {
                tracing::error!("ffmpeg fatal error: {}", &e);
                ffmpeg_sender.send(FfmpegMessage::EncoderFatalError(e)).unwrap();
            }
        }
        _ => {}
    }
}

pub fn handle_decoder_events(ffmpeg_event: FfmpegEvent, ffmpeg_sender: &Sender<FfmpegMessage>) {
    match ffmpeg_event {
        FfmpegEvent::Progress(p) => {
            ffmpeg_sender.send(FfmpegMessage::Progress(p)).unwrap();
        }
        FfmpegEvent::Done | FfmpegEvent::LogEOF => {
            ffmpeg_sender.send(FfmpegMessage::DecoderFinished).unwrap();
        }
        FfmpegEvent::Log(LogLevel::Fatal, e) => {
            tracing::error!("ffmpeg fatal error: {}", &e);
            ffmpeg_sender.send(FfmpegMessage::DecoderFatalError(e)).unwrap();
        }
        FfmpegEvent::Log(LogLevel::Warning | LogLevel::Error, e) => {
            tracing::warn!("ffmpeg log: {}", &e);
        }
        _ => {}
    }
}
