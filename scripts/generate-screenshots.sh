#!/usr/bin/env bash
# Generate PNG screenshots and GIF for each theme.
# Requires: vhs (https://github.com/charmbracelet/vhs)
#
# Usage:
#   ./scripts/generate-screenshots.sh          # all PNGs + GIF
#   ./scripts/generate-screenshots.sh png      # PNGs only
#   ./scripts/generate-screenshots.sh gif      # GIF only
#   ./scripts/generate-screenshots.sh png nord  # single theme PNG

set -euo pipefail

# Check vhs is installed
if ! command -v vhs &>/dev/null; then
    echo "Error: vhs is required but not installed."
    echo "Install: https://github.com/charmbracelet/vhs#installation"
    echo "  Arch:   pacman -S vhs"
    echo "  macOS:  brew install vhs"
    echo "  Go:     go install github.com/charmbracelet/vhs@latest"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SCREENSHOTS_DIR="$PROJECT_DIR/assets/screenshots"
BIN="$PROJECT_DIR/target/release/examples/showcase"

mkdir -p "$SCREENSHOTS_DIR"

# Build once
echo "Building showcase..."
cargo build --example showcase --release --quiet
echo "Binary ready: $BIN"

# theme_id:filename
THEMES=(
    "catppuccin:catppuccin"
    "dracula:dracula"
    "nord:nord"
    "gruvbox:gruvbox"
    "one-dark:one-dark"
    "solarized:solarized-dark"
    "tailwind:tailwind-dark"
    "tokyo-night:tokyo-night"
    "rose-pine:rose-pine"
    "terminal:terminal-native"
)

generate_png() {
    local id="$1"
    local filename="$2"
    local output="$SCREENSHOTS_DIR/${filename}.png"
    local tape="/tmp/tk-screenshot-${filename}.tape"

    echo "  PNG: $filename..."

    cat > "$tape" <<TAPE
Output "$output"
Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 800
Set Padding 20

Hide
Type "$BIN $id"
Enter
Sleep 2s
Show
Sleep 500ms
Screenshot "$output"
Hide
Type "q"
Sleep 300ms
TAPE

    if ! vhs "$tape" 2>/dev/null; then
        echo "    WARN: vhs failed for $filename"
    fi
    rm -f "$tape"
}

generate_gif() {
    local output="$PROJECT_DIR/assets/showcase.gif"
    local tape="/tmp/tk-showcase.tape"

    echo "  GIF: showcase.gif..."

    cat > "$tape" <<TAPE
Output "$output"
Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 800
Set Padding 20
Set Framerate 15
Set PlaybackSpeed 1.2

Hide
Type "$BIN"
Enter
Sleep 2s
Show

Sleep 2s

Down
Sleep 1.2s
Down
Sleep 1.2s
Down
Sleep 1.2s
Down
Sleep 1.2s
Down
Sleep 1.2s
Down
Sleep 1.2s
Down
Sleep 1.2s
Down
Sleep 1.2s
Down
Sleep 1.2s

Sleep 1.5s

Up
Sleep 0.8s
Up
Sleep 0.8s
Up
Sleep 0.8s
Up
Sleep 0.8s
Up
Sleep 0.8s
Up
Sleep 0.8s
Up
Sleep 0.8s
Up
Sleep 0.8s
Up
Sleep 2s

Hide
Type "q"
Sleep 300ms
TAPE

    if ! vhs "$tape" 2>/dev/null; then
        echo "    WARN: vhs failed for GIF"
    fi
    rm -f "$tape"
}

MODE="${1:-all}"
SINGLE="${2:-}"

case "$MODE" in
    png)
        if [ -n "$SINGLE" ]; then
            for entry in "${THEMES[@]}"; do
                id="${entry%%:*}"
                filename="${entry##*:}"
                if [ "$id" = "$SINGLE" ] || [ "$filename" = "$SINGLE" ]; then
                    generate_png "$id" "$filename"
                    break
                fi
            done
        else
            for entry in "${THEMES[@]}"; do
                generate_png "${entry%%:*}" "${entry##*:}"
            done
        fi
        ;;
    gif)
        generate_gif
        ;;
    all)
        for entry in "${THEMES[@]}"; do
            generate_png "${entry%%:*}" "${entry##*:}"
        done
        generate_gif
        ;;
    *)
        echo "Usage: $0 [png|gif|all] [theme_id]"
        exit 1
        ;;
esac

echo ""
echo "Done. Assets:"
ls -lh "$SCREENSHOTS_DIR"/*.png 2>/dev/null | awk '{print "  " $5 "\t" $NF}'
ls -lh "$PROJECT_DIR/assets/showcase.gif" 2>/dev/null | awk '{print "  " $5 "\t" $NF}'
