#!/bin/bash

# lingpdf Build Script for release packages
# Supports: DMG (macOS), EXE, MSI (Windows)

set -e

PROJECT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
TARGET_DIR="$PROJECT_DIR/target"
DIST_DIR="$PROJECT_DIR/dist"

mkdir -p "$DIST_DIR"

echo "======================================"
echo "  lingpdf Package Builder"
echo "======================================"

# Function to check required tools
check_tools() {
    echo "Checking required tools..."
    
    if ! command -v cargo &> /dev/null; then
        echo "ERROR: cargo is not installed!"
        exit 1
    fi
    echo "✓ cargo"
}

# Build release
build_release() {
    local target=$1
    echo ""
    echo "Building for $target..."
    cargo build --release --target "$target"
    echo "✓ Built: $target"
}

# Package macOS DMG
package_macos_dmg() {
    local target=$1
    local arch=""
    
    case $target in
        "aarch64-apple-darwin") arch="arm64" ;;
        "x86_64-apple-darwin") arch="x86_64" ;;
        *) return 1 ;;
    esac
    
    echo ""
    echo "Creating macOS DMG..."
    
    local app_name="lingpdf"
    local app_bundle="$DIST_DIR/$app_name-$arch.app"
    local dmg_path="$DIST_DIR/lingpdf-macos-$arch.dmg"
    local binary="$TARGET_DIR/$target/release/lingpdf"
    local libpdfium="$PROJECT_DIR/lib/libpdfium.dylib"
    
    # Clean and create app bundle
    rm -rf "$app_bundle"
    mkdir -p "$app_bundle/Contents/MacOS"
    mkdir -p "$app_bundle/Contents/Resources"
    
    # Copy binary
    cp "$binary" "$app_bundle/Contents/MacOS/"
    chmod +x "$app_bundle/Contents/MacOS/lingpdf"
    
    # Copy libpdfium
    if [ -f "$libpdfium" ]; then
        cp "$libpdfium" "$app_bundle/Contents/MacOS/"
    fi
    
    # Copy icon
    if [ -f "resources/macos/icon.icns" ]; then
        cp "resources/macos/icon.icns" "$app_bundle/Contents/Resources/"
    elif [ -f "resources/icon.svg" ]; then
        cp "resources/icon.svg" "$app_bundle/Contents/Resources/icon.svg"
    fi
    
    # Create Info.plist
    cat > "$app_bundle/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>lingpdf</string>
    <key>CFBundleIdentifier</key>
    <string>com.lingpdf.app</string>
    <key>CFBundleName</key>
    <string>lingpdf</string>
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
</dict>
</plist>
EOF
    
    # Create DMG using hdiutil
    if command -v hdiutil &> /dev/null; then
        hdiutil create -volname "lingpdf" -srcfolder "$app_bundle" -ov -format UDZO "$dmg_path"
        echo "✓ Created DMG: $dmg_path"
    else
        echo "Warning: hdiutil not found, skipping DMG"
    fi
}

# Package Windows EXE
package_windows_exe() {
    local target="x86_64-pc-windows-msvc"
    
    echo ""
    echo "Creating Windows EXE package..."
    
    local binary="$TARGET_DIR/$target/release/lingpdf.exe"
    local package_dir="$DIST_DIR/lingpdf-windows-x86_64"
    local libpdfium="$PROJECT_DIR/lib/pdfium.dll"
    
    rm -rf "$package_dir"
    mkdir -p "$package_dir"
    
    cp "$binary" "$package_dir/"
    
    if [ -f "$libpdfium" ]; then
        cp "$libpdfium" "$package_dir/"
    fi
    
    # Copy icon
    if [ -f "resources/windows/icon.ico" ]; then
        cp "resources/windows/icon.ico" "$package_dir/"
    fi
    
    # Create ZIP
    cd "$DIST_DIR"
    zip -r "lingpdf-windows-x86_64.zip" "lingpdf-windows-x86_64"
    cd "$PROJECT_DIR"
    
    echo "✓ Created: dist/lingpdf-windows-x86_64.zip"
}

