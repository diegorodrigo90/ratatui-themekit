//! # ratatui-themekit
//!
//! Semantic theme system for [ratatui](https://ratatui.rs) applications.
//!
//! Instead of hardcoding `Color::Rgb(...)` throughout your TUI, define
//! semantic slots (`accent`, `success`, `error`, `text_dim`, etc.) and
//! let the active theme provide the concrete colors. Switch themes at
//! runtime with zero code changes.
//!
//! ## Quick Start
//!
//! ```rust
//! use ratatui::style::Style;
//! use ratatui_themekit::{Theme, CatppuccinMocha};
//!
//! let theme: &dyn Theme = &CatppuccinMocha;
//!
//! // Use semantic slots — never hardcode colors
//! let header_style = Style::default().fg(theme.accent());
//! let error_style = Style::default().fg(theme.error());
//! let dim_style = Style::default().fg(theme.text_dim());
//! ```
//!
//! ## Built-in Themes
//!
//! | Theme | ID | Description |
//! |-------|----|-------------|
//! | [`CatppuccinMocha`] | `catppuccin` | Warm dark with pastels |
//! | [`Dracula`] | `dracula` | Dark with vivid colors |
//! | [`Nord`] | `nord` | Arctic blue-gray |
//! | [`GruvboxDark`] | `gruvbox` | Retro warm dark |
//! | [`OneDark`] | `one-dark` | Atom's iconic theme |
//! | [`SolarizedDark`] | `solarized` | Precision dark |
//! | [`NoColor`] | `no-color` | `NO_COLOR` compliant (all reset) |
//!
//! ## Resolve by ID
//!
//! ```rust
//! use ratatui_themekit::resolve_theme;
//!
//! let theme = resolve_theme("dracula");
//! assert_eq!(theme.id(), "dracula");
//! ```
//!
//! ## `NO_COLOR` Support
//!
//! Respects the [NO_COLOR](https://no-color.org/) convention:
//!
//! ```rust
//! use ratatui_themekit::no_color_active;
//!
//! if no_color_active() {
//!     // All colors are Color::Reset — safe for pipes, CI, screen readers
//! }
//! ```
//!
//! ## Custom Themes
//!
//! Implement the [`Theme`] trait — only 15 required methods. The rest
//! have sensible defaults derived from the core slots.
//!
//! ```rust
//! use ratatui::style::Color;
//! use ratatui_themekit::Theme;
//!
//! struct MyTheme;
//!
//! impl Theme for MyTheme {
//!     fn name(&self) -> &str { "My Theme" }
//!     fn id(&self) -> &str { "my-theme" }
//!     fn accent(&self) -> Color { Color::Rgb(249, 115, 22) } // orange
//!     fn accent_dim(&self) -> Color { Color::Rgb(120, 80, 40) }
//!     fn text(&self) -> Color { Color::Rgb(220, 220, 220) }
//!     fn text_dim(&self) -> Color { Color::Rgb(120, 120, 120) }
//!     fn text_bright(&self) -> Color { Color::Rgb(255, 255, 255) }
//!     fn success(&self) -> Color { Color::Green }
//!     fn error(&self) -> Color { Color::Red }
//!     fn warning(&self) -> Color { Color::Yellow }
//!     fn info(&self) -> Color { Color::Cyan }
//!     fn diff_added(&self) -> Color { Color::Green }
//!     fn diff_removed(&self) -> Color { Color::Red }
//!     fn diff_context(&self) -> Color { Color::DarkGray }
//!     fn border(&self) -> Color { Color::DarkGray }
//!     fn surface(&self) -> Color { Color::Rgb(40, 40, 40) }
//! }
//! ```

mod custom;
mod themes;

use ratatui::style::Color;

// Re-export all built-in themes
pub use themes::{
    CatppuccinMocha, Dracula, GruvboxDark, NoColor, Nord, OneDark, SolarizedDark, TailwindDark,
    TerminalNative,
};

// Re-export custom theme (serde-powered user themes)
pub use custom::CustomTheme;

// ── Theme trait ──────────────────────────────────────────────────

/// Semantic color contract for ratatui applications.
///
/// Define **what** each color means, not what RGB value it is.
/// Every render function uses these slots. Swap themes at runtime
/// and every widget updates automatically.
///
/// # Required methods (15)
///
/// Core identity + 13 color slots + surface. All other methods
/// have defaults derived from these.
///
/// # Derived methods (5+)
///
/// `block_*` and `indicator_*` methods derive from core slots.
/// Override them for fine-grained control.
pub trait Theme: Send + Sync {
    /// Human-readable theme name (e.g. `"Catppuccin Mocha"`).
    fn name(&self) -> &str;

