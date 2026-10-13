$ErrorActionPreference = 'Stop'
$tgcxRoot = Split-Path -Parent $PSScriptRoot
$tgcxVswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
$tgcxVs = & $tgcxVswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -version '[17.0,18.0)' -property installationPath
if (-not $tgcxVs) { throw 'Visual Studio 2022 with x86 C++ tools is required' }
Push-Location $tgcxRoot
try {
    python tools/gen_signatures.py
    if ($LASTEXITCODE -ne 0) { throw 'Signature generation failed' }
    & (Join-Path $tgcxVs 'MSBuild\Current\Bin\MSBuild.exe') dll/tgcx/tgcx.vcxproj /t:Rebuild /p:Configuration=Release /p:Platform=Win32 /nologo /verbosity:minimal
    if ($LASTEXITCODE -ne 0) { throw 'DLL build failed' }
} finally { Pop-Location }
