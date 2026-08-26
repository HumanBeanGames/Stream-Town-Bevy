param(
    [string]$OutputDirectory = "",
    [string[]]$Scenario = @(),
    [switch]$UpdateBaseline,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
$scenarioPath = Join-Path $workspace "assets\acceptance\visual-scenarios.json"
$scenarios = Get-Content -LiteralPath $scenarioPath -Raw | ConvertFrom-Json
$selectedScenarios = @($scenarios.scenarios)
if ($Scenario.Count -gt 0) {
    $selectedScenarios = @($selectedScenarios | Where-Object { $Scenario -contains $_.name })
    if ($selectedScenarios.Count -ne $Scenario.Count) {
        throw "One or more requested visual-acceptance scenarios are unknown"
    }
    if ($UpdateBaseline) {
        throw "Updating the curated baseline requires the complete scenario matrix"
    }
}
if ([string]::IsNullOrWhiteSpace($OutputDirectory)) {
    $stamp = Get-Date -Format "yyyyMMdd-HHmmss"
    $OutputDirectory = Join-Path $workspace ".stream-town\diagnostics\visual-$stamp"
}
$OutputDirectory = [System.IO.Path]::GetFullPath($OutputDirectory)
if ((Test-Path -LiteralPath $OutputDirectory) -and
    (Get-ChildItem -LiteralPath $OutputDirectory -Force | Select-Object -First 1)) {
    throw "Capture directory is not empty: $OutputDirectory"
}
[System.IO.Directory]::CreateDirectory($OutputDirectory) | Out-Null

Push-Location $workspace
try {
    if (-not $SkipBuild) {
        cargo build -p stream_town_game
        if ($LASTEXITCODE -ne 0) {
            throw "The visual-acceptance game build failed with exit code $LASTEXITCODE"
        }
    }
    $executable = Join-Path $workspace "target\debug\stream_town_game.exe"
    if (-not (Test-Path -LiteralPath $executable)) {
        throw "The visual-acceptance executable is missing: $executable"
    }
    $ffmpegRuntime = Get-ChildItem (Join-Path $workspace "target\debug\build") -Directory -Filter "ffmpeg-sys-next-*" |
        ForEach-Object { Join-Path $_.FullName "out" } |
        Where-Object {
            (Test-Path -LiteralPath $_) -and
            @(Get-ChildItem -LiteralPath $_ -Filter "avcodec-*.dll" -File).Count -gt 0
        } |
        Sort-Object { (Get-Item -LiteralPath $_).LastWriteTime } -Descending |
        Select-Object -First 1

    foreach ($case in $selectedScenarios) {
        $capturePath = Join-Path $OutputDirectory "$($case.name).png"
        Write-Host "Capturing $($case.name)..."
        $start = New-Object System.Diagnostics.ProcessStartInfo
        $start.FileName = $executable
        $start.WorkingDirectory = $workspace
        $start.UseShellExecute = $false
        $start.CreateNoWindow = $true
        $streamTownKeys = @($start.Environment.Keys | Where-Object { $_ -like "STREAM_TOWN_*" })
        foreach ($key in $streamTownKeys) {
            $start.Environment.Remove($key) | Out-Null
        }
        # Acceptance processes must be deterministic and must never inherit a
        # developer's Twitch-enabled .stream-town/config.ron. The checked-in
        # configuration keeps both IRC and direct broadcasting disabled.
        $start.Environment["STREAM_TOWN_CONFIG"] = Join-Path $workspace "assets\config\game.ron"
        $start.Environment["STREAM_TOWN_SCREENSHOT"] = $capturePath
        $start.Environment["STREAM_TOWN_SCREENSHOT_DELAY"] = [string]$case.delay_seconds
        $start.Environment["STREAM_TOWN_EXIT_AFTER_SCREENSHOT"] = "1"
        $start.Environment["STREAM_TOWN_DISABLE_DIRECT_BROADCAST"] = "1"
        if ($null -ne $ffmpegRuntime) {
            $start.Environment["PATH"] = "$ffmpegRuntime;$($start.Environment['PATH'])"
        }
        foreach ($property in $case.environment.PSObject.Properties) {
            $start.Environment[$property.Name] = [string]$property.Value
        }
        $process = [System.Diagnostics.Process]::Start($start)
        if (-not $process.WaitForExit(240000)) {
            $process.Kill($true)
            throw "Visual-acceptance scenario timed out: $($case.name)"
        }
        if ($process.ExitCode -ne 0) {
            throw "Visual-acceptance scenario $($case.name) failed with exit code $($process.ExitCode)"
        }
        if (-not (Test-Path -LiteralPath $capturePath)) {
            throw "Visual-acceptance scenario did not write $capturePath"
        }
    }

    $arguments = @(
        "run", "-p", "xtask", "--", "visual-acceptance",
        "--capture-dir", $OutputDirectory
    )
    if ($UpdateBaseline) {
        $arguments += "--update-baseline"
    }
    foreach ($name in $Scenario) {
        $arguments += @("--scenario", $name)
    }
    cargo @arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Visual acceptance comparison failed with exit code $LASTEXITCODE"
    }
    Write-Host "Visual acceptance passed. Full-resolution captures: $OutputDirectory"
}
finally {
    Pop-Location
}
