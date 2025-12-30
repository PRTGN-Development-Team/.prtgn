#define MyAppName "prtgn"
#define MyAppVersion GetEnv("APP_VERSION")
; #define MyAppVersion "9.0.2"
#define MyAppPublisher "PRTGN Development Team"
#define MyAppURL "https://github.com/PRTGN-Development-Team/.prtgn"
#define MyAppExeName "prtgn.exe"
#define PrtgnAssocKey StringChange("PRTGN File Format", " ", "") + ".prtgn"
#define PrtgnWAVAssocKey StringChange("PRTGN WAV Format", " ", "") + ".prtgn_wav"
#define PrtgnFLACAssocKey StringChange("PRTGN FLAC Format", " ", "") + ".prtgn_flac"


[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{95A5ABC3-BFE7-4BE2-B3F1-82E15897A241}
AppName={#MyAppName}
AppVersion={#emit MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={localappdata}\{#MyAppName}
UninstallDisplayIcon={app}\{#MyAppExeName}
; "ArchitecturesAllowed=x64compatible" specifies that Setup cannot run
; on anything but x64 and Windows 11 on Arm.
ArchitecturesAllowed=x64compatible
; "ArchitecturesInstallIn64BitMode=x64compatible" requests that the
; install be done in "64-bit mode" on x64 or Windows 11 on Arm,
; meaning it should use the native 64-bit Program Files directory and
; the 64-bit view of the registry.
ArchitecturesInstallIn64BitMode=x64compatible
ChangesAssociations=yes
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=LICENSE
InfoBeforeFile=README.md
; Remove the following line to run in administrative install mode (install for all users).
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputBaseFilename=prtgn_v{#emit MyAppVersion}_x86_64
SetupIconFile=prtgn_logo.ico
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
#ifdef GITHUB_ACTIONS
Source: "target/x86_64-pc-windows-gnu/release/prtgn.exe"; DestDir: "{app}"; Flags: ignoreversion
#else
Source: "target/release/prtgn.exe"; DestDir: "{app}"; Flags: ignoreversion
#endif
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Registry]
; Root: HKA; Subkey: "Software\Classes\{#MyAppAssocExt}\OpenWithProgids"; ValueType: string; ValueName: "{#MyAppAssocKey}"; ValueData: ""; Flags: uninsdeletevalue
; Root: HKA; Subkey: "Software\Classes\{#MyAppAssocKey}"; ValueType: string; ValueName: ""; ValueData: "{#MyAppAssocName}"; Flags: uninsdeletekey
; Root: HKA; Subkey: "Software\Classes\{#MyAppAssocKey}\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#MyAppExeName},0"
; Root: HKA; Subkey: "Software\Classes\{#MyAppAssocKey}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"

Root: HKA; Subkey: "Software\Classes\.prtgn\OpenWithProgids"; ValueType: string; ValueName: "{#PrtgnAssocKey}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\{#PrtgnAssocKey}"; ValueType: string; ValueName: ""; ValueData: "PRTGN File Format"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\{#PrtgnAssocKey}\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#MyAppExeName},0"
Root: HKA; Subkey: "Software\Classes\{#PrtgnAssocKey}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""

Root: HKA; Subkey: "Software\Classes\.prtgn_wav\OpenWithProgids"; ValueType: string; ValueName: "{#PrtgnWAVAssocKey}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\{#PrtgnWAVAssocKey}"; ValueType: string; ValueName: ""; ValueData: "PRTGN WAV Format"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\{#PrtgnWAVAssocKey}\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#MyAppExeName},0"
Root: HKA; Subkey: "Software\Classes\{#PrtgnWAVAssocKey}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""

Root: HKA; Subkey: "Software\Classes\.prtgn_flac\OpenWithProgids"; ValueType: string; ValueName: "{#PrtgnFLACAssocKey}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\{#PrtgnFLACAssocKey}"; ValueType: string; ValueName: ""; ValueData: "PRTGN FLAC Format"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\{#PrtgnFLACAssocKey}\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#MyAppExeName},0"
Root: HKA; Subkey: "Software\Classes\{#PrtgnFLACAssocKey}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"