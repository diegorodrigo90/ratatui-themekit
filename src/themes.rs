//! Built-in theme implementations.

use ratatui::style::Color;

use crate::Theme;

/// Catppuccin Mocha — warm dark theme with pastel colors.
pub struct CatppuccinMocha;

impl Theme for CatppuccinMocha {
    fn name(&self) -> &'static str {
        "Catppuccin Mocha"
    }
    fn id(&self) -> &'static str {
        "catppuccin"
    }
    fn accent(&self) -> Color {
        Color::Rgb(137, 180, 250)
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(108, 112, 134)
    }
    fn text(&self) -> Color {
        Color::Rgb(205, 214, 244)
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(127, 132, 156)
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(245, 224, 220)
    }
    fn success(&self) -> Color {
        Color::Rgb(166, 227, 161)
    }
    fn error(&self) -> Color {
        Color::Rgb(243, 139, 168)
    }
    fn warning(&self) -> Color {
        Color::Rgb(249, 226, 175)
    }
    fn info(&self) -> Color {
        Color::Rgb(137, 220, 235)
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(166, 227, 161)
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(243, 139, 168)
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(127, 132, 156)
    }
    fn border(&self) -> Color {
        Color::Rgb(69, 71, 90)
    }
    fn surface(&self) -> Color {
        Color::Rgb(49, 50, 68)
    }
}

/// Dracula — dark theme with vivid colors.
pub struct Dracula;

impl Theme for Dracula {
    fn name(&self) -> &'static str {
        "Dracula"
    }
    fn id(&self) -> &'static str {
        "dracula"
    }
    fn accent(&self) -> Color {
        Color::Rgb(189, 147, 249)
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(98, 114, 164)
    }
    fn text(&self) -> Color {
        Color::Rgb(248, 248, 242)
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(98, 114, 164)
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(255, 255, 255)
    }
    fn success(&self) -> Color {
        Color::Rgb(80, 250, 123)
    }
    fn error(&self) -> Color {
        Color::Rgb(255, 85, 85)
    }
    fn warning(&self) -> Color {
        Color::Rgb(241, 250, 140)
    }
    fn info(&self) -> Color {
        Color::Rgb(139, 233, 253)
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(80, 250, 123)
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(255, 85, 85)
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(98, 114, 164)
    }
    fn border(&self) -> Color {
        Color::Rgb(68, 71, 90)
    }
    fn surface(&self) -> Color {
        Color::Rgb(50, 52, 68)
    }
}

/// Nord — arctic blue-gray color palette.
pub struct Nord;

impl Theme for Nord {
    fn name(&self) -> &'static str {
        "Nord"
    }
    fn id(&self) -> &'static str {
        "nord"
    }
    fn accent(&self) -> Color {
        Color::Rgb(136, 192, 208)
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(76, 86, 106)
    }
    fn text(&self) -> Color {
        Color::Rgb(216, 222, 233)
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(76, 86, 106)
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(236, 239, 244)
    }
    fn success(&self) -> Color {
        Color::Rgb(163, 190, 140)
    }
    fn error(&self) -> Color {
        Color::Rgb(191, 97, 106)
    }
    fn warning(&self) -> Color {
        Color::Rgb(235, 203, 139)
    }
    fn info(&self) -> Color {
        Color::Rgb(136, 192, 208)
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(163, 190, 140)
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(191, 97, 106)
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(76, 86, 106)
    }
    fn border(&self) -> Color {
        Color::Rgb(59, 66, 82)
    }
    fn surface(&self) -> Color {
        Color::Rgb(46, 52, 64)
    }
}

/// Gruvbox Dark — retro warm dark theme.
pub struct GruvboxDark;

