//! Theme resolution — find themes by ID, list available themes.

use crate::Theme;
use crate::themes::{BUILTIN_THEMES, CATPPUCCIN_MOCHA, NO_COLOR, ThemeData};

/// Resolves a theme by its config ID string.
///
/// Falls back to Catppuccin Mocha for unknown IDs. If [`no_color_active()`]
/// is true, always returns `NoColor`.
///
/// ```rust
/// use ratatui_themekit::resolve_theme;
///
/// let theme = resolve_theme("dracula");
/// assert_eq!(theme.id(), "dracula");
/// ```
#[must_use]
pub fn resolve_theme(id: &str) -> Box<dyn Theme> {
    if no_color_active() {
        return Box::new(NO_COLOR);
    }
    for theme in BUILTIN_THEMES {
        if theme.id == id {
            return Box::new(*theme);
        }
    }
    Box::new(CATPPUCCIN_MOCHA)
}

/// Returns all built-in theme instances (excludes `NoColor`).
#[must_use]
pub fn builtin_themes() -> Vec<Box<dyn Theme>> {
    BUILTIN_THEMES
        .iter()
        .map(|t| Box::new(*t) as Box<dyn Theme>)
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

/// Returns the default theme (Catppuccin Mocha).
#[must_use]
pub fn default_theme() -> ThemeData {
    CATPPUCCIN_MOCHA
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_known_theme() {
        let theme = resolve_theme("dracula");
        assert_eq!(theme.id(), "dracula");
    }

    #[test]
    fn resolve_unknown_falls_back() {
        assert_eq!(resolve_theme("nonexistent").id(), "catppuccin");
    }

    #[test]
    fn resolve_empty_falls_back() {
        assert_eq!(resolve_theme("").id(), "catppuccin");
    }

    #[test]
    fn resolve_all_known_ids() {
        for id in available_theme_ids() {
            let theme = resolve_theme(id);
            assert_eq!(theme.id(), id);
        }
    }

    #[test]
    fn builtin_count() {
        assert_eq!(BUILTIN_THEMES.len(), 10);
        assert_eq!(available_theme_ids().len(), 10);
        assert_eq!(builtin_themes().len(), 10);
    }

    #[test]
    fn ids_match_builtins() {
        let ids = available_theme_ids();
        let themes = builtin_themes();
        for (id, theme) in ids.iter().zip(themes.iter()) {
            assert_eq!(*id, theme.id());
        }
    }

    #[test]
    fn default_theme_is_catppuccin() {
        assert_eq!(default_theme().id, "catppuccin");
    }
}
