# teasr markdown capture

Render any `.md` file as a styled screenshot — no dev server, no external tools.

## Features

| Feature | Description |
|---------|-------------|
| **GitHub Flavored** | Tables, task lists, autolinks, strikethrough |
| **Themes** | Light and dark GitHub-style themes |
| **Custom CSS** | Bring your own stylesheet |
| **Full page** | Capture entire document or just the viewport |

## Example config

```toml
[[scenes]]
type = "web"
uri = "./README.md"
theme = "light"
```

> **Tip:** Any `.md` file passed to a `web` scene is auto-detected and rendered with the bundled GitHub-style template — add `wait` and `snapshot` steps to capture multiple states.
