//! Solarized Dark theme.

use ratatui::style::Color;

use super::ThemeData;

/// Solarized Dark — precision-engineered dark theme.
pub const SOLARIZED_DARK: ThemeData = ThemeData {
    name: "Solarized Dark",
    id: "solarized",
    accent: Color::Rgb(38, 139, 210),
    accent_dim: Color::Rgb(88, 110, 117),
    text: Color::Rgb(147, 161, 161),
    text_dim: Color::Rgb(88, 110, 117),
    text_bright: Color::Rgb(253, 246, 227),
    success: Color::Rgb(133, 153, 0),
    error: Color::Rgb(220, 50, 47),
    warning: Color::Rgb(181, 137, 0),
    info: Color::Rgb(42, 161, 152),
    diff_added: Color::Rgb(133, 153, 0),
    diff_removed: Color::Rgb(220, 50, 47),
    diff_context: Color::Rgb(88, 110, 117),
    border: Color::Rgb(7, 54, 66),
    surface: Color::Rgb(7, 54, 66), // #073642 — Solarized base02 (highlight)
    background: Color::Rgb(0, 43, 54), // #002b36 — Solarized base03 (background)
};
