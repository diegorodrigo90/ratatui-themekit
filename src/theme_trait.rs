//! The `Theme` trait — semantic color contract.

use ratatui::style::Color;

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
