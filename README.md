# ratatui-themekit

[![Crates.io](https://img.shields.io/crates/v/ratatui-themekit.svg)](https://crates.io/crates/ratatui-themekit)
[![docs.rs](https://docs.rs/ratatui-themekit/badge.svg)](https://docs.rs/ratatui-themekit)
[![CI](https://github.com/diegorodrigo90/ratatui-themekit/actions/workflows/ci.yml/badge.svg)](https://github.com/diegorodrigo90/ratatui-themekit/actions)
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
use ratatui_themekit::{Theme, ThemeExt, CatppuccinMocha};

let t = CatppuccinMocha;

// Tailwind-like builders — semantic, chainable, themed
let title = t.fg_accent("Ralph Engine").bold().build();
let ok    = t.fg_success("Passing").bold().build();
let hint  = t.fg_dim("press ? for help").build();
```

## Quick Start

```bash
cargo add ratatui-themekit
```

```rust
use ratatui::text::Line;
use ratatui_themekit::{Theme, ThemeExt, resolve_theme};

let theme = resolve_theme("catppuccin");
let t = theme.as_ref();

// Build themed spans — never touch Style::default() again
let header = Line::from(vec![
    t.fg_accent("App v1.0").bold().build(),
    t.fg_border(" | ").build(),
    t.fg_bright("Ready").build(),
]);

// Widget styles (border_style, title_style)
let border = t.style_border();
let title = t.style_accent();
```

## ThemeExt Builders (Tailwind-like API)

Import `ThemeExt` to get chainable builders on any `Theme`:

```rust
use ratatui_themekit::{Theme, ThemeExt, CatppuccinMocha};

let t = CatppuccinMocha;

// Semantic span builders — color from theme slot
t.fg_accent("title")           // accent color
t.fg_dim("hint")               // dimmed text
t.fg_bright("emphasis")        // bright text
t.fg_success("ok").bold()      // success + bold
t.fg_error("fail").italic()    // error + italic
t.fg_warning("warn")           // warning
t.fg_info("note")              // informational
t.fg_added("+line")            // diff added
t.fg_removed("-line")          // diff removed
t.fg_border("---")             // border/separator

// Modifiers chain
t.fg_accent("title").bold().italic().build()

// Background color
t.fg_accent("badge").on(Color::Red).build()

// Badge (text on colored background)
t.badge(" RUNNING ", Color::Green).build()

// Progress bar
t.bar(75).width(20).build()  // ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱ 75%

// Separator line
t.separator_line(40)  // · · · · · · · · · · · ·

// Style helpers for widget APIs (border_style, title_style)
t.style_accent()    // Style with accent fg
t.style_border()    // Style with border fg
t.style_error()     // Style with error fg
t.style_warning()   // Style with warning fg
```

### Dynamic Colors

For colors not from a named theme slot (e.g., computed at runtime):

```rust
use ratatui_themekit::builders::{ThemedSpan, style_fg};

// Dynamic color span
let block_color = theme.block_file_edit();
ThemedSpan::with_color("text", block_color).bold().build()

// Dynamic color style (for widgets)
style_fg(block_color)
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

## 11 Built-in Themes

| Theme | ID | Style |
|-------|----|-------|
| **Catppuccin Mocha** | `catppuccin` | Warm dark, pastel accents (default) |
| **Dracula** | `dracula` | Dark, vivid purples and greens |
| **Nord** | `nord` | Arctic blue-gray, calm |
| **Gruvbox Dark** | `gruvbox` | Retro warm, earthy tones |
| **One Dark** | `one-dark` | Atom's classic blue |
| **Solarized Dark** | `solarized` | Precision-engineered |
| **Tailwind Dark** | `tailwind` | Tailwind CSS palette |
| **Tokyo Night** | `tokyo-night` | Vivid blue accents |
| **Rosé Pine** | `rose-pine` | Muted, elegant rose tones |
| **Terminal Native** | `terminal` | Named ANSI colors only |
| **No Color** | `no-color` | All `Color::Reset` for `NO_COLOR` |

Themes are pure data (`ThemeData` constants) — zero boilerplate, zero code duplication.

## Custom Themes

Implement the `Theme` trait — 15 required methods, 10+ derived automatically:

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
// All ThemeExt builders work automatically!
```

### Serde Custom Themes

Load themes from config files with the `serde` feature:

```bash
cargo add ratatui-themekit --features serde
```

```rust
use ratatui_themekit::CustomTheme;

let toml = r#"
name = "My Theme"
id = "my-theme"
accent = { Rgb = [249, 115, 22] }
success = "Green"
error = "Red"
"#;

let theme: CustomTheme = toml::from_str(toml).unwrap();
```

## NO_COLOR Support

Automatically respects the [NO_COLOR](https://no-color.org/) standard:

```rust
use ratatui_themekit::resolve_theme;

// When NO_COLOR is set, resolve_theme returns NoColor automatically
let theme = resolve_theme("catppuccin"); // → NoColor if NO_COLOR is set
```

## Runtime Theme Switching

```rust
use ratatui_themekit::{resolve_theme, available_theme_ids};

// List available themes for a settings menu
for id in available_theme_ids() {
    println!("{id}");
}

// Switch at runtime — zero code changes needed
let mut current = resolve_theme("catppuccin");
current = resolve_theme("dracula"); // instant
```

## Design Philosophy

- **Semantic over literal** — slots describe meaning, not appearance
- **Builders over manual styling** — `t.fg_accent("x").bold()` not `Span::styled("x", Style::default().fg(...))`
- **Derived defaults** — implement 15 methods, get 25+ color slots
- **Zero opinion on layout** — only colors, never widget structure
- **`NO_COLOR` native** — accessibility built in, not bolted on
- **`Send + Sync`** — safe for async TUI architectures

## License

MIT
