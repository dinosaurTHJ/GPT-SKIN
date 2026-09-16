# ReTheme Theme Skill

Install the complete ReTheme theme-development Skill without cloning the ReTheme repository:

```bash
pnpm dlx @duxweb/retheme-theme-skill install
```

Restart Codex, then ask it to use `retheme-theme-development` to create or update a theme.

The package includes the Skill references, starter theme, and a native validator for macOS Apple Silicon/Intel, Windows x64, and Linux x64.

Create a starter theme:

```bash
pnpm dlx @duxweb/retheme-theme-skill create ./my-theme
```

Validate a theme directory or source ZIP directly:

```bash
pnpm dlx @duxweb/retheme-theme-skill validate /absolute/path/to/theme
```
