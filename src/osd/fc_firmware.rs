use std::fmt::Display;

use super::error::OsdFileError;

#[derive(Debug)]
pub enum FcFirmware {
    Betaflight,
    Inav,
    ArduPilot,
    Kiss,
}

impl TryFrom<&str> for FcFirmware {
    type Error = OsdFileError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BTFL" => Ok(FcFirmware::Betaflight),
            "INAV" => Ok(FcFirmware::Inav),
            "ARDU" => Ok(FcFirmware::ArduPilot),
            "KISS" => Ok(FcFirmware::Kiss),
            _ => Err(OsdFileError::UnknownFcFirmware(value.to_string())),
        }
    }
}

impl TryFrom<&[u8]> for FcFirmware {
    type Error = OsdFileError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let string = std::str::from_utf8(value)?;
        string.try_into()
    }
}

impl Display for FcFirmware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FcFirmware::Betaflight => "BetaFlight",
                FcFirmware::Inav => "INAV",
                FcFirmware::ArduPilot => "ArduPilot",
                FcFirmware::Kiss => "KISS",
            }
        )
    }
}
