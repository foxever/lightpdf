#!/bin/bash

# Build script for LightPDF
# Supports: macOS (arm64, x86_64), Linux (x86_64, aarch64), Windows (x86_64)

set -e

PROJECT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
TARGET_DIR="$PROJECT_DIR/target"
DIST_DIR="$PROJECT_DIR/dist"

mkdir -p "$DIST_DIR"

echo "======================================"
echo "  LightPDF Build Script"
echo "======================================"
echo ""

# Function to check required tools
check_tools() {
    echo "Checking required tools..."

    if ! command -v cargo &> /dev/null; then
        echo "ERROR: cargo (Rust toolchain) is not installed!"
        exit 1
    fi

    echo "✓ cargo is installed"
}

# Build for current platform
build_current() {
    echo ""
    echo "Building for current platform..."
    
    cargo build --release
    
    echo "✓ Build completed for current platform"
}

# Build for macOS arm64
build_macos_arm64() {
    echo ""
    echo "Building for macOS (arm64)..."
    
    if [[ "$(uname -s)" == "Darwin" ]]; then
        cargo build --release --target aarch64-apple-darwin
    else
        if command -v cross &> /dev/null; then
            cross build --release --target aarch64-apple-darwin
        else
            echo "ERROR: cross is not installed! Install with: cargo install cross"
            exit 1
        fi
    fi
    
    echo "✓ Build completed for macOS (arm64)"
}

# Build for macOS x86_64
build_macos_x86_64() {
    echo ""
    echo "Building for macOS (x86_64)..."
    
    if [[ "$(uname -s)" == "Darwin" ]]; then
        cargo build --release --target x86_64-apple-darwin
    else
        if command -v cross &> /dev/null; then
            cross build --release --target x86_64-apple-darwin
        else
            echo "ERROR: cross is not installed! Install with: cargo install cross"
            exit 1
        fi
    fi
    
    echo "✓ Build completed for macOS (x86_64)"
}

# Build for Linux x86_64
build_linux_x86_64() {
    echo ""
    echo "Building for Linux (x86_64)..."
    
    if [[ "$(uname -s)" == "Linux" && "$(uname -m)" == "x86_64" ]]; then
        cargo build --release --target x86_64-unknown-linux-gnu
    else
        if command -v cross &> /dev/null; then
            cross build --release --target x86_64-unknown-linux-gnu
        else
            echo "ERROR: cross is not installed! Install with: cargo install cross"
            exit 1
        fi
    fi
    
    echo "✓ Build completed for Linux (x86_64)"
}

# Build for Linux aarch64
build_linux_aarch64() {
    echo ""
    echo "Building for Linux (aarch64)..."
    
    if [[ "$(uname -s)" == "Linux" && "$(uname -m)" == "aarch64" ]]; then
        cargo build --release --target aarch64-unknown-linux-gnu
    else
        if command -v cross &> /dev/null; then
            cross build --release --target aarch64-unknown-linux-gnu
        else
            echo "ERROR: cross is not installed! Install with: cargo install cross"
            exit 1
        fi
    fi
    
    echo "✓ Build completed for Linux (aarch64)"
}

# Build for Windows x86_64
build_windows_x86_64() {
    echo ""
    echo "Building for Windows (x86_64)..."
    
    if command -v cross &> /dev/null; then
        cross build --release --target x86_64-pc-windows-msvc
    else
        echo "ERROR: cross is not installed! Install with: cargo install cross"
        exit 1
    fi
    
    echo "✓ Build completed for Windows (x86_64)"
}

