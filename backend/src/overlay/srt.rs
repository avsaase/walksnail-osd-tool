use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

use crate::srt::{SrtFrameData, SrtOptions};

#[inline]
pub fn overlay_srt_data(
    image: &mut RgbaImage,
    srt_data: &Option<SrtFrameData>,
    font: &rusttype::Font,
    srt_options: &SrtOptions,
) {
    if srt_data.is_none() {
        return;
    }

    let mut data;
    match srt_data {
        Some(d) => {
            data = d;
        },
        None => return
    }
    
    let time_str = if srt_options.show_time {
        let minutes = data.flight_time / 60;
        let seconds = data.flight_time % 60;
        format!("Time:{}:{:0>2}  ", minutes, seconds % 60)
    } else {
        "".into()
    };

    let sbat_str = if srt_options.show_sbat {
        format!("SBat:{: >4.1}V  ", data.sky_bat)
    } else {
        "".into()
    };

    let gbat_str = if srt_options.show_gbat {
        format!("GBat:{: >4.1}V  ", data.ground_bat)
    } else {
        "".into()
    };

    let signal_str = if srt_options.show_signal {
        format!("Signal:{}  ", data.signal)
    } else {
        "".into()
    };

    let latency_str = if srt_options.show_latency {
        format!("Latency:{: >3}ms  ", data.latency)
    } else {
        "".into()
    };

    let bitrate_str = if srt_options.show_bitrate {
        format!("Bitrate:{: >4.1}Mbps  ", data.bitrate_mbps)
    } else {
        "".into()
    };

    let distance_str = if srt_options.show_distance {
        let distance = data.distance;
        if distance > 999 {
            let km = distance as f32 / 1000.0;
            format!("Distance:{:.2}km", km)
        } else {
            format!("Distance:{: >3}m", data.distance)
        }
    } else {
        "".into()
    };

    let srt_string = format!("{time_str}{sbat_str}{gbat_str}{signal_str}{latency_str}{bitrate_str}{distance_str}");

    let image_dimensions = image.dimensions();

    let x_pos = srt_options.position.x / 100.0 * image_dimensions.0 as f32;
    let y_pos = srt_options.position.y / 100.0 * image_dimensions.1 as f32;
    let scale = srt_options.scale / 1080.0 * image_dimensions.1 as f32;

    draw_text_mut(
        image,
        Rgba([240u8, 240u8, 240u8, 10u8]),
        x_pos as i32,
        y_pos as i32,
        rusttype::Scale::uniform(scale),
        font,
        &srt_string,
    );
}
