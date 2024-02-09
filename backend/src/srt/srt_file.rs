use std::{path::PathBuf, time::Duration};

use parse_display::ParseError;

use derivative::Derivative;

use super::{error::SrtFileError, frame::SrtFrame, SrtFrameData, frame::SrtDebugFrameData};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SrtFile {
    pub file_path: PathBuf,
    pub has_distance: bool,
    pub has_debug: bool,
    pub duration: Duration,
    #[derivative(Debug = "ignore")]
    pub frames: Vec<SrtFrame>,
}

impl SrtFile {
    #[tracing::instrument(ret, err)]
    pub fn open(path: PathBuf) -> Result<Self, SrtFileError> {
        let mut has_distance = false;
        let mut has_debug = false;
        let srt_frames = srtparse::from_file(&path)?
            .iter()
            .map(|i| -> Result<SrtFrame, SrtFileError> {
                let debugData : Result<SrtDebugFrameData, ParseError> = i.text.parse();
                let mut dd : Option<SrtDebugFrameData> = None;
                if !debugData.is_err() 
                {
                    dd = Some(debugData?);
                    has_debug = true;
                }
                //let data: SrtFrameData = SrtFrameData {
                //    signal: 0, channel: 0, flight_time: 0, sky_bat: 0.0, ground_bat: 0.0, latency: 0, bitrate_mbps: 0.0, distance: 0
                //};
                let data: Result<SrtFrameData, ParseError> = i.text.parse();
                let mut d: Option<SrtFrameData> = None;
                if !data.is_err() 
                {
                    let ad = data?;
                    has_distance |= ad.distance > 0;
                    d = Some(ad);
                }
                Ok(SrtFrame {
                    start_time_secs: i.start_time.into_duration().as_secs_f32(),
                    end_time_secs: i.end_time.into_duration().as_secs_f32(),
                    data: d,
                    debugData: dd,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let duration = Duration::from_secs_f32(srt_frames.last().unwrap().end_time_secs);

        Ok(Self {
            file_path: path,
            has_distance,
            has_debug,
            duration,
            frames: srt_frames,
        })
    }
}
