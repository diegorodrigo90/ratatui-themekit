//! Theme-aware builders for ratatui primitives.
//!
//! Like Tailwind CSS utilities but for terminal UI. Chainable builders
//! that produce themed `Span`, `Line`, and progress bars.
//!
//! ```rust
//! use ratatui_themekit::{CatppuccinMocha, ThemeExt};
//!
//! let t = CatppuccinMocha;
//!
//! // Themed spans — color from theme, chainable modifiers
//! let title = t.fg_accent("Ralph Engine").bold();
//! let ok = t.fg_success("● Passing").bold();
//! let err = t.fg_error("✗ Failed");
//! let hint = t.fg_dim("press ? for help");
//!
//! // Use in ratatui Lines directly
//! let line = ratatui::text::Line::from(vec![
//!     title.build(),
//!     t.fg_dim(" │ ").build(),
//!     ok.build(),
//! ]);
//! ```

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::Theme;

/// Extension trait — adds Tailwind-like builder methods to any `Theme`.
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
        ThemedBar {
            percent: percent.min(100),
            width: 12,
            filled: self.accent(),
            empty: self.border(),
        }
    }

    /// Themed separator line.
    fn separator_line(&self, width: u16) -> Line<'static> {
        Line::styled(
            " · ".repeat((width / 3) as usize),
            Style::default().fg(self.border()),
        )
    }

    /// Badge-style span: text with colored background.
    fn badge<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>, bg: Color) -> ThemedSpan<'a> {
        ThemedSpan::new(text, self.surface()).on(bg).bold()
    }

    // ── Style helpers (for widget APIs that take Style, not Span) ──

    /// Style with accent foreground. Use for `border_style`, `title_style`, etc.
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

/// Creates a `Style` with the given foreground color.
///
/// Convenience function for widget APIs (`border_style`, `title_style`)
/// that accept `Style` instead of `Span`.
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

// Blanket impl — every Theme gets ThemeExt
impl<T: Theme + ?Sized> ThemeExt for T {}

// ── Themed Span ─────────────────────────────────────────────────

/// Chainable span builder. Color pre-set from theme; add modifiers.
///
/// Converts to `Span` via `.build()` or `Into<Span>`.
pub struct ThemedSpan<'a> {
    text: std::borrow::Cow<'a, str>,
    fg: Color,
    bg: Option<Color>,
    modifiers: Modifier,
}

impl<'a> ThemedSpan<'a> {
    /// Creates a new themed span with the given text and foreground color.
    ///
    /// Use this for dynamic colors that aren't covered by `ThemeExt` methods.
    /// For theme-semantic colors, prefer `t.fg_accent()`, `t.fg_dim()`, etc.
    ///
    /// ```rust
    /// use ratatui::style::Color;
    /// use ratatui_themekit::builders::ThemedSpan;
    ///
    /// let span = ThemedSpan::with_color("text", Color::Rgb(100, 200, 50)).bold().build();
    /// ```
    #[must_use]
    pub fn with_color(text: impl Into<std::borrow::Cow<'a, str>>, fg: Color) -> Self {
        Self {
            text: text.into(),
            fg,
            bg: None,
            modifiers: Modifier::empty(),
        }
    }

    /// Internal constructor used by ThemeExt methods.
    pub(crate) fn new(text: impl Into<std::borrow::Cow<'a, str>>, fg: Color) -> Self {
        Self::with_color(text, fg)
    }

    /// Add bold modifier.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.modifiers |= Modifier::BOLD;
        self
    }

    /// Add italic modifier.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.modifiers |= Modifier::ITALIC;
        self
    }

    /// Add dim modifier.
    #[must_use]
    pub fn dimmed(mut self) -> Self {
        self.modifiers |= Modifier::DIM;
        self
    }

    /// Set background color.
    #[must_use]
    pub fn on(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Build into a ratatui `Span`.
    #[must_use]
    pub fn build(self) -> Span<'a> {
        let mut style = Style::default().fg(self.fg);
        if let Some(bg) = self.bg {
            style = style.bg(bg);
        }
        if !self.modifiers.is_empty() {
            style = style.add_modifier(self.modifiers);
        }
        Span::styled(self.text, style)
    }
}

impl<'a> From<ThemedSpan<'a>> for Span<'a> {
    fn from(ts: ThemedSpan<'a>) -> Self {
        ts.build()
    }
}

// ── Themed Bar ──────────────────────────────────────────────────

/// Chainable progress bar builder.
pub struct ThemedBar {
    percent: u8,
    width: u16,
    filled: Color,
    empty: Color,
}

impl ThemedBar {
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
            Span::styled("▰".repeat(filled), Style::default().fg(self.filled)),
            Span::styled("▱".repeat(empty), Style::default().fg(self.empty)),
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
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::{CatppuccinMocha, Dracula, NoColor, Theme};

    // ── fg_* builders produce correct colors ────────────────────

    #[test]
    fn fg_accent_uses_theme_accent() {
        let t = CatppuccinMocha;
        let span = t.fg_accent("hello").build();
        assert_eq!(span.style.fg, Some(t.accent()));
    }

    #[test]
    fn fg_dim_uses_theme_text_dim() {
        let t = CatppuccinMocha;
        let span = t.fg_dim("muted").build();
        assert_eq!(span.style.fg, Some(t.text_dim()));
    }

    #[test]
    fn fg_bright_uses_theme_text_bright() {
        let t = CatppuccinMocha;
        let span = t.fg_bright("bold").build();
        assert_eq!(span.style.fg, Some(t.text_bright()));
    }

    #[test]
    fn fg_text_uses_theme_text() {
        let t = CatppuccinMocha;
        let span = t.fg_text("normal").build();
        assert_eq!(span.style.fg, Some(t.text()));
    }

