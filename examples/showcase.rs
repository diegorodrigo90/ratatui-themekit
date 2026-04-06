//! Interactive showcase of all ratatui-themekit features.
//!
//! Run: `cargo run --example showcase`
//! Run with theme: `cargo run --example showcase -- tokyo-night`
//!
//! Controls: ↑/↓ switch themes, mouse scroll in list, click input to type, q/Esc quit.
//!
//! ## What is themekit vs ratatui?
//!
//! Lines marked `// ← themekit` show what the library provides.
//! Everything else (layout, event loop, state) is standard ratatui.
//! The value of themekit: zero `Style::default().fg(...)` boilerplate,
//! semantic colors that change when you switch themes.

use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Gauge, List, ListItem, Paragraph, Row, Table, Tabs};
use ratatui_themekit::{BUILTIN_THEMES, Theme, ThemeExt, zebra_rows};

/// Animated state that ticks every frame.
struct AnimState {
    tick: u64,
    theme_index: usize,
    progress: u8,
    progress_dir: bool,
    active_tab: usize,
    list_selected: usize,
    list_len: usize,
    /// Stored area of the list widget for mouse hit-testing.
    list_area: Rect,
    /// Input field state.
    input_focused: bool,
    input_buffer: String,
    input_cursor: usize,
    input_area: Rect,
    /// Stored area of the tab header for mouse hit-testing.
    tab_area: Rect,
}

impl AnimState {
    fn new(theme_index: usize) -> Self {
        Self {
            tick: 0,
            theme_index,
            progress: 0,
            progress_dir: true,
            active_tab: 0,
            list_selected: 0,
            list_len: 8,
            list_area: Rect::default(),
            input_focused: false,
            input_buffer: String::new(),
            input_cursor: 0,
            input_area: Rect::default(),
            tab_area: Rect::default(),
        }
    }

    fn tick(&mut self) {
        self.tick += 1;

        // Progress bar: bounce 0→100→0 every ~200 ticks (~4s at 50ms)
        if self.tick % 2 == 0 {
            if self.progress_dir {
                if self.progress >= 100 {
                    self.progress_dir = false;
                } else {
                    self.progress += 1;
                }
            } else if self.progress == 0 {
                self.progress_dir = true;
            } else {
                self.progress -= 1;
            }
        }

        // GIF mode: auto-animate when not interactive OR THEMEKIT_GIF=1
        let gif_mode = std::env::var("THEMEKIT_GIF").is_ok_and(|v| v == "1")
            || !std::io::IsTerminal::is_terminal(&std::io::stdin());
        if gif_mode {
            // Tabs: cycle every ~2s
            if self.tick % 40 == 0 {
                self.active_tab = (self.active_tab + 1) % 4;
            }
            // List: auto-scroll
            if self.tick % 20 == 0 {
                self.list_selected = (self.list_selected + 1) % self.list_len;
            }
            // Theme: cycle every ~3s
            if self.tick % 60 == 0 && self.tick > 0 {
                self.theme_index = (self.theme_index + 1) % BUILTIN_THEMES.len();
            }
            // Input: auto-type
            self.auto_type_input();
        }
    }

    /// Simulates typing in GIF mode — types a demo message, pauses, clears, repeats.
    fn auto_type_input(&mut self) {
        const DEMO_TEXT: &str = "Hello themekit!\nMultiline too";
        let text_len = DEMO_TEXT.len();
        let type_interval = 6_usize; // ticks between chars (~300ms)
        let pause_ticks = 80_usize; // pause after full text (~4s)
        let total_cycle = text_len * type_interval + pause_ticks;

        // Wrap tick to usize range — safe because total_cycle << usize::MAX
        let tick =
            usize::try_from(self.tick % u64::try_from(total_cycle).unwrap_or(1)).unwrap_or(0);
        let cycle_pos = tick % total_cycle;
        let char_index = cycle_pos / type_interval;

        if char_index < text_len {
            // Typing phase
            self.input_focused = true;
            DEMO_TEXT[..=char_index].clone_into(&mut self.input_buffer);
            self.input_cursor = self.input_buffer.len();
        } else {
            // Pause phase — show full text, then clear at end
            let pause_elapsed = cycle_pos - text_len * type_interval;
            if pause_elapsed > pause_ticks - 10 {
                // Last 10 ticks: clear
                self.input_buffer.clear();
                self.input_cursor = 0;
                self.input_focused = false;
            }
        }
    }

