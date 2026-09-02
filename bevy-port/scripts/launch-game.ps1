param(
    [switch]$Debug
)

$ErrorActionPreference = 'Stop'
$workspaceRoot = Resolve-Path (Join-Path $PSScriptRoot '..')
$cargoArguments = @('run', '--release', '-p', 'stream_town_game')
if ($Debug) {
    $cargoArguments = @('run', '-p', 'stream_town_game')
}
$nativeRuntime = Join-Path $workspaceRoot 'vcpkg_installed\x64-windows\bin'
$ffmpegRuntimePresent = (Test-Path -LiteralPath $nativeRuntime -PathType Container) -and
    (@(Get-ChildItem -LiteralPath $nativeRuntime -File -Filter 'avcodec-*.dll').Count -gt 0)
if (-not $ffmpegRuntimePresent) {
    throw "The linked FFmpeg runtime is missing from: $nativeRuntime"
}

Push-Location $workspaceRoot
$previousProcessPath = $env:PATH
try {
    $env:PATH = "$nativeRuntime;$previousProcessPath"
    & cargo @cargoArguments
    exit $LASTEXITCODE
}
finally {
    $env:PATH = $previousProcessPath
    Pop-Location
}
