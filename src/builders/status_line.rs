//! Themed status line builder — key-value pairs for status bars.

use ratatui::style::Color;
use ratatui::text::Line;

use super::ThemedSpan;

/// Builds a status bar `Line` from key-value pairs.
///
/// Keys are dim, values are bright, separators are border-colored.
/// Common pattern for footer bars, headers, and metadata lines.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let status = CatppuccinMocha.status_line()
///     .kv("Mode", "Normal")
///     .kv("File", "main.rs")
///     .kv("Ln", "42")
///     .build();
/// ```
pub struct ThemedStatusLine<'a> {
    pairs: Vec<(&'a str, std::borrow::Cow<'a, str>, Option<Color>)>,
    separator: &'a str,
    key_color: Color,
    value_color: Color,
    separator_color: Color,
}

impl<'a> ThemedStatusLine<'a> {
    /// Creates a new status line builder from a theme.
    pub(crate) fn new(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            pairs: Vec::new(),
            separator: " \u{00b7} ",
            key_color: theme.text_dim(),
            value_color: theme.text_bright(),
            separator_color: theme.border(),
        }
    }

    /// Add a key-value pair.
    #[must_use]
    pub fn kv(mut self, key: &'a str, value: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.pairs.push((key, value.into(), None));
        self
    }

    /// Add a key-value pair with a custom value color.
    ///
    /// Prefer theme colors (`t.success()`, `t.error()`, etc.).
    /// Use this for computed or data-driven colors only.
    #[must_use]
    pub fn kv_colored(
        mut self,
        key: &'a str,
        value: impl Into<std::borrow::Cow<'a, str>>,
        color: Color,
    ) -> Self {
        self.pairs.push((key, value.into(), Some(color)));
        self
    }

    /// Set the separator between all pairs (default: ` · `).
    ///
    /// This is a global setting — call it anywhere in the chain.
    #[must_use]
    pub fn separator(mut self, sep: &'a str) -> Self {
        self.separator = sep;
        self
    }

    /// Build into a ratatui `Line`.
    #[must_use]
    pub fn build(self) -> Line<'a> {
        let mut spans = Vec::new();

        for (i, (key, value, color)) in self.pairs.into_iter().enumerate() {
            if i > 0 {
                spans.push(
                    ThemedSpan::new(
                        std::borrow::Cow::Borrowed(self.separator),
                        self.separator_color,
                    )
                    .build(),
                );
            }

            spans.push(
                ThemedSpan::new(std::borrow::Cow::Borrowed(key), self.key_color)
                    .bold()
                    .build(),
            );
            spans.push(ThemedSpan::new(std::borrow::Cow::Borrowed(": "), self.key_color).build());

            let val_color = color.unwrap_or(self.value_color);
            spans.push(ThemedSpan::new(value, val_color).build());
        }

        Line::from(spans)
    }
}

impl<'a> From<ThemedStatusLine<'a>> for Line<'a> {
    fn from(sl: ThemedStatusLine<'a>) -> Self {
        sl.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CatppuccinMocha, Theme};

    fn make_status() -> ThemedStatusLine<'static> {
        ThemedStatusLine::new(&CatppuccinMocha)
    }

    #[test]
    fn empty_status_line() {
        let line = make_status().build();
        assert!(line.spans.is_empty());
    }

    #[test]
    fn single_pair() {
        let line = make_status().kv("Mode", "Normal").build();
        assert_eq!(line.spans.len(), 3);
    }

    #[test]
    fn two_pairs_have_separator() {
        let line = make_status().kv("A", "1").kv("B", "2").build();
        assert_eq!(line.spans.len(), 7);
    }

    #[test]
    fn custom_separator() {
        let line = make_status()
            .separator(" | ")
            .kv("A", "1")
            .kv("B", "2")
            .build();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains(" | "));
    }

    #[test]
    fn colored_value_uses_theme_color() {
        let color = CatppuccinMocha.success();
        let line = make_status().kv_colored("Status", "OK", color).build();
        assert_eq!(line.spans.len(), 3);
        assert_eq!(line.spans[2].style.fg, Some(color));
    }

    #[test]
    fn owned_value_accepted() {
        let val = format!("v{}", 42);
        let line = make_status().kv("Version", val).build();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains("v42"));
    }

    #[test]
    fn into_line_conversion() {
        let line: Line<'_> = make_status().kv("K", "V").into();
        assert_eq!(line.spans.len(), 3);
    }
}
