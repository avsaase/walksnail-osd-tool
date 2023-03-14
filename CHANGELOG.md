# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Make main window resizable in vertical direction to accomodate retina displays and screens with lower resolutions.
- Display errors from ffmpeg

### Changed
- Improved formatting of "About" window.
- Improved display of render status when rendering is finished or cancelled.

### Fixed
- Check for `hevc_videotoolbox` encoder on MacOS.
- Stop ffmpeg decoder when encoder returns error.
- Fixd version info display.

## [0.1.0-beta1] - 2023-03-11

### Added
First beta release with limited features.