    #[test]
    fn fg_success_uses_theme_success() {
        let t = CatppuccinMocha;
        let span = t.fg_success("ok").build();
        assert_eq!(span.style.fg, Some(t.success()));
    }

    #[test]
    fn fg_error_uses_theme_error() {
        let t = CatppuccinMocha;
        let span = t.fg_error("fail").build();
        assert_eq!(span.style.fg, Some(t.error()));
    }

    #[test]
    fn fg_warning_uses_theme_warning() {
        let t = CatppuccinMocha;
        let span = t.fg_warning("warn").build();
        assert_eq!(span.style.fg, Some(t.warning()));
    }

    #[test]
    fn fg_info_uses_theme_info() {
        let t = CatppuccinMocha;
        let span = t.fg_info("info").build();
        assert_eq!(span.style.fg, Some(t.info()));
    }

    #[test]
    fn fg_added_uses_theme_diff_added() {
        let t = CatppuccinMocha;
        let span = t.fg_added("+line").build();
        assert_eq!(span.style.fg, Some(t.diff_added()));
    }

    #[test]
    fn fg_removed_uses_theme_diff_removed() {
        let t = CatppuccinMocha;
        let span = t.fg_removed("-line").build();
        assert_eq!(span.style.fg, Some(t.diff_removed()));
    }

    #[test]
    fn fg_border_uses_theme_border() {
        let t = CatppuccinMocha;
        let span = t.fg_border("---").build();
        assert_eq!(span.style.fg, Some(t.border()));
    }

    // ── Modifier chaining ───────────────────────────────────────

    #[test]
    fn bold_adds_bold_modifier() {
        let t = CatppuccinMocha;
        let span = t.fg_accent("title").bold().build();
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn italic_adds_italic_modifier() {
        let t = CatppuccinMocha;
        let span = t.fg_dim("hint").italic().build();
        assert!(span.style.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn dimmed_adds_dim_modifier() {
        let t = CatppuccinMocha;
        let span = t.fg_text("faded").dimmed().build();
        assert!(span.style.add_modifier.contains(Modifier::DIM));
    }

    #[test]
    fn chained_modifiers_combine() {
        let t = CatppuccinMocha;
        let span = t.fg_accent("both").bold().italic().build();
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
        assert!(span.style.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn on_sets_background() {
        let t = CatppuccinMocha;
        let span = t.fg_accent("badge").on(Color::Red).build();
        assert_eq!(span.style.bg, Some(Color::Red));
    }

    // ── Into<Span> conversion ───────────────────────────────────

    #[test]
    fn themed_span_converts_to_span() {
        let t = CatppuccinMocha;
        let themed = t.fg_success("ok").bold();
        let span: Span<'_> = themed.into();
        assert_eq!(span.style.fg, Some(t.success()));
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    // ── Badge builder ───────────────────────────────────────────

    #[test]
    fn badge_has_background_and_bold() {
        let t = CatppuccinMocha;
        let span = t.badge(" RUNNING ", Color::Green).build();
        assert_eq!(span.style.bg, Some(Color::Green));
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    // ── Bar builder ─────────────────────────────────────────────

    #[test]
    fn bar_at_zero_is_all_empty() {
        let t = CatppuccinMocha;
        let line = t.bar(0).build();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.starts_with("▱"));
        assert!(text.contains("0%"));
    }

    #[test]
    fn bar_at_100_is_all_filled() {
        let t = CatppuccinMocha;
        let line = t.bar(100).build();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.starts_with("▰"));
        assert!(text.contains("100%"));
    }

    #[test]
    fn bar_at_50_is_half() {
        let t = CatppuccinMocha;
        let line = t.bar(50).width(10).build();
        let filled: String = line.spans[0].content.to_string();
        let empty: String = line.spans[1].content.to_string();
        assert_eq!(filled.chars().count(), 5);
        assert_eq!(empty.chars().count(), 5);
    }

    #[test]
    fn bar_clamps_above_100() {
        let t = CatppuccinMocha;
        let line = t.bar(200).build();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains("100%"));
    }

    #[test]
    fn bar_width_changes_length() {
        let t = CatppuccinMocha;
        let line = t.bar(50).width(20).build();
        let filled: String = line.spans[0].content.to_string();
        assert_eq!(filled.chars().count(), 10);
    }

    // ── Separator ───────────────────────────────────────────────

    #[test]
    fn separator_line_produces_content() {
        let t = CatppuccinMocha;
        let line = t.separator_line(30);
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains("·"), "separator should contain dots");
    }

    // ── Works with dyn Theme ────────────────────────────────────

    #[test]
    fn builders_work_on_dyn_theme() {
        let t: &dyn Theme = &Dracula;
        let span = t.fg_accent("test").bold().build();
        assert_eq!(span.style.fg, Some(t.accent()));
    }

    // ── NoColor produces Reset ──────────────────────────────────

    #[test]
    fn no_color_builders_produce_reset() {
        let t = NoColor;
        let span = t.fg_accent("text").build();
        assert_eq!(span.style.fg, Some(Color::Reset));
    }

    // ── Owned strings work ──────────────────────────────────────

    #[test]
    fn with_color_creates_span_with_dynamic_color() {
        let span = ThemedSpan::with_color("dynamic", Color::Rgb(100, 200, 50))
            .bold()
            .build();
        assert_eq!(span.style.fg, Some(Color::Rgb(100, 200, 50)));
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn owned_string_accepted() {
        let t = CatppuccinMocha;
        let text = format!("dynamic {}", 42);
        let span = t.fg_accent(text).build();
        assert!(span.content.contains("42"));
    }
}
