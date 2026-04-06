//! The `Theme` trait — semantic color contract.

use ratatui::style::Color;

/// Blends background toward surface at 70% for visible stripe contrast.
/// Falls back to `surface` when background is `Color::Reset`.
fn blend_bg_surface(bg: Color, surface: Color) -> Color {
    match (bg, surface) {
        (Color::Rgb(br, bg_g, bb), Color::Rgb(sr, sg, sb)) => {
            // 70% from background toward surface — visible stripe contrast
            let lerp = |a: u8, b: u8| -> u8 {
                let a16 = i32::from(a);
                let b16 = i32::from(b);
                (a16 + (b16 - a16) * 7 / 10) as u8
            };
            Color::Rgb(lerp(br, sr), lerp(bg_g, sg), lerp(bb, sb))
        }
        _ => surface,
    }
}

/// Semantic color contract for ratatui applications.
///
/// Define **what** each color means, not what RGB value it is.
/// Every render function uses these slots. Swap themes at runtime
/// and every widget updates automatically.
///
/// # Required methods (15)
///
/// Core identity + 13 color slots + surface.
///
/// # Derived methods (10+)
///
/// `block_*` and `indicator_*` methods derive from core slots.
/// Override them for fine-grained control.
pub trait Theme: Send + Sync {
    /// Human-readable theme name (e.g. `"Catppuccin Mocha"`).
    fn name(&self) -> &str;
    /// Short identifier for config files (e.g. `"catppuccin"`).
    fn id(&self) -> &str;

    // ── Brand ────────────────────────────────────────────────
    /// Primary brand/accent color.
    fn accent(&self) -> Color;
    /// Secondary accent (less prominent highlights).
    fn accent_dim(&self) -> Color;

    // ── Text ─────────────────────────────────────────────────
    /// Default text color.
    fn text(&self) -> Color;
    /// Dimmed/muted text (timestamps, hints, inactive elements).
    fn text_dim(&self) -> Color;
    /// Bright text for emphasis (bold titles, active elements).
    fn text_bright(&self) -> Color;

    // ── Status ��──────────────────────────────────────────────
    /// Success / passed / running.
    fn success(&self) -> Color;
    /// Error / failed.
    fn error(&self) -> Color;
    /// Warning / pending / in-progress.
    fn warning(&self) -> Color;
    /// Informational / neutral highlight.
    fn info(&self) -> Color;

    // ── Diff ─────────────────────────────────────────────────
    /// Lines added.
    fn diff_added(&self) -> Color;
    /// Lines removed.
    fn diff_removed(&self) -> Color;
    /// Context/unchanged lines.
    fn diff_context(&self) -> Color;

    // ── Structure ─��──────────────────────────────────────────
    /// Border/separator color.
    fn border(&self) -> Color;
    /// Background highlight for focused/selected elements.
    fn surface(&self) -> Color;

    // ── Derived defaults ─────────────────────────────────────

    /// Application background color.
    ///
    /// Defaults to `Color::Reset` (terminal background). Override for
    /// apps that force a specific background (e.g., fullscreen dashboards).
    fn background(&self) -> Color {
        Color::Reset
    }

    /// Zebra stripe background — alternating row color for tables/lists.
    ///
    /// Defaults to a blend between `background` and `surface` (midpoint).
    /// Override for precise control. Falls back to `surface` when
    /// `background` is `Color::Reset` (no real background defined).
    fn stripe(&self) -> Color {
        blend_bg_surface(self.background(), self.surface())
    }

    /// Color for file-read operations.
    fn block_file_read(&self) -> Color {
        self.text_dim()
    }
    /// Color for file-edit operations.
    fn block_file_edit(&self) -> Color {
        self.diff_added()
    }
    /// Color for command/shell operations.
    fn block_command(&self) -> Color {
        self.text_bright()
    }
    /// Color for thinking/reasoning indicators.
    fn block_thinking(&self) -> Color {
        self.text_dim()
    }
    /// Color for passed indicators.
    fn block_pass(&self) -> Color {
        self.success()
    }
    /// Color for failed indicators.
    fn block_fail(&self) -> Color {
        self.error()
    }
    /// Color for system messages.
    fn block_system(&self) -> Color {
        self.text_dim()
    }
    /// Pending indicator color.
    fn indicator_pending(&self) -> Color {
        self.text_dim()
    }
    /// Running indicator color.
    fn indicator_running(&self) -> Color {
        self.warning()
    }
    /// Passed indicator color.
    fn indicator_passed(&self) -> Color {
        self.success()
    }
    /// Failed indicator color.
    fn indicator_failed(&self) -> Color {
        self.error()
    }
    /// Skipped indicator color.
    fn indicator_skipped(&self) -> Color {
        self.text_dim()
    }
}
