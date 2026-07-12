$ErrorActionPreference = 'Stop'
$targetDir = [System.IO.Path]::GetFullPath((Join-Path $HOME '.cargo/bin'))
New-Item -ItemType Directory -Force -Path $targetDir | Out-Null

Set-Location $PSScriptRoot
cargo build --release

$source = Join-Path $PSScriptRoot 'target\release\las.exe'
$destination = Join-Path $targetDir 'las.exe'
Copy-Item $source $destination -Force

$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$pathEntries = @($userPath -split ';' | Where-Object { $_ -and $_ -ne $targetDir })
$updatedPath = @($targetDir) + $pathEntries -join ';'
[Environment]::SetEnvironmentVariable('Path', $updatedPath, 'User')
$env:Path = "$targetDir;$env:Path"

Write-Host 'Installation completed.'
Write-Host "Run now: las run program.las"
Write-Host 'If the command is still not found, reopen the terminal.'
