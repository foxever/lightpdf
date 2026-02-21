.PHONY: help build build-macos-arm64 build-macos-x86_64 build-linux-x86_64 build-linux-aarch64 build-windows-x86_64 build-all clean

help:
	@echo "lingpdf Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  help                Show this help message"
	@echo "  build               Build for current platform"
	@echo "  build-macos-arm64   Build for macOS (arm64)"
	@echo "  build-macos-x86_64  Build for macOS (x86_64)"
	@echo "  build-linux-x86_64  Build for Linux (x86_64)"
	@echo "  build-linux-aarch64 Build for Linux (aarch64)"
	@echo "  build-windows-x86_64 Build for Windows (x86_64)"
	@echo "  build-all           Build all platforms"
	@echo "  clean               Clean build artifacts"
	@echo ""

build:
	@./build.sh --current

build-macos-arm64:
	@./build.sh --macos-arm64

build-macos-x86_64:
	@./build.sh --macos-x86_64

build-linux-x86_64:
	@./build.sh --linux-x86_64

build-linux-aarch64:
	@./build.sh --linux-aarch64

build-windows-x86_64:
	@./build.sh --windows-x86_64

build-all:
	@./build.sh --all

clean:
	@./build.sh --clean
