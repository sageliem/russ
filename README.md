# RSS reader for the terminal
--------------------------
### simple RSS/Atom reader written in Rust with vim-like navigation
--------------------------
## Configuration
Add feeds with the config file at `$HOME/.config/russ/config.toml`:

```toml
feeds = [
    "https://example.com/rss"
]
```

Feeds are stored as JSON files (with hash filenames) at `$HOME/.russ/feeds`.

## Navigation
Rudimentary vim-like navigation:
- `jk` to scroll up and down
- `Enter` to select
- `q` to go back/quit
