//! Dracula theme.

use ratatui::style::Color;

use super::ThemeData;

/// Dracula — dark theme with vivid colors.
pub const DRACULA: ThemeData = ThemeData {
    name: "Dracula",
    id: "dracula",
    accent: Color::Rgb(189, 147, 249),
    accent_dim: Color::Rgb(98, 114, 164),
    text: Color::Rgb(248, 248, 242),
    text_dim: Color::Rgb(98, 114, 164),
    text_bright: Color::Rgb(255, 255, 255),
    success: Color::Rgb(80, 250, 123),
    error: Color::Rgb(255, 85, 85),
    warning: Color::Rgb(241, 250, 140),
    info: Color::Rgb(139, 233, 253),
    diff_added: Color::Rgb(80, 250, 123),
    diff_removed: Color::Rgb(255, 85, 85),
    diff_context: Color::Rgb(98, 114, 164),
    border: Color::Rgb(68, 71, 90),
    surface: Color::Rgb(55, 59, 81),
    background: Color::Rgb(40, 42, 54), // #282a36 — Dracula Background
};
