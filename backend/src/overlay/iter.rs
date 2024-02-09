use std::{iter::Peekable, vec::IntoIter};

use crossbeam_channel::{Receiver, Sender};
use ffmpeg_sidecar::{
    child::FfmpegChild,
    event::{FfmpegEvent, OutputVideoFrame},
    iter::FfmpegIterator,
};
use image::{Rgba, RgbaImage};

use super::{overlay_osd, overlay_srt_data};
use crate::{
    ffmpeg::{handle_decoder_events, FromFfmpegMessage, ToFfmpegMessage},
    font,
    osd::{self, OsdOptions},
    srt::{self, SrtOptions},
};

pub struct FrameOverlayIter<'a> {
    decoder_iter: FfmpegIterator,
    decoder_process: FfmpegChild,
    osd_frames_iter: Peekable<IntoIter<osd::Frame>>,
    srt_frames_iter: Peekable<IntoIter<srt::SrtFrame>>,
    font_file: font::FontFile,
    osd_options: OsdOptions,
    srt_options: SrtOptions,
    srt_font: rusttype::Font<'a>,
    current_osd_frame: osd::Frame,
    current_srt_frame: srt::SrtFrame,
    ffmpeg_sender: Sender<FromFfmpegMessage>,
    ffmpeg_receiver: Receiver<ToFfmpegMessage>,
    chroma_key: Option<Rgba<u8>>,
}

impl<'a> FrameOverlayIter<'a> {
    #[tracing::instrument(skip(decoder_iter, decoder_process, osd_frames, font_file), level = "debug")]
    pub fn new(
        decoder_iter: FfmpegIterator,
        decoder_process: FfmpegChild,
        osd_frames: Vec<osd::Frame>,
        srt_frames: Vec<srt::SrtFrame>,
        font_file: font::FontFile,
        srt_font: rusttype::Font<'a>,
        osd_options: &OsdOptions,
        srt_options: &SrtOptions,
        ffmpeg_sender: Sender<FromFfmpegMessage>,
        ffmpeg_receiver: Receiver<ToFfmpegMessage>,
        chroma_key: Option<[f32; 3]>,
    ) -> Self {
        let mut osd_frames_iter = osd_frames.into_iter();
        let mut srt_frames_iter = srt_frames.into_iter();
        let first_osd_frame = osd_frames_iter.next().unwrap();
        let first_srt_frame = srt_frames_iter.next().unwrap();
        let chroma_key =
            chroma_key.map(|c| Rgba([(c[0] * 255.0) as u8, (c[1] * 255.0) as u8, (c[2] * 255.0) as u8, 255]));
        Self {
            decoder_iter,
            decoder_process,
            osd_frames_iter: osd_frames_iter.peekable(),
            srt_frames_iter: srt_frames_iter.peekable(),
            font_file,
            osd_options: osd_options.clone(),
            srt_options: srt_options.clone(),
            srt_font: srt_font.clone(),
            current_osd_frame: first_osd_frame,
            current_srt_frame: first_srt_frame,
            ffmpeg_sender,
            ffmpeg_receiver,
            chroma_key,
        }
    }
}

impl Iterator for FrameOverlayIter<'_> {
    type Item = OutputVideoFrame;

    fn next(&mut self) -> Option<Self::Item> {
        //  On every iteration check if the render should be stopped
        while let Ok(ToFfmpegMessage::AbortRender) = self.ffmpeg_receiver.try_recv() {
            self.decoder_process.quit().unwrap();
        }

        self.decoder_iter.find_map(|e| match e {
            FfmpegEvent::OutputFrame(mut video_frame) => {
                // For every video frame check if frame time is later than the next OSD frame time.
                // If so advance the iterator over the OSD frames so we use the correct OSD frame
                // for this video frame
                if let Some(next_osd_frame) = self.osd_frames_iter.peek() {
                    let next_osd_frame_secs = next_osd_frame.time_millis as f32 / 1000.0;
                    if video_frame.timestamp > next_osd_frame_secs * self.osd_options.osd_playback_speed_factor {
                        self.current_osd_frame = self.osd_frames_iter.next().unwrap();
                    }
                }

                if let Some(next_srt_frame) = self.srt_frames_iter.peek() {
                    let next_srt_start_time_secs = next_srt_frame.start_time_secs;
                    if video_frame.timestamp > next_srt_start_time_secs {
                        self.current_srt_frame = self.srt_frames_iter.next().unwrap();
                    }
                }

                let mut frame_image = if let Some(chroma_key) = self.chroma_key {
                    RgbaImage::from_pixel(video_frame.width, video_frame.height, chroma_key)
                } else {
                    RgbaImage::from_raw(video_frame.width, video_frame.height, video_frame.data).unwrap()
                };

                overlay_osd(
                    &mut frame_image,
                    &self.current_osd_frame,
                    &self.font_file,
                    &self.osd_options,
                );

                overlay_srt_data(
                    &mut frame_image,
                    &self.current_srt_frame.data,
                    &self.srt_font,
                    &self.srt_options,
                );

                video_frame.data = frame_image.as_raw().to_vec();
                Some(video_frame)
            }
            other_event => {
                handle_decoder_events(other_event, &self.ffmpeg_sender);
                None
            }
        })
    }
}
