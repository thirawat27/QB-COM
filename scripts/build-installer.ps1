# QB-COM Windows Installer Builder (PowerShell)
# Usage: .\build-installer.ps1

param(
    [switch]$Silent,
    [string]$Version = "1.0.0"
)

$ErrorActionPreference = "Stop"

function Write-Header {
    param([string]$Text)
    Write-Host ""
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host "  $Text" -ForegroundColor Cyan
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host ""
}

function Test-Command {
    param([string]$Command)
    return [bool](Get-Command $Command -ErrorAction SilentlyContinue)
}

Write-Header "QB-COM Windows Installer Builder"

# Check prerequisites
Write-Host "Checking prerequisites..." -ForegroundColor Yellow

if (-not (Test-Command "makensis")) {
    Write-Host "[ERROR] NSIS is not installed or not in PATH." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install NSIS first:"
    Write-Host "  1. Download from: https://nsis.sourceforge.io/Download"
    Write-Host "  2. Install with default settings"
    Write-Host "  3. Add NSIS to your PATH (e.g., C:\Program Files (x86)\NSIS)"
    exit 1
}
Write-Host "[OK] NSIS found" -ForegroundColor Green

if (-not (Test-Command "cargo")) {
    Write-Host "[ERROR] Rust/Cargo is not installed." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install Rust from: https://rustup.rs/"
    exit 1
}
Write-Host "[OK] Rust found" -ForegroundColor Green

# Get version from Cargo.toml
$ cargoToml = Get-Content "..\Cargo.toml" -Raw
if ($cargoToml -match 'version\s*=\s*"([^"]+)"') {
    $Version = $matches[1]
}
Write-Host "Building QB-COM version: $Version" -ForegroundColor Yellow
Write-Host ""

# Build release binary
Write-Host "[1/3] Building release binary..." -ForegroundColor Yellow
Push-Location ..
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "Build failed!"
    }
} finally {
    Pop-Location
}
Write-Host "[OK] Build successful" -ForegroundColor Green
Write-Host ""

# Verify binary exists
$binaryPath = "..\target\release\qb.exe"
if (-not (Test-Path $binaryPath)) {
    Write-Host "[ERROR] qb.exe not found at $binaryPath" -ForegroundColor Red
    exit 1
}

# Get binary size
$binarySize = (Get-Item $binaryPath).Length / 1MB
Write-Host "Binary size: $([math]::Round($binarySize, 2)) MB" -ForegroundColor Gray
Write-Host ""

# Create installer directory
$installerDir = "..\installer"
if (-not (Test-Path $installerDir)) {
    New-Item -ItemType Directory -Path $installerDir | Out-Null
}

# Build installer
Write-Host "[2/3] Building installer with NSIS..." -ForegroundColor Yellow
Push-Location $installerDir
try {
    $output = makensis qb-com.nsi 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host $output -ForegroundColor Red
        throw "Installer creation failed!"
    }
    Write-Host $output -ForegroundColor Gray
} finally {
    Pop-Location
}
Write-Host "[OK] Installer created successfully" -ForegroundColor Green
Write-Host ""

# Find created installer
$installer = Get-ChildItem "$installerDir\QB-COM-Setup-*.exe" | Select-Object -First 1
if ($installer) {
    $installerSize = $installer.Length / 1MB
    Write-Host "[3/3] Installer details:" -ForegroundColor Yellow
    Write-Host "  File: $($installer.Name)" -ForegroundColor White
    Write-Host "  Path: $($installer.FullName)" -ForegroundColor White
    Write-Host "  Size: $([math]::Round($installerSize, 2)) MB" -ForegroundColor White
} else {
    Write-Host "[WARNING] Could not find created installer" -ForegroundColor Yellow
}

Write-Header "Build Complete!"

if (-not $Silent) {
    Write-Host "Press any key to exit..."
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}
