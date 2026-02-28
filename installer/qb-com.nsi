; QB-COM Installer Script for NSIS
; Requires: NSIS 3.0+
; Build: makensis qb-com.nsi

!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "x64.nsh"

; Application Information
!define APP_NAME "QB-COM"
!define APP_VERSION "1.0.0"
!define APP_PUBLISHER "Thirawat27"
!define APP_URL "https://github.com/thirawat27/QB-COM"
!define APP_EXE "qb.exe"
!define INSTALL_DIR "$PROGRAMFILES64\QB-COM"

; Installer Configuration
Name "${APP_NAME} ${APP_VERSION}"
OutFile "QB-COM-Setup-${APP_VERSION}.exe"
InstallDir "${INSTALL_DIR}"
InstallDirRegKey HKCU "Software\QB-COM" "InstallDir"
RequestExecutionLevel admin
SetCompressor lzma

; Interface Settings
!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "..\LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

; Languages
!insertmacro MUI_LANGUAGE "English"

; Variables
Var STARTMENU_FOLDER

; Function to modify PATH
Function AddToPath
    Exch $0
    Push $1
    Push $2
    Push $3
    
    ; Read current PATH
    ReadRegStr $1 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path"
    
    ; Check if already in PATH
    Push "$1"
    Push "$0"
    Call StrStr
    Pop $2
    StrCmp $2 "" 0 done
    
    ; Add to PATH
    StrCpy $3 "$1;$0"
    WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path" "$3"
    
    ; Notify Windows of environment change
    SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
    
done:
    Pop $3
    Pop $2
    Pop $1
    Pop $0
FunctionEnd

; Function to remove from PATH
Function RemoveFromPath
    Exch $0
    Push $1
    Push $2
    Push $3
    Push $4
    Push $5
    
    ; Read current PATH
    ReadRegStr $1 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path"
    
    ; Find and remove our path
    StrCpy $2 "$1"
    StrCpy $1 ""
    
loop:
    StrCpy $3 $2 1
    StrCmp $3 "" done
    StrCpy $4 $2 1 -1
    StrCmp $4 ";" 0 +3
    StrCpy $2 $2 -1
    Goto loop
    
    Push $2
    Push ";"
    Call SplitFirstStrPart
    Pop $3
    Pop $2
    
    StrCmp $3 $0 loop
    StrCmp $1 "" 0 +3
    StrCpy $1 "$3"
    Goto +2
    StrCpy $1 "$1;$3"
    
    StrCmp $2 "" done loop
    
done:
    WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path" "$1"
    SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
    
    Pop $5
    Pop $4
    Pop $3
    Pop $2
    Pop $1
    Pop $0
FunctionEnd

; Helper function: Find substring
Function StrStr
    Exch $R1 ; string to find
    Exch
    Exch $R0 ; string to search in
    Push $R2
    Push $R3
    Push $R4
    Push $R5
    StrLen $R2 $R1
    StrLen $R3 $R0
    IntOp $R2 $R2 - 1
    IntOp $R3 $R3 - 1
    StrCpy $R5 -1
next:
    IntOp $R5 $R5 + 1
    IntCmp $R5 $R3 done
    StrCpy $R4 $R0 $R2 $R5
    StrCmp $R4 $R1 done
    Goto next
done:
    StrCpy $R1 $R5
    Pop $R5
    Pop $R4
    Pop $R3
    Pop $R2
    Pop $R0
    Exch $R1
FunctionEnd

; Helper function: Split string
Function SplitFirstStrPart
    Exch $R0 ; separator
    Exch
    Exch $R1 ; string
    Push $R2
    Push $R3
    StrLen $R2 $R0
    StrCpy $R3 $R1 $R2
    StrCmp $R3 $R0 +3
    StrCpy $R0 ""
    Goto +5
    StrCpy $R1 $R1 $R2 0
    IntOp $R2 $R2 * -1
    StrCpy $R1 $R1 $R2
    StrCpy $R0 1
    StrCmp $R0 1 +2
    StrCpy $R1 ""
    Pop $R3
    Pop $R2
    Exch $R1
    Exch
    Exch $R0
