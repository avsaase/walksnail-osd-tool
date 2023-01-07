use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Coordinates<T> {
    pub x: T,
    pub y: T,
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

impl From<Dimension<u32>> for String {
    fn from(value: Dimension<u32>) -> Self {
        format!("{}x{}", value.width, value.height)
    }
}
