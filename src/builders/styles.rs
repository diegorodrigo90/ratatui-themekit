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
#[derive(Debug, Clone)]
pub struct ListStyles {
    /// Base list style.
    pub base: Style,
    /// Selected item highlight style.
    pub highlight: Style,
    /// Highlight symbol (e.g. `"▶ "`).
    pub symbol: String,
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
            symbol: "\u{25b6} ".to_owned(),
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
