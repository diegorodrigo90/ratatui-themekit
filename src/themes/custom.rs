//! User-defined themes loaded from configuration files.
//!
//! Requires the `serde` feature:
//! ```toml
//! ratatui-themekit = { version = "0.2", features = ["serde"] }
//! ```

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ratatui::style::Color;

use crate::Theme;

/// A theme defined by user configuration (TOML, YAML, JSON).
///
/// All fields use ratatui's `Color` type which supports:
/// - Named ANSI: `"Red"`, `"Green"`, `"Blue"`, `"DarkGray"`
/// - RGB: `{ Rgb = [249, 115, 22] }` (with serde feature)
/// - Indexed: `{ Indexed = 42 }` (256-color)
///
/// # Example (TOML)
///
/// ```toml
/// [theme]
/// name = "My Theme"
/// id = "my-theme"
/// accent = { Rgb = [249, 115, 22] }
/// success = "Green"
/// error = "Red"
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CustomTheme {
    /// Theme display name.
    pub name: String,
    /// Config identifier.
    pub id: String,
    /// Primary accent color.
    pub accent: Color,
    /// Secondary accent.
    #[cfg_attr(feature = "serde", serde(default = "default_dark_gray"))]
    pub accent_dim: Color,
    /// Default text.
    #[cfg_attr(feature = "serde", serde(default = "default_white"))]
    pub text: Color,
    /// Dimmed text.
    #[cfg_attr(feature = "serde", serde(default = "default_gray"))]
    pub text_dim: Color,
    /// Bright text.
    #[cfg_attr(feature = "serde", serde(default = "default_white"))]
    pub text_bright: Color,
    /// Success color.
    #[cfg_attr(feature = "serde", serde(default = "default_green"))]
    pub success: Color,
    /// Error color.
    #[cfg_attr(feature = "serde", serde(default = "default_red"))]
    pub error: Color,
    /// Warning color.
    #[cfg_attr(feature = "serde", serde(default = "default_yellow"))]
    pub warning: Color,
    /// Info color.
    #[cfg_attr(feature = "serde", serde(default = "default_cyan"))]
    pub info: Color,
    /// Diff added.
    #[cfg_attr(feature = "serde", serde(default = "default_green"))]
    pub diff_added: Color,
    /// Diff removed.
    #[cfg_attr(feature = "serde", serde(default = "default_red"))]
    pub diff_removed: Color,
    /// Diff context.
    #[cfg_attr(feature = "serde", serde(default = "default_dark_gray"))]
    pub diff_context: Color,
    /// Border color.
    #[cfg_attr(feature = "serde", serde(default = "default_dark_gray"))]
    pub border: Color,
    /// Surface/background highlight.
    #[cfg_attr(feature = "serde", serde(default = "default_black"))]
    pub surface: Color,
    /// Application background color.
    #[cfg_attr(feature = "serde", serde(default = "default_reset"))]
    pub background: Color,
}

