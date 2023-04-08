# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Save OSD and SRT options between program runs.
- Custom position and text size of SRT data.
- Option to adjust OSD playback speed to correct for OSD lag with <=32.37.10 firmware.

### Changed
- When loading a SRT file with distance data the distance checkbox doesn't get automatically checked.
- Options sections can be collapsed to save screen space.

## [0.1.0]

### Fixed
- Parsing of firmware version 32.37.10 SRT data.

## [0.1.0-beta4]

### Added
- Render data from the SRT file on the video. Select which values are rendered.
- Automatically load the matching OSD and SRT files when importing a video (they must be in the same folder and have the same file name).
- Upscale output video to 1440p to get better compression on YouTube.

### Changed
- New UI layout with better support for different screen sizes.
- Many small UI tweaks.

### Fixed
- Show correct number of characters in font file.

## [0.1.0-beta3] - 2023-03-23

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
