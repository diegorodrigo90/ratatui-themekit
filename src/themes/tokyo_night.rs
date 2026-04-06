//! Tokyo Night theme.

use ratatui::style::Color;

use super::ThemeData;

/// Tokyo Night — vivid blue accents (by enkia).
pub const TOKYO_NIGHT: ThemeData = ThemeData {
    name: "Tokyo Night",
    id: "tokyo-night",
    accent: Color::Rgb(122, 162, 247),
    accent_dim: Color::Rgb(61, 89, 161),
    text: Color::Rgb(169, 177, 214),
    text_dim: Color::Rgb(120, 124, 153),
    text_bright: Color::Rgb(192, 202, 245),
    success: Color::Rgb(158, 206, 106),
    error: Color::Rgb(247, 118, 142),
    warning: Color::Rgb(224, 175, 104),
    info: Color::Rgb(125, 207, 255),
    diff_added: Color::Rgb(158, 206, 106),
    diff_removed: Color::Rgb(247, 118, 142),
    diff_context: Color::Rgb(120, 124, 153),
    border: Color::Rgb(41, 53, 90),
    surface: Color::Rgb(32, 35, 48),
    background: Color::Rgb(26, 27, 38), // #1a1b26 — Tokyo Night bg
};