    /// Animated dots with fixed width (3 chars): `.  ` → `.. ` → `...`
    fn dots(&self) -> &'static str {
        match (self.tick / 12) % 3 {
            0 => ".  ",
            1 => ".. ",
            _ => "...",
        }
    }

    fn theme(&self) -> &'static ratatui_themekit::ThemeData {
        &BUILTIN_THEMES[self.theme_index]
    }
}

fn main() {
    let initial_id = std::env::args().nth(1).unwrap_or("catppuccin".to_owned());
    let initial_index = BUILTIN_THEMES
        .iter()
        .position(|t| t.id == initial_id)
        .unwrap_or(0);

    let mut state = AnimState::new(initial_index);
    let mut terminal = ratatui::init();
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::EnableMouseCapture,
        ratatui::crossterm::event::EnableBracketedPaste
    )
    .ok();

    loop {
        terminal
            .draw(|frame| render_showcase(frame, &mut state))
            .expect("render failed");

        state.tick();

        if event::poll(std::time::Duration::from_millis(50)).unwrap_or(false) {
            match event::read() {
                Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                    // q always quits, Esc quits when not in input
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                    if state.input_focused {
                        handle_input_key(&mut state, key.code, key.modifiers);
                    } else {
                        match key.code {
                            KeyCode::Esc => break,
                            KeyCode::Up | KeyCode::Char('k') => {
                                state.theme_index = if state.theme_index == 0 {
                                    BUILTIN_THEMES.len() - 1
                                } else {
                                    state.theme_index - 1
                                };
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                state.theme_index = (state.theme_index + 1) % BUILTIN_THEMES.len();
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::Mouse(mouse)) => {
                    let pos = ratatui::layout::Position::new(mouse.column, mouse.row);

                    match mouse.kind {
                        MouseEventKind::Down(_) => {
                            if state.tab_area.contains(pos) {
                                // Click on tab header — calculate which tab by x position
                                let tab_names = ["Overview", "Logs", "Config", "Metrics"];
                                let mut x_offset = state.tab_area.x;
                                for (i, name) in tab_names.iter().enumerate() {
                                    let is_last = i == tab_names.len() - 1;
                                    let name_width = u16::try_from(name.len()).unwrap_or(10);
                                    let tab_width = name_width + if is_last { 0 } else { 3 };
                                    if pos.x >= x_offset && pos.x < x_offset + tab_width {
                                        state.active_tab = i;
                                        break;
                                    }
                                    x_offset += tab_width;
                                }
                            }
                            state.input_focused = state.input_area.contains(pos);
                        }
                        MouseEventKind::ScrollUp if state.list_area.contains(pos) => {
                            state.list_selected = state.list_selected.saturating_sub(1);
                        }
                        MouseEventKind::ScrollDown if state.list_area.contains(pos) => {
                            state.list_selected =
                                (state.list_selected + 1).min(state.list_len.saturating_sub(1));
                        }
                        _ => {}
                    }
                }
                Ok(Event::Paste(text)) if state.input_focused => {
                    state.input_buffer.insert_str(state.input_cursor, &text);
                    state.input_cursor += text.len();
                }
                _ => {}
            }
        }
    }

    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::DisableMouseCapture,
        ratatui::crossterm::event::DisableBracketedPaste
    )
    .ok();
    ratatui::restore();
}

