# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Load last used OSD font file on startup (@dz0ny).
- Option to render video with a chroma key background instead of the input video so the OSD can be overlayed in a video editor.
- Support for Betaflight 4.5 four color fonts.
- Support for 4K and 2.7K DVR ([#43](https://github.com/avsaase/walksnail-osd-tool/pull/43), @mmosca).

### Fixed

- Bug that caused font files with unexpected number of characters to not open.

## [0.2.0] - 2023-04-23

### Added

- Save OSD and SRT options between program runs.
- Custom position and text size of SRT data.
- Option to adjust OSD playback speed to correct for OSD lag with <=32.37.10 firmware.
- Check for app updates during startup.
- Hide/mask OSD elements from the rendered video ([demo](https://i.imgur.com/u8xi2tX.mp4)).
- Tooltips explaining options and settings.

### Changed

- When loading a SRT file with distance data the distance checkbox doesn't get automatically checked.
- Options sections can be collapsed to save screen space.

## [0.1.0] - 2023-03-31

### Fixed

- Parsing of firmware version 32.37.10 SRT data.

## [0.1.0-beta4] - 2023-03-28

### Added

- Render data from the SRT file on the video. Select which values are rendered.
- Automatically load the matching OSD and SRT files when importing a video (they must be in the same folder and have the same file name).
- Upscale output video to 1440p to get better compression on YouTube.

### Changed

- New UI layout with better support for different screen sizes.
- Many small UI tweaks.

### Fixed

- Show correct number of characters in font file.

## [0.1.0-beta3] - 2023-03-21

### Added

- Open files by dropping them on the window.
- Improve render speed.
- Logging of ffmpeg errors and warnings.
- Option to select undetected encoders (use at your own risk).
- Dark theme (default light, toggle by clicking the sun/moon icon in the top right).

### Changed

- Improved handling of ffmpeg events.

### Fixed

- Issue with non-critical ffmpeg errors stopping the render process.
- Output videos not playable in some video players.

## [0.1.0-beta2] - 2023-03-15

### Added

- Make main window resizable in vertical direction to accomodate retina displays and screens with lower resolutions.
- Display errors from ffmpeg.
- Display tooltip when hovering over start render button when it is disabled.

### Changed

- Improved formatting of "About" window.
- Improved display of render status when rendering is finished or cancelled.

### Fixed

- Check for `hevc_videotoolbox` encoder on MacOS.
- Stop ffmpeg decoder when encoder returns error.
- Fixed version info display.
- Properly disable buttons that cannot be used.

## [0.1.0-beta1] - 2023-03-11

### Added

First beta release with limited features.
