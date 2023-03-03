use std::{
    iter::Peekable,
    sync::mpsc::{Receiver, Sender},
    vec::IntoIter,
};

use ffmpeg_sidecar::{
    child::FfmpegChild,
    event::{FfmpegEvent, OutputVideoFrame},
    iter::FfmpegIterator,
};

use crate::{font, osd, overlay::overlay_osd_on_video};

use super::StopRenderMessage;

pub struct FrameOverlayIter {
    ffmpeg_iter: FfmpegIterator,
    ffmpeg_child: FfmpegChild,
    osd_frames_iter: Peekable<IntoIter<osd::Frame>>,
    font_file: font::FontFile,
    horizontal_offset: i32,
    vertical_offset: i32,
    current_osd_frame: osd::Frame,
    render_progress_sender: Sender<FfmpegEvent>,
    stop_render_receiver: Receiver<StopRenderMessage>,
}

impl FrameOverlayIter {
    pub fn new(
        ffmpeg_iter: FfmpegIterator,
        ffmpeg_child: FfmpegChild,
        osd_frames: Vec<osd::Frame>,
        font_file: font::FontFile,
        horizontal_offset: i32,
        vertical_offset: i32,
        render_progress_sender: Sender<FfmpegEvent>,
        stop_render_receiver: Receiver<StopRenderMessage>,
    ) -> Self {
        let mut osd_frames_iter = osd_frames.into_iter();
        let first_osd_frame = osd_frames_iter.next().unwrap();
        Self {
            ffmpeg_iter,
            ffmpeg_child,
            osd_frames_iter: osd_frames_iter.peekable(),
            font_file,
            horizontal_offset,
            vertical_offset,
            current_osd_frame: first_osd_frame,
            render_progress_sender,
            stop_render_receiver,
        }
    }
}

impl Iterator for FrameOverlayIter {
    type Item = OutputVideoFrame;

    fn next(&mut self) -> Option<Self::Item> {
        //  On every iteration check if the render should be stopped
        if self.stop_render_receiver.try_recv().is_ok() {
            self.ffmpeg_child.quit().unwrap();
        }

        self.ffmpeg_iter.find_map(|e| match e {
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
            FfmpegEvent::Progress(p) => {
                self.render_progress_sender.send(FfmpegEvent::Progress(p)).unwrap();
                None
            }
            FfmpegEvent::Done => {
                self.render_progress_sender.send(FfmpegEvent::Done).unwrap();
                None
            }
            FfmpegEvent::LogEOF => {
                self.render_progress_sender.send(FfmpegEvent::LogEOF).unwrap();
                None
            }
            _ => None,
        })
    }
}
