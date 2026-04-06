//! Theme-aware builders for ratatui primitives.
//!
//! Chainable builders that produce themed `Span`, `Line`, `Block`,
//! and widget style bundles — zero boilerplate, semantic colors.
//!
//! ```rust
//! use ratatui_themekit::{CatppuccinMocha, ThemeExt};
//!
//! let t = CatppuccinMocha;
//!
//! // Span builder
//! let title = t.fg_accent("Ralph Engine").bold();
//!
//! // Line compositor
//! let line = t.line()
//!     .accent("Dashboard")
//!     .dim(" | ")
//!     .success("3 passed")
//!     .build();
//!
//! // Block builder
//! let panel = t.block(" Status ").focused(true).build();
//!
//! // Status bar
//! let status = t.status_line()
//!     .kv("Mode", "Normal")
//!     .kv("File", "main.rs")
//!     .build();
//!
//! // Widget style bundles
//! let table = t.table_styles();
//! let list = t.list_styles();
//! let tabs = t.tab_styles();
//! let state = t.state_styles();
//! ```

mod bar;
mod block;
mod line;
mod span;
mod status_line;
mod styles;

use ratatui::style::{Color, Style};
use ratatui::text::Line;

use crate::Theme;

pub use bar::ThemedBar;
pub use block::ThemedBlock;
pub use line::ThemedLine;
pub use span::ThemedSpan;
pub use status_line::ThemedStatusLine;
pub use styles::{
    GaugeStyles, InputStyles, ListStyles, NotificationStyles, ScrollbarStyles, StateStyles,
    TabStyles, TableStyles, zebra_rows,
};

/// Extension trait — adds chainable builder methods to any `Theme`.
///
/// Import `ThemeExt` to get span builders, line compositors, block
/// builders, status lines, and widget style bundles on any theme.
pub trait ThemeExt: Theme {
    // ── Span builders ───────────────────────────────────────────

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

    // ── Composite builders ──────────────────────────────────────

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

    /// Themed block builder with title.
    fn block<'a>(&self, title: &'a str) -> ThemedBlock<'a> {
        ThemedBlock::new(self, Some(title))
    }

    /// Themed block builder without title.
    fn block_plain(&self) -> ThemedBlock<'static> {
        ThemedBlock::new(self, None)
    }

    /// Themed line compositor.
    fn line(&self) -> ThemedLine<'static> {
        ThemedLine::new(self)
    }

    /// Themed status line (key-value pairs).
    fn status_line(&self) -> ThemedStatusLine<'static> {
        ThemedStatusLine::new(self)
    }

    // ── Widget style bundles ────────────────────────────────────

    /// Style bundle for `Table` (header, row, highlight, stripe).
    fn table_styles(&self) -> TableStyles {
        TableStyles::from_theme(self)
    }

    /// Style bundle for `List` (base, highlight, symbol).
    fn list_styles(&self) -> ListStyles {
        ListStyles::from_theme(self)
    }

    /// Style bundle for `Tabs` (active, inactive).
    fn tab_styles(&self) -> TabStyles {
        TabStyles::from_theme(self)
    }

    /// Style bundle for `Gauge` (filled, base).
    fn gauge_styles(&self) -> GaugeStyles {
        GaugeStyles::from_theme(self)
    }

    /// Style bundle for input fields (text, placeholder, cursor, prompt, border).
    fn input_styles(&self) -> InputStyles {
        InputStyles::from_theme(self)
    }

    /// Style bundle for scrollbar widgets (track, thumb).
    fn scrollbar_styles(&self) -> ScrollbarStyles {
        ScrollbarStyles::from_theme(self)
    }

    /// Style bundle for notifications/toasts by severity.
    fn notification_styles(&self) -> NotificationStyles {
        NotificationStyles::from_theme(self)
    }

    /// State-aware style resolver (normal, focused, selected, disabled).
    fn state_styles(&self) -> StateStyles {
        StateStyles::from_theme(self)
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
    /// Style with info foreground.
    fn style_info(&self) -> Style {
        Style::default().fg(self.info())
    }
    /// Style with surface background.
    fn style_surface(&self) -> Style {
        Style::default().bg(self.surface())
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
    fn style_info_and_surface() {
        let t = CatppuccinMocha;
        assert_eq!(t.style_info().fg, Some(t.info));
        assert_eq!(t.style_surface().bg, Some(t.surface));
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

    // ── Composite builders via ThemeExt ─────────────────────────

    #[test]
    fn block_builder_via_theme_ext() {
        let block = CatppuccinMocha.block(" Test ").build();
        let _ = block;
    }

    #[test]
    fn block_plain_via_theme_ext() {
        let block = CatppuccinMocha.block_plain().build();
        let _ = block;
    }

    #[test]
    fn line_builder_via_theme_ext() {
        let line = CatppuccinMocha
            .line()
            .accent("A")
            .dim(" | ")
            .success("B")
            .build();
        assert_eq!(line.spans.len(), 3);
    }

    #[test]
    fn status_line_via_theme_ext() {
        let line = CatppuccinMocha
            .status_line()
            .kv("Mode", "Normal")
            .kv("File", "main.rs")
            .build();
        assert_eq!(line.spans.len(), 7); // pair(3) + sep(1) + pair(3)
    }

    #[test]
    fn table_styles_via_theme_ext() {
        let ts = CatppuccinMocha.table_styles();
        assert_eq!(ts.header.fg, Some(CatppuccinMocha.accent));
    }

    #[test]
    fn list_styles_via_theme_ext() {
        let ls = CatppuccinMocha.list_styles();
        assert_eq!(ls.highlight.fg, Some(CatppuccinMocha.accent));
    }

    #[test]
    fn tab_styles_via_theme_ext() {
        let ts = CatppuccinMocha.tab_styles();
        assert_eq!(ts.active.fg, Some(CatppuccinMocha.accent));
    }

    #[test]
    fn gauge_styles_via_theme_ext() {
        let gs = CatppuccinMocha.gauge_styles();
        assert_eq!(gs.filled.fg, Some(CatppuccinMocha.accent));
    }

    #[test]
    fn state_styles_via_theme_ext() {
        let ss = CatppuccinMocha.state_styles();
        assert_eq!(ss.normal.fg, Some(CatppuccinMocha.text));
        assert_eq!(ss.focused.fg, Some(CatppuccinMocha.accent));
    }

    #[test]
    fn input_styles_via_theme_ext() {
        let is = CatppuccinMocha.input_styles();
        assert_eq!(is.cursor.fg, Some(CatppuccinMocha.accent));
    }

    #[test]
    fn scrollbar_styles_via_theme_ext() {
        let ss = CatppuccinMocha.scrollbar_styles();
        assert_eq!(ss.track.fg, Some(CatppuccinMocha.border));
    }

    #[test]
    fn notification_styles_via_theme_ext() {
        let ns = CatppuccinMocha.notification_styles();
        assert_eq!(ns.error.fg, Some(CatppuccinMocha.error));
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
