//! Rose Pine theme.

use ratatui::style::Color;

use super::ThemeData;

/// Rosé Pine — elegant muted rose tones (by mvllow).
pub const ROSE_PINE: ThemeData = ThemeData {
    name: "Rosé Pine",
    id: "rose-pine",
    accent: Color::Rgb(235, 188, 186),
    accent_dim: Color::Rgb(196, 167, 231),
    text: Color::Rgb(224, 222, 244),
    text_dim: Color::Rgb(144, 140, 170),
    text_bright: Color::Rgb(224, 222, 244),
    success: Color::Rgb(49, 116, 143),
    error: Color::Rgb(235, 111, 146),
    warning: Color::Rgb(246, 193, 119),
    info: Color::Rgb(156, 207, 216),
    diff_added: Color::Rgb(156, 207, 216),
    diff_removed: Color::Rgb(235, 111, 146),
    diff_context: Color::Rgb(110, 106, 134),
    border: Color::Rgb(38, 35, 58),
    surface: Color::Rgb(64, 61, 82),
};
