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
- [ ] Anything else? Open a feature request [here](https://github.com/avsaase/walksnail-osd-tool/issues/new?assignees=&labels=enhancement&template=feature_request.yaml).

## Installation

### Windows
Download and run the installer from the [latest release](https://github.com/avsaase/walksnail-osd-tool/releases).

### MacOS
Download the app bundle for your processor architecture from the [latest release](https://github.com/avsaase/walksnail-osd-tool/releases) and drag it to your Applications folder.

<details>
<summary>About unsigned binaries</summary>
    
The MacOS binaries provided by this project are not signed so you have to jump through some hoops to run them.

#### Intel processors
When you open the app for the first time you will get a warning that it cannot be opened because the developer cannot be verified. Click "Cancel", go to System Settings -> Privacy & Security -> Click "Open Anyway". 

#### ARM processors (M1, etc.)
You need to sign the app yourself by running
```
codesign --force --deep -s - /Applications/Walksnail\ OSD\ Tool.app
```
When you try to open the app you may get a similar error as described above for Intel Macs. The same steps should fix it.

If you think all this very annoying you can donate some money [here](https://www.buymeacoffee.com/avsaase) so I can pay Apple for a developer account.
</details>

### Linux
The project builds on Ubuntu in CI but I haven't tried runnning it myself. I don't know enough about packaging for Linux to make release binaries so for now you need to build from source.

### Building from source
1. Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2. Run `cargo build --release --git https://github.com/avsaase/walksnail-osd-tool.git`. The executable will be placed in `$HOME/.cargo/bin/`.
3. To run the app you need the `ffmpeg` and `ffprobe` binaries in your `path` or placed next to the executable you just build.
4. Run the app with `walksnail-osd-tool`.

## Disclaimer
This project is not affiliated with Walksnail/Caddx.
