@echo off
setlocal enabledelayedexpansion

echo ==========================================
echo   QB-COM Windows Installer Builder
echo ==========================================
echo.

:: Check if NSIS is installed
where makensis >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] NSIS is not installed or not in PATH.
    echo.
    echo Please install NSIS first:
    echo   1. Download from: https://nsis.sourceforge.io/Download
    echo   2. Install with default settings
    echo   3. Add NSIS to your PATH environment variable
    echo      (e.g., C:\Program Files (x86)\NSIS)
    echo.
    pause
    exit /b 1
)

echo [OK] NSIS found
echo.

:: Check if Rust is installed
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust is not installed.
    echo.
    echo Please install Rust from: https://rustup.rs/
    echo.
    pause
    exit /b 1
)

echo [OK] Rust found
echo.

:: Get version from Cargo.toml
for /f "tokens=2 delims=\"" %%a in ('findstr "version" ..\Cargo.toml ^| head -1') do (
    set VERSION=%%a
)
echo Building QB-COM version: %VERSION%
echo.

:: Build release binary
echo [1/3] Building release binary...
cargo build --release --manifest-path ..\Cargo.toml
if %errorlevel% neq 0 (
    echo [ERROR] Build failed!
    pause
    exit /b 1
)
echo [OK] Build successful
echo.

:: Check if binary exists
if not exist "..\target\release\qb.exe" (
    echo [ERROR] qb.exe not found in target\release\
    pause
    exit /b 1
)

echo [2/3] Copying files to installer directory...
if not exist "..\installer" mkdir "..\installer"
echo [OK] Files ready
echo.

echo [3/3] Building installer with NSIS...
cd ..\installer
makensis qb-com.nsi
if %errorlevel% neq 0 (
    echo [ERROR] Installer creation failed!
    cd ..\scripts
    pause
    exit /b 1
)

cd ..\scripts
echo.
echo ==========================================
echo   Installer created successfully!
echo ==========================================
echo.
echo Output: installer\QB-COM-Setup-%VERSION%.exe
echo.

:: List the installer file
dir /b ..\installer\QB-COM-Setup-*.exe 2>nul

echo.
echo Press any key to exit...
pause >nul
