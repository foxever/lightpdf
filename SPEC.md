# LightPDF - Project Specification

## 1. Overview

A lightweight, cross-platform PDF reader built with Rust and GPUI, using PDFium as the rendering engine. Inspired by SumatraPDF's clean and efficient interface design.

## 2. Tech Stack

| Module | Technology | Reason |
|--------|------------|--------|
| Language | Rust | Memory safety, high performance, cross-platform |
| UI Framework | GPUI | Native Rust UI framework with GPU acceleration |
| PDF Engine | PDFium (pdfium-render) | High-quality rendering, Chrome's PDF engine |
| Build | Cargo + Cross | Official Rust build tools with cross-compilation |

## 3. Core Features

### 3.1 Multi-Tab Interface
- **Tab Bar**: Display multiple open documents as tabs
- **Tab Management**: Open, close, switch between tabs
- **Tab States**: Modified indicator for unsaved changes

### 3.2 PDF Rendering
- High-fidelity page rendering (text, images, vectors)
- Page zoom (50%-200%)
- Page rotation (90° clockwise/counter-clockwise)
- Outline navigation (sidebar)

### 3.3 Sidebar
- **Bookmarks/Outline**: Tree view of document table of contents
- **Favorites**: User-defined bookmarks with custom titles
- **Toggle Sidebar**: Show/hide sidebar with one click

### 3.4 Menu Bar
- **File Menu**: New window, Open, Close, Save As, Print, Recent files, Favorites, Exit
- **View Menu**: Zoom (Fit Width, Fit Page, Actual Size), Rotate, Full Screen, Toggle Sidebar
- **Go Menu**: Previous Page, Next Page, Go to Page, First Page, Last Page
- **Zoom Menu**: Zoom In, Zoom Out, Fit Width, Fit Page, Actual Size
- **Favorites Menu**: Add to Favorites, Manage Favorites
- **Settings Menu**: Theme, Language, About
- **Help Menu**: Documentation, About

### 3.5 Toolbar
- **Navigation**: Open File, Previous Page, Next Page, Go to Page
- **View**: Fit Width, Fit Page, Actual Size, Rotate Clockwise, Rotate Counter-Clockwise
- **Zoom**: Zoom In, Zoom Out
- **Actions**: Full Screen, Toggle Sidebar, Search

### 3.6 Status Bar
- Page information (current / total)
- Zoom level
- File name

### 3.7 UI
- Modern, clean interface
- Light/dark themes
- Multi-language (English, Chinese, Spanish)

### 3.8 Cross-platform
- macOS menu bar integration (native system menu on macOS)
- Native window controls

## 4. Platform Support

| Platform | Architectures | Menu Placement |
|----------|---------------|----------------|
| Windows | x86_64, aarch64 | Window menu bar |
| macOS | x86_64, arm64 | System menu bar |
| Linux | x86_64, aarch64 | Window menu bar |

## 5. Keyboard Shortcuts

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Window | Ctrl+N | Cmd+N |
| Open File | Ctrl+O | Cmd+O |
| Close Tab | Ctrl+W | Cmd+W |
| Save As | Ctrl+Shift+S | Cmd+Shift+S |
| Print | Ctrl+P | Cmd+P |
| Exit | Ctrl+Q | Cmd+Q |
| Zoom In | Ctrl++ | Cmd++ |
| Zoom Out | Ctrl+- | Cmd+- |
| Actual Size | Ctrl+0 | Cmd+0 |
| Fit Width | Ctrl+2 | Cmd+2 |
| Fit Page | Ctrl+1 | Cmd+1 |
| Previous Page | PageUp, Ctrl+Left | PageUp, Cmd+Left |
| Next Page | PageDown, Ctrl+Right | PageDown, Cmd+Right |
| First Page | Home | Home |
| Last Page | End | End |
| Go to Page | Ctrl+G | Cmd+G |
| Rotate Clockwise | Ctrl+R | Cmd+R |
| Rotate Counter-Clockwise | Ctrl+Shift+R | Cmd+Shift+R |
| Full Screen | F11 | F11, Cmd+Ctrl+F |
| Toggle Sidebar | F9 | F9 |
| Add to Favorites | Ctrl+D | Cmd+D |
| Next Tab | Ctrl+Tab | Cmd+Tab |
| Previous Tab | Ctrl+Shift+Tab | Cmd+Shift+Tab |
| Close Tab | Ctrl+W | Cmd+W |

## 6. UI Layout

```
┌──────────────────────────────────────────────────────────────────┐
│  [Tab 1] [Tab 2] [Tab 3]  [+]                    │  ← Tab Bar
├──────────────────────────────────────────────────────────────────┤
│  File  View  Go  Zoom  Favorites  Settings  Help    │  ← Menu Bar
├────────┬───────────────────────────────────────────────────────┤
│      │  📂 ◀ ▶  ↔ □ 1:1 ↻ ↺  ⏎ 🔍            │  ← Toolbar
│      ├───────────────────────────────────────────────────────┤
│ Side │                                                   │
│  bar │                PDF View Area                      │
│      │                                                   │
│      │                                                   │
│      │                                                   │
│      │                                                   │
├──────┴───────────────────────────────────────────────────────┤
│  Page: 1 / 100    Zoom: 100%    File: doc.pdf  │  ← Status Bar
└───────────────────────────────────────────────────────────────┘
```

