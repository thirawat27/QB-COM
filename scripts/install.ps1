# QB-COM One-Liner Installer for Windows
# Usage: iwr -useb https://raw.githubusercontent.com/thirawat27/QB-COM/main/scripts/install.ps1 | iex
# Or: powershell -Command "iwr -useb https://raw.githubusercontent.com/thirawat27/QB-COM/main/scripts/install.ps1 | iex"

param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:LOCALAPPDATA\QB-COM",
    [switch]$AddToPath = $true
)

$ErrorActionPreference = "Stop"
$RepoUrl = "https://github.com/thirawat27/QB-COM"
$ProgressPreference = 'SilentlyContinue'

function Write-Header {
    param([string]$Text)
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  $Text" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host ""
}

function Write-Step {
    param([string]$Text)
    Write-Host "[+] $Text" -ForegroundColor Green
}

function Write-Error {
    param([string]$Text)
    Write-Host "[X] $Text" -ForegroundColor Red
}

Write-Header "QB-COM Installer"

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "x86" }
Write-Step "Detected architecture: $arch"

# Create install directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Write-Step "Created installation directory: $InstallDir"
}

# Download latest release
$downloadUrl = if ($Version -eq "latest") {
    "$RepoUrl/releases/latest/download/qb-com-windows-$arch.zip"
} else {
    "$RepoUrl/releases/download/v$Version/qb-com-windows-$arch.zip"
}

$tempZip = "$env:TEMP\qb-com-install.zip"

Write-Step "Downloading QB-COM..."
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tempZip -UseBasicParsing
    Write-Step "Downloaded successfully"
} catch {
    Write-Error "Failed to download from $downloadUrl"
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}

# Extract
Write-Step "Extracting files..."
Expand-Archive -Path $tempZip -DestinationPath $InstallDir -Force
Remove-Item $tempZip -Force
Write-Step "Extracted to $InstallDir"

# Add to PATH
if ($AddToPath) {
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($currentPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$InstallDir", "User")
        Write-Step "Added to user PATH"
        Write-Host "    Please restart your terminal to use 'qb' command" -ForegroundColor Yellow
    } else {
        Write-Step "Already in PATH"
    }
}

# Verify installation
$qbExe = Join-Path $InstallDir "qb.exe"
if (Test-Path $qbExe) {
    Write-Step "Installation successful!"
    Write-Host ""
    Write-Host "QB-COM installed at: $InstallDir" -ForegroundColor White
    Write-Host ""
    Write-Host "Quick start:" -ForegroundColor Yellow
    Write-Host "  qb run hello.bas    - Run a BASIC program" -ForegroundColor Gray
    Write-Host "  qb repl             - Start interactive mode" -ForegroundColor Gray
    Write-Host "  qb --help           - Show all commands" -ForegroundColor Gray
    Write-Host ""
} else {
    Write-Error "Installation failed - qb.exe not found"
    exit 1
}