impl Theme for GruvboxDark {
    fn name(&self) -> &'static str {
        "Gruvbox Dark"
    }
    fn id(&self) -> &'static str {
        "gruvbox"
    }
    fn accent(&self) -> Color {
        Color::Rgb(215, 153, 33)
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(124, 111, 100)
    }
    fn text(&self) -> Color {
        Color::Rgb(235, 219, 178)
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(146, 131, 116)
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(251, 241, 199)
    }
    fn success(&self) -> Color {
        Color::Rgb(184, 187, 38)
    }
    fn error(&self) -> Color {
        Color::Rgb(251, 73, 52)
    }
    fn warning(&self) -> Color {
        Color::Rgb(250, 189, 47)
    }
    fn info(&self) -> Color {
        Color::Rgb(131, 165, 152)
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(184, 187, 38)
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(251, 73, 52)
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(146, 131, 116)
    }
    fn border(&self) -> Color {
        Color::Rgb(80, 73, 69)
    }
    fn surface(&self) -> Color {
        Color::Rgb(60, 56, 54)
    }
}

/// One Dark — Atom's iconic dark theme.
pub struct OneDark;

impl Theme for OneDark {
    fn name(&self) -> &'static str {
        "One Dark"
    }
    fn id(&self) -> &'static str {
        "one-dark"
    }
    fn accent(&self) -> Color {
        Color::Rgb(97, 175, 239)
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(92, 99, 112)
    }
    fn text(&self) -> Color {
        Color::Rgb(171, 178, 191)
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(92, 99, 112)
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(220, 223, 228)
    }
    fn success(&self) -> Color {
        Color::Rgb(152, 195, 121)
    }
    fn error(&self) -> Color {
        Color::Rgb(224, 108, 117)
    }
    fn warning(&self) -> Color {
        Color::Rgb(229, 192, 123)
    }
    fn info(&self) -> Color {
        Color::Rgb(86, 182, 194)
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(152, 195, 121)
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(224, 108, 117)
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(92, 99, 112)
    }
    fn border(&self) -> Color {
        Color::Rgb(62, 68, 81)
    }
    fn surface(&self) -> Color {
        Color::Rgb(40, 44, 52)
    }
}

/// Solarized Dark — precision-engineered color scheme.
pub struct SolarizedDark;

impl Theme for SolarizedDark {
    fn name(&self) -> &'static str {
        "Solarized Dark"
    }
    fn id(&self) -> &'static str {
        "solarized"
    }
    fn accent(&self) -> Color {
        Color::Rgb(38, 139, 210)
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(88, 110, 117)
    }
    fn text(&self) -> Color {
        Color::Rgb(131, 148, 150)
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(88, 110, 117)
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(238, 232, 213)
    }
    fn success(&self) -> Color {
        Color::Rgb(133, 153, 0)
    }
    fn error(&self) -> Color {
        Color::Rgb(220, 50, 47)
    }
    fn warning(&self) -> Color {
        Color::Rgb(181, 137, 0)
    }
    fn info(&self) -> Color {
        Color::Rgb(42, 161, 152)
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(133, 153, 0)
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(220, 50, 47)
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(88, 110, 117)
    }
    fn border(&self) -> Color {
        Color::Rgb(7, 54, 66)
    }
    fn surface(&self) -> Color {
        Color::Rgb(0, 43, 54)
    }
}

/// Tailwind Dark — built from ratatui's Tailwind CSS palette.
///
/// Uses `ratatui::style::palette::tailwind` constants directly.
/// These are the exact Tailwind CSS colors — widely recognized
/// and battle-tested for UI design.
pub struct TailwindDark;

