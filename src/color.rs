use std::fmt;

use crate::level::Level;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const RED: Color = Color        { r: 255, g: 0,   b: 0 };
    pub const GREEN: Color = Color      { r: 0,   g: 255, b: 0 };
    pub const BLUE: Color = Color       { r: 0,   g: 0,   b: 255 };
    pub const YELLOW: Color = Color     { r: 255, g: 255, b: 0 };
    pub const MAGENTA: Color = Color    { r: 255, g: 0,   b: 255 };
    pub const CYAN: Color = Color       { r: 0,   g: 255, b: 255 };
    pub const WHITE: Color = Color      { r: 255, g: 255, b: 255 };
    pub const BLACK: Color = Color      { r: 0,   g: 0,   b: 0 };
    pub const GRAY: Color = Color       { r: 128, g: 128, b: 128 };

    pub const CL_ERROR: Color = Color   { r: 255, g: 55,  b: 55 };
    pub const CL_WARN: Color = Color    { r: 255, g: 166, b: 55 };
    pub const CL_INFO: Color = Color    { r: 62, g: 255, b: 55 };
    pub const CL_DEBUG: Color = Color   { r: 48,  g: 166, b: 255 };
    pub const CL_TRACE: Color = Color   { r: 255, g: 55,  b: 255 };
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl From<Level> for Color {
    fn from(level: Level) -> Self {
        match level {
            Level::Error => Color::CL_ERROR,
            Level::Warn => Color::CL_WARN,
            Level::Info => Color::CL_INFO,
            Level::Debug => Color::CL_DEBUG,
            Level::Trace => Color::CL_TRACE,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.r, self.g, self.b)
    }
}

impl From<String> for Color {
    fn from(color: String) -> Self {
        let mut parts = color.split(',');
        let r = match parts.next() {
            Some(r) => r.trim().parse::<u8>().unwrap_or(0),
            None => 0,
        };
        let g = match parts.next() {
            Some(g) => g.trim().parse::<u8>().unwrap_or(0),
            None => 0,
        };
        let b = match parts.next() {
            Some(b) => b.trim().parse::<u8>().unwrap_or(0),
            None => 0,
        };
        Self { r, g, b }
    }
}