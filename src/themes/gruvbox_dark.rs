//! Gruvbox Dark theme.

use ratatui::style::Color;

use super::ThemeData;

/// Gruvbox Dark — retro warm dark theme.
pub const GRUVBOX_DARK: ThemeData = ThemeData {
    name: "Gruvbox Dark",
    id: "gruvbox",
    accent: Color::Rgb(215, 153, 33),
    accent_dim: Color::Rgb(146, 131, 116),
    text: Color::Rgb(235, 219, 178),
    text_dim: Color::Rgb(146, 131, 116),
    text_bright: Color::Rgb(251, 241, 199),
    success: Color::Rgb(142, 192, 124),
    error: Color::Rgb(204, 36, 29),
    warning: Color::Rgb(250, 189, 47),
    info: Color::Rgb(131, 165, 152),
    diff_added: Color::Rgb(142, 192, 124),
    diff_removed: Color::Rgb(204, 36, 29),
    diff_context: Color::Rgb(146, 131, 116),
    border: Color::Rgb(80, 73, 69),
    surface: Color::Rgb(60, 56, 54),
    background: Color::Rgb(40, 40, 40), // #282828 — Gruvbox bg0
};
