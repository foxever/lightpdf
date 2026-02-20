# LightPDF

A lightweight, cross-platform PDF reader built with Rust and GPUI.

## Features

- **PDF Rendering**: High-fidelity page rendering using PDFium engine
- **Navigation**: Page navigation, zoom, rotation
- **Outline**: Table of contents sidebar
- **Themes**: Light/dark mode support
- **i18n**: Multi-language support (English, Chinese, Spanish)

## Tech Stack

- **Language**: Rust
- **UI Framework**: GPUI 0.2.2
- **PDF Engine**: PDFium (pdfium-render 0.8)
- **Platforms**: Windows / macOS / Linux

## Quick Start

### Requirements
- Rust 1.70+

### Build & Run

```bash
# Clone
git clone <repository-url>
cd lightpdf

# Run
cargo run

# Open a PDF file
cargo run -- <path-to-pdf>
```

### Cross-platform Build

```bash
# Current platform
./build.sh

# All platforms
./build.sh --all

# Or use Makefile
make build        # Current platform
make build-all    # All platforms
```

## Usage

| Action | Control |
|--------|---------|
| Open file | Click 📂 button |
| Navigate | ◀ / ▶ buttons |
| Zoom | − / + buttons |
| Rotate | ↻ / ↺ buttons |
| Theme | Click moon/sun icon |
| Language | Click flag icon |

## Roadmap

### Done
- [x] PDF rendering with PDFium
- [x] Page navigation and zoom
- [x] Page rotation
- [x] Outline navigation
- [x] Light/dark themes
- [x] Multi-language support
- [x] Cross-platform CI/CD

### TODO
- [ ] Bookmarks
- [ ] Recent files
- [ ] Full-screen mode
- [ ] Drag & drop
- [ ] Print support
- [ ] Search

## License

MIT License