/// Handle key events when the input field is focused.
fn handle_input_key(
    state: &mut AnimState,
    code: KeyCode,
    modifiers: ratatui::crossterm::event::KeyModifiers,
) {
    use ratatui::crossterm::event::KeyModifiers;
    match code {
        KeyCode::Esc => {
            state.input_focused = false;
        }
        KeyCode::Char(c) => {
            state.input_buffer.insert(state.input_cursor, c);
            state.input_cursor += c.len_utf8();
        }
        KeyCode::Backspace => {
            if state.input_cursor > 0 {
                let prev = state.input_buffer[..state.input_cursor]
                    .char_indices()
                    .next_back()
                    .map_or(0, |(i, _)| i);
                state.input_buffer.drain(prev..state.input_cursor);
                state.input_cursor = prev;
            }
        }
        KeyCode::Delete => {
            if state.input_cursor < state.input_buffer.len() {
                let next = state.input_buffer[state.input_cursor..]
                    .char_indices()
                    .nth(1)
                    .map_or(state.input_buffer.len(), |(i, _)| state.input_cursor + i);
                state.input_buffer.drain(state.input_cursor..next);
            }
        }
        KeyCode::Left => {
            if state.input_cursor > 0 {
                state.input_cursor = state.input_buffer[..state.input_cursor]
                    .char_indices()
                    .next_back()
                    .map_or(0, |(i, _)| i);
            }
        }
        KeyCode::Right => {
            if state.input_cursor < state.input_buffer.len() {
                state.input_cursor = state.input_buffer[state.input_cursor..]
                    .char_indices()
                    .nth(1)
                    .map_or(state.input_buffer.len(), |(i, _)| state.input_cursor + i);
            }
        }
        KeyCode::Home => {
            // Go to start of current line
            state.input_cursor = state.input_buffer[..state.input_cursor]
                .rfind('\n')
                .map_or(0, |i| i + 1);
        }
        KeyCode::End => {
            state.input_cursor = state.input_buffer[state.input_cursor..]
                .find('\n')
                .map_or(state.input_buffer.len(), |i| state.input_cursor + i);
        }
        KeyCode::Enter => {
            if modifiers.contains(KeyModifiers::ALT) {
                // Alt+Enter: insert newline
                state.input_buffer.insert(state.input_cursor, '\n');
                state.input_cursor += 1;
            }
            // Enter alone: do nothing (no submit in showcase)
        }
        _ => {}
    }
}

fn render_showcase(frame: &mut Frame<'_>, state: &mut AnimState) {
    let t = state.theme();
    let area = frame.area();

    // ← themekit: full-screen canvas with themed background + default text
    frame.render_widget(
        ratatui::widgets::Block::default().style(t.style_base()),
        area,
    );

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, main_chunks[0], t);
    render_body(frame, main_chunks[1], t, state);
    render_footer(frame, main_chunks[2], t);
}

fn render_header(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme) {
    let version = env!("CARGO_PKG_VERSION");
    let theme_count = BUILTIN_THEMES.len();
    let title = format!(" ratatui-themekit v{version} \u{2014} {} ", t.name());
    let block = t.block(&title).build(); // ← themekit: themed block with title
    let subtitle = format!(" \u{2014} {theme_count} themes, widget builders, state-aware styles");
    let text = t
        .line() // ← themekit: line compositor
        .accent_bold("  Semantic theme system")
        .dim(subtitle)
        .build();
    let paragraph = Paragraph::new(text).block(block); // ratatui: standard Paragraph
    frame.render_widget(paragraph, area);
}

fn render_body(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, state: &mut AnimState) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14),
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(columns[0]);

    render_spans(frame, left[0], t);
    render_line_compositor(frame, left[1], t, state);
    render_status_line_demo(frame, left[2], t, state);
    render_block_demo(frame, left[3], t);
    render_input_demo(frame, left[4], t, state);
    render_state_styles(frame, left[5], t);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(columns[1]);

    render_table(frame, right[0], t);
    render_list(frame, right[1], t, state);
    render_tabs(frame, right[2], t, state);
    render_bar(frame, right[3], t, state);
    render_gauge(frame, right[4], t, state);
    render_notifications(frame, right[5], t);
    render_themes_list(frame, right[6], t, state.theme_index);
}

