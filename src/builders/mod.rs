//! Theme-aware builders for ratatui primitives.
//!
//! Chainable builders that produce themed `Span`, `Line`, and progress bars.
//!
//! ```rust
//! use ratatui_themekit::{CatppuccinMocha, ThemeExt};
//!
//! let t = CatppuccinMocha;
//!
//! let title = t.fg_accent("Ralph Engine").bold();
//! let ok = t.fg_success("Passing").bold();
//! let hint = t.fg_dim("press ? for help");
//!
//! let line = ratatui::text::Line::from(vec![
//!     title.build(),
//!     t.fg_dim(" | ").build(),
//!     ok.build(),
//! ]);
//! ```

mod bar;
mod span;

use ratatui::style::{Color, Style};
use ratatui::text::Line;

use crate::Theme;

pub use bar::ThemedBar;
pub use span::ThemedSpan;

/// Extension trait — adds chainable builder methods to any `Theme`.
///
/// Import `ThemeExt` to get `fg_accent()`, `fg_success()`, `fg_dim()`,
/// `bar()`, `separator_line()` on any `&dyn Theme` or concrete theme.
pub trait ThemeExt: Theme {
    /// Span with accent color.
    fn fg_accent<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.accent())
    }
    /// Span with dim text.
    fn fg_dim<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.text_dim())
    }
    /// Span with bright text.
    fn fg_bright<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.text_bright())
    }
    /// Span with default text.
    fn fg_text<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.text())
    }
    /// Span with success color.
    fn fg_success<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.success())
    }
    /// Span with error color.
    fn fg_error<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.error())
    }
    /// Span with warning color.
    fn fg_warning<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.warning())
    }
    /// Span with info color.
    fn fg_info<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.info())
    }
    /// Span with diff-added color.
    fn fg_added<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.diff_added())
    }
    /// Span with diff-removed color.
    fn fg_removed<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.diff_removed())
    }
    /// Span with border color.
    fn fg_border<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.border())
    }

    /// Themed progress bar builder.
    fn bar(&self, percent: u8) -> ThemedBar {
        ThemedBar::new(percent, self.accent(), self.border())
    }

    /// Themed separator line.
    fn separator_line(&self, width: u16) -> Line<'static> {
        Line::styled(
            " \u{00b7} ".repeat((width / 3) as usize),
            Style::default().fg(self.border()),
        )
    }

    /// Badge-style span: text with colored background.
    fn badge<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>, bg: Color) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.surface()).on(bg).bold()
    }

    // ── Style helpers (for widget APIs that take Style) ──────

    /// Style with accent foreground.
    fn style_accent(&self) -> Style {
        Style::default().fg(self.accent())
    }
    /// Style with border foreground.
    fn style_border(&self) -> Style {
        Style::default().fg(self.border())
    }
    /// Style with error foreground.
    fn style_error(&self) -> Style {
        Style::default().fg(self.error())
    }
    /// Style with warning foreground.
    fn style_warning(&self) -> Style {
        Style::default().fg(self.warning())
    }
    /// Style with success foreground.
    fn style_success(&self) -> Style {
        Style::default().fg(self.success())
    }
    /// Style with bright text foreground.
    fn style_bright(&self) -> Style {
        Style::default().fg(self.text_bright())
    }
    /// Style with dim text foreground.
    fn style_dim(&self) -> Style {
        Style::default().fg(self.text_dim())
    }
}

// Blanket impl — every Theme gets ThemeExt
impl<T: Theme + ?Sized> ThemeExt for T {}

/// Creates a `Style` with the given foreground color.
///
/// Convenience for widget APIs that accept `Style`.
///
/// ```rust
/// use ratatui::style::Color;
/// use ratatui_themekit::builders::style_fg;
///
/// let s = style_fg(Color::Green);
/// ```
#[must_use]
pub fn style_fg(color: Color) -> Style {
    Style::default().fg(color)
}

#[cfg(test)]
mod tests {
    use ratatui::style::Modifier;

