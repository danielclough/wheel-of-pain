# Awful Archaeology - Wheel of Pain

Interactive spinning wheel for Milo Rossi's Awful Archaeology series.

## Features

- 🎯 **Spinning Wheel**: Random selection with smooth animation
- 🖱️ **Manual Control**: Drag to spin the wheel manually
- 🏷️ **Category Filtering**: Word cloud interface with frequency-based sizing
- 📺 **Episode Filter**: View existing episodes, future topics, or both
- 📝 **Detailed Modals**: Full information including claims, descriptions, and YouTube links

## Setup

1. Install Rust and Trunk:
```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

2. Create your `data.yaml` file in the project root with your entries

3. Run the development server:
```bash
trunk serve
```

4. Open http://localhost:8080

## Build for Production

```bash
trunk build --release
```

The output will be in the `dist/` directory.

## Project Structure

```
wheel-of-pain/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── data.yaml
└── src/
    └── main.rs
```

## YAML Format

```yaml
-
  title: Entry Title
  claim: The pseudoarchaeological claim
  description: Detailed debunking
  category: [category1, category2]
  episode: 1  # or [1, 1.5] for multiple parts
  url: https://youtube.com/...  # or list of URLs
  drink: Optional drink featured
  sponsor: Optional sponsor
```

## Technologies

- **Leptos 0.8**: Reactive UI framework
- **WASM**: Runs entirely in the browser
- **SVG**: Smooth wheel graphics
- **Serde**: YAML parsing

## License

MIT
