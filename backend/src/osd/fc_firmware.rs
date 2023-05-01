use std::fmt::Display;

use super::error::OsdFileError;

#[derive(Debug)]
pub enum FcFirmware {
    Betaflight,
    Inav,
    ArduPilot,
    Kiss,
    KissUltra,
    Unknown,
}

impl TryFrom<&str> for FcFirmware {
    type Error = OsdFileError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use FcFirmware::*;
        match value {
            "BTFL" => Ok(Betaflight),
            "INAV" => Ok(Inav),
            "ARDU" => Ok(ArduPilot),
            "KISS" => Ok(Kiss),
            "ULTR" => Ok(KissUltra),
            _ => Ok(Unknown),
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
                FcFirmware::KissUltra => "KISS ULTRA",
                FcFirmware::Unknown => "Unknown",
            }
        )
    }
}