impl Theme for TailwindDark {
    fn name(&self) -> &'static str {
        "Tailwind Dark"
    }
    fn id(&self) -> &'static str {
        "tailwind"
    }
    fn accent(&self) -> Color {
        ratatui::style::palette::tailwind::BLUE.c400
    }
    fn accent_dim(&self) -> Color {
        ratatui::style::palette::tailwind::SLATE.c500
    }
    fn text(&self) -> Color {
        ratatui::style::palette::tailwind::SLATE.c200
    }
    fn text_dim(&self) -> Color {
        ratatui::style::palette::tailwind::SLATE.c500
    }
    fn text_bright(&self) -> Color {
        ratatui::style::palette::tailwind::SLATE.c50
    }
    fn success(&self) -> Color {
        ratatui::style::palette::tailwind::EMERALD.c400
    }
    fn error(&self) -> Color {
        ratatui::style::palette::tailwind::RED.c400
    }
    fn warning(&self) -> Color {
        ratatui::style::palette::tailwind::AMBER.c400
    }
    fn info(&self) -> Color {
        ratatui::style::palette::tailwind::CYAN.c400
    }
    fn diff_added(&self) -> Color {
        ratatui::style::palette::tailwind::GREEN.c400
    }
    fn diff_removed(&self) -> Color {
        ratatui::style::palette::tailwind::RED.c400
    }
    fn diff_context(&self) -> Color {
        ratatui::style::palette::tailwind::SLATE.c500
    }
    fn border(&self) -> Color {
        ratatui::style::palette::tailwind::SLATE.c700
    }
    fn surface(&self) -> Color {
        ratatui::style::palette::tailwind::SLATE.c800
    }
}

/// Terminal-native theme — uses ANSI named colors.
///
/// Instead of hardcoding RGB values, this theme uses the 16 standard
/// ANSI colors (`Color::Red`, `Color::Green`, etc.). These colors
/// are defined by the user's terminal emulator theme — if they have
/// Catppuccin, Dracula, or Nord installed in their terminal, this
/// theme automatically matches.
///
/// Best for: apps that should "blend in" with the user's terminal.
pub struct TerminalNative;

impl Theme for TerminalNative {
    fn name(&self) -> &'static str {
        "Terminal Native"
    }
    fn id(&self) -> &'static str {
        "terminal"
    }
    fn accent(&self) -> Color {
        Color::Blue
    }
    fn accent_dim(&self) -> Color {
        Color::DarkGray
    }
    fn text(&self) -> Color {
        Color::White
    }
    fn text_dim(&self) -> Color {
        Color::Gray
    }
    fn text_bright(&self) -> Color {
        Color::White
    }
    fn success(&self) -> Color {
        Color::Green
    }
    fn error(&self) -> Color {
        Color::Red
    }
    fn warning(&self) -> Color {
        Color::Yellow
    }
    fn info(&self) -> Color {
        Color::Cyan
    }
    fn diff_added(&self) -> Color {
        Color::Green
    }
    fn diff_removed(&self) -> Color {
        Color::Red
    }
    fn diff_context(&self) -> Color {
        Color::DarkGray
    }
    fn border(&self) -> Color {
        Color::DarkGray
    }
    fn surface(&self) -> Color {
        Color::Black
    }
}

/// `NO_COLOR` compliant theme — all colors reset.
///
/// Used when `NO_COLOR` env var is set. Safe for pipes, CI,
/// screen readers, and minimal terminals.
pub struct NoColor;

impl Theme for NoColor {
    fn name(&self) -> &'static str {
        "No Color"
    }
    fn id(&self) -> &'static str {
        "no-color"
    }
    fn accent(&self) -> Color {
        Color::Reset
    }
    fn accent_dim(&self) -> Color {
        Color::Reset
    }
    fn text(&self) -> Color {
        Color::Reset
    }
    fn text_dim(&self) -> Color {
        Color::Reset
    }
    fn text_bright(&self) -> Color {
        Color::Reset
    }
    fn success(&self) -> Color {
        Color::Reset
    }
    fn error(&self) -> Color {
        Color::Reset
    }
    fn warning(&self) -> Color {
        Color::Reset
    }
    fn info(&self) -> Color {
        Color::Reset
    }
    fn diff_added(&self) -> Color {
        Color::Reset
    }
    fn diff_removed(&self) -> Color {
        Color::Reset
    }
    fn diff_context(&self) -> Color {
        Color::Reset
    }
    fn border(&self) -> Color {
        Color::Reset
    }
    fn surface(&self) -> Color {
        Color::Reset
    }
}
