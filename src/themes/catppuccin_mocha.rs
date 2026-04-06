//! Catppuccin Mocha theme.

use ratatui::style::Color;

use super::ThemeData;

/// Catppuccin Mocha — warm dark theme with pastel colors.
pub const CATPPUCCIN_MOCHA: ThemeData = ThemeData {
    name: "Catppuccin Mocha",
    id: "catppuccin",
    accent: Color::Rgb(137, 180, 250),
    accent_dim: Color::Rgb(108, 112, 134),
    text: Color::Rgb(205, 214, 244),
    text_dim: Color::Rgb(127, 132, 156),
    text_bright: Color::Rgb(245, 224, 220),
    success: Color::Rgb(166, 227, 161),
    error: Color::Rgb(243, 139, 168),
    warning: Color::Rgb(249, 226, 175),
    info: Color::Rgb(137, 220, 235),
    diff_added: Color::Rgb(166, 227, 161),
    diff_removed: Color::Rgb(243, 139, 168),
    diff_context: Color::Rgb(127, 132, 156),
    border: Color::Rgb(69, 71, 90),
    surface: Color::Rgb(49, 50, 68),
    background: Color::Rgb(30, 30, 46), // #1e1e2e — Mocha Base
};