# Package macOS app
package_macos() {
    local target=$1
    local arch=""
    
    case $target in
        "aarch64-apple-darwin")
            arch="arm64"
            ;;
        "x86_64-apple-darwin")
            arch="x86_64"
            ;;
        *)
            echo "ERROR: Invalid macOS target: $target"
            return 1
            ;;
    esac
    
    echo ""
    echo "Packaging macOS ($arch) app..."
    
    local app_name="LightPDF"
    local app_bundle="$DIST_DIR/$app_name-$arch.app"
    local binary="$TARGET_DIR/$target/release/lightpdf"
    local libpdfium="$PROJECT_DIR/lib/libpdfium.dylib"
    local icon_svg="$PROJECT_DIR/resources/icon.svg"
    
    # Clean and create app bundle
    rm -rf "$app_bundle"
    mkdir -p "$app_bundle/Contents/MacOS"
    mkdir -p "$app_bundle/Contents/Resources"
    
    # Copy binary
    cp "$binary" "$app_bundle/Contents/MacOS/"
    chmod +x "$app_bundle/Contents/MacOS/lightpdf"
    
    # Copy libpdfium
    if [ -f "$libpdfium" ]; then
        cp "$libpdfium" "$app_bundle/Contents/MacOS/"
    fi
    
    # Copy icon resources
    if [ -f "$icon_svg" ]; then
        cp "$icon_svg" "$app_bundle/Contents/Resources/icon.svg"
    fi
    
    # Try to copy .icns if available
    local icns_file="$PROJECT_DIR/resources/macos/icon.icns"
    if [ -f "$icns_file" ]; then
        cp "$icns_file" "$app_bundle/Contents/Resources/icon.icns"
    fi
    
    # Create Info.plist
    cat > "$app_bundle/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>lightpdf</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.lightpdf</string>
    <key>CFBundleName</key>
    <string>LightPDF</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>CFBundleIconFile</key>
    <string>icon</string>
</dict>
</plist>
EOF
    
    echo "✓ macOS app package created at: $app_bundle"
}

# Package Linux
package_linux() {
    local target=$1
    local arch=""
    
    case $target in
        "x86_64-unknown-linux-gnu")
            arch="x86_64"
            ;;
        "aarch64-unknown-linux-gnu")
            arch="aarch64"
            ;;
        *)
            echo "ERROR: Invalid Linux target: $target"
            return 1
            ;;
    esac
    
    echo ""
    echo "Packaging Linux ($arch)..."
    
    local binary="$TARGET_DIR/$target/release/lightpdf"
    local package_dir="$DIST_DIR/lightpdf-linux-$arch"
    local libpdfium="$PROJECT_DIR/lib/libpdfium.so"
    local icon_svg="$PROJECT_DIR/resources/icon.svg"
    
    rm -rf "$package_dir"
    mkdir -p "$package_dir"
    mkdir -p "$package_dir/icons"
    
    cp "$binary" "$package_dir/"
    chmod +x "$package_dir/lightpdf"
    
    # Copy libpdfium if exists
    if [ -f "$libpdfium" ]; then
        cp "$libpdfium" "$package_dir/"
    fi
    
    # Copy icon resources
    if [ -f "$icon_svg" ]; then
        cp "$icon_svg" "$package_dir/icons/icon.svg"
    fi
    
    # Copy PNG icons if available
    for size in 16 32 48 128 256; do
        local png_icon="$PROJECT_DIR/resources/linux/icon_${size}x${size}.png"
        if [ -f "$png_icon" ]; then
            cp "$png_icon" "$package_dir/icons/"
        fi
    done
    
    # Copy licenses if exists
    if [ -d "$PROJECT_DIR/licenses" ]; then
        cp -r "$PROJECT_DIR/licenses" "$package_dir/"
    fi
    
    # Create desktop entry
    cat > "$package_dir/lightpdf.desktop" << EOF
[Desktop Entry]
Name=LightPDF
Comment=A lightweight, cross-platform PDF reader
Exec=lightpdf
Icon=lightpdf
Terminal=false
Type=Application
Categories=Office;Viewer;
EOF
    
    # Create README
    cat > "$package_dir/README.txt" << EOF
LightPDF - A lightweight, cross-platform PDF reader

Usage:
  ./lightpdf [PDF file]

Note: If using dynamic linking, place libpdfium.so in the same directory.
EOF
    
    echo "✓ Linux package created at: $package_dir"
}

