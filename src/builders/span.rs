//! Chainable themed span builder.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

/// Chainable span builder. Color pre-set from theme; add modifiers.
///
/// Converts to `Span` via `.build()` or `Into<Span>`.
///
/// ```rust
/// use ratatui_themekit::{ThemeExt, CatppuccinMocha};
///
/// let span = CatppuccinMocha.fg_accent("title").bold().build();
/// ```
pub struct ThemedSpan<'a> {
    text: std::borrow::Cow<'a, str>,
    fg: Color,
    bg: Option<Color>,
    modifiers: Modifier,
}

impl<'a> ThemedSpan<'a> {
    /// Creates a themed span with a dynamic color.
    ///
    /// Use for colors not from a named theme slot (e.g., computed at runtime).
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

    /// Internal constructor used by `ThemeExt` methods.
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
