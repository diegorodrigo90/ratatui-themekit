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
//! use ratatui::text::Line;
//! use ratatui_themekit::{Theme, ThemeExt, CatppuccinMocha};
//!
//! let t = CatppuccinMocha;
//!
//! let header = Line::from(vec![
//!     t.fg_accent("App v1.0").bold().build(),
//!     t.fg_border(" | ").build(),
//!     t.fg_success("Ready").build(),
//! ]);
//! ```
//!
//! ## 11 Built-in Themes
//!
//! | Theme | ID | Description |
//! |-------|----|-------------|
//! | `CatppuccinMocha` | `catppuccin` | Warm dark with pastels (default) |
//! | `Dracula` | `dracula` | Dark with vivid colors |
//! | `Nord` | `nord` | Arctic blue-gray |
//! | `GruvboxDark` | `gruvbox` | Retro warm dark |
//! | `OneDark` | `one-dark` | Atom's iconic theme |
//! | `SolarizedDark` | `solarized` | Precision dark |
//! | `TailwindDark` | `tailwind` | Tailwind CSS palette |
//! | `TokyoNight` | `tokyo-night` | Vivid blue accents |
//! | `RosePine` | `rose-pine` | Elegant muted rose |
//! | `TerminalNative` | `terminal` | ANSI named colors only |
//! | `NoColor` | `no-color` | `NO_COLOR` compliant |
//!
//! ## `ThemeExt` Builders
//!
//! ```rust
//! use ratatui_themekit::{ThemeExt, CatppuccinMocha};
//!
//! let t = CatppuccinMocha;
//! let title = t.fg_accent("Title").bold().build();
//! let hint = t.fg_dim("press ? for help").build();
//! let bar = t.bar(75).width(20).build();
//! ```
//!
//! ## Resolve by ID
//!
//! ```rust
//! use ratatui_themekit::resolve_theme;
//!
//! let theme = resolve_theme("dracula");
//! assert_eq!(theme.id(), "dracula");
//! ```

pub mod builders;
mod custom;
mod themes;

use ratatui::style::Color;

// Re-export the core data type
pub use themes::ThemeData;

// Re-export all SCREAMING_SNAKE constants
pub use themes::{
    BUILTIN_THEMES, CATPPUCCIN_MOCHA, DRACULA, GRUVBOX_DARK, NO_COLOR, NORD, ONE_DARK, ROSE_PINE,
    SOLARIZED_DARK, TAILWIND_DARK, TERMINAL_NATIVE, TOKYO_NIGHT,
};

// PascalCase aliases — ergonomic: `let t = CatppuccinMocha;`
#[allow(non_upper_case_globals, missing_docs, clippy::wildcard_imports)]
mod aliases {
    use super::themes::*;
    pub const CatppuccinMocha: ThemeData = CATPPUCCIN_MOCHA;
    pub const Dracula: ThemeData = DRACULA;
    pub const Nord: ThemeData = NORD;
    pub const GruvboxDark: ThemeData = GRUVBOX_DARK;
    pub const OneDark: ThemeData = ONE_DARK;
    pub const SolarizedDark: ThemeData = SOLARIZED_DARK;
    pub const TailwindDark: ThemeData = TAILWIND_DARK;
    pub const TokyoNight: ThemeData = TOKYO_NIGHT;
    pub const RosePine: ThemeData = ROSE_PINE;
    pub const TerminalNative: ThemeData = TERMINAL_NATIVE;
    pub const NoColor: ThemeData = NO_COLOR;
}
pub use aliases::*;

// Re-export custom theme (serde-powered user themes)
pub use custom::CustomTheme;

// Re-export builders (the chainable builder utilities)
pub use builders::ThemeExt;

// ── Theme trait ────────────────────────────────────────────────

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
/// # Derived methods (10+)
///
/// `block_*` and `indicator_*` methods derive from core slots.
/// Override them for fine-grained control.
pub trait Theme: Send + Sync {
    /// Human-readable theme name (e.g. `"Catppuccin Mocha"`).
    fn name(&self) -> &str;
    /// Short identifier for config files (e.g. `"catppuccin"`).
    fn id(&self) -> &str;

    // ── Brand ────────────────────────────────────────────────
    /// Primary brand/accent color.
    fn accent(&self) -> Color;
    /// Secondary accent (less prominent highlights).
    fn accent_dim(&self) -> Color;

    // ── Text ─────────────────────────────────────────────────
    /// Default text color.
    fn text(&self) -> Color;
    /// Dimmed/muted text (timestamps, hints, inactive elements).
    fn text_dim(&self) -> Color;
    /// Bright text for emphasis (bold titles, active elements).
    fn text_bright(&self) -> Color;

    // ── Status ───────────────────────────────────────────────
    /// Success / passed / running.
    fn success(&self) -> Color;
    /// Error / failed.
    fn error(&self) -> Color;
    /// Warning / pending / in-progress.
    fn warning(&self) -> Color;
    /// Informational / neutral highlight.
    fn info(&self) -> Color;

