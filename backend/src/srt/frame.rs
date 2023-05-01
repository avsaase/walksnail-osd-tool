use parse_display::FromStr;

#[derive(Debug, Clone)]
pub struct SrtFrame {
    pub start_time_secs: f32,
    pub end_time_secs: f32,
    pub data: SrtFrameData,
}

#[derive(Debug, FromStr, Clone, PartialEq)]
#[display("Signal:{signal} CH:{channel} FlightTime:{flight_time} SBat:{sky_bat}V GBat:{ground_bat}V Delay:{latency}ms Bitrate:{bitrate_mbps}Mbps Distance:{distance}m")]
pub struct SrtFrameData {
    pub signal: u8,
    pub channel: u8,
    pub flight_time: u32,
    pub sky_bat: f32,
    pub ground_bat: f32,
    pub latency: u32,
    pub bitrate_mbps: f32,
    pub distance: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pre_v31_36_8_srt_frame_data() {
        let line = "Signal:4 CH:8 FlightTime:0 SBat:4.7V GBat:7.2V Delay:32ms Bitrate:25Mbps Distance:7m";
        let parsed = line.parse::<SrtFrameData>();
        assert_eq!(
            parsed.expect("Failed to parse SRT frame data"),
            SrtFrameData {
                signal: 4,
                channel: 8,
                flight_time: 0,
                sky_bat: 4.7,
                ground_bat: 7.2,
                latency: 32,
                bitrate_mbps: 25.0,
                distance: 7
            }
        )
    }

    #[test]
    fn parse_v32_37_10_srt_frame_data() {
        let line = "Signal:4 CH:7 FlightTime:0 SBat:16.7V GBat:12.5V Delay:25ms Bitrate:25.0Mbps Distance:1m";
        let parsed = line.parse::<SrtFrameData>();
        assert_eq!(
            parsed.expect("Failed to parse SRT frame data"),
            SrtFrameData {
                signal: 4,
                channel: 7,
                flight_time: 0,
                sky_bat: 16.7,
                ground_bat: 12.5,
                latency: 25,
                bitrate_mbps: 25.0,
                distance: 1
            }
        )
    }
}
