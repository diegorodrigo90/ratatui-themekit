//! # ratatui-themekit
//!
//! Semantic theme system for [ratatui](https://ratatui.rs).
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

pub mod builders;
mod resolve;
mod theme_trait;
mod themes;

// ── Core exports ──���────────────────────────────────────────────

pub use builders::ThemeExt;
pub use builders::{
    GaugeStyles, ListStyles, StateStyles, TabStyles, TableStyles, ThemedBar, ThemedBlock,
    ThemedLine, ThemedSpan, ThemedStatusLine, zebra_rows,
};
pub use theme_trait::Theme;
pub use themes::CustomTheme;
pub use themes::ThemeData;

// ── Resolution ─────────────────────────────────────────────────

pub use resolve::{
    available_theme_ids, builtin_themes, default_theme, no_color_active, resolve_theme,
};

// ── Theme constants (SCREAMING_SNAKE) ──────────────────────────

pub use themes::{
    BUILTIN_THEMES, CATPPUCCIN_MOCHA, DRACULA, GRUVBOX_DARK, NO_COLOR, NORD, ONE_DARK, ROSE_PINE,
    SOLARIZED_DARK, TAILWIND_DARK, TERMINAL_NATIVE, TOKYO_NIGHT,
};

// ── Theme aliases (PascalCase for ergonomic usage) ─────────────

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

// ── Integration tests ─��────────────────────────────────────────

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::*;

    #[test]
    fn all_builtin_themes_have_unique_ids() {
        let ids: Vec<&str> = BUILTIN_THEMES.iter().map(|t| t.id).collect();
        let mut dedup = ids.clone();
        dedup.sort_unstable();
        dedup.dedup();
        assert_eq!(ids.len(), dedup.len());
    }

    #[test]
    fn every_builtin_has_non_empty_name_and_id() {
        for theme in BUILTIN_THEMES {
            assert!(!theme.name.is_empty());
            assert!(!theme.id.is_empty());
            assert!(!theme.id.contains(' '));
        }
    }

    #[test]
    fn every_builtin_has_distinct_status_colors() {
        for theme in BUILTIN_THEMES {
            assert_ne!(theme.success, theme.error, "{}: success == error", theme.id);
        }
    }

    #[test]
    fn every_builtin_has_distinct_diff_colors() {
        for theme in BUILTIN_THEMES {
            assert_ne!(
                theme.diff_added, theme.diff_removed,
                "{}: added == removed",
                theme.id
            );
        }
    }

    #[test]
    fn every_builtin_surface_differs_from_text() {
        for theme in BUILTIN_THEMES {
            assert_ne!(theme.surface, theme.text, "{}: surface == text", theme.id);
        }
    }

    #[test]
    fn derived_methods_use_core_slots() {
        let t = CatppuccinMocha;
        assert_eq!(t.block_pass(), t.success());
        assert_eq!(t.block_fail(), t.error());
        assert_eq!(t.indicator_passed(), t.success());
    }

    #[test]
    fn no_color_is_all_reset() {
        assert_eq!(NoColor.accent, Color::Reset);
        assert_eq!(NoColor.text, Color::Reset);
        assert_eq!(NoColor.error, Color::Reset);
    }

    #[test]
    fn tokyo_night_has_blue_accent() {
        assert_eq!(TokyoNight.accent, Color::Rgb(122, 162, 247));
    }

    #[test]
    fn rose_pine_has_rose_accent() {
        assert_eq!(RosePine.accent, Color::Rgb(235, 188, 186));
    }

    #[test]
    fn theme_data_display() {
        assert_eq!(CatppuccinMocha.to_string(), "Catppuccin Mocha (catppuccin)");
    }

    #[test]
    fn theme_data_equality() {
        assert_eq!(CatppuccinMocha, CATPPUCCIN_MOCHA);
        assert_ne!(CatppuccinMocha, Dracula);
    }

    #[test]
    fn theme_data_copy() {
        let a = CatppuccinMocha;
        let b = a; // Copy
        assert_eq!(a, b);
    }

    #[test]
    fn custom_theme_with_trait() {
        let t = CustomTheme {
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
        let theme: &dyn Theme = &t;
        assert_eq!(theme.accent(), Color::Magenta);
        assert_eq!(theme.block_pass(), theme.success());
    }
}
