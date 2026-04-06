//! Built-in themes and theme data structure.
//!
//! Each theme lives in its own file and exports a single `ThemeData` const.
//! The `BUILTIN_THEMES` registry collects all of them in display order.
//! Adding a new theme: create a file, define the const, add to registry.

mod catppuccin_mocha;
mod custom;
mod dracula;
mod gruvbox_dark;
mod no_color;
mod nord;
mod one_dark;
mod rose_pine;
mod solarized_dark;
mod tailwind_dark;
mod terminal_native;
mod tokyo_night;

use ratatui::style::Color;

use crate::Theme;

// Re-export all theme constants
pub use catppuccin_mocha::CATPPUCCIN_MOCHA;
pub use dracula::DRACULA;
pub use gruvbox_dark::GRUVBOX_DARK;
pub use no_color::NO_COLOR;
pub use nord::NORD;
pub use one_dark::ONE_DARK;
pub use rose_pine::ROSE_PINE;
pub use solarized_dark::SOLARIZED_DARK;
pub use tailwind_dark::TAILWIND_DARK;
pub use terminal_native::TERMINAL_NATIVE;
pub use tokyo_night::TOKYO_NIGHT;

// Re-export custom theme (serde runtime themes)
pub use custom::CustomTheme;

/// All built-in themes in display order (`NoColor` excluded — it's special).
///
/// This is the single source of truth. `resolve_theme`, `builtin_themes`,
/// and `available_theme_ids` all derive from this registry.
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

// ── ThemeData ──────────────────────────────────────────────────

/// A theme defined as pure data — 15 color slots.
///
/// Implements `Theme` automatically. Built-in themes are `const` values
/// of this type in individual files. User themes use [`CustomTheme`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeData {
    /// Human-readable display name.
    pub name: &'static str,
    /// Short config identifier (e.g. `"catppuccin"`).
    pub id: &'static str,
    /// Primary accent color.
    pub accent: Color,
    /// Muted accent.
    pub accent_dim: Color,
    /// Default text.
    pub text: Color,
    /// Dimmed text.
    pub text_dim: Color,
    /// Bright/emphasis text.
    pub text_bright: Color,
    /// Success/pass.
    pub success: Color,
    /// Error/fail.
    pub error: Color,
    /// Warning/pending.
    pub warning: Color,
    /// Informational.
    pub info: Color,
    /// Diff added lines.
    pub diff_added: Color,
    /// Diff removed lines.
    pub diff_removed: Color,
    /// Diff context/unchanged.
    pub diff_context: Color,
    /// Panel borders.
    pub border: Color,
    /// Background highlight (selected/focused).
    pub surface: Color,
}

impl std::fmt::Display for ThemeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

impl Theme for ThemeData {
    fn name(&self) -> &str {
        self.name
    }
    fn id(&self) -> &str {
        self.id
    }
    fn accent(&self) -> Color {
        self.accent
    }
    fn accent_dim(&self) -> Color {
        self.accent_dim
    }
    fn text(&self) -> Color {
        self.text
    }
    fn text_dim(&self) -> Color {
        self.text_dim
    }
    fn text_bright(&self) -> Color {
        self.text_bright
    }
    fn success(&self) -> Color {
        self.success
    }
    fn error(&self) -> Color {
        self.error
    }
    fn warning(&self) -> Color {
        self.warning
    }
    fn info(&self) -> Color {
        self.info
    }
    fn diff_added(&self) -> Color {
        self.diff_added
    }
    fn diff_removed(&self) -> Color {
        self.diff_removed
    }
    fn diff_context(&self) -> Color {
        self.diff_context
    }
    fn border(&self) -> Color {
        self.border
    }
    fn surface(&self) -> Color {
        self.surface
    }
}
