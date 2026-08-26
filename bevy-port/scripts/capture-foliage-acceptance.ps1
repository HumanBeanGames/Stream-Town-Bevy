param(
    [string]$OutputDirectory = ""
)

$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($OutputDirectory)) {
    $stamp = Get-Date -Format "yyyyMMdd-HHmmss"
    $OutputDirectory = Join-Path $workspace ".stream-town\diagnostics\foliage-$stamp"
}
$OutputDirectory = [System.IO.Path]::GetFullPath($OutputDirectory)
if ((Test-Path -LiteralPath $OutputDirectory) -and
    (Get-ChildItem -LiteralPath $OutputDirectory -Force | Select-Object -First 1)) {
    throw "Capture directory is not empty: $OutputDirectory"
}

Push-Location $workspace
try {
    $env:STREAM_TOWN_AUTOSTART = "1"
    $env:STREAM_TOWN_SMOKE_FOLIAGE = "1"
    $env:STREAM_TOWN_FOLIAGE_CAPTURE_DIR = $OutputDirectory
    $env:STREAM_TOWN_DISABLE_DIRECT_BROADCAST = "1"
    cargo run -p stream_town_game
    if ($LASTEXITCODE -ne 0) {
        throw "The foliage capture game process failed with exit code $LASTEXITCODE"
    }
}
finally {
    Remove-Item Env:\STREAM_TOWN_AUTOSTART -ErrorAction SilentlyContinue
    Remove-Item Env:\STREAM_TOWN_SMOKE_FOLIAGE -ErrorAction SilentlyContinue
    Remove-Item Env:\STREAM_TOWN_FOLIAGE_CAPTURE_DIR -ErrorAction SilentlyContinue
    Remove-Item Env:\STREAM_TOWN_DISABLE_DIRECT_BROADCAST -ErrorAction SilentlyContinue
    Pop-Location
}

$manifestPath = Join-Path $OutputDirectory "foliage-sweep-manifest.json"
if (-not (Test-Path -LiteralPath $manifestPath)) {
    throw "The foliage capture did not write its manifest: $manifestPath"
}
$manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
if (-not $manifest.structural_passed) {
    throw "Foliage structural acceptance failed. Inspect $manifestPath"
}
if (($manifest.frames | Measure-Object).Count -ne 12) {
    throw "Foliage capture did not produce all 12 expected frames"
}
$missingFrames = @($manifest.frames | Where-Object {
    -not (Test-Path -LiteralPath (Join-Path $OutputDirectory $_.path))
})
if ($missingFrames.Count -ne 0) {
    $missingNames = ($missingFrames | ForEach-Object { $_.path }) -join ", "
    throw "Foliage capture manifest references missing PNG frames: $missingNames"
}

$ffmpeg = Get-Command ffmpeg -ErrorAction SilentlyContinue
if ($null -ne $ffmpeg) {
    $videoPath = Join-Path $OutputDirectory "foliage-sweep.mp4"
    & $ffmpeg.Source -hide_banner -loglevel error -y -framerate 4 `
        -i (Join-Path $OutputDirectory "foliage-sweep-%02d.png") `
        -c:v libx264 -pix_fmt yuv420p $videoPath
    if ($LASTEXITCODE -ne 0) {
        throw "ffmpeg failed to assemble the foliage acceptance video"
    }
    Write-Host "Foliage acceptance passed. Video: $videoPath"
}
else {
    Write-Host "Foliage acceptance passed. Frames: $OutputDirectory"
}