    // ── Diff ─────────────────────────────────────────────────
    /// Lines added.
    fn diff_added(&self) -> Color;
    /// Lines removed.
    fn diff_removed(&self) -> Color;
    /// Context/unchanged lines.
    fn diff_context(&self) -> Color;

    // ── Structure ────────────────────────────────────────────
    /// Border/separator color.
    fn border(&self) -> Color;
    /// Background highlight for focused/selected elements.
    fn surface(&self) -> Color;

    // ── Derived defaults (override for fine control) ─────────
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

// ── Resolution helpers ─────────────────────────────────────────

/// Resolves a theme by its config ID string.
///
/// Falls back to Catppuccin Mocha for unknown IDs. If [`no_color_active()`]
/// is true, always returns `NoColor`.
#[must_use]
pub fn resolve_theme(id: &str) -> Box<dyn Theme> {
    if no_color_active() {
        return Box::new(NO_COLOR);
    }
    // Look up in the registry — single source of truth
    for theme in BUILTIN_THEMES {
        if theme.id == id {
            return Box::new(theme.clone());
        }
    }
    // Fallback
    Box::new(CATPPUCCIN_MOCHA)
}

/// Returns all built-in theme instances (excludes `NoColor`).
#[must_use]
pub fn builtin_themes() -> Vec<Box<dyn Theme>> {
    BUILTIN_THEMES
        .iter()
        .map(|t| Box::new(t.clone()) as Box<dyn Theme>)
        .collect()
}

/// Returns all built-in theme IDs (excludes `"no-color"`).
#[must_use]
pub fn available_theme_ids() -> Vec<&'static str> {
    BUILTIN_THEMES.iter().map(|t| t.id).collect()
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
    fn resolve_empty_string_falls_back() {
        let theme = resolve_theme("");
        assert_eq!(theme.id(), "catppuccin");
    }

    #[test]
    fn all_builtin_themes_have_unique_ids() {
        let ids: Vec<&str> = BUILTIN_THEMES.iter().map(|t| t.id).collect();
        let mut dedup = ids.clone();
        dedup.sort_unstable();
        dedup.dedup();
        assert_eq!(ids.len(), dedup.len(), "duplicate theme IDs found");
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
    fn resolve_all_known_ids() {
        for id in available_theme_ids() {
            let theme = resolve_theme(id);
            assert_eq!(theme.id(), id, "resolve_theme({id}) returned wrong theme");
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
        assert_eq!(t.accent(), Color::Reset);
        assert_eq!(t.text(), Color::Reset);
        assert_eq!(t.error(), Color::Reset);
        assert_eq!(t.border(), Color::Reset);
    }

    #[test]
    fn every_builtin_has_non_empty_name_and_id() {
        for theme in BUILTIN_THEMES {
            assert!(!theme.name.is_empty());
            assert!(!theme.id.is_empty());
            assert!(!theme.id.contains(' '), "id '{}' has spaces", theme.id);
        }
    }

    #[test]
    fn every_builtin_has_distinct_status_colors() {
        for theme in BUILTIN_THEMES {
            assert_ne!(
                theme.success, theme.error,
                "theme '{}': success == error",
                theme.id
            );
        }
    }

    #[test]
    fn every_builtin_has_distinct_diff_colors() {
        for theme in BUILTIN_THEMES {
            assert_ne!(
                theme.diff_added, theme.diff_removed,
                "theme '{}': added == removed",
                theme.id
            );
        }
    }

    #[test]
    fn every_builtin_surface_differs_from_text() {
        for theme in BUILTIN_THEMES {
            assert_ne!(
                theme.surface, theme.text,
                "theme '{}': surface == text",
                theme.id
            );
        }
    }

    #[test]
    fn terminal_native_uses_named_colors() {
        assert_eq!(TerminalNative.success, Color::Green);
        assert_eq!(TerminalNative.error, Color::Red);
        assert_eq!(TerminalNative.accent, Color::Blue);
    }

    #[test]
    fn tokyo_night_has_blue_accent() {
        assert_eq!(TokyoNight.accent, Color::Rgb(122, 162, 247));
        assert_ne!(TokyoNight.accent, TokyoNight.error);
    }

    #[test]
    fn rose_pine_has_rose_accent() {
        assert_eq!(RosePine.accent, Color::Rgb(235, 188, 186));
        assert_ne!(RosePine.accent, RosePine.error);
    }

    #[test]
    fn custom_theme_implements_trait() {
        let custom = CustomTheme {
            name: "Test".to_owned(),
            id: "test".to_owned(),
            accent: Color::Magenta,
            accent_dim: Color::DarkGray,
            text: Color::White,
            text_dim: Color::Gray,
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
        let theme: &dyn Theme = &custom;
        assert_eq!(theme.name(), "Test");
        assert_eq!(theme.accent(), Color::Magenta);
        assert_eq!(theme.block_pass(), theme.success());
    }

    #[test]
    fn builtin_count_is_correct() {
        assert_eq!(BUILTIN_THEMES.len(), 10, "expected 10 built-in themes");
        assert_eq!(available_theme_ids().len(), 10);
    }
}
