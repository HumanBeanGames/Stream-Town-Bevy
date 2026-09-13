param(
    [string]$Town,
    [switch]$NewTown,
    [switch]$Debug,
    [switch]$SkipBuild,
    [switch]$NoLaunch,
    [switch]$Wait,
    [switch]$NoProfiling
)

$ErrorActionPreference = 'Stop'
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$saveDirectory = Join-Path $workspaceRoot '.stream-town\saves'

function Resolve-StreamTownTwitchClientId {
    $configured = [Environment]::GetEnvironmentVariable(
        'STREAM_TOWN_TWITCH_CLIENT_ID',
        [EnvironmentVariableTarget]::Process
    )
    if (-not [string]::IsNullOrWhiteSpace($configured)) {
        return $configured.Trim()
    }
    $credentialListing = (& cmdkey.exe /list 2>$null) -join "`n"
    $credential = [regex]::Match(
        $credentialListing,
        'broadcast:([A-Za-z0-9]+):[^\s]+\.stream-town-twitch'
    )
    if (-not $credential.Success) {
        throw 'No authorized Stream Town broadcaster credential was found. Authorize Twitch from the Secrets menu first.'
    }
    return $credential.Groups[1].Value
}

if (Get-Process -Name stream_town_game -ErrorAction SilentlyContinue) {
    throw 'Stream Town is already running. Exit it normally first so the active town is saved before redeploying.'
}
if ($NewTown) {
    if ([string]::IsNullOrWhiteSpace($Town)) {
        throw 'A town name is required with -NewTown.'
    }
    $requestedTown = [System.IO.Path]::GetFileNameWithoutExtension($Town.Trim())
    if ($requestedTown -ne $Town.Trim() -or
        $requestedTown.IndexOfAny([System.IO.Path]::GetInvalidFileNameChars()) -ge 0) {
        throw "Town '$Town' is not a valid save name."
    }
    if (-not (Test-Path -LiteralPath $saveDirectory -PathType Container)) {
        New-Item -ItemType Directory -Path $saveDirectory -Force | Out-Null
    }
    $newSavePath = Join-Path $saveDirectory "$requestedTown.stbevy"
    if (Test-Path -LiteralPath $newSavePath) {
        throw "Town '$requestedTown' already exists. Redeploy it without -NewTown."
    }
    $selectedSave = [System.IO.FileInfo]::new($newSavePath)
}
else {
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
}

$profile = if ($Debug) { 'debug' } else { 'release' }
$cargoArguments = @('build', '-p', 'stream_town_game')
if (-not $Debug) {
    $cargoArguments += '--release'
}
if (-not $NoProfiling) {
    $cargoArguments += @('--features', 'stream-profiling')
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

    $nativeLibraries = @(Get-ChildItem -LiteralPath $nativeRuntime -File -Filter '*.dll')
    if ($nativeLibraries.Count -eq 0) {
        throw "No native runtime libraries were found in: $nativeRuntime"
    }
    $executableDirectory = Split-Path -Parent $executable
    foreach ($library in $nativeLibraries) {
        Copy-Item `
            -LiteralPath $library.FullName `
            -Destination (Join-Path $executableDirectory $library.Name) `
            -Force
    }

    $profilingStatus = if ($NoProfiling) { 'disabled' } else { 'enabled' }
    $deploymentKind = if ($NewTown) { 'new town' } else { 'saved town' }
    Write-Host "Redeploy ready: $($selectedSave.BaseName) ($deploymentKind, $profile); profiling $profilingStatus; staged $($nativeLibraries.Count) native runtime libraries."
    if ($NoLaunch) {
        return
    }

    $previousResumePath = [Environment]::GetEnvironmentVariable(
        'STREAM_TOWN_AUTO_RESUME_PATH',
        [EnvironmentVariableTarget]::Process
    )
    $previousAutoGoLive = [Environment]::GetEnvironmentVariable(
        'STREAM_TOWN_AUTO_GO_LIVE',
        [EnvironmentVariableTarget]::Process
    )
    $previousSavePath = [Environment]::GetEnvironmentVariable(
        'STREAM_TOWN_SAVE_PATH',
        [EnvironmentVariableTarget]::Process
    )
    $previousNewTownName = [Environment]::GetEnvironmentVariable(
        'STREAM_TOWN_NEW_TOWN_NAME',
        [EnvironmentVariableTarget]::Process
    )
    $previousAutostart = [Environment]::GetEnvironmentVariable(
        'STREAM_TOWN_AUTOSTART',
        [EnvironmentVariableTarget]::Process
    )
    $previousClientId = [Environment]::GetEnvironmentVariable(
        'STREAM_TOWN_TWITCH_CLIENT_ID',
        [EnvironmentVariableTarget]::Process
    )
    $twitchClientId = Resolve-StreamTownTwitchClientId
    $previousProcessPath = $env:PATH
    try {
        if ($NewTown) {
            Remove-Item Env:\STREAM_TOWN_AUTO_RESUME_PATH -ErrorAction SilentlyContinue
            $env:STREAM_TOWN_SAVE_PATH = $selectedSave.FullName
            $env:STREAM_TOWN_NEW_TOWN_NAME = $selectedSave.BaseName
            $env:STREAM_TOWN_AUTOSTART = '1'
        }
        else {
            Remove-Item Env:\STREAM_TOWN_SAVE_PATH -ErrorAction SilentlyContinue
            Remove-Item Env:\STREAM_TOWN_NEW_TOWN_NAME -ErrorAction SilentlyContinue
            Remove-Item Env:\STREAM_TOWN_AUTOSTART -ErrorAction SilentlyContinue
            $env:STREAM_TOWN_AUTO_RESUME_PATH = $selectedSave.FullName
        }
        $env:STREAM_TOWN_AUTO_GO_LIVE = '1'
        $env:STREAM_TOWN_TWITCH_CLIENT_ID = $twitchClientId
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
        if ($null -eq $previousAutoGoLive) {
            Remove-Item Env:\STREAM_TOWN_AUTO_GO_LIVE -ErrorAction SilentlyContinue
        }
        else {
            $env:STREAM_TOWN_AUTO_GO_LIVE = $previousAutoGoLive
        }
        if ($null -eq $previousSavePath) {
            Remove-Item Env:\STREAM_TOWN_SAVE_PATH -ErrorAction SilentlyContinue
        }
        else {
            $env:STREAM_TOWN_SAVE_PATH = $previousSavePath
        }
        if ($null -eq $previousNewTownName) {
            Remove-Item Env:\STREAM_TOWN_NEW_TOWN_NAME -ErrorAction SilentlyContinue
        }
        else {
            $env:STREAM_TOWN_NEW_TOWN_NAME = $previousNewTownName
        }
        if ($null -eq $previousAutostart) {
            Remove-Item Env:\STREAM_TOWN_AUTOSTART -ErrorAction SilentlyContinue
        }
        else {
            $env:STREAM_TOWN_AUTOSTART = $previousAutostart
        }
        if ($null -eq $previousClientId) {
            Remove-Item Env:\STREAM_TOWN_TWITCH_CLIENT_ID -ErrorAction SilentlyContinue
        }
        else {
            $env:STREAM_TOWN_TWITCH_CLIENT_ID = $previousClientId
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