# Package Windows
package_windows() {
    local target="x86_64-pc-windows-msvc"
    
    echo ""
    echo "Packaging Windows (x86_64)..."
    
    local binary="$TARGET_DIR/$target/release/lightpdf.exe"
    local package_dir="$DIST_DIR/lightpdf-windows-x86_64"
    local libpdfium="$PROJECT_DIR/lib/pdfium.dll"
    local icon_svg="$PROJECT_DIR/resources/icon.svg"
    
    rm -rf "$package_dir"
    mkdir -p "$package_dir"
    mkdir -p "$package_dir/icons"
    
    cp "$binary" "$package_dir/"
    
    # Copy pdfium dll if exists
    if [ -f "$libpdfium" ]; then
        cp "$libpdfium" "$package_dir/"
    fi
    
    # Copy icon resources
    if [ -f "$icon_svg" ]; then
        cp "$icon_svg" "$package_dir/icons/icon.svg"
    fi
    
    # Try to copy .ico if available
    local ico_file="$PROJECT_DIR/resources/windows/icon.ico"
    if [ -f "$ico_file" ]; then
        cp "$ico_file" "$package_dir/icon.ico"
    fi
    
    # Copy PNG icons if available
    for size in 16 32 48 256; do
        local png_icon="$PROJECT_DIR/resources/windows/icon_${size}x${size}.png"
        if [ -f "$png_icon" ]; then
            cp "$png_icon" "$package_dir/icons/"
        fi
    done
    
    # Create README
    cat > "$package_dir/README.txt" << EOF
LightPDF - A lightweight, cross-platform PDF reader

Usage:
  lightpdf.exe [PDF file]

Note: If using dynamic linking, place pdfium.dll in the same directory.
EOF
    
    echo "✓ Windows package created at: $package_dir"
}

# Clean build artifacts
clean() {
    echo ""
    echo "Cleaning build artifacts..."
    cargo clean
    rm -rf "$DIST_DIR"
    echo "✓ Clean completed"
}

# Show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --current          Build for current platform (default)"
    echo "  --macos-arm64      Build for macOS (arm64)"
    echo "  --macos-x86_64     Build for macOS (x86_64)"
    echo "  --linux-x86_64     Build for Linux (x86_64)"
    echo "  --linux-aarch64    Build for Linux (aarch64)"
    echo "  --windows-x86_64   Build for Windows (x86_64)"
    echo "  --all              Build all platforms"
    echo "  --clean            Clean build artifacts"
    echo "  --help             Show this help message"
    echo ""
}

# Main
check_tools

case $1 in
    --current|current)
        build_current
        ;;
    --macos-arm64|macos-arm64)
        build_macos_arm64
        package_macos "aarch64-apple-darwin"
        ;;
    --macos-x86_64|macos-x86_64)
        build_macos_x86_64
        package_macos "x86_64-apple-darwin"
        ;;
    --linux-x86_64|linux-x86_64)
        build_linux_x86_64
        package_linux "x86_64-unknown-linux-gnu"
        ;;
    --linux-aarch64|linux-aarch64)
        build_linux_aarch64
        package_linux "aarch64-unknown-linux-gnu"
        ;;
    --windows-x86_64|windows-x86_64)
        build_windows_x86_64
        package_windows
        ;;
    --all|all)
        build_macos_arm64
        package_macos "aarch64-apple-darwin"
        
        build_macos_x86_64
        package_macos "x86_64-apple-darwin"
        
        build_linux_x86_64
        package_linux "x86_64-unknown-linux-gnu"
        
        build_linux_aarch64
        package_linux "aarch64-unknown-linux-gnu"
        
        build_windows_x86_64
        package_windows
        ;;
    --clean|clean)
        clean
        ;;
    --help|help)
        show_usage
        ;;
    *)
        build_current
        ;;
esac

echo ""
echo "======================================"
echo "  Done!"
echo "======================================"
