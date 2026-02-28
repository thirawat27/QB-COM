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

if (-not (Test-Command "iscc")) {
    Write-Host "[ERROR] Inno Setup is not installed or not in PATH." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install Inno Setup first:"
    Write-Host "  1. Download from: https://jrsoftware.org/isdl.php"
    Write-Host "  2. Install with default settings"
    Write-Host "  3. Add Inno Setup to your PATH (e.g., C:\Program Files (x86)\Inno Setup 6)"
    exit 1
}
Write-Host "[OK] Inno Setup found" -ForegroundColor Green

if (-not (Test-Command "cargo")) {
    Write-Host "[ERROR] Rust/Cargo is not installed." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install Rust from: https://rustup.rs/"
    exit 1
}
Write-Host "[OK] Rust found" -ForegroundColor Green

# Get version from Cargo.toml
$cargoToml = Get-Content "..\Cargo.toml" -Raw
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
Write-Host "[2/3] Building installer with Inno Setup..." -ForegroundColor Yellow
Push-Location $installerDir
try {
    $output = iscc qb-com.iss 2>&1
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

# Verify installer exists
$installerPath = "$installerDir\QB-COM-Setup.exe"
if (Test-Path $installerPath) {
    $installerSize = (Get-Item $installerPath).Length / 1MB
    Write-Host "[3/3] Verifying installer..." -ForegroundColor Yellow
    Write-Host "Installer size: $([math]::Round($installerSize, 2)) MB" -ForegroundColor Gray
    Write-Host "Installer location: $installerPath" -ForegroundColor Gray
    Write-Host "[OK] Installer verified" -ForegroundColor Green
} else {
    Write-Host "[WARNING] Installer not found at expected location" -ForegroundColor Yellow
}

Write-Header "Build Complete!"
Write-Host "Installer: $installerPath" -ForegroundColor White
Write-Host ""