    /// Short identifier for config files (e.g. `"catppuccin"`).
    fn id(&self) -> &str;

    // ── Brand ─────────────────────────────────────────────────

    /// Primary brand/accent color.
    fn accent(&self) -> Color;

    /// Secondary accent (less prominent highlights).
    fn accent_dim(&self) -> Color;

    // ── Text ──────────────────────────────────────────────────

    /// Default text color.
    fn text(&self) -> Color;

    /// Dimmed/muted text (timestamps, hints, inactive elements).
    fn text_dim(&self) -> Color;

    /// Bright text for emphasis (bold titles, active elements).
    fn text_bright(&self) -> Color;

    // ── Status ────────────────────────────────────────────────

    /// Success / passed / running.
    fn success(&self) -> Color;

    /// Error / failed.
    fn error(&self) -> Color;

    /// Warning / pending / in-progress.
    fn warning(&self) -> Color;

    /// Informational / neutral highlight.
    fn info(&self) -> Color;

    // ── Diff ──────────────────────────────────────────────────

    /// Lines added.
    fn diff_added(&self) -> Color;

    /// Lines removed.
    fn diff_removed(&self) -> Color;

    /// Context/unchanged lines.
    fn diff_context(&self) -> Color;

    // ── Structure ─────────────────────────────────────────────

    /// Border/separator color.
    fn border(&self) -> Color;

    /// Background highlight for focused/selected elements.
    fn surface(&self) -> Color;

    // ── Derived defaults (override for fine control) ──────────

    /// Color for file-read operations.
    fn block_file_read(&self) -> Color {
        self.text_dim()
    }

    /// Color for file-edit operations.
    fn block_file_edit(&self) -> Color {
        self.diff_added()
    }

    /// Color for command/shell operations.
    fn block_command(&self) -> Color {
        self.text_bright()
    }

    /// Color for thinking/reasoning indicators.
    fn block_thinking(&self) -> Color {
        self.text_dim()
    }

    /// Color for passed indicators.
    fn block_pass(&self) -> Color {
        self.success()
    }

    /// Color for failed indicators.
    fn block_fail(&self) -> Color {
        self.error()
    }

    /// Color for system messages.
    fn block_system(&self) -> Color {
        self.text_dim()
    }

    /// Pending indicator color.
    fn indicator_pending(&self) -> Color {
        self.text_dim()
    }

    /// Running indicator color.
    fn indicator_running(&self) -> Color {
        self.warning()
    }

    /// Passed indicator color.
    fn indicator_passed(&self) -> Color {
        self.success()
    }

    /// Failed indicator color.
    fn indicator_failed(&self) -> Color {
        self.error()
    }

    /// Skipped indicator color.
    fn indicator_skipped(&self) -> Color {
        self.text_dim()
    }
}

// ── Resolution helpers ──────────────────────────────────────────

/// Resolves a theme by its config ID string.
///
/// Returns the matching built-in theme, or falls back to
/// [`CatppuccinMocha`] for unknown IDs. If [`no_color_active()`]
/// returns true, always returns [`NoColor`].
#[must_use]
pub fn resolve_theme(id: &str) -> Box<dyn Theme> {
    if no_color_active() {
        return Box::new(NoColor);
    }
    match id {
        "dracula" => Box::new(Dracula),
        "nord" => Box::new(Nord),
        "gruvbox" => Box::new(GruvboxDark),
        "one-dark" => Box::new(OneDark),
        "solarized" => Box::new(SolarizedDark),
        "tailwind" => Box::new(TailwindDark),
        "terminal" => Box::new(TerminalNative),
        "no-color" => Box::new(NoColor),
        // "catppuccin" and any unknown ID → default theme
        _ => Box::new(CatppuccinMocha),
    }
}

/// Returns all built-in theme instances.
#[must_use]
pub fn builtin_themes() -> Vec<Box<dyn Theme>> {
    vec![
        Box::new(CatppuccinMocha),
        Box::new(Dracula),
        Box::new(Nord),
        Box::new(GruvboxDark),
        Box::new(OneDark),
        Box::new(SolarizedDark),
        Box::new(TailwindDark),
        Box::new(TerminalNative),
    ]
}

/// Returns all built-in theme IDs.
#[must_use]
pub fn available_theme_ids() -> Vec<&'static str> {
    vec![
        "catppuccin",
        "dracula",
        "nord",
        "gruvbox",
        "one-dark",
        "solarized",
        "tailwind",
        "terminal",
    ]
}

