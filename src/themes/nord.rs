//! Nord theme.

use ratatui::style::Color;

use super::ThemeData;

/// Nord — arctic blue-gray theme.
pub const NORD: ThemeData = ThemeData {
    name: "Nord",
    id: "nord",
    accent: Color::Rgb(136, 192, 208),
    accent_dim: Color::Rgb(76, 86, 106),
    text: Color::Rgb(216, 222, 233),
    text_dim: Color::Rgb(76, 86, 106),
    text_bright: Color::Rgb(236, 239, 244),
    success: Color::Rgb(163, 190, 140),
    error: Color::Rgb(191, 97, 106),
    warning: Color::Rgb(235, 203, 139),
    info: Color::Rgb(129, 161, 193),
    diff_added: Color::Rgb(163, 190, 140),
    diff_removed: Color::Rgb(191, 97, 106),
    diff_context: Color::Rgb(76, 86, 106),
    border: Color::Rgb(59, 66, 82),
    surface: Color::Rgb(46, 52, 64),
    background: Color::Rgb(36, 40, 50), // #2e3440 — Nord Polar Night
};