/// All lines here use themekit: `t.line()`, `t.fg_*()`, `t.badge()`.
/// Without themekit you'd write `Span::styled("text", Style::default().fg(Color::Rgb(...)))`.
fn render_spans(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme) {
    let block = t.block(" Span Builders (ThemeExt) ").build(); // ← themekit
    let text = Text::from(vec![
        t.line() // ← themekit: each line uses semantic color names
            .accent_bold("fg_accent")
            .dim("  \u{2014} primary brand color")
            .build(),
        t.line()
            .dim("fg_dim")
            .dim("     \u{2014} muted/inactive text")
            .build(),
        t.line()
            .bright("fg_bright")
            .dim("  \u{2014} emphasis text")
            .build(),
        t.line()
            .text("fg_text")
            .dim("    \u{2014} default text color")
            .build(),
        t.line()
            .success("fg_success")
            .dim(" \u{2014} pass / ok")
            .build(),
        t.line()
            .error("fg_error")
            .dim("   \u{2014} fail / error")
            .build(),
        t.line()
            .warning("fg_warning")
            .dim(" \u{2014} pending / warn")
            .build(),
        t.line()
            .info("fg_info")
            .dim("    \u{2014} informational")
            .build(),
        Line::from(vec![
            t.fg_added("+added").build(),
            t.fg_dim("  ").build(),
            t.fg_removed("-removed").build(),
            t.fg_dim("  \u{2014} diff colors").build(),
        ]),
        Line::from(vec![
            t.fg_accent("bold").bold().build(),
            t.fg_dim(" ").build(),
            t.fg_accent("italic").italic().build(),
            t.fg_dim(" ").build(),
            t.fg_accent("underline").underlined().build(),
            t.fg_dim(" ").build(),
            t.fg_accent("strike").crossed_out().build(),
            t.fg_dim(" ").build(),
            t.fg_accent("reverse").reversed().build(),
            t.fg_dim(" \u{2014} modifiers").build(),
        ]),
        Line::from(vec![
            t.badge(" PASS ", t.success()).build(),
            t.fg_dim(" ").build(),
            t.badge(" FAIL ", t.error()).build(),
            t.fg_dim(" ").build(),
            t.badge(" WARN ", t.warning()).build(),
            t.fg_dim(" \u{2014} badge()").build(),
        ]),
    ]);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

/// `t.line()` composes multi-span lines without `vec![]` boilerplate. ← themekit
fn render_line_compositor(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, state: &AnimState) {
    let block = t.block(" Line Compositor ").build();

    // Animate the counts based on progress
    let passed = (u32::from(state.progress) * 47 / 100).max(1);
    let failed = 3u32.saturating_sub(u32::from(state.progress) / 40);
    let pending = 50u32.saturating_sub(passed + failed);

    let text = Text::from(vec![
        t.line()
            .accent_bold("Dashboard")
            .dim(" | ")
            .success_bold(format!("{passed} passed"))
            .dim(", ")
            .error_bold(format!("{failed} failed"))
            .dim(", ")
            .warning(format!("{pending} pending"))
            .build(),
        t.line()
            .dim("[")
            .info("12:34:56")
            .dim("] ")
            .accent("agent.claude")
            .dim(" \u{2192} ")
            .success("task complete")
            .build(),
        t.line()
            .bright("File: ")
            .text("src/main.rs")
            .dim(" (")
            .colored("142 lines", Color::Rgb(180, 180, 180))
            .dim(")")
            .build(),
    ]);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

/// `t.status_line().kv("Key", "Val")` builds key-value status bars. ← themekit
fn render_status_line_demo(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, state: &AnimState) {
    let block = t.block(" Status Line ").build();

    let p = state.progress;

    // Animated build phases with spinning dots
    let dots = state.dots();
    let compiling = format!("compiling{dots}");
    let linking = format!("linking{dots}");
    let testing = format!("testing{dots}");
    let (build_label, build_color): (&str, _) = if p < 25 {
        (compiling.as_str(), t.warning())
    } else if p < 50 {
        (linking.as_str(), t.info())
    } else if p < 75 {
        (testing.as_str(), t.accent())
    } else {
        ("\u{2713} passed", t.success())
    };

    let line_num = format!("{}", 42 + p / 10);
    let coverage = format!("{}%", 80 + u16::from(p) / 5);
    let tests_run = format!("{}/113", u16::from(p) * 113 / 100);
    let tests_color = if p >= 75 { t.success() } else { t.text() };

    let text = Text::from(vec![
        t.status_line()
            .kv("Mode", "Normal")
            .kv("File", "main.rs")
            .kv("Ln", line_num.as_str())
            .build(),
        t.status_line()
            .separator(" | ")
            .kv_colored("Build", build_label, build_color)
            .kv_colored("Tests", tests_run.as_str(), tests_color)
            .kv("Coverage", coverage.as_str())
            .build(),
    ]);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

/// `t.block("Title").focused(true).build()` — themed Block widget. ← themekit
fn render_block_demo(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let normal = t.block(" Normal ").build();
    let p1 = Paragraph::new(t.line().dim("Default borders").build()).block(normal);
    frame.render_widget(p1, chunks[0]);

    let focused = t.block(" Focused ").focused(true).build();
    let p2 = Paragraph::new(t.line().accent("Accent borders").build()).block(focused);
    frame.render_widget(p2, chunks[1]);
}

/// `t.input_styles()` — themed input field colors. ← themekit
/// Input handling (keys, mouse, cursor) is standard ratatui.
fn render_input_demo(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, state: &mut AnimState) {
    let is = t.input_styles();
    let focused = state.input_focused;

    let border = if focused {
        is.border_focused
    } else {
        is.border
    };
    let block = ratatui::widgets::Block::new()
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(border)
        .title(if focused {
            " Input (Esc to unfocus) "
        } else {
            " Input (click to focus) "
        })
        .title_style(is.prompt)
        .style(ratatui::style::Style::default().bg(t.surface()));

    let inner = block.inner(area);
    state.input_area = area; // use outer rect for click hit-testing
    frame.render_widget(block, area);

    if state.input_buffer.is_empty() && !focused {
        // Placeholder
        frame.render_widget(
            Paragraph::new(Line::styled(
                " Type here... (Alt+Enter = newline)",
                is.placeholder,
            )),
            inner,
        );
    } else {
        // Render buffer with cursor — supports multiline via Alt+Enter
        let before = &state.input_buffer[..state.input_cursor];
        let after = &state.input_buffer[state.input_cursor..];

        let cursor_char = if focused && state.tick % 20 < 12 {
            "\u{2588}"
        } else if focused {
            " "
        } else {
            ""
        };

        // Build full text then split by newlines
        let full = format!(" > {before}{cursor_char}{after}");
        let lines: Vec<Line<'_>> = full
            .lines()
            .enumerate()
            .map(|(i, line_text)| {
                if i == 0 {
                    // First line has the prompt
                    t.line().text(line_text.to_owned()).build()
                } else {
                    t.line().dim("   ").text(line_text.to_owned()).build()
                }
            })
            .collect();

        let total_lines = lines.len();
        let visible_height = inner.height as usize;

        // Auto-scroll: keep cursor line visible
        let cursor_line = before.matches('\n').count();
        let scroll_offset = if total_lines > visible_height {
            cursor_line.saturating_sub(visible_height.saturating_sub(1))
        } else {
            0
        };

        let text = Text::from(lines);
        #[allow(clippy::cast_possible_truncation)]
        let paragraph = Paragraph::new(text).scroll((scroll_offset as u16, 0));
        frame.render_widget(paragraph, inner);

        // Show scrollbar when content overflows
        if total_lines > visible_height {
            let sbs = t.scrollbar_styles();
            let scrollbar = ratatui::widgets::Scrollbar::new(
                ratatui::widgets::ScrollbarOrientation::VerticalRight,
            )
            .track_style(sbs.track)
            .thumb_style(sbs.thumb);
            let mut scroll_state =
                ratatui::widgets::ScrollbarState::new(total_lines).position(scroll_offset);
            frame.render_stateful_widget(scrollbar, inner, &mut scroll_state);
        }
    }
}

/// `t.state_styles()` — normal/focused/selected/disabled. ← themekit
fn render_state_styles(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme) {
    let block = t.block(" State-Aware Styles ").build();
    let ss = t.state_styles();
    let text = Text::from(vec![
        Line::styled("  Normal   \u{2014} default text", ss.normal),
        Line::styled("  Focused  \u{2014} accent + bold", ss.focused),
        Line::styled("  Selected \u{2014} bright + surface bg", ss.selected),
        Line::styled("  Disabled \u{2014} dimmed text", ss.disabled),
    ]);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

/// `t.table_styles()` + `zebra_rows()` — themed table with striping. ← themekit
fn render_table(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme) {
    let block = t.block(" Table (zebra rows) ").build();
    let ts = t.table_styles();

    let header = Row::new(["Name", "Role", "Status"]).style(ts.header);
    let rows = vec![
        Row::new(["Alice", "Engineer", "Active"]),
        Row::new(["Bob", "Designer", "Away"]),
        Row::new(["Carol", "PM", "Active"]),
        Row::new(["Dave", "QA", "Offline"]),
        Row::new(["Eve", "DevOps", "Active"]),
    ];
    let striped = zebra_rows(rows, ts.stripe);

    let table = Table::new(
        striped,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .row_highlight_style(ts.highlight)
    .block(block);

    frame.render_widget(table, area);
}

/// `t.list_styles()` + `t.scrollbar_styles()` — themed list + scrollbar. ← themekit
/// Mouse scroll hit-testing and `ListState` are standard ratatui.
fn render_list(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, state: &mut AnimState) {
    let block = t.block(" List + Scrollbar ").build();
    let ls = t.list_styles();
    let sbs = t.scrollbar_styles();

    let items = vec![
        ListItem::new(
            t.line()
                .success("\u{2713}")
                .dim(" ")
                .text("Build succeeded")
                .build(),
        ),
        ListItem::new(
            t.line()
                .success("\u{2713}")
                .dim(" ")
                .text("Lint passed")
                .build(),
        ),
        ListItem::new(
            t.line()
                .error("\u{2717}")
                .dim(" ")
                .text("Test failed: auth_test")
                .build(),
        ),
        ListItem::new(
            t.line()
                .warning("\u{26a0}")
                .dim(" ")
                .text("Deprecation warning")
                .build(),
        ),
        ListItem::new(
            t.line()
                .info("\u{2139}")
                .dim(" ")
                .text("Coverage: 87%")
                .build(),
        ),
        ListItem::new(
            t.line()
                .success("\u{2713}")
                .dim(" ")
                .text("Format check passed")
                .build(),
        ),
        ListItem::new(
            t.line()
                .success("\u{2713}")
                .dim(" ")
                .text("Security audit clean")
                .build(),
        ),
        ListItem::new(
            t.line()
                .warning("\u{26a0}")
                .dim(" ")
                .text("Unused dependency found")
                .build(),
        ),
    ];

    let total = items.len();
    let scroll_pos = state.list_selected.min(total.saturating_sub(1));

    let inner = block.inner(area);
    state.list_area = inner;
    frame.render_widget(block, area);

    let list = List::new(items)
        .highlight_style(ls.highlight)
        .highlight_symbol(ls.symbol)
        .style(ls.base);
    let mut list_state = ratatui::widgets::ListState::default().with_selected(Some(scroll_pos));
    frame.render_stateful_widget(list, inner, &mut list_state);

    // Scrollbar synced with selection
    let scrollbar =
        ratatui::widgets::Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .track_style(sbs.track)
            .thumb_style(sbs.thumb);
    let mut scroll_state = ratatui::widgets::ScrollbarState::new(total).position(scroll_pos);
    frame.render_stateful_widget(scrollbar, inner, &mut scroll_state);
}

/// `t.tab_styles()` — active/inactive tab styling. ← themekit
fn render_tabs(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, state: &mut AnimState) {
    let block = t.block(" Tabs ").build();
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let tab_rows = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(inner);

    // Tab header — store area for mouse click hit-testing
    state.tab_area = tab_rows[0];
    let ts = t.tab_styles();
    let tabs = Tabs::new(["Overview", "Logs", "Config", "Metrics"])
        .style(ts.inactive)
        .highlight_style(ts.active)
        .select(state.active_tab)
        .divider(" | ");
    frame.render_widget(tabs, tab_rows[0]);

    // Tab content — changes per active tab
    let content: Vec<Line<'_>> = match state.active_tab {
        0 => vec![
            t.line()
                .success("  ● ")
                .text("All systems operational")
                .build(),
            t.line().dim("  Uptime: 99.97%").build(),
        ],
        1 => vec![
            t.line().dim("  [14:32] ").text("Build completed").build(),
            t.line()
                .dim("  [14:33] ")
                .warning("Slow query: 340ms")
                .build(),
        ],
        2 => vec![
            t.line().accent("  theme: ").text(t.name()).build(),
            t.line().accent("  locale: ").text("en").build(),
        ],
        _ => vec![
            t.line().text("  Requests: ").accent_bold("1.2k/s").build(),
            t.line().text("  Latency:  ").success("12ms p95").build(),
        ],
    };
    frame.render_widget(Paragraph::new(content), tab_rows[1]);
}

/// `t.bar(percent).width(n).build()` — themed progress bar. ← themekit
fn render_bar(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, state: &AnimState) {
    let block = t.block(" Progress Bar ").build();
    let bar = t.bar(state.progress).width(30).build();
    let paragraph = Paragraph::new(bar).block(block);
    frame.render_widget(paragraph, area);
}

/// `t.gauge_styles()` — themed Gauge widget colors. ← themekit
fn render_gauge(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, state: &AnimState) {
    let block = t.block(" Gauge ").build();
    let gs = t.gauge_styles();
    let ratio = f64::from(state.progress) / 100.0;
    let gauge = Gauge::default()
        .gauge_style(gs.filled)
        .ratio(ratio)
        .label(format!("{}%", state.progress))
        .block(block);
    frame.render_widget(gauge, area);
}

/// `t.notification_styles()` — info/success/warning/error severity. ← themekit
fn render_notifications(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme) {
    let block = t.block(" Notifications ").build();
    let ns = t.notification_styles();
    let text = Text::from(vec![
        Line::styled("  \u{2139} Build deployed successfully", ns.info),
        Line::styled("  \u{2713} All 132 tests passed", ns.success),
        Line::styled("  \u{26a0} Dependency update available", ns.warning),
    ]);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_themes_list(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme, selected: usize) {
    let block = t
        .block(" Built-in Themes (\u{2191}\u{2193} to switch) ")
        .build();
    let lines: Vec<Line<'_>> = BUILTIN_THEMES
        .iter()
        .enumerate()
        .map(|(i, theme)| {
            if i == selected {
                t.line()
                    .accent_bold("\u{25b6} ")
                    .accent_bold(theme.name)
                    .dim(format!(" ({})", theme.id))
                    .build()
            } else {
                t.line()
                    .dim("  ")
                    .text(theme.name)
                    .dim(format!(" ({})", theme.id))
                    .build()
            }
        })
        .collect();
    let paragraph = Paragraph::new(Text::from(lines)).block(block);
    frame.render_widget(paragraph, area);
}

/// `t.status_line()` for the footer bar. ← themekit
fn render_footer(frame: &mut Frame<'_>, area: Rect, t: &dyn Theme) {
    let version = env!("CARGO_PKG_VERSION");
    let theme_count = format!("{}", BUILTIN_THEMES.len());
    let status = t
        .status_line()
        .kv("Theme", t.name())
        .kv_colored("v", version, t.accent())
        .kv("Themes", theme_count.as_str())
        .kv("Navigate", "\u{2191}\u{2193}")
        .kv("Quit", "q")
        .build();
    let paragraph = Paragraph::new(status);
    frame.render_widget(paragraph, area);
}