/// Checks if the `NO_COLOR` environment variable is set.
///
/// Respects <https://no-color.org/>. When active, all colors should
/// be `Color::Reset` for accessibility (pipes, CI, screen readers).
#[must_use]
pub fn no_color_active() -> bool {
    std::env::var_os("NO_COLOR").is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_known_theme() {
        let theme = resolve_theme("dracula");
        assert_eq!(theme.id(), "dracula");
        assert_eq!(theme.name(), "Dracula");
    }

    #[test]
    fn resolve_unknown_falls_back() {
        let theme = resolve_theme("nonexistent");
        assert_eq!(theme.id(), "catppuccin");
    }

    #[test]
    fn all_builtin_themes_have_unique_ids() {
        let themes = builtin_themes();
        let ids: Vec<&str> = themes.iter().map(|t| t.id()).collect();
        let mut dedup = ids.clone();
        dedup.sort_unstable();
        dedup.dedup();
        assert_eq!(ids.len(), dedup.len());
    }

    #[test]
    fn available_ids_match_builtins() {
        let ids = available_theme_ids();
        let themes = builtin_themes();
        assert_eq!(ids.len(), themes.len());
        for (id, theme) in ids.iter().zip(themes.iter()) {
            assert_eq!(*id, theme.id());
        }
    }

    #[test]
    fn derived_methods_use_core_slots() {
        let t = CatppuccinMocha;
        assert_eq!(t.block_pass(), t.success());
        assert_eq!(t.block_fail(), t.error());
        assert_eq!(t.block_file_read(), t.text_dim());
        assert_eq!(t.indicator_passed(), t.success());
        assert_eq!(t.indicator_failed(), t.error());
    }

    #[test]
    fn no_color_theme_is_all_reset() {
        let t = NoColor;
        assert_eq!(t.accent(), ratatui::style::Color::Reset);
        assert_eq!(t.text(), ratatui::style::Color::Reset);
        assert_eq!(t.error(), ratatui::style::Color::Reset);
        assert_eq!(t.border(), ratatui::style::Color::Reset);
    }

    #[test]
    fn every_builtin_has_distinct_status_colors() {
        for theme in builtin_themes() {
            assert_ne!(
                theme.success(),
                theme.error(),
                "theme '{}' has same success and error",
                theme.id()
            );
        }
    }

    #[test]
    fn every_builtin_has_distinct_diff_colors() {
        for theme in builtin_themes() {
            assert_ne!(
                theme.diff_added(),
                theme.diff_removed(),
                "theme '{}' has same diff_added and diff_removed",
                theme.id()
            );
        }
    }

    #[test]
    fn terminal_native_uses_named_colors() {
        let t = TerminalNative;
        assert_eq!(t.success(), ratatui::style::Color::Green);
        assert_eq!(t.error(), ratatui::style::Color::Red);
        assert_eq!(t.accent(), ratatui::style::Color::Blue);
    }

    #[test]
    fn tailwind_dark_uses_palette_constants() {
        let t = TailwindDark;
        // Tailwind colors are RGB — just verify they're not Reset or named
        assert_ne!(t.accent(), ratatui::style::Color::Reset);
        assert_ne!(t.success(), ratatui::style::Color::Reset);
    }

    #[test]
    fn resolve_all_known_ids() {
        for id in available_theme_ids() {
            let theme = resolve_theme(id);
            assert_eq!(theme.id(), id, "resolve_theme({id}) returned wrong theme");
        }
    }

    #[test]
    fn custom_theme_implements_trait() {
        let custom = CustomTheme {
            name: "Test".to_owned(),
            id: "test".to_owned(),
            accent: ratatui::style::Color::Magenta,
            accent_dim: ratatui::style::Color::DarkGray,
            text: ratatui::style::Color::White,
            text_dim: ratatui::style::Color::Gray,
            text_bright: ratatui::style::Color::White,
            success: ratatui::style::Color::Green,
            error: ratatui::style::Color::Red,
            warning: ratatui::style::Color::Yellow,
            info: ratatui::style::Color::Cyan,
            diff_added: ratatui::style::Color::Green,
            diff_removed: ratatui::style::Color::Red,
            diff_context: ratatui::style::Color::DarkGray,
            border: ratatui::style::Color::DarkGray,
            surface: ratatui::style::Color::Black,
        };
        let theme: &dyn Theme = &custom;
        assert_eq!(theme.name(), "Test");
        assert_eq!(theme.id(), "test");
        assert_eq!(theme.accent(), ratatui::style::Color::Magenta);
        // Derived methods work
        assert_eq!(theme.block_pass(), theme.success());
    }
}
