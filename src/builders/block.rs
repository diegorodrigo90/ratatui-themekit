//! Themed block builder — wraps `ratatui::widgets::Block`.

use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders};

use crate::Theme;

/// Chainable builder for themed `Block` widgets.
///
/// Sets `border_style`, `title_style`, and background from the theme.
/// Call `.focused(true)` to switch to accent-colored borders.
///
/// ```rust
/// use ratatui_themekit::{CatppuccinMocha, ThemeExt};
///
/// let block = CatppuccinMocha.block(" Dashboard ").build();
/// let focused = CatppuccinMocha.block(" Active ").focused(true).build();
/// ```
pub struct ThemedBlock<'a> {
    title: Option<&'a str>,
    border_type: BorderType,
    borders: Borders,
    focused: bool,
    border_color: ratatui::style::Color,
    title_color: ratatui::style::Color,
    surface_color: ratatui::style::Color,
    accent_color: ratatui::style::Color,
}

impl<'a> ThemedBlock<'a> {
    /// Creates a themed block from a theme reference.
    pub(crate) fn new(theme: &(impl Theme + ?Sized), title: Option<&'a str>) -> Self {
        Self {
            title,
            border_type: BorderType::Rounded,
            borders: Borders::ALL,
            focused: false,
            border_color: theme.border(),
            title_color: theme.accent(),
            surface_color: theme.surface(),
            accent_color: theme.accent(),
        }
    }

    /// Mark this block as focused (accent-colored border).
    #[must_use]
    pub fn focused(mut self, is_focused: bool) -> Self {
        self.focused = is_focused;
        self
    }

    /// Set border type (default: `Rounded`).
    #[must_use]
    pub fn border_type(mut self, bt: BorderType) -> Self {
        self.border_type = bt;
        self
    }

    /// Set which borders to show (default: `ALL`).
    #[must_use]
    pub fn borders(mut self, b: Borders) -> Self {
        self.borders = b;
        self
    }

    /// Remove the title, keeping themed borders and background.
    #[must_use]
    pub fn untitled(mut self) -> Self {
        self.title = None;
        self
    }

    /// Build into a ratatui `Block`.
    #[must_use]
    pub fn build(self) -> Block<'a> {
        let border_fg = if self.focused {
            self.accent_color
        } else {
            self.border_color
        };

        let mut block = Block::new()
            .borders(self.borders)
            .border_type(self.border_type)
            .border_style(Style::default().fg(border_fg))
            .style(Style::default().bg(self.surface_color));

        if let Some(title) = self.title {
            block = block.title(title).title_style(
                Style::default()
                    .fg(self.title_color)
                    .add_modifier(Modifier::BOLD),
            );
        }

        block
    }
}

#[cfg(test)]
mod tests {
    use ratatui::widgets::BorderType;

    use super::*;
    use crate::CatppuccinMocha;

    #[test]
    fn block_has_rounded_borders_by_default() {
        let _ = ThemedBlock::new(&CatppuccinMocha, Some(" Test ")).build();
    }

    #[test]
    fn focused_block_builds() {
        let _ = ThemedBlock::new(&CatppuccinMocha, Some(" Focus "))
            .focused(true)
            .build();
    }

    #[test]
    fn untitled_block_builds() {
        let _ = ThemedBlock::new(&CatppuccinMocha, Some(" Gone "))
            .untitled()
            .build();
    }

    #[test]
    fn custom_border_type() {
        let _ = ThemedBlock::new(&CatppuccinMocha, Some(" Custom "))
            .border_type(BorderType::Double)
            .build();
    }

    #[test]
    fn no_color_block_builds() {
        let _ = ThemedBlock::new(&crate::NoColor, Some(" NC ")).build();
    }

    #[test]
    fn block_without_title() {
        let _ = ThemedBlock::new(&CatppuccinMocha, None).build();
    }
}