impl Theme for CustomTheme {
    fn name(&self) -> &str {
        &self.name
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn accent(&self) -> Color {
        self.accent
    }
    fn accent_dim(&self) -> Color {
        self.accent_dim
    }
    fn text(&self) -> Color {
        self.text
    }
    fn text_dim(&self) -> Color {
        self.text_dim
    }
    fn text_bright(&self) -> Color {
        self.text_bright
    }
    fn success(&self) -> Color {
        self.success
    }
    fn error(&self) -> Color {
        self.error
    }
    fn warning(&self) -> Color {
        self.warning
    }
    fn info(&self) -> Color {
        self.info
    }
    fn diff_added(&self) -> Color {
        self.diff_added
    }
    fn diff_removed(&self) -> Color {
        self.diff_removed
    }
    fn diff_context(&self) -> Color {
        self.diff_context
    }
    fn border(&self) -> Color {
        self.border
    }
    fn surface(&self) -> Color {
        self.surface
    }
    fn background(&self) -> Color {
        self.background
    }
}

impl std::fmt::Display for CustomTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

// Serde default functions
#[cfg(feature = "serde")]
fn default_dark_gray() -> Color {
    Color::DarkGray
}
#[cfg(feature = "serde")]
fn default_white() -> Color {
    Color::White
}
#[cfg(feature = "serde")]
fn default_gray() -> Color {
    Color::Gray
}
#[cfg(feature = "serde")]
fn default_green() -> Color {
    Color::Green
}
#[cfg(feature = "serde")]
fn default_red() -> Color {
    Color::Red
}
#[cfg(feature = "serde")]
fn default_yellow() -> Color {
    Color::Yellow
}
#[cfg(feature = "serde")]
fn default_cyan() -> Color {
    Color::Cyan
}
#[cfg(feature = "serde")]
fn default_black() -> Color {
    Color::Black
}
#[cfg(feature = "serde")]
fn default_reset() -> Color {
    Color::Reset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_theme_all_slots_accessible() {
        let t = CustomTheme {
            name: "Test".to_owned(),
            id: "test".to_owned(),
            accent: Color::Magenta,
            accent_dim: Color::DarkGray,
            text: Color::White,
            text_dim: Color::Gray,
            text_bright: Color::White,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Cyan,
            diff_added: Color::Green,
            diff_removed: Color::Red,
            diff_context: Color::DarkGray,
            border: Color::DarkGray,
            surface: Color::Black,
            background: Color::Reset,
        };
        let theme: &dyn Theme = &t;
        assert_eq!(theme.accent(), Color::Magenta);
        assert_eq!(theme.block_pass(), theme.success());
    }

    #[test]
    fn custom_theme_display() {
        let t = CustomTheme {
            name: "My Theme".to_owned(),
            id: "my-theme".to_owned(),
            accent: Color::Red,
            accent_dim: Color::DarkGray,
            text: Color::White,
            text_dim: Color::Gray,
            text_bright: Color::White,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Cyan,
            diff_added: Color::Green,
            diff_removed: Color::Red,
            diff_context: Color::DarkGray,
            border: Color::DarkGray,
            surface: Color::Black,
            background: Color::Reset,
        };
        assert_eq!(t.to_string(), "My Theme (my-theme)");
    }

    #[test]
    fn custom_theme_equality() {
        let a = CustomTheme {
            name: "A".to_owned(),
            id: "a".to_owned(),
            accent: Color::Red,
            accent_dim: Color::DarkGray,
            text: Color::White,
            text_dim: Color::Gray,
            text_bright: Color::White,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Cyan,
            diff_added: Color::Green,
            diff_removed: Color::Red,
            diff_context: Color::DarkGray,
            border: Color::DarkGray,
            surface: Color::Black,
            background: Color::Reset,
        };
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn custom_theme_deserialize_minimal() {
        let toml_str = r#"
name = "Minimal"
id = "minimal"
accent = "Magenta"
"#;
        let theme: CustomTheme = toml::from_str(toml_str).unwrap();
        assert_eq!(theme.name, "Minimal");
        assert_eq!(theme.accent, Color::Magenta);
        assert_eq!(theme.success, Color::Green); // default
    }

    #[cfg(feature = "serde")]
    #[test]
    fn custom_theme_deserialize_rgb() {
        let toml_str = r#"
name = "RGB"
id = "rgb"
accent = { Rgb = [249, 115, 22] }
"#;
        let theme: CustomTheme = toml::from_str(toml_str).unwrap();
        assert_eq!(theme.accent, Color::Rgb(249, 115, 22));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn custom_theme_serialize_roundtrip() {
        let theme = CustomTheme {
            name: "RT".to_owned(),
            id: "rt".to_owned(),
            accent: Color::Rgb(100, 200, 50),
            accent_dim: Color::DarkGray,
            text: Color::White,
            text_dim: Color::Gray,
            text_bright: Color::White,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Cyan,
            diff_added: Color::Green,
            diff_removed: Color::Red,
            diff_context: Color::DarkGray,
            border: Color::DarkGray,
            surface: Color::Black,
            background: Color::Reset,
        };
        let toml_str = toml::to_string(&theme).unwrap();
        let parsed: CustomTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(theme, parsed);
    }
}
