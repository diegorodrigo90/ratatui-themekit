//! Tailwind Dark theme.

use ratatui::style::Color;

use super::ThemeData;

/// Tailwind Dark — based on Tailwind CSS color palette.
pub const TAILWIND_DARK: ThemeData = ThemeData {
    name: "Tailwind Dark",
    id: "tailwind",
    accent: Color::Rgb(99, 102, 241),
    accent_dim: Color::Rgb(67, 56, 202),
    text: Color::Rgb(226, 232, 240),
    text_dim: Color::Rgb(148, 163, 184),
    text_bright: Color::Rgb(248, 250, 252),
    success: Color::Rgb(34, 197, 94),
    error: Color::Rgb(239, 68, 68),
    warning: Color::Rgb(234, 179, 8),
    info: Color::Rgb(14, 165, 233),
    diff_added: Color::Rgb(34, 197, 94),
    diff_removed: Color::Rgb(239, 68, 68),
    diff_context: Color::Rgb(148, 163, 184),
    border: Color::Rgb(51, 65, 85),
    surface: Color::Rgb(30, 41, 59),
    background: Color::Rgb(15, 23, 42), // #0f172a — Tailwind slate-900
};
