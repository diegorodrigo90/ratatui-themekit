//! All built-in theme color definitions.
//!
//! Each constant defines the 15 color slots for one theme.
//! Adding a new theme: create a const here + register in `BUILTIN_THEMES`.

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
};

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
};

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
};

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
};

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
    surface: Color::Rgb(0, 43, 54),
};

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
};

/// Tokyo Night — vivid blue accents (by enkia).
pub const TOKYO_NIGHT: ThemeData = ThemeData {
    name: "Tokyo Night",
    id: "tokyo-night",
    accent: Color::Rgb(122, 162, 247),
    accent_dim: Color::Rgb(61, 89, 161),
    text: Color::Rgb(169, 177, 214),
    text_dim: Color::Rgb(120, 124, 153),
    text_bright: Color::Rgb(192, 202, 245),
    success: Color::Rgb(158, 206, 106),
    error: Color::Rgb(247, 118, 142),
    warning: Color::Rgb(224, 175, 104),
    info: Color::Rgb(125, 207, 255),
    diff_added: Color::Rgb(158, 206, 106),
    diff_removed: Color::Rgb(247, 118, 142),
    diff_context: Color::Rgb(120, 124, 153),
    border: Color::Rgb(41, 53, 90),
    surface: Color::Rgb(32, 35, 48),
};

/// Rosé Pine — elegant muted rose tones (by mvllow).
pub const ROSE_PINE: ThemeData = ThemeData {
    name: "Rosé Pine",
    id: "rose-pine",
    accent: Color::Rgb(235, 188, 186),
    accent_dim: Color::Rgb(196, 167, 231),
    text: Color::Rgb(224, 222, 244),
    text_dim: Color::Rgb(144, 140, 170),
    text_bright: Color::Rgb(224, 222, 244),
    success: Color::Rgb(49, 116, 143),
    error: Color::Rgb(235, 111, 146),
    warning: Color::Rgb(246, 193, 119),
    info: Color::Rgb(156, 207, 216),
    diff_added: Color::Rgb(156, 207, 216),
    diff_removed: Color::Rgb(235, 111, 146),
    diff_context: Color::Rgb(110, 106, 134),
    border: Color::Rgb(38, 35, 58),
    surface: Color::Rgb(64, 61, 82),
};

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
};

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
};

/// All built-in themes in display order (`NoColor` excluded — it's special).
pub static BUILTIN_THEMES: &[ThemeData] = &[
    CATPPUCCIN_MOCHA,
    DRACULA,
    NORD,
    GRUVBOX_DARK,
    ONE_DARK,
    SOLARIZED_DARK,
    TAILWIND_DARK,
    TOKYO_NIGHT,
    ROSE_PINE,
    TERMINAL_NATIVE,
];
