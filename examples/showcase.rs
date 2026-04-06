//! Showcase of all ratatui-themekit features.
//!
//! Run: `cargo run --example showcase`

use ratatui::style::Color;
use ratatui::text::Line;
use ratatui_themekit::builders::{ThemedSpan, style_fg};
use ratatui_themekit::{ThemeExt, available_theme_ids, builtin_themes, resolve_theme};

fn main() {
    println!("ratatui-themekit showcase\n");

    // ── Available themes ────────────────────────────────────────
    println!("Built-in themes:");
    for theme in builtin_themes() {
        println!("  {} ({})", theme.name(), theme.id());
    }
    println!("\nTheme IDs: {:?}\n", available_theme_ids());

    // ── Resolve by ID ───────────────────────────────────────────
    let theme = resolve_theme("catppuccin");
    let t = theme.as_ref();
    println!("Active theme: {} ({})", t.name(), t.id());

    // ── ThemeExt builders ───────────────────────────────────────
    println!("\nThemeExt span builders:");
    let _accent = t.fg_accent("accent text").bold().build();
    let _dim = t.fg_dim("dimmed text").build();
    let _bright = t.fg_bright("bright text").build();
    let _success = t.fg_success("success").bold().build();
    let _error = t.fg_error("error").italic().build();
    let _warning = t.fg_warning("warning").build();
    let _info = t.fg_info("info").build();
    let _added = t.fg_added("+added line").build();
    let _removed = t.fg_removed("-removed line").build();
    let _border = t.fg_border("---border---").build();
    println!("  All 11 fg_* builders produce themed Span values");

    // ── Modifier chaining ───────────────────────────────────────
    let _chained = t.fg_accent("bold+italic").bold().italic().build();
    let _bg = t.fg_accent("with bg").on(Color::DarkGray).build();
    println!("  Modifiers: .bold(), .italic(), .dimmed(), .on(bg)");

    // ── Badge builder ───────────────────────────────────────────
    let _badge = t.badge(" RUNNING ", Color::Green).build();
    println!("  Badge: text on colored background");

    // ── Bar builder ─────────────────────────────────────────────
    let bar_line: Line<'_> = t.bar(75).width(20).build();
    let bar_text: String = bar_line
        .spans
        .iter()
        .map(|s| s.content.to_string())
        .collect();
    println!("  Bar: {bar_text}");

    // ── Separator ───────────────────────────────────────────────
    let sep = t.separator_line(30);
    let sep_text: String = sep.spans.iter().map(|s| s.content.to_string()).collect();
    println!("  Separator: {sep_text}");

    // ── Style helpers (for widget APIs) ─────────────────────────
    let _style_a = t.style_accent();
    let _style_b = t.style_border();
    let _style_e = t.style_error();
    let _style_w = t.style_warning();
    println!("  Style helpers: style_accent(), style_border(), etc.");

    // ── Dynamic colors ──────────────────────────────────────────
    let dynamic_color = t.block_file_edit();
    let _dynamic_span = ThemedSpan::with_color("dynamic text", dynamic_color)
        .bold()
        .build();
    let _dynamic_style = style_fg(dynamic_color);
    println!("  Dynamic: ThemedSpan::with_color(), style_fg()");

    // ── Into<Span> conversion ───────────────────────────────────
    let themed = t.fg_success("ok");
    let _span: ratatui::text::Span<'_> = themed.into();
    println!("  Into<Span>: implicit conversion works");

    // ── Compose a full Line ─────────────────────────────────────
    let line = Line::from(vec![
        t.fg_accent("App v1.0").bold().build(),
        t.fg_border(" | ").build(),
        t.fg_success("Ready").bold().build(),
        t.fg_border(" | ").build(),
        t.fg_dim("press ? for help").build(),
    ]);
    let line_text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
    println!("\nComposed line: {line_text}");

    println!("\nAll features working.");
}
