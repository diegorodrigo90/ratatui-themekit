# Examples

## Showcase

Interactive demo of all ratatui-themekit builders and widgets.

```bash
cargo run --example showcase
```

Controls: `↑`/`↓` switch themes, `q` to quit.

## Theme Gallery

All 10 built-in themes rendered with the showcase:

### Catppuccin Mocha (default)
![Catppuccin Mocha](../assets/screenshots/catppuccin.png)

### Dracula
![Dracula](../assets/screenshots/dracula.png)

### Nord
![Nord](../assets/screenshots/nord.png)

### Gruvbox Dark
![Gruvbox Dark](../assets/screenshots/gruvbox.png)

### One Dark
![One Dark](../assets/screenshots/one-dark.png)

### Solarized Dark
![Solarized Dark](../assets/screenshots/solarized-dark.png)

### Tailwind Dark
![Tailwind Dark](../assets/screenshots/tailwind-dark.png)

### Tokyo Night
![Tokyo Night](../assets/screenshots/tokyo-night.png)

### Rose Pine
![Rose Pine](../assets/screenshots/rose-pine.png)

### Terminal Native
![Terminal Native](../assets/screenshots/terminal-native.png)

## Regenerating Screenshots

```bash
./scripts/generate-screenshots.sh        # all PNGs + GIF
./scripts/generate-screenshots.sh png    # PNGs only
./scripts/generate-screenshots.sh gif    # GIF only
```

Requires [vhs](https://github.com/charmbracelet/vhs).
