//! Chainable themed progress bar builder.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// Chainable progress bar builder.
///
/// ```rust
/// use ratatui_themekit::{ThemeExt, CatppuccinMocha};
///
/// let bar = CatppuccinMocha.bar(75).width(20).build();
/// ```
pub struct ThemedBar {
    percent: u8,
    width: u16,
    filled: Color,
    empty: Color,
}

impl ThemedBar {
    /// Creates a new bar with the given percentage and colors.
    pub(crate) fn new(percent: u8, filled: Color, empty: Color) -> Self {
        Self {
            percent: percent.min(100),
            width: 12,
            filled,
            empty,
        }
    }

    /// Set bar width in characters.
    #[must_use]
    pub fn width(self, w: u16) -> Self {
        Self { width: w, ..self }
    }

    /// Build into a `Line`.
    #[must_use]
    pub fn build(&self) -> Line<'static> {
        let pct = self.percent as usize;
        let w = self.width as usize;
        let filled = w * pct / 100;
        let empty = w - filled;
        Line::from(vec![
            Span::styled("\u{25b0}".repeat(filled), Style::default().fg(self.filled)),
            Span::styled("\u{25b1}".repeat(empty), Style::default().fg(self.empty)),
            Span::styled(
                format!(" {pct}%"),
                Style::default()
                    .fg(self.filled)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_at_zero_is_all_empty() {
        let line = ThemedBar::new(0, Color::Green, Color::DarkGray).build();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains("0%"));
    }

    #[test]
    fn bar_at_100_is_all_filled() {
        let line = ThemedBar::new(100, Color::Green, Color::DarkGray).build();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains("100%"));
    }

    #[test]
    fn bar_at_50_width_10() {
        let line = ThemedBar::new(50, Color::Green, Color::DarkGray)
            .width(10)
            .build();
        let filled: String = line.spans[0].content.to_string();
        assert_eq!(filled.chars().count(), 5);
    }

    #[test]
    fn bar_clamps_above_100() {
        let line = ThemedBar::new(200, Color::Green, Color::DarkGray).build();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains("100%"));
    }

    #[test]
    fn bar_zero_width_no_panic() {
        let line = ThemedBar::new(50, Color::Green, Color::DarkGray)
            .width(0)
            .build();
        assert!(!line.spans.is_empty());
    }
}
