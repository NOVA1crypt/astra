# Astra

Navigateur web cross-platform construit en Rust avec WebKit.

## Plateformes

| OS | Moteur |
|----|--------|
| macOS | WebKit (WKWebView) |
| Linux | WebKit2GTK |
| Windows | WebView2 (Edge) |

## Fonctionnalités

- Navigation avec historique (retour / avant)
- Barre d'adresse — URL directe, domaine ou recherche
- Rechargement de page
- Page d'accueil personnalisée
- Indicateur de chargement
- Raccourcis clavier (`Cmd/Ctrl+L`, `Cmd/Ctrl+R`, `F5`)
- Moteur de recherche : Brave Search

## Stack

- **Rust** — logique principale
- **wry** — WebView cross-platform
- **tao** — fenêtre native cross-platform
- **rusqlite** — historique (SQLite)

## Installation

### Prérequis

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
git clone https://github.com/TON_USER/astra.git
cd astra
cargo run
```

### Release

```
cargo build --release
./target/release/astra
```

## Structure

```
src/
├── main.rs
├── app.rs              — event loop
├── browser/
│   ├── window.rs       — fenêtre + WebViews
│   └── ipc.rs          — messages toolbar ↔ Rust
├── features/
│   └── history.rs      — historique SQLite
└── utils/
    └── url.rs          — résolution URL / recherche
ui/
├── toolbar/            — barre de navigation (HTML/CSS/JS)
└── newtab/             — page d'accueil
```

## Licence

MIT
