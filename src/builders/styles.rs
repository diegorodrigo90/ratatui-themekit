//! Widget style bundles — pre-built `Style` sets for common widgets.
//!
//! Each bundle groups the styles a widget needs. Users keep full control
//! of the widget API while getting semantic colors for free.

use ratatui::style::{Modifier, Style};

/// Style bundle for `ratatui::widgets::Table`.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let ts = CatppuccinMocha.table_styles();
/// // Use ts.header, ts.row, ts.highlight, ts.stripe with Table::new(...)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TableStyles {
    /// Header row style (bold, accent foreground).
    pub header: Style,
    /// Default row style.
    pub row: Style,
    /// Selected/highlighted row style.
    pub highlight: Style,
    /// Alternating (even) row background for zebra striping.
    pub stripe: Style,
}

/// Style bundle for `ratatui::widgets::List`.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let ls = CatppuccinMocha.list_styles();
/// // Use ls.base, ls.highlight, ls.symbol with List::new(...)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ListStyles {
    /// Base list style.
    pub base: Style,
    /// Selected item highlight style.
    pub highlight: Style,
    /// Highlight symbol (e.g. `"▶ "`).
    pub symbol: &'static str,
}

/// Style bundle for `ratatui::widgets::Tabs`.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let ts = CatppuccinMocha.tab_styles();
/// // Use ts.active, ts.inactive with Tabs::new(...)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TabStyles {
    /// Active/selected tab style.
    pub active: Style,
    /// Inactive tab style.
    pub inactive: Style,
}

/// Style bundle for `ratatui::widgets::Gauge`.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let gs = CatppuccinMocha.gauge_styles();
/// // Use gs.filled, gs.base with Gauge::default()
/// ```
#[derive(Debug, Clone, Copy)]
pub struct GaugeStyles {
    /// Filled portion style.
    pub filled: Style,
    /// Background/unfilled style.
    pub base: Style,
}

/// State-aware style resolver.
///
/// Returns the right `Style` based on widget state (focused, selected,
/// disabled). Inspired by Blessed's `style.focus` and Textual's `:focus`.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let state = CatppuccinMocha.state_styles();
/// let is_focused = true;
/// let style = if is_focused { state.focused } else { state.normal };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct StateStyles {
    /// Default/unfocused state.
    pub normal: Style,
    /// Widget has keyboard focus.
    pub focused: Style,
    /// Item is selected in a list/table.
    pub selected: Style,
    /// Widget is disabled/non-interactive.
    pub disabled: Style,
}

/// Style bundle for text input fields (search bars, chat input, command palettes).
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let is = CatppuccinMocha.input_styles();
/// // Use is.text, is.placeholder, is.cursor, is.prompt,
/// //     is.border, is.border_focused with your input widget
/// ```
#[derive(Debug, Clone, Copy)]
pub struct InputStyles {
    /// Typed text style.
    pub text: Style,
    /// Placeholder/hint text style (dimmed).
    pub placeholder: Style,
    /// Cursor style (accent, reversed for block cursor).
    pub cursor: Style,
    /// Prompt prefix style (e.g., `> ` or `/ `).
    pub prompt: Style,
    /// Border when unfocused.
    pub border: Style,
    /// Border when focused (accent color).
    pub border_focused: Style,
}

/// Style bundle for scrollbar widgets.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let ss = CatppuccinMocha.scrollbar_styles();
/// // Use ss.track, ss.thumb with Scrollbar widget
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ScrollbarStyles {
    /// Scrollbar track (background rail).
    pub track: Style,
    /// Scrollbar thumb (draggable indicator).
    pub thumb: Style,
}

/// Style bundle for notifications/toasts by severity.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let ns = CatppuccinMocha.notification_styles();
/// // Use ns.info, ns.success, ns.warning, ns.error for toast borders/text
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NotificationStyles {
    /// Informational notification border/accent.
    pub info: Style,
    /// Success notification border/accent.
    pub success: Style,
    /// Warning notification border/accent.
    pub warning: Style,
    /// Error notification border/accent.
    pub error: Style,
    /// Notification body text.
    pub body: Style,
    /// Notification background.
    pub background: Style,
}

// ── Factory functions ───────────────────────────────────────────

impl TableStyles {
    /// Create table styles from a theme.
    pub(crate) fn from_theme(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            header: Style::default()
                .fg(theme.accent())
                .add_modifier(Modifier::BOLD),
            row: Style::default().fg(theme.text()),
            highlight: Style::default()
                .fg(theme.text_bright())
                .bg(theme.surface())
                .add_modifier(Modifier::BOLD),
            stripe: Style::default().bg(theme.surface()),
        }
    }
}

impl ListStyles {
    /// Create list styles from a theme.
    pub(crate) fn from_theme(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            base: Style::default().fg(theme.text()),
            highlight: Style::default()
                .fg(theme.accent())
                .bg(theme.surface())
                .add_modifier(Modifier::BOLD),
            symbol: "\u{25b6} ",
        }
    }
}

