# ratatui-themekit

[![Crates.io](https://img.shields.io/crates/v/ratatui-themekit.svg)](https://crates.io/crates/ratatui-themekit)
[![docs.rs](https://docs.rs/ratatui-themekit/badge.svg)](https://docs.rs/ratatui-themekit)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Semantic theme system for [ratatui](https://ratatui.rs).**

Stop hardcoding `Color::Rgb(...)` everywhere. Define what colors **mean** and let themes provide the actual values.

## The Problem

```rust
// Without themekit — colors scattered everywhere, no consistency
let style = Style::default().fg(Color::Rgb(166, 227, 161)); // what is this?
let err = Style::default().fg(Color::Rgb(243, 139, 168));   // hope I remember
```

## The Solution

```rust
// With themekit — semantic, consistent, swappable
let style = Style::default().fg(theme.success());  // always the right green
let err = Style::default().fg(theme.error());      // always the right red
```

## Quick Start

```bash
cargo add ratatui-themekit
```

```rust
use ratatui::style::Style;
use ratatui_themekit::{Theme, resolve_theme};

// Resolve from user config
let theme = resolve_theme("catppuccin");

// Use semantic slots everywhere
let header = Style::default().fg(theme.accent()).bold();
let status = Style::default().fg(theme.success());
let border = Style::default().fg(theme.border());
let dimmed = Style::default().fg(theme.text_dim());
```

## 20 Semantic Color Slots

| Category | Slots | Purpose |
|----------|-------|---------|
| **Brand** | `accent`, `accent_dim` | Primary UI color, subtle highlights |
| **Text** | `text`, `text_dim`, `text_bright` | Default, muted, emphasized text |
| **Status** | `success`, `error`, `warning`, `info` | Semantic status indicators |
| **Diff** | `diff_added`, `diff_removed`, `diff_context` | Code diff rendering |
| **Structure** | `border`, `surface` | Panel borders, focused backgrounds |
| **Derived** | `block_*`, `indicator_*` | Auto-derived from core slots |

## 7 Built-in Themes

| Theme | ID | Style |
|-------|----|-------|
| **Catppuccin Mocha** | `catppuccin` | Warm dark, pastel accents |
| **Dracula** | `dracula` | Dark, vivid purples and greens |
| **Nord** | `nord` | Arctic blue-gray, calm |
| **Gruvbox Dark** | `gruvbox` | Retro warm, earthy tones |
| **One Dark** | `one-dark` | Atom's classic blue |
| **Solarized Dark** | `solarized` | Precision-engineered |
| **No Color** | `no-color` | All `Color::Reset` for `NO_COLOR` |

## Custom Themes

Implement the `Theme` trait — 15 required methods, 5+ derived automatically:

```rust
use ratatui::style::Color;
use ratatui_themekit::Theme;

struct TokyoNight;

impl Theme for TokyoNight {
    fn name(&self) -> &str { "Tokyo Night" }
    fn id(&self) -> &str { "tokyo-night" }
    fn accent(&self) -> Color { Color::Rgb(122, 162, 247) }
    fn accent_dim(&self) -> Color { Color::Rgb(61, 89, 161) }
    fn text(&self) -> Color { Color::Rgb(169, 177, 214) }
    fn text_dim(&self) -> Color { Color::Rgb(86, 95, 137) }
    fn text_bright(&self) -> Color { Color::Rgb(195, 202, 235) }
    fn success(&self) -> Color { Color::Rgb(158, 206, 106) }
    fn error(&self) -> Color { Color::Rgb(247, 118, 142) }
    fn warning(&self) -> Color { Color::Rgb(224, 175, 104) }
    fn info(&self) -> Color { Color::Rgb(125, 207, 255) }
    fn diff_added(&self) -> Color { Color::Rgb(158, 206, 106) }
    fn diff_removed(&self) -> Color { Color::Rgb(247, 118, 142) }
    fn diff_context(&self) -> Color { Color::Rgb(86, 95, 137) }
    fn border(&self) -> Color { Color::Rgb(41, 46, 66) }
    fn surface(&self) -> Color { Color::Rgb(30, 32, 48) }
}
```

## NO_COLOR Support

Automatically respects the [NO_COLOR](https://no-color.org/) standard:

```rust
use ratatui_themekit::{resolve_theme, no_color_active};

// When NO_COLOR is set, resolve_theme returns NoColor automatically
let theme = resolve_theme("catppuccin"); // returns NoColor if NO_COLOR is set

// Or check manually
if no_color_active() {
    println!("Colors disabled by environment");
}
```

## Runtime Theme Switching

```rust
use ratatui_themekit::{Theme, resolve_theme, available_theme_ids};

// List available themes for a settings menu
for id in available_theme_ids() {
    println!("{}", id);
}

// Switch at runtime — zero code changes needed
let mut current_theme = resolve_theme("catppuccin");
current_theme = resolve_theme("dracula"); // instant switch
```

## Design Philosophy

- **Semantic over literal** — slots describe meaning, not appearance
- **Derived defaults** — implement 15 methods, get 20+ color slots
- **Zero opinion on layout** — only colors, never widget structure
- **`NO_COLOR` native** — accessibility built in, not bolted on
- **`Send + Sync`** — safe for async TUI architectures

## License

MIT
