param(
    [string]$Town,
    [switch]$Debug,
    [switch]$SkipBuild,
    [switch]$NoLaunch,
    [switch]$Wait
)

$ErrorActionPreference = 'Stop'
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$saveDirectory = Join-Path $workspaceRoot '.stream-town\saves'

if (Get-Process -Name stream_town_game -ErrorAction SilentlyContinue) {
    throw 'Stream Town is already running. Exit it normally first so the active town is saved before redeploying.'
}
if (-not (Test-Path -LiteralPath $saveDirectory -PathType Container)) {
    throw "Save directory does not exist: $saveDirectory"
}

$townSaves = @(Get-ChildItem -LiteralPath $saveDirectory -File -Filter '*.stbevy')
if ($townSaves.Count -eq 0) {
    throw "No current town saves were found in $saveDirectory"
}

if ([string]::IsNullOrWhiteSpace($Town)) {
    $selectedSave = $townSaves |
        Sort-Object LastWriteTimeUtc, Name -Descending |
        Select-Object -First 1
}
else {
    $requestedTown = [System.IO.Path]::GetFileNameWithoutExtension($Town.Trim())
    $selectedSave = $townSaves |
        Where-Object { $_.BaseName -ieq $requestedTown } |
        Select-Object -First 1
    if ($null -eq $selectedSave) {
        $available = ($townSaves.BaseName | Sort-Object) -join ', '
        throw "Town '$Town' was not found. Available towns: $available"
    }
}

$profile = if ($Debug) { 'debug' } else { 'release' }
$cargoArguments = @('build', '-p', 'stream_town_game')
if (-not $Debug) {
    $cargoArguments += '--release'
}

Push-Location $workspaceRoot
try {
    if (-not $SkipBuild) {
        & cargo @cargoArguments
        if ($LASTEXITCODE -ne 0) {
            throw "The $profile game build failed with exit code $LASTEXITCODE"
        }
    }

    $executable = Join-Path $workspaceRoot "target\$profile\stream_town_game.exe"
    if (-not (Test-Path -LiteralPath $executable -PathType Leaf)) {
        throw "The $profile game executable does not exist: $executable"
    }
    $nativeRuntime = Join-Path $workspaceRoot 'vcpkg_installed\x64-windows\bin'
    $ffmpegRuntimePresent = (Test-Path -LiteralPath $nativeRuntime -PathType Container) -and
        (@(Get-ChildItem -LiteralPath $nativeRuntime -File -Filter 'avcodec-*.dll').Count -gt 0)
    if (-not $ffmpegRuntimePresent) {
        throw "The linked FFmpeg runtime is missing from: $nativeRuntime"
    }

    Write-Host "Redeploy ready: $($selectedSave.BaseName) ($profile)"
    if ($NoLaunch) {
        return
    }

    $previousResumePath = [Environment]::GetEnvironmentVariable(
        'STREAM_TOWN_AUTO_RESUME_PATH',
        [EnvironmentVariableTarget]::Process
    )
    $previousProcessPath = $env:PATH
    try {
        $env:STREAM_TOWN_AUTO_RESUME_PATH = $selectedSave.FullName
        $env:PATH = "$nativeRuntime;$previousProcessPath"
        $game = Start-Process `
            -FilePath $executable `
            -WorkingDirectory $workspaceRoot `
            -PassThru
    }
    finally {
        if ($null -eq $previousResumePath) {
            Remove-Item Env:\STREAM_TOWN_AUTO_RESUME_PATH -ErrorAction SilentlyContinue
        }
        else {
            $env:STREAM_TOWN_AUTO_RESUME_PATH = $previousResumePath
        }
        $env:PATH = $previousProcessPath
    }

    Write-Host "Started $($selectedSave.BaseName) with process ID $($game.Id)."
    if ($Wait) {
        $game.WaitForExit()
        exit $game.ExitCode
    }
}
finally {
    Pop-Location
}
