use std::{iter::Peekable, vec::IntoIter};

use crossbeam_channel::{Receiver, Sender};
use ffmpeg_sidecar::{
    child::FfmpegChild,
    event::{FfmpegEvent, OutputVideoFrame},
    iter::FfmpegIterator,
};

use crate::{
    ffmpeg::{handle_decoder_events, FromFfmpegMessage, ToFfmpegMessage},
    font, osd,
    overlay::overlay_osd_on_video,
};

pub struct FrameOverlayIter {
    decoder_iter: FfmpegIterator,
    decoder_process: FfmpegChild,
    osd_frames_iter: Peekable<IntoIter<osd::Frame>>,
    font_file: font::FontFile,
    horizontal_offset: i32,
    vertical_offset: i32,
    current_osd_frame: osd::Frame,
    ffmpeg_sender: Sender<FromFfmpegMessage>,
    ffmpeg_receiver: Receiver<ToFfmpegMessage>,
}

impl FrameOverlayIter {
    #[tracing::instrument(skip(decoder_iter, decoder_process, osd_frames, font_file), level = "debug")]
    pub fn new(
        decoder_iter: FfmpegIterator,
        decoder_process: FfmpegChild,
        osd_frames: Vec<osd::Frame>,
        font_file: font::FontFile,
        horizontal_offset: i32,
        vertical_offset: i32,
        ffmpeg_sender: Sender<FromFfmpegMessage>,
        ffmpeg_receiver: Receiver<ToFfmpegMessage>,
    ) -> Self {
        let mut osd_frames_iter = osd_frames.into_iter();
        let first_osd_frame = osd_frames_iter.next().unwrap();
        Self {
            decoder_iter,
            decoder_process,
            osd_frames_iter: osd_frames_iter.peekable(),
            font_file,
            horizontal_offset,
            vertical_offset,
            current_osd_frame: first_osd_frame,
            ffmpeg_sender,
            ffmpeg_receiver,
        }
    }
}

impl Iterator for FrameOverlayIter {
    type Item = OutputVideoFrame;

    fn next(&mut self) -> Option<Self::Item> {
        //  On every iteration check if the render should be stopped
        if let Ok(ToFfmpegMessage::AbortRender) = self.ffmpeg_receiver.try_recv() {
            self.decoder_process.quit().unwrap();
        }

        self.decoder_iter.find_map(|e| match e {
            FfmpegEvent::OutputFrame(video_frame) => {
                // For every video frame check if frame time is later than the next OSD frame time.
                // If so advance the iterator over the OSD frames so we use the correct OSD frame
                // for this video frame
                if let Some(next_osd_frame) = self.osd_frames_iter.peek() {
                    let next_osd_frame_secs = next_osd_frame.time_millis as f32 / 1000.0;
                    if video_frame.timestamp > next_osd_frame_secs {
                        self.current_osd_frame = self.osd_frames_iter.next().unwrap();
                    }
                }

                Some(overlay_osd_on_video(
                    video_frame,
                    &self.current_osd_frame,
                    &self.font_file,
                    self.horizontal_offset,
                    self.vertical_offset,
                ))
            }
            other_event => {
                handle_decoder_events(other_event, &self.ffmpeg_sender);
                None
            }
        })
    }
}
