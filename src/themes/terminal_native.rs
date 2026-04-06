//! Terminal Native theme.

use ratatui::style::Color;

use super::ThemeData;

/// Terminal Native — ANSI named colors only (no truecolor needed).
pub const TERMINAL_NATIVE: ThemeData = ThemeData {
    name: "Terminal Native",
    id: "terminal",
    accent: Color::Blue,
    accent_dim: Color::DarkGray,
    text: Color::White,
    text_dim: Color::DarkGray,
    text_bright: Color::White,
    success: Color::Green,
    error: Color::Red,
    warning: Color::Yellow,
    info: Color::Cyan,
    diff_added: Color::Green,
    diff_removed: Color::Red,
    diff_context: Color::DarkGray,
    border: Color::DarkGray,
    surface: Color::Black,
    background: Color::Reset, // Respect terminal's own background
};
