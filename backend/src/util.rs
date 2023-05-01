use std::fmt::Display;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
pub const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Coordinates<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coordinates<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Dimension<T> {
    pub width: T,
    pub height: T,
}

impl Display for Dimension<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct AppUpdate {
    #[derivative(Default(value = "true"))]
    pub check_on_startup: bool,
}