    use super::*;
    use crate::{CATPPUCCIN_MOCHA, CatppuccinMocha, Dracula, NoColor, Theme};

    // ── fg_* builders ───────────────────────────────────────────

    #[test]
    fn fg_accent_uses_theme_accent() {
        let span = CatppuccinMocha.fg_accent("hello").build();
        assert_eq!(span.style.fg, Some(CATPPUCCIN_MOCHA.accent));
    }

    #[test]
    fn fg_dim_uses_theme_text_dim() {
        let span = CatppuccinMocha.fg_dim("muted").build();
        assert_eq!(span.style.fg, Some(CATPPUCCIN_MOCHA.text_dim));
    }

    #[test]
    fn fg_success_uses_theme_success() {
        let span = CatppuccinMocha.fg_success("ok").build();
        assert_eq!(span.style.fg, Some(CATPPUCCIN_MOCHA.success));
    }

    #[test]
    fn fg_error_uses_theme_error() {
        let span = CatppuccinMocha.fg_error("fail").build();
        assert_eq!(span.style.fg, Some(CATPPUCCIN_MOCHA.error));
    }

    // ── Modifiers ───────────────────────────────────────────────

    #[test]
    fn bold_adds_bold_modifier() {
        let span = CatppuccinMocha.fg_accent("title").bold().build();
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn chained_modifiers_combine() {
        let span = CatppuccinMocha.fg_accent("both").bold().italic().build();
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
        assert!(span.style.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn on_sets_background() {
        let span = CatppuccinMocha.fg_accent("badge").on(Color::Red).build();
        assert_eq!(span.style.bg, Some(Color::Red));
    }

    // ── Badge / separator ───────────────────────────────────────

    #[test]
    fn badge_has_background_and_bold() {
        let span = CatppuccinMocha.badge(" RUN ", Color::Green).build();
        assert_eq!(span.style.bg, Some(Color::Green));
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn separator_produces_content() {
        let line = CatppuccinMocha.separator_line(30);
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains('\u{00b7}'));
    }

    // ── Style helpers ───────────────────────────────────────────

    #[test]
    fn style_helpers_produce_correct_colors() {
        let t = CatppuccinMocha;
        assert_eq!(t.style_accent().fg, Some(t.accent));
        assert_eq!(t.style_border().fg, Some(t.border));
        assert_eq!(t.style_error().fg, Some(t.error));
    }

    #[test]
    fn style_fg_with_dynamic_color() {
        let s = style_fg(Color::Rgb(1, 2, 3));
        assert_eq!(s.fg, Some(Color::Rgb(1, 2, 3)));
    }

    // ── dyn Theme + NoColor ─────────────────────────────────────

    #[test]
    fn builders_work_on_dyn_theme() {
        let t: &dyn Theme = &Dracula;
        let span = t.fg_accent("test").bold().build();
        assert_eq!(span.style.fg, Some(t.accent()));
    }

    #[test]
    fn no_color_builders_produce_reset() {
        let span = NoColor.fg_accent("text").build();
        assert_eq!(span.style.fg, Some(Color::Reset));
    }

    // ── Edge cases ──────────────────────────────────────────────

    #[test]
    fn empty_string_produces_empty_span() {
        let span = CatppuccinMocha.fg_accent("").build();
        assert!(span.content.is_empty());
    }

    #[test]
    fn owned_string_accepted() {
        let text = format!("dynamic {}", 42);
        let span = CatppuccinMocha.fg_accent(text).build();
        assert!(span.content.contains("42"));
    }

    #[test]
    fn separator_zero_width_no_panic() {
        let line = CatppuccinMocha.separator_line(0);
        assert!(line.spans.len() <= 1);
    }

    #[test]
    fn with_color_reset() {
        let span = ThemedSpan::with_color("text", Color::Reset).build();
        assert_eq!(span.style.fg, Some(Color::Reset));
    }

    #[test]
    fn into_span_conversion() {
        let themed = CatppuccinMocha.fg_success("ok").bold();
        let span: ratatui::text::Span<'_> = themed.into();
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }
}
