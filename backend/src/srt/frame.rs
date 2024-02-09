use parse_display::FromStr;

#[derive(Debug, Clone)]
pub struct SrtFrame {
    pub start_time_secs: f32,
    pub end_time_secs: f32,
    pub data: Option<SrtFrameData>,
    pub debugData: Option<SrtDebugFrameData>,
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

#[derive(Debug, FromStr, Clone, PartialEq)]
#[display("CH:{channel} MCS:{signal} SP[ {sp1} {sp2}  {sp3} {sp4}] GP[ {gp1}  {gp2}  {gp3}  {gp4}] GTP:{gtp} GTP0:{gtp0} STP:{stp} STP0:{stp0} GSNR:{gsnr} SSNR:{ssnr} Gtemp:{gtemp} Stemp:{stemp} Delay:{latency}ms Frame:{frame}  Gerr:{gerr} SErr:{serr} {serr_ext}, [iso:{iso},mode={iso_mode}, exp:{iso_exp}] [gain:{gain} exp:{gain_exp}ms]")]
pub struct SrtDebugFrameData {
    pub signal: u8,
    pub channel: u8,
    //pub flight_time: u32,
    //pub sky_bat: f32,
    //pub ground_bat: f32,
    pub latency: u32,
    //pub bitrate_mbps: f32,
    //pub distance: u32,
    pub sp1: u16,
    pub sp2: u16,
    pub sp3: u16,
    pub sp4: u16,
    pub gp1: u16,
    pub gp2: u16,
    pub gp3: u16,
    pub gp4: u16,
    pub gtp: u16,
    pub gtp0: u16,
    pub stp: u16,
    pub stp0: u16,
    pub gsnr: f32,
    pub ssnr: f32,
    pub gtemp: f32,
    pub stemp: f32,
    pub frame: u16,
    pub gerr: u16,
    pub serr: u16,
    pub serr_ext: u16,
    pub iso: u32,
    pub iso_mode: String,
    pub iso_exp: u32,
    pub gain: f32,
    pub gain_exp: f32,
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

    #[test]
    fn parse_v37_42_3_debug_src_frame_data() {
        let line = "CH:1 MCS:4 SP[ 45 152  47 149] GP[ 49  48  45  47] GTP:27 GTP0:00 STP:24 STP0:00 GSNR:25.9 SSNR:17.8 Gtemp:50 Stemp:82 Delay:31ms Frame:60  Gerr:0 SErr:0 42, [iso:0,mode=max, exp:0] [gain:0.00 exp:0.000ms]";
        let parsed = line.parse::<SrtDebugFrameData>();
        assert_eq!(
            parsed.expect("Failed to parse SRT frame data"),
            SrtDebugFrameData {
                signal: 4,
                channel: 1,
                //flight_time: 0,
                //sky_bat: 0,
                //ground_bat: 0,
                latency: 31,
                //bitrate_mbps: 0,
                //distance: 0,
                sp1: 45,
                sp2: 152,
                sp3: 47,
                sp4: 149,
                gp1: 49,
                gp2: 48,
                gp3: 45,
                gp4: 47,
                gtp: 27,
                gtp0: 0,
                stp: 24,
                stp0: 0,
                gsnr: 25.9,
                ssnr: 17.8,
                gtemp: 50.0,
                stemp: 82.0,
                frame: 60,
                gerr: 0,
                serr: 0,
                serr_ext: 42,
                iso: 0,
                iso_mode: "max".to_string(),
                iso_exp: 0,
                gain: 0.0,
                gain_exp: 0.0
            }
        )
 

    }
}
