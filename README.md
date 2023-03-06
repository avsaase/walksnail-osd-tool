# Walksnail OSD Tool
Cross-platform tool for overlaying the Walksnail Avatar Goggle and VRX OSD recording on top of the video recording.

![App Screenshot](https://user-images.githubusercontent.com/880421/222804317-2f5b8ef4-970d-4ae5-b249-c0d7d267b06d.png)

## Features
- [x] Immediately start rendering the video. No intermediate PNG files!
- [x] Hardware-accelerated encoding powered by ffmpeg.
- [x] Choose between h264 and h265 encoding (more can be added later).
- [x] View basic information about the video, OSD and font files.
- [x] Preview OSD frames before rendering.
- [x] Automatically center the OSD or position it manually.
- [x] Selectable output video bitrate (more encoder settings will be added later).
- [ ] Mask OSD items.
- [ ] Display info from the `.srt` file on the video.
- [ ] Anything else? Suggestions are welcome :).

## Installation

### Windows
1. Go to the [latest release](https://github.com/avsaase/walksnail-osd-tool/releases), download and run the installer.
2. During installation you get the option to disable installing the `ffmpeg` dependencies. You can disable this if you already have `ffmpeg` and `ffprobe` installed and in your `$Path`. Otherwise leave it enabled.

### MacOS
1. Go to the [latest release](https://github.com/avsaase/walksnail-osd-tool/releases), download the app bundle and drag it to your Applications folder.
2. The binary is not signed so the first time you try to run it you will get an error that it is software from an unidentified developer. Go into System Settings -> Privacy & Security -> Click "Open Anyway". The app uses the included `ffmpeg` and `ffprobe` binaries so when running it the first time you may get a similar warning two more times. Repeat the same steps to fix it. (If you think this is annoying you can give me some money [here](https://www.buymeacoffee.com/avsaase) so I can pay Apple for a developer account.)

### Linux
The project builds on Ubuntu in CI but I haven't tried runnning it myself. For now you need to build from source.

### Building from source
1. Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2. Run `cargo build --release --git https://github.com/avsaase/walksnail-osd-tool.git`. The executable will be placed in `$HOME/.cargo/bin/`.
3. To run the app you need the `ffmpeg` and `ffprobe` binaries in your `path` or placed next to the executable you just build.
4. Run the app with `walksnail-osd-tool`.

## Disclaimer
This project is not affiliated with Walksnail/Caddx.