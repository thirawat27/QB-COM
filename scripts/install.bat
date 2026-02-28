@echo off
setlocal enabledelayedexpansion

:: QB-COM One-Liner Installer for Windows
:: Usage: curl -fsSL https://raw.githubusercontent.com/thirawat27/QB-COM/main/scripts/install.bat | cmd

set "INSTALL_DIR=%LOCALAPPDATA%\QB-COM"
set "REPO_URL=https://github.com/thirawat27/QB-COM"

echo ========================================
echo   QB-COM Installer
echo ========================================
echo.

:: Check Windows version
systeminfo | findstr /B /C:"OS Name" | findstr "Windows" >nul
if errorlevel 1 (
    echo [X] This installer is for Windows only.
    exit /b 1
)

:: Create install directory
echo [+] Creating installation directory...
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

:: Download latest release
echo [+] Downloading QB-COM...
set "DOWNLOAD_URL=%REPO_URL%/releases/latest/download/qb-com-windows-x64.zip"
set "TEMP_ZIP=%TEMP%\qb-com-install.zip"

powershell -Command "try { Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP_ZIP%' -UseBasicParsing } catch { exit 1 }"
if errorlevel 1 (
    echo [X] Failed to download QB-COM.
    echo     Please check your internet connection.
    exit /b 1
)

echo [+] Download successful.

:: Extract
echo [+] Extracting files...
powershell -Command "Expand-Archive -Path '%TEMP_ZIP%' -DestinationPath '%INSTALL_DIR%' -Force"
del "%TEMP_ZIP%" >nul 2>&1

:: Add to PATH
echo [+] Adding to PATH...
for /f "tokens=2*" %%a in ('reg query HKCU\Environment /v PATH 2^>nul ^| findstr PATH') do set "USER_PATH=%%b"
if not defined USER_PATH set "USER_PATH="

echo %USER_PATH% | findstr /I /C:"%INSTALL_DIR%" >nul
if errorlevel 1 (
    reg add HKCU\Environment /v PATH /t REG_EXPAND_SZ /d "%USER_PATH%;%INSTALL_DIR%" /f >nul
    echo [+] Added to user PATH.
    echo     Please restart your terminal to use 'qb' command.
) else (
    echo [+] Already in PATH.
)

:: Verify
echo [+] Verifying installation...
if exist "%INSTALL_DIR%\qb.exe" (
    echo.
    echo ========================================
    echo   Installation Complete!
    echo ========================================
    echo.
    echo QB-COM installed at: %INSTALL_DIR%
    echo.
    echo Quick start:
    echo   qb run hello.bas    - Run a BASIC program
    echo   qb repl             - Start interactive mode
    echo   qb --help           - Show all commands
    echo.
    echo Repository: %REPO_URL%
    echo.
) else (
    echo [X] Installation failed - qb.exe not found.
    exit /b 1
)
