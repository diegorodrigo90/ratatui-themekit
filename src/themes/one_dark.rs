//! One Dark theme.

use ratatui::style::Color;

use super::ThemeData;

/// One Dark — Atom editor's iconic theme.
pub const ONE_DARK: ThemeData = ThemeData {
    name: "One Dark",
    id: "one-dark",
    accent: Color::Rgb(97, 175, 239),
    accent_dim: Color::Rgb(92, 99, 112),
    text: Color::Rgb(171, 178, 191),
    text_dim: Color::Rgb(92, 99, 112),
    text_bright: Color::Rgb(209, 212, 219),
    success: Color::Rgb(152, 195, 121),
    error: Color::Rgb(224, 108, 117),
    warning: Color::Rgb(229, 192, 123),
    info: Color::Rgb(86, 182, 194),
    diff_added: Color::Rgb(152, 195, 121),
    diff_removed: Color::Rgb(224, 108, 117),
    diff_context: Color::Rgb(92, 99, 112),
    border: Color::Rgb(62, 68, 81),
    surface: Color::Rgb(44, 49, 58),
};
