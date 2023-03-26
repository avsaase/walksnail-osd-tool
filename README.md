<p align="center">
<img width="256" height="176" src="https://user-images.githubusercontent.com/880421/224411816-c0cf1331-c856-42e9-a3d6-1c23b7da7886.png">
</p>
<h1 align="center">Walksnail OSD Tool</h1>

[![Latest release](https://img.shields.io/github/v/release/avsaase/walksnail-osd-tool?include_prereleases&label=latest%20release)](https://github.com/avsaase/walksnail-osd-tool/releases/latest)
[![Latest build](https://img.shields.io/github/last-commit/avsaase/walksnail-osd-tool/master?label=latest%20build)](https://nightly.link/avsaase/walksnail-osd-tool/workflows/release.yaml/master/walksnail-osd-tool-all-platforms.zip)
[![Totally awesome](https://img.shields.io/badge/totally%20awesome-true-blue)](https://github.com/avsaase/walksnail-osd-tool)

Cross-platform tool for overlaying the OSD from the Walksnail Avatar HD FPV system on top of the video recording.

## Features
- [x] Easy to use graphic user interface.
- [x] Immediately start rendering the video. No intermediate PNG files!
- [x] Hardware-accelerated encoding powered by ffmpeg.
- [x] Choose between h264 and h265 codecs (more can be added later).
- [x] View basic information about the video, OSD and font files.
- [x] Preview OSD frames before rendering.
- [x] Automatically center the OSD or position it manually.
- [x] Selectable output video bitrate (more encoder settings will be added later).
- [ ] Mask OSD items.
- [ ] Display info from the `.srt` file on the video.
- [ ] Anything else? Open a feature request [here](https://github.com/avsaase/walksnail-osd-tool/issues/new?assignees=&labels=enhancement&template=feature_request.yaml).

## Installation

### Windows
Download and run the installer from the [latest release](https://github.com/avsaase/walksnail-osd-tool/releases/latest).

### MacOS
Download the app bundle for your processor architecture from the [latest release](https://github.com/avsaase/walksnail-osd-tool/releases/latest) and drag it to your Applications folder.

<details>
<summary>Instructions for running the first time</summary>
    
The MacOS binaries provided by this project are not signed with a "Developer ID Certificate". When you try to run the app for the first time you may get a warning from MacOS that it may be malicious software, the developer cannot be verified, it may be damaged, etc. Close the warning, go to System Settings -> Privacy & Security and click "Open Anyway". This should only be required the first time you open the app.

If you think this is annoying you can donate some money [here](https://www.buymeacoffee.com/avsaase) so I can pay Apple for a developer account.
</details>

### Linux
The project builds on Ubuntu in CI but I haven't tried runnning it myself. I don't know enough about packaging for Linux to make release binaries so for now you need to build from source.

### Building from source
1. Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2. Run `cargo install --release --git https://github.com/avsaase/walksnail-osd-tool.git`. The executable will be installed in `$HOME/.cargo/bin/` and added to your path.
3. To run the app you need the `ffmpeg` and `ffprobe` binaries in your `path` or placed next to the executable you just build.
4. Run the app with `walksnail-osd-tool`.

## Disclaimer
This project is not affiliated with Walksnail/Caddx.
