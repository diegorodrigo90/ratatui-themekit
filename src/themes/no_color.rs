//! No Color theme.

use ratatui::style::Color;

use super::ThemeData;

/// No Color — all `Color::Reset`. Respects <https://no-color.org/>.
pub const NO_COLOR: ThemeData = ThemeData {
    name: "No Color",
    id: "no-color",
    accent: Color::Reset,
    accent_dim: Color::Reset,
    text: Color::Reset,
    text_dim: Color::Reset,
    text_bright: Color::Reset,
    success: Color::Reset,
    error: Color::Reset,
    warning: Color::Reset,
    info: Color::Reset,
    diff_added: Color::Reset,
    diff_removed: Color::Reset,
    diff_context: Color::Reset,
    border: Color::Reset,
    surface: Color::Reset,
    background: Color::Reset, // No color override
};
