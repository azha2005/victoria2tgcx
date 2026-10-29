param([switch]$CompileOnly)
$ErrorActionPreference = 'Stop'
$tgcxVolunteerRoot = Split-Path -Parent $PSScriptRoot
Push-Location (Join-Path $tgcxVolunteerRoot 'rust/volunteers')
try {
    cargo fmt --check
    if ($LASTEXITCODE -ne 0) { throw 'Volunteer format check failed' }
    if (-not $CompileOnly) {
        cargo test --offline
        if ($LASTEXITCODE -ne 0) { throw 'Volunteer tests failed' }
        cargo clippy --offline --all-targets -- -D warnings
        if ($LASTEXITCODE -ne 0) { throw 'Volunteer lint failed' }
    }
    cargo build --release --offline
    if ($LASTEXITCODE -ne 0) { throw 'Volunteer x86 build failed' }
} finally { Pop-Location }