impl TabStyles {
    /// Create tab styles from a theme.
    pub(crate) fn from_theme(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            active: Style::default()
                .fg(theme.accent())
                .add_modifier(Modifier::BOLD),
            inactive: Style::default().fg(theme.text_dim()),
        }
    }
}

impl GaugeStyles {
    /// Create gauge styles from a theme.
    pub(crate) fn from_theme(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            filled: Style::default()
                .fg(theme.accent())
                .add_modifier(Modifier::BOLD),
            base: Style::default().fg(theme.border()),
        }
    }
}

impl ScrollbarStyles {
    /// Create scrollbar styles from a theme.
    pub(crate) fn from_theme(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            track: Style::default().fg(theme.border()),
            thumb: Style::default().fg(theme.text_dim()),
        }
    }
}

impl NotificationStyles {
    /// Create notification styles from a theme.
    pub(crate) fn from_theme(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            info: Style::default().fg(theme.info()),
            success: Style::default().fg(theme.success()),
            warning: Style::default().fg(theme.warning()),
            error: Style::default().fg(theme.error()),
            body: Style::default().fg(theme.text_bright()),
            background: Style::default().bg(theme.surface()),
        }
    }
}

impl InputStyles {
    /// Create input styles from a theme.
    pub(crate) fn from_theme(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            text: Style::default().fg(theme.text()),
            placeholder: Style::default().fg(theme.text_dim()),
            cursor: Style::default()
                .fg(theme.accent())
                .add_modifier(Modifier::REVERSED),
            prompt: Style::default()
                .fg(theme.accent())
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(theme.border()),
            border_focused: Style::default().fg(theme.accent()),
        }
    }
}

impl StateStyles {
    /// Create state-aware styles from a theme.
    pub(crate) fn from_theme(theme: &(impl crate::Theme + ?Sized)) -> Self {
        Self {
            normal: Style::default().fg(theme.text()),
            focused: Style::default()
                .fg(theme.accent())
                .add_modifier(Modifier::BOLD),
            selected: Style::default().fg(theme.text_bright()).bg(theme.surface()),
            disabled: Style::default().fg(theme.text_dim()),
        }
    }

    /// Resolve the appropriate style for a given state.
    #[must_use]
    pub fn resolve(&self, focused: bool, selected: bool, disabled: bool) -> Style {
        if disabled {
            self.disabled
        } else if selected {
            self.selected
        } else if focused {
            self.focused
        } else {
            self.normal
        }
    }
}

