//! Themed line compositor — build multi-span `Line` without boilerplate.

use ratatui::style::Color;
use ratatui::text::{Line, Span};

use super::ThemedSpan;

/// Chainable line builder that collects themed spans.
///
/// Eliminates the `vec![]` + `.build()` boilerplate when composing
/// multi-colored lines.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let line = CatppuccinMocha.line()
///     .accent("Dashboard")
///     .dim(" | ")
///     .success("3 passed")
///     .dim(", ")
///     .error("1 failed")
///     .build();
/// ```
pub struct ThemedLine<'a> {
    spans: Vec<ThemedSpan<'a>>,
    raw_spans: Vec<(usize, Span<'a>)>,
    accent_color: Color,
    dim_color: Color,
    bright_color: Color,
    text_color: Color,
    success_color: Color,
    error_color: Color,
    warning_color: Color,
    info_color: Color,
    border_color: Color,
}

impl<'a> ThemedLine<'a> {
    /// Creates a new line builder from a theme.
    pub(crate) fn new(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            spans: Vec::new(),
            raw_spans: Vec::new(),
            accent_color: theme.accent(),
            dim_color: theme.text_dim(),
            bright_color: theme.text_bright(),
            text_color: theme.text(),
            success_color: theme.success(),
            error_color: theme.error(),
            warning_color: theme.warning(),
            info_color: theme.info(),
            border_color: theme.border(),
        }
    }

    /// Append a span with accent color.
    #[must_use]
    pub fn accent(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.accent_color));
        self
    }

    /// Append a span with dim color.
    #[must_use]
    pub fn dim(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.dim_color));
        self
    }

    /// Append a span with bright color.
    #[must_use]
    pub fn bright(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.bright_color));
        self
    }

    /// Append a span with default text color.
    #[must_use]
    pub fn text(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.text_color));
        self
    }

    /// Append a span with success color.
    #[must_use]
    pub fn success(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.success_color));
        self
    }

    /// Append a span with error color.
    #[must_use]
    pub fn error(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.error_color));
        self
    }

    /// Append a span with warning color.
    #[must_use]
    pub fn warning(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.warning_color));
        self
    }

    /// Append a span with info color.
    #[must_use]
    pub fn info(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.info_color));
        self
    }

    /// Append a span with border color.
    #[must_use]
    pub fn border(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans.push(ThemedSpan::new(text, self.border_color));
        self
    }

    /// Append a bold accent span.
    #[must_use]
    pub fn accent_bold(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans
            .push(ThemedSpan::new(text, self.accent_color).bold());
        self
    }

    /// Append a bold success span.
    #[must_use]
    pub fn success_bold(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans
            .push(ThemedSpan::new(text, self.success_color).bold());
        self
    }

    /// Append a bold error span.
    #[must_use]
    pub fn error_bold(mut self, text: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.spans
            .push(ThemedSpan::new(text, self.error_color).bold());
        self
    }

    /// Append a span with a custom color.
    ///
    /// Prefer semantic methods (`accent()`, `success()`, etc.) for themed UI.
    /// Use this for computed or data-driven colors only.
    #[must_use]
    pub fn colored(mut self, text: impl Into<std::borrow::Cow<'a, str>>, color: Color) -> Self {
        self.spans.push(ThemedSpan::new(text, color));
        self
    }

    /// Append a pre-built ratatui `Span` (preserves all styling).
    #[must_use]
    pub fn span(mut self, span: Span<'a>) -> Self {
        self.raw_spans.push((self.spans.len(), span));
        self
    }

    /// Build into a ratatui `Line`.
    #[must_use]
    pub fn build(self) -> Line<'a> {
        let mut built: Vec<Span<'a>> = self.spans.into_iter().map(ThemedSpan::build).collect();

        // Insert raw spans at their recorded positions
        for (pos, span) in self.raw_spans.into_iter().rev() {
            let idx = pos.min(built.len());
            built.insert(idx, span);
        }

        Line::from(built)
    }
}

impl<'a> From<ThemedLine<'a>> for Line<'a> {
    fn from(tl: ThemedLine<'a>) -> Self {
        tl.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CatppuccinMocha;

    fn make_line() -> ThemedLine<'static> {
        ThemedLine::new(&CatppuccinMocha)
    }

    #[test]
    fn empty_line_produces_empty() {
        let line = make_line().build();
        assert!(line.spans.is_empty());
    }

    #[test]
    fn single_accent_span() {
        let line = make_line().accent("hello").build();
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content.as_ref(), "hello");
    }

    #[test]
    fn multi_span_composition() {
        let line = make_line().accent("A").dim(" | ").success("B").build();
        assert_eq!(line.spans.len(), 3);
    }

    #[test]
    fn all_semantic_colors() {
        let line = make_line()
            .accent("a")
            .dim("b")
            .bright("c")
            .text("d")
            .success("e")
            .error("f")
            .warning("g")
            .info("h")
            .border("i")
            .build();
        assert_eq!(line.spans.len(), 9);
    }

    #[test]
    fn bold_variants() {
        let line = make_line()
            .accent_bold("A")
            .success_bold("B")
            .error_bold("C")
            .build();
        assert_eq!(line.spans.len(), 3);
    }

    #[test]
    fn colored_span() {
        let line = make_line().colored("custom", Color::Rgb(255, 0, 0)).build();
        assert_eq!(line.spans.len(), 1);
    }

    #[test]
    fn raw_span_preserves_style() {
        use ratatui::style::{Modifier, Style};
        let styled = Span::styled(
            "raw",
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
        let line = make_line()
            .accent("before")
            .span(styled)
            .dim("after")
            .build();
        assert_eq!(line.spans.len(), 3);
        assert_eq!(line.spans[1].style.fg, Some(Color::Magenta));
        assert_eq!(line.spans[1].style.bg, Some(Color::Black));
        assert!(line.spans[1].style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn into_line_conversion() {
        let line: Line<'_> = make_line().accent("test").into();
        assert_eq!(line.spans.len(), 1);
    }

    #[test]
    fn owned_strings_accepted() {
        let text = format!("dynamic {}", 42);
        let line = make_line().accent(text).build();
        assert!(line.spans[0].content.contains("42"));
    }
}
