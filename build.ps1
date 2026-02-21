# Build script for lingpdf (Windows)
# Supports: Windows (x86_64)

param(
    [switch]$Current,
    [switch]$Windows,
    [switch]$Clean,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

$ProjectDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$TargetDir = Join-Path $ProjectDir "target"
$DistDir = Join-Path $ProjectDir "dist"

if (-not (Test-Path $DistDir)) {
    New-Item -ItemType Directory -Path $DistDir -Force | Out-Null
}

Write-Host "======================================"
Write-Host "  lingpdf Build Script (Windows)"
Write-Host "======================================"
Write-Host ""

function Check-Tools {
    Write-Host "Checking required tools..."

    if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
        Write-Host "ERROR: cargo (Rust toolchain) is not installed!" -ForegroundColor Red
        exit 1
    }

    Write-Host "✓ cargo is installed" -ForegroundColor Green
}

function Build-Current {
    Write-Host ""
    Write-Host "Building for current platform..."
    
    cargo build --release
    
    Write-Host "✓ Build completed for current platform" -ForegroundColor Green
}

function Build-Windows {
    Write-Host ""
    Write-Host "Building for Windows (x86_64)..."
    
    cargo build --release --target x86_64-pc-windows-msvc
    
    Write-Host "✓ Build completed for Windows (x86_64)" -ForegroundColor Green
}

function Package-Windows {
    Write-Host ""
    Write-Host "Packaging Windows (x86_64)..."
    
    $Target = "x86_64-pc-windows-msvc"
    $Binary = Join-Path $TargetDir "$Target\release\lingpdf.exe"
    $PackageDir = Join-Path $DistDir "lingpdf-windows-x86_64"
    
    if (Test-Path $PackageDir) {
        Remove-Item -Path $PackageDir -Recurse -Force
    }
    
    New-Item -ItemType Directory -Path $PackageDir -Force | Out-Null
    
    Copy-Item -Path $Binary -Destination $PackageDir
    
    $ReadmeContent = @"
lingpdf - A lightweight, cross-platform PDF reader

Usage:
  lingpdf.exe [PDF file]
"@
    
    $ReadmePath = Join-Path $PackageDir "README.txt"
    Set-Content -Path $ReadmePath -Value $ReadmeContent
    
    Write-Host "✓ Windows package created at: $PackageDir" -ForegroundColor Green
}

function Clean-Build {
    Write-Host ""
    Write-Host "Cleaning build artifacts..."
    cargo clean
    if (Test-Path $DistDir) {
        Remove-Item -Path $DistDir -Recurse -Force
    }
    Write-Host "✓ Clean completed" -ForegroundColor Green
}

function Show-Usage {
    Write-Host "Usage: .\build.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -Current    Build for current platform (default)"
    Write-Host "  -Windows    Build for Windows (x86_64)"
    Write-Host "  -Clean      Clean build artifacts"
    Write-Host "  -Help       Show this help message"
    Write-Host ""
}

Check-Tools

if ($Help) {
    Show-Usage
} elseif ($Clean) {
    Clean-Build
} elseif ($Windows) {
    Build-Windows
    Package-Windows
} else {
    Build-Current
}

Write-Host ""
Write-Host "======================================"
Write-Host "  Done!"
Write-Host "======================================"