/// Applies alternating (zebra) background to table rows.
///
/// Even-indexed rows get the `stripe` style applied as background.
/// Odd-indexed rows keep their current style.
///
/// ```rust
/// use ratatui::widgets::Row;
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let rows = vec![
///     Row::new(["Alice", "Online"]),
///     Row::new(["Bob", "Offline"]),
///     Row::new(["Carol", "Online"]),
/// ];
/// let striped = ratatui_themekit::builders::zebra_rows(rows, CatppuccinMocha.table_styles().stripe);
/// ```
#[must_use]
pub fn zebra_rows(
    rows: Vec<ratatui::widgets::Row<'_>>,
    stripe_style: Style,
) -> Vec<ratatui::widgets::Row<'_>> {
    rows.into_iter()
        .enumerate()
        .map(|(i, row)| {
            if i % 2 == 0 {
                row.style(stripe_style)
            } else {
                row
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;
    use ratatui::widgets::Row;

    use super::*;
    use crate::{CatppuccinMocha, Dracula, NoColor};

    // ── TableStyles ─────────────────────────────────────────────

    #[test]
    fn table_header_is_bold_accent() {
        let ts = TableStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ts.header.fg, Some(CatppuccinMocha.accent));
        assert!(ts.header.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn table_highlight_has_surface_bg() {
        let ts = TableStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ts.highlight.bg, Some(CatppuccinMocha.surface));
    }

    #[test]
    fn table_stripe_has_surface_bg() {
        let ts = TableStyles::from_theme(&Dracula);
        assert_eq!(ts.stripe.bg, Some(Dracula.surface));
    }

    // ── ListStyles ──────────────────────────────────────────────

    #[test]
    fn list_highlight_uses_accent() {
        let ls = ListStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ls.highlight.fg, Some(CatppuccinMocha.accent));
    }

    #[test]
    fn list_symbol_is_arrow() {
        let ls = ListStyles::from_theme(&CatppuccinMocha);
        assert!(ls.symbol.contains('\u{25b6}'));
    }

    // ── TabStyles ───────────────────────────────────────────────

    #[test]
    fn tab_active_is_accent_bold() {
        let ts = TabStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ts.active.fg, Some(CatppuccinMocha.accent));
        assert!(ts.active.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn tab_inactive_is_dim() {
        let ts = TabStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ts.inactive.fg, Some(CatppuccinMocha.text_dim));
    }

    // ── GaugeStyles ─────────────────────────────────────────────

    #[test]
    fn gauge_filled_is_accent() {
        let gs = GaugeStyles::from_theme(&CatppuccinMocha);
        assert_eq!(gs.filled.fg, Some(CatppuccinMocha.accent));
    }

    // ── StateStyles ─────────────────────────────────────────────

    #[test]
    fn state_resolve_disabled_wins() {
        let ss = StateStyles::from_theme(&CatppuccinMocha);
        let style = ss.resolve(true, true, true);
        assert_eq!(style, ss.disabled);
    }

    #[test]
    fn state_resolve_selected_over_focused() {
        let ss = StateStyles::from_theme(&CatppuccinMocha);
        let style = ss.resolve(true, true, false);
        assert_eq!(style, ss.selected);
    }

    #[test]
    fn state_resolve_focused() {
        let ss = StateStyles::from_theme(&CatppuccinMocha);
        let style = ss.resolve(true, false, false);
        assert_eq!(style, ss.focused);
    }

    #[test]
    fn state_resolve_normal() {
        let ss = StateStyles::from_theme(&CatppuccinMocha);
        let style = ss.resolve(false, false, false);
        assert_eq!(style, ss.normal);
    }

    // ── InputStyles ──────────────────────────────────────────────

    #[test]
    fn input_placeholder_is_dim() {
        let is = InputStyles::from_theme(&CatppuccinMocha);
        assert_eq!(is.placeholder.fg, Some(CatppuccinMocha.text_dim));
    }

    #[test]
    fn input_cursor_is_accent_reversed() {
        let is = InputStyles::from_theme(&CatppuccinMocha);
        assert_eq!(is.cursor.fg, Some(CatppuccinMocha.accent));
        assert!(is.cursor.add_modifier.contains(Modifier::REVERSED));
    }

    #[test]
    fn input_border_focused_is_accent() {
        let is = InputStyles::from_theme(&CatppuccinMocha);
        assert_eq!(is.border_focused.fg, Some(CatppuccinMocha.accent));
    }

    // ── ScrollbarStyles ─────────────────────────────────────────

    #[test]
    fn scrollbar_thumb_is_dim() {
        let ss = ScrollbarStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ss.thumb.fg, Some(CatppuccinMocha.text_dim));
    }

    #[test]
    fn scrollbar_track_is_border() {
        let ss = ScrollbarStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ss.track.fg, Some(CatppuccinMocha.border));
    }

    // ── NotificationStyles ──────────────────────────────────────

    #[test]
    fn notification_error_uses_error_color() {
        let ns = NotificationStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ns.error.fg, Some(CatppuccinMocha.error));
    }

    #[test]
    fn notification_info_uses_info_color() {
        let ns = NotificationStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ns.info.fg, Some(CatppuccinMocha.info));
    }

    #[test]
    fn notification_success_uses_success_color() {
        let ns = NotificationStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ns.success.fg, Some(CatppuccinMocha.success));
    }

    #[test]
    fn notification_background_uses_surface() {
        let ns = NotificationStyles::from_theme(&CatppuccinMocha);
        assert_eq!(ns.background.bg, Some(CatppuccinMocha.surface));
    }

    // ── NoColor ─────────────────────────────────────────────────

    #[test]
    fn no_color_table_uses_reset() {
        let ts = TableStyles::from_theme(&NoColor);
        assert_eq!(ts.header.fg, Some(Color::Reset));
    }

    #[test]
    fn no_color_state_uses_reset() {
        let ss = StateStyles::from_theme(&NoColor);
        assert_eq!(ss.normal.fg, Some(Color::Reset));
    }

    #[test]
    fn no_color_input_uses_reset() {
        let is = InputStyles::from_theme(&NoColor);
        assert_eq!(is.text.fg, Some(Color::Reset));
        assert_eq!(is.placeholder.fg, Some(Color::Reset));
    }

    #[test]
    fn no_color_scrollbar_uses_reset() {
        let ss = ScrollbarStyles::from_theme(&NoColor);
        assert_eq!(ss.track.fg, Some(Color::Reset));
    }

    #[test]
    fn no_color_notification_uses_reset() {
        let ns = NotificationStyles::from_theme(&NoColor);
        assert_eq!(ns.info.fg, Some(Color::Reset));
        assert_eq!(ns.error.fg, Some(Color::Reset));
    }

    // ── Zebra rows ──────────────────────────────────────────────

    #[test]
    fn zebra_applies_to_even_rows() {
        let stripe = Style::default().bg(Color::DarkGray);
        let rows = vec![Row::new(["a"]), Row::new(["b"]), Row::new(["c"])];
        let striped = zebra_rows(rows, stripe);
        assert_eq!(striped.len(), 3);
    }

    #[test]
    fn zebra_empty_rows() {
        let striped = zebra_rows(Vec::new(), Style::default());
        assert!(striped.is_empty());
    }
}
