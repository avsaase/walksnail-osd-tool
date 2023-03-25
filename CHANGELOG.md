# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- New UI layout with better support for different screen sizes.

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
