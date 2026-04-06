//! Theme data structure and built-in theme catalog.

mod catalog;
mod custom;

use ratatui::style::Color;

use crate::Theme;

// Re-export everything from submodules
pub use catalog::*;
pub use custom::CustomTheme;

/// A theme defined as pure data — 15 color slots.
///
/// Implements `Theme` automatically. Built-in themes are `const` values
/// of this type. See [`catalog`] for all available themes.
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