# Package Windows MSI
package_windows_msi() {
    local target="x86_64-pc-windows-msvc"
    
    echo ""
    echo "Creating Windows MSI installer..."
    
    # Check for WiX
    if ! command -v candle &> /dev/null && ! command -v heat &> /dev/null && ! command -v wix &> /dev/null; then
        echo "WiX not found. Installing..."
        # Try to install WiX
        if command -v dotnet &> /dev/null; then
            dotnet tool install --global wix 2>/dev/null || true
        fi
    fi
    
    local package_dir="$DIST_DIR/lingpdf-windows-x86_64"
    local msi_path="$DIST_DIR/lingpdf-windows-x86_64.msi"
    
    if [ -d "$package_dir" ]; then
        # Create a simple MSI using WiX if available
        if command -v candle &> /dev/null; then
            # WiX v4
            cat > "$DIST_DIR/product.wxs" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">
    <Package Name="lingpdf" Version="0.1.0" Manufacturer="lingpdf" UpgradeCode="A1B2C3D4-E5F6-7890-ABCD-EF1234567890">
        <MajorUpgrade DowngradeErrorMessage="A newer version of lingpdf is already installed." />
        <MediaTemplate EmbedCab="yes" />
        <Feature Id="ProductFeature" Title="lingpdf" Level="1">
            <ComponentGroupRef Id="ProductComponents" />
        </Feature>
    </Package>
    <Fragment>
        <StandardDirectory Id="ProgramFiles64Folder">
            <Directory Id="INSTALLFOLDER" Name="lingpdf" />
        </StandardDirectory>
        <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER">
            <Component>
                <File Source="lingpdf-windows-x86_64\lingpdf.exe" />
            </Component>
        </ComponentGroup>
    </Fragment>
</Wix>
EOF
            candle.exe -nologo -out "$msi_path" "$DIST_DIR/product.wxs" 2>/dev/null || echo "WiX failed"
        fi
        
        if [ ! -f "$msi_path" ]; then
            echo "Warning: MSI creation failed. Creating self-extracting archive instead..."
            cd "$DIST_DIR"
            7z a -sfx7z.sfx "lingpdf-windows-x86_64-installer.exe" "lingpdf-windows-x86_64" 2>/dev/null || \
            zip -r "lingpdf-windows-x86_64-portable.zip" "lingpdf-windows-x86_64"
            cd "$PROJECT_DIR"
        fi
    fi
    
    if [ -f "$msi_path" ]; then
        echo "✓ Created MSI: $msi_path"
    fi
}

# Main
check_tools

case "$1" in
    --macos-dmg)
        build_release "aarch64-apple-darwin"
        package_macos_dmg "aarch64-apple-darwin"
        ;;
    --windows-exe)
        build_release "x86_64-pc-windows-msvc"
        package_windows_exe
        ;;
    --windows-msi)
        build_release "x86_64-pc-windows-msvc"
        package_windows_exe
        package_windows_msi
        ;;
    --all)
        # Build for all platforms
        if [[ "$(uname -s)" == "Darwin" ]]; then
            build_release "aarch64-apple-darwin"
            package_macos_dmg "aarch64-apple-darwin"
        fi
        # Windows (cross-compile)
        if command -v cross &> /dev/null; then
            build_release "x86_64-pc-windows-msvc"
            package_windows_exe
        fi
        ;;
    *)
        echo "Usage: $0 [OPTIONS]"
        echo ""
        echo "Options:"
        echo "  --macos-dmg     Build macOS DMG"
        echo "  --windows-exe   Build Windows EXE package"
        echo "  --windows-msi   Build Windows MSI installer"
        echo "  --all           Build all platforms"
        echo ""
        ;;
esac

echo ""
echo "======================================"
echo "  Done!"
echo "======================================"
echo ""
echo "Output files:"
ls -la "$DIST_DIR" 2>/dev/null || echo "  No output files yet"
echo ""
