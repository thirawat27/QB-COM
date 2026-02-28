; QB-COM Installer Script for Inno Setup
; Requires: Inno Setup 6.0+
; Build: ISCC qb-com.iss

#define AppName "QB-COM"
#define AppVersion "1.0.0"
#define AppPublisher "Thirawat27"
#define AppURL "https://github.com/thirawat27/QB-COM"
#define AppExeName "qb.exe"

[Setup]
; Application Information
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}
AppUpdatesURL={#AppURL}
AppCopyright=Copyright (C) 2024 {#AppPublisher}

; Installer Configuration
DefaultDirName={commonpf64}\QB-COM
DefaultGroupName=QB-COM
AllowNoIcons=yes
OutputBaseFilename=QB-COM-Setup
Compression=lzma2/max
SolidCompression=yes
InternalCompressLevel=max
WizardStyle=modern
WizardImageFile=compiler:wizmodernimage.bmp
WizardSmallImageFile=compiler:wizmodernsmallimage.bmp
UninstallIconFile=compiler:setup.ico
UninstallDisplayIcon={app}\{#AppExeName}
ChangesAssociations=yes
CreateAppDir=yes
DisableDirPage=no
DisableProgramGroupPage=no
DisableReadyPage=yes
DisableFinishedPage=no
DisableWelcomePage=no
AlwaysShowDirOnReadyPage=yes
AlwaysShowGroupOnReadyPage=yes
ShowLanguageDialog=yes
AppComments=QBasic/QuickBASIC 4.5 + QB64 Compiler and Runtime
AppContact={#AppURL}
AppReadmeFile={app}\README.md

; Privileges
PrivilegesRequired=admin
MinVersion=6.1sp1

; Architecture
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64

[Files]
; Main executable
Source: "..\target\release\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion

; Documentation
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion

; Examples directory
Source: "..\examples\*"; DestDir: "{app}\examples"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
; Start Menu shortcuts
Name: "{group}\QB-COM CLI"; Filename: "{app}\{#AppExeName}"; Comment: "QB-COM QBasic Compiler and Interpreter"
Name: "{group}\Uninstall QB-COM"; Filename: "{uninstallexe}"; Comment: "Uninstall QB-COM"
Name: "{group}\README"; Filename: "{app}\README.md"; Comment: "QB-COM Documentation"

; Desktop shortcut
Name: "{autodesktop}\QB-COM"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon; Comment: "QB-COM QBasic Compiler and Interpreter"

[Registry]
; Register uninstaller
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: string; ValueName: "DisplayName"; ValueData: "{#AppName}"
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: string; ValueName: "DisplayVersion"; ValueData: "{#AppVersion}"
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: string; ValueName: "Publisher"; ValueData: "{#AppPublisher}"
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: string; ValueName: "DisplayIcon"; ValueData: "{app}\{#AppExeName}"
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: string; ValueName: "URLInfoAbout"; ValueData: "{#AppURL}"
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: string; ValueName: "HelpLink"; ValueData: "{#AppURL}"
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: string; ValueName: "InstallLocation"; ValueData: "{app}"
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: dword; ValueName: "NoModify"; ValueData: 1
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppName}"; ValueType: dword; ValueName: "NoRepair"; ValueData: 1

[Run]
; Optional: Run after installation
Filename: "{app}\README.md"; Description: "View README documentation"; Flags: postinstall shellexec skipifsilent

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Additional icons:"
Name: "addtopath"; Description: "Add QB-COM to system &PATH"; GroupDescription: "System integration:"

[InstallDelete]
; Clean up old files if any exist
Type: filesandordirs; Name: "{app}\examples"

[UninstallDelete]
; Remove all created directories
Type: filesandordirs; Name: "{app}"

[Code]
// Function to add to PATH
procedure AddToPath(Path: String);
var
    OldPath: String;
    NewPath: String;
begin
    // Get current PATH from system
    RegQueryStringValue(HKLM, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', OldPath);

    // Check if already in PATH
    if Pos(';' + LowerCase(Path) + ';', ';' + LowerCase(OldPath) + ';') = 0 then
    begin
        if OldPath = '' then
            NewPath := Path
        else
            NewPath := OldPath + ';' + Path;

        // Write new PATH
        RegWriteExpandStringValue(HKLM, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', NewPath);

    end;
end;

// Function to remove from PATH
procedure RemoveFromPath(Path: String);
var
    OldPath: String;
    NewPath: String;
    PathParts: TStringList;
    i: Integer;
    Part: String;
begin
    // Get current PATH from system
    RegQueryStringValue(HKLM, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', OldPath);

    // Split PATH into parts
    PathParts := TStringList.Create;
    try
        PathParts.Delimiter := ';';
        PathParts.DelimitedText := OldPath;

        // Remove our path from the list
        NewPath := '';
        for i := 0 to PathParts.Count - 1 do
        begin
            Part := PathParts[i];
            if (LowerCase(Trim(Part)) <> LowerCase(Trim(Path))) and (Part <> '') then
            begin
                if NewPath = '' then
                    NewPath := Part
                else
                    NewPath := NewPath + ';' + Part;
            end;
        end;

        // Write new PATH
        RegWriteExpandStringValue(HKLM, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', NewPath);

    finally
        PathParts.Free;
    end;
end;

// After installation
procedure CurStepChanged(CurStep: TSetupStep);
begin
    if CurStep = ssPostInstall then
    begin
        // Add to PATH if selected
        if IsTaskSelected('addtopath') then
        begin
            AddToPath(ExpandConstant('{app}'));
        end;
    end;
end;

// Before uninstallation
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
    if CurUninstallStep = usUninstall then
    begin
        // Remove from PATH
        RemoveFromPath(ExpandConstant('{app}'));
    end;
end;

// Check if running silently
function ShouldSkipPage(PageID: Integer): Boolean;
begin
    if WizardSilent then
    begin
        // Skip pages in silent mode
        Result := (PageID = wpLicense) or (PageID = wpSelectComponents) or (PageID = wpSelectTasks);
    end
    else
        Result := False;
end;

// Initialize setup
function InitializeSetup(): Boolean;
var
    Version: TWindowsVersion;
begin
    // Check Windows version
    GetWindowsVersionEx(Version);
    if Version.NTPlatform and (Version.Major < 6) or ((Version.Major = 6) and (Version.Minor < 1)) then
    begin
        MsgBox('QB-COM requires Windows 7 or later.', mbError, MB_OK);
        Result := False;
        Exit;
    end;

    // Check for 64-bit Windows
    if not Is64BitInstallMode then
    begin
        MsgBox('QB-COM requires 64-bit Windows.', mbError, MB_OK);
        Result := False;
        Exit;
    end;

    Result := True;
end;