FunctionEnd

; Installer Sections
Section "QB-COM Core" SecCore
    SectionIn RO
    
    ; Check for 64-bit Windows
    ${If} ${RunningX64}
        ; OK - Continue installation
    ${Else}
        MessageBox MB_OK "QB-COM requires 64-bit Windows."
        Abort
    ${EndIf}
    
    ; Create installation directory
    SetOutPath "$INSTDIR"
    
    ; Install main executable
    File "..\target\release\qb.exe"
    
    ; Install additional files
    File "..\LICENSE"
    File "..\README.md"
    
    ; Create directories
    CreateDirectory "$INSTDIR\examples"
    SetOutPath "$INSTDIR\examples"
    File /r "..\examples\*.*"
    
    ; Store installation folder
    WriteRegStr HKCU "Software\QB-COM" "InstallDir" $INSTDIR
    
    ; Create uninstaller
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    
    ; Register uninstaller in Windows
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "DisplayName" "${APP_NAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "DisplayVersion" "${APP_VERSION}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "Publisher" "${APP_PUBLISHER}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "DisplayIcon" "$INSTDIR\qb.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "UninstallString" "$INSTDIR\Uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "InstallLocation" "$INSTDIR"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "EstimatedSize" 10240
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM" \
        "NoRepair" 1
SectionEnd

Section "Add to PATH" SecPath
    Push "$INSTDIR"
    Call AddToPath
SectionEnd

Section "Start Menu Shortcuts" SecStartMenu
    CreateDirectory "$SMPROGRAMS\QB-COM"
    CreateShortcut "$SMPROGRAMS\QB-COM\QB-COM CLI.lnk" "$INSTDIR\qb.exe"
    CreateShortcut "$SMPROGRAMS\QB-COM\Uninstall QB-COM.lnk" "$INSTDIR\Uninstall.exe"
    CreateShortcut "$SMPROGRAMS\QB-COM\README.lnk" "$INSTDIR\README.md"
SectionEnd

Section "Desktop Shortcut" SecDesktop
    CreateShortcut "$DESKTOP\QB-COM.lnk" "$INSTDIR\qb.exe"
SectionEnd

; Descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
    !insertmacro MUI_DESCRIPTION_TEXT ${SecCore} "QB-COM QBasic Compiler and Interpreter"
    !insertmacro MUI_DESCRIPTION_TEXT ${SecPath} "Add QB-COM to your system PATH"
    !insertmacro MUI_DESCRIPTION_TEXT ${SecStartMenu} "Create Start Menu shortcuts"
    !insertmacro MUI_DESCRIPTION_TEXT ${SecDesktop} "Create Desktop shortcut"
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; Uninstaller Section
Section "Uninstall"
    ; Remove from PATH first
    Push "$INSTDIR"
    Call RemoveFromPath
    
    ; Remove installed files
    Delete "$INSTDIR\qb.exe"
    Delete "$INSTDIR\LICENSE"
    Delete "$INSTDIR\README.md"
    Delete "$INSTDIR\Uninstall.exe"
    
    ; Remove examples directory
    RMDir /r "$INSTDIR\examples"
    
    ; Remove installation directory
    RMDir "$INSTDIR"
    
    ; Remove Start Menu shortcuts
    RMDir /r "$SMPROGRAMS\QB-COM"
    
    ; Remove Desktop shortcut
    Delete "$DESKTOP\QB-COM.lnk"
    
    ; Remove registry keys
    DeleteRegKey HKCU "Software\QB-COM"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QB-COM"
SectionEnd

; Check if running silently
Function .onInit
    ${If} ${Silent}
        ; Silent install - select all sections except desktop
        SectionSetFlags ${SecCore} 17
        SectionSetFlags ${SecPath} 17
        SectionSetFlags ${SecStartMenu} 17
        SectionSetFlags ${SecDesktop} 0
    ${EndIf}
FunctionEnd