## 7. Project Structure

```
lightpdf/
├── src/
│   ├── main.rs              # Entry point
│   ├── app/
│   │   ├── mod.rs           # Core app logic, UI rendering
│   │   ├── state.rs         # Global state management
│   │   ├── tabs.rs          # Tab management
│   │   ├── menu.rs          # Menu bar
│   │   └── ui/
│   │       ├── toolbar.rs    # Toolbar
│   │       ├── sidebar.rs    # Sidebar (outline/favorites)
│   │       ├── tab_bar.rs    # Tab bar
│   │       ├── status_bar.rs # Status bar
│   │       └── pdf_view.rs # PDF view
│   ├── pdf/                 # PDF handling
│   │   ├── mod.rs           # Document management, rendering
│   │   ├── loader.rs        # File loading
│   │   └── renderer.rs      # Page rendering helpers
│   ├── i18n.rs              # Internationalization
│   ├── theme.rs             # Theme configuration
│   ├── platform/            # Platform adapters
│   │   ├── mod.rs
│   │   ├── windows.rs       # Windows menu integration
│   │   ├── macos.rs         # macOS menu integration
│   │   └── linux.rs         # Linux menu integration
│   └── utils/               # Utilities
│       ├── favorites.rs     # Favorites management
│       ├── recent_files.rs # Recent files
│       └── shortcuts.rs    # Keyboard shortcuts
├── Cargo.toml
├── build.sh                 # Linux/macOS build script
├── build.ps1                # Windows build script
└── .github/workflows/       # CI/CD
```

## 8. Data Storage (Cross-platform)

Using `dirs-next` for standard platform directories:

| Data Type | Windows | macOS | Linux |
|-----------|---------|-------|
| Config/Favorites | `%APPDATA%/lightpdf/config.json` | `~/Library/Application Support/lightpdf/config.json` | `~/.config/lightpdf/config.json` |
| Recent Files | Same as config | Same as config | Same as config |
| Cache | `%LOCALAPPDATA%/lightpdf/cache/` | `~/Library/Caches/lightpdf/` | `~/.cache/lightpdf/` |

## 9. Dependencies

```toml
[dependencies]
gpui = { version = "0.2.2", default-features = false }
pdfium-render = "0.8"
image = { version = "0.25", features = ["png", "jpeg"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["fs", "rt-multi-thread"] }
dirs-next = "2.0.0"
```

## 10. Implementation Roadmap

### Phase 1: Tab System (Priority: High)
- [ ] Tab bar UI
- [ ] Tab state management
- [ ] Open/close/switch tabs
- [ ] Multi-tab PDF rendering

### Phase 2: Menu Bar (Priority: High)
- [ ] File menu (Open, Close, Save As, Print)
- [ ] View menu (Zoom, Rotate, Full Screen)
- [ ] Go menu (Page navigation)
- [ ] Zoom menu
- [ ] Favorites menu
- [ ] Settings menu (Theme, Language)
- [ ] Help menu
- [ ] Platform-specific menu integration

### Phase 3: Toolbar (Priority: High)
- [ ] Toolbar UI
- [ ] Navigation buttons
- [ ] View buttons
- [ ] Zoom buttons
- [ ] Action buttons

### Phase 4: Sidebar Enhancements (Priority: Medium)
- [ ] Favorites management
- [ ] Favorites persistence
- [ ] Sidebar toggle

### Phase 5: Recent Files & Favorites (Priority: Medium)
- [ ] Recent files list
- [ ] Add/remove favorites
- [ ] Favorites persistence
- [ ] Recent files persistence

### Phase 6: Status Bar (Priority: Low)
- [ ] Page info
- [ ] Zoom display
- [ ] File name

### Completed
- [x] Project architecture
- [x] GPUI integration
- [x] PDFium rendering
- [x] Page navigation and zoom
- [x] Page rotation
- [x] Outline parsing
- [x] Theme switching
- [x] i18n support
- [x] Cross-platform CI/CD

### Planned
- [ ] Multi-tab interface
- [ ] Full menu bar
- [ ] Enhanced toolbar
- [ ] Favorites management
- [ ] Recent files
- [ ] Full-screen mode
- [ ] Drag & drop
- [ ] Print support
- [ ] Search

## 11. Internationalization (i18n)

### Supported Languages
- English (en)
- Chinese (zh)
- Spanish (es)

### i18n Structure
- All UI text in `src/i18n.rs
- Language selection in Settings menu
- Default language follows system locale

## 12. License

MIT License
