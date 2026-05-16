# Astra

A cross-platform web browser built in Rust with WebKit-based web views.

## Platforms

| OS | Engine |
|----|--------|
| macOS | WebKit (WKWebView) |
| Linux | WebKit2GTK |
| Windows | WebView2 (Edge) |

## Features

- Browsing history with back and forward navigation
- Address bar for direct URLs, domains, or search queries
- Page reload
- Custom home page
- Loading indicator
- Keyboard shortcuts (`Cmd/Ctrl+L`, `Cmd/Ctrl+R`, `F5`)
- Brave Search as the default search engine

## Stack

- **Rust** - core application logic
- **wry** - cross-platform WebView integration
- **tao** - native cross-platform windowing
- **rusqlite** - SQLite-backed browsing history

## Installation

### Requirements

**macOS**
```
xcode-select --install
```

**Linux**
```
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev
```

**Windows**
```
winget install Microsoft.EdgeWebView2Runtime
```

### Build

```
git clone https://github.com/NOVA1crypt/astra.git
cd astra
cargo run
```

### Release Build

```
cargo build --release
./target/release/astra
```

## Project Structure

```
src/
├── main.rs
├── app.rs              - event loop
├── browser/
│   ├── window.rs       - window and WebViews
│   └── ipc.rs          - toolbar to Rust messages
├── features/
│   └── history.rs      - SQLite history
└── utils/
    └── url.rs          - URL and search resolution
ui/
├── toolbar/            - navigation bar (HTML/CSS/JS)
└── newtab/             - home page
```

## License

MIT
