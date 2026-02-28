@echo off
setlocal enabledelayedexpansion

echo ==========================================
echo   QB-COM Development Setup (Windows)
echo ==========================================
echo.

:: Check if Rust is installed
echo Checking for Rust installation...
where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo.
    echo [WARNING] Rust is not installed!
    echo.
    echo Would you like to install Rust now?
    echo   1. Yes - Install Rust
    echo   2. No - Skip (you'll need to install manually)
    echo.
    set /p choice="Enter choice (1-2): "
    
    if "!choice!"=="1" (
        echo.
        echo Opening Rust installer in your browser...
        start https://rustup.rs/
        echo Please install Rust and run this setup again.
        pause
        exit /b 1
    ) else (
        echo Please install Rust from https://rustup.rs/ and run this setup again.
        pause
        exit /b 1
    )
)

echo [OK] Rust found: 
rustc --version
echo.

:: Check if cargo is working
echo Checking Cargo...
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Cargo is not working properly!
    echo Please check your Rust installation.
    pause
    exit /b 1
)
echo [OK] Cargo is working
echo.

:: Build the project
echo ==========================================
echo Building QB-COM...
echo ==========================================
echo.

cargo build --release
if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Build failed!
    echo Please check the error messages above.
    pause
    exit /b 1
)

echo.
echo [OK] Build successful!
echo.

:: Run tests
echo ==========================================
echo Running tests...
echo ==========================================
echo.

cargo test
if %errorlevel% neq 0 (
    echo.
    echo [WARNING] Some tests failed!
    echo You can still use the compiler, but some features may not work correctly.
    echo.
)

:: Check if binary was created
if exist "target\release\qb.exe" (
    echo.
    echo ==========================================
    echo   Setup Complete!
    echo ==========================================
    echo.
    echo QB-COM has been built successfully!
    echo.
    echo Binary location: target\release\qb.exe
    echo.
    echo Quick commands:
    echo   qb --help              Show help
    echo   qb run examples\hello.bas   Run example
    echo   qb repl                Start interactive mode
    echo.
    echo To install system-wide, run: scripts\build-installer.bat
    echo.
) else (
    echo [ERROR] Could not find built binary!
    echo.
    pause
    exit /b 1
)

pause
