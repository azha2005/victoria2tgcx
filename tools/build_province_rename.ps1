$ErrorActionPreference='Stop'
$tgcxRoot=Split-Path -Parent $PSScriptRoot
$tgcxVswhere=Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
$tgcxVs=& $tgcxVswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -version '[17.0,18.0)' -property installationPath
if(-not $tgcxVs){throw 'Visual Studio 2022 x86 tools required'}
Push-Location $tgcxRoot
try {
    python tools/check_exe.py
    if($LASTEXITCODE -ne 0){throw 'Executable gate failed'}
    python tools/gen_rename_signatures.py
    if($LASTEXITCODE -ne 0){throw 'Signature generation failed'}
    & (Join-Path $tgcxVs 'MSBuild/Current/Bin/MSBuild.exe') dll/province_rename/tgcx_rename.vcxproj /t:Rebuild /p:Configuration=Release /p:Platform=Win32 /nologo /verbosity:minimal
    if($LASTEXITCODE -ne 0){throw 'Province rename build failed'}
} finally {Pop-Location}
