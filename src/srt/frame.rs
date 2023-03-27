use parse_display::FromStr;

#[derive(Debug, Clone)]
pub struct SrtFrame {
    pub start_time_secs: f32,
    pub end_time_secs: f32,
    pub data: SrtFrameData,
}

#[derive(Debug, FromStr, Clone)]
#[display("Signal:{signal} CH:{channel} FlightTime:{flight_time} SBat:{sky_bat}V GBat:{ground_bat}V Delay:{latency}ms Bitrate:{bitrate_mbps}Mbps Distance:{distance}m")]
pub struct SrtFrameData {
    pub signal: u8,
    pub channel: u8,
    pub flight_time: u32,
    pub sky_bat: f32,
    pub ground_bat: f32,
    pub latency: u32,
    pub bitrate_mbps: u32,
    pub distance: u32,
}
