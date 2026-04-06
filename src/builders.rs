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
    fn new(text: impl Into<std::borrow::Cow<'a, str>>, fg: Color) -> Self {
        Self {
            text: text.into(),
            fg,
            bg: None,
            modifiers: Modifier::empty(),
        }
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
