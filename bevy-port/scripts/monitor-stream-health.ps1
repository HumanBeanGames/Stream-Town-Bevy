param(
    [ValidateRange(1, 60)]
    [int]$WindowMinutes = 5,
    [ValidateRange(1.0, 240.0)]
    [double]$MinimumFps = 28.0
)

$ErrorActionPreference = 'Stop'
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$diagnosticsDirectory = Join-Path $workspaceRoot '.stream-town\diagnostics'
$broadcastLog = Join-Path $diagnosticsDirectory 'direct-broadcast.log'
$historyPath = Join-Path $diagnosticsDirectory 'six-hour-fps-monitor.csv'
$now = [DateTimeOffset]::Now
$windowStart = $now.AddMinutes(-$WindowMinutes)

function Read-Metric([string]$Line, [string]$Name) {
    $match = [regex]::Match($Line, "(?:^| )$([regex]::Escape($Name))=([0-9]+(?:\.[0-9]+)?)")
    if (-not $match.Success) {
        return $null
    }
    return [double]::Parse($match.Groups[1].Value, [Globalization.CultureInfo]::InvariantCulture)
}

function New-Result([string]$Status, [string]$Reason, [object[]]$Samples, $Process) {
    $captured = @($Samples | ForEach-Object { $_.Captured })
    $encoded = @($Samples | ForEach-Object { $_.Encoded })
    $averageCaptured = if ($captured.Count -gt 0) {
        [Math]::Round(($captured | Measure-Object -Average).Average, 3)
    } else { 0.0 }
    $averageEncoded = if ($encoded.Count -gt 0) {
        [Math]::Round(($encoded | Measure-Object -Average).Average, 3)
    } else { 0.0 }
    $minimumCaptured = if ($captured.Count -gt 0) {
        [Math]::Round(($captured | Measure-Object -Minimum).Minimum, 3)
    } else { 0.0 }
    $minimumEncoded = if ($encoded.Count -gt 0) {
        [Math]::Round(($encoded | Measure-Object -Minimum).Minimum, 3)
    } else { 0.0 }

    [ordered]@{
        Status = $Status
        Reason = $Reason
        WindowStart = $windowStart.ToString('o')
        WindowEnd = $now.ToString('o')
        Samples = $Samples.Count
        AverageCapturedFps = $averageCaptured
        AverageEncodedFps = $averageEncoded
        MinimumCapturedFps = $minimumCaptured
        MinimumEncodedFps = $minimumEncoded
        VideoDrops = [long](($Samples | Measure-Object -Property VideoDrops -Sum).Sum)
        CaptureReplacements = [long](($Samples | Measure-Object -Property CaptureReplacements -Sum).Sum)
        CadenceSkips = [long](($Samples | Measure-Object -Property CadenceSkips -Sum).Sum)
        ProcessId = if ($null -ne $Process) { $Process.Id } else { 0 }
        WorkingSetMiB = if ($null -ne $Process) { [Math]::Round($Process.WorkingSet64 / 1MB, 1) } else { 0.0 }
        PrivateMemoryMiB = if ($null -ne $Process) { [Math]::Round($Process.PrivateMemorySize64 / 1MB, 1) } else { 0.0 }
        Responding = $null -ne $Process -and $Process.Responding
    }
}

if (-not (Test-Path -LiteralPath $diagnosticsDirectory -PathType Container)) {
    New-Item -ItemType Directory -Path $diagnosticsDirectory -Force | Out-Null
}

$gameProcess = Get-Process -Name stream_town_game -ErrorAction SilentlyContinue |
    Sort-Object StartTime -Descending |
    Select-Object -First 1

$samples = @()
if (Test-Path -LiteralPath $broadcastLog -PathType Leaf) {
    foreach ($line in Get-Content -LiteralPath $broadcastLog -Tail 1000) {
        if ($line -notmatch '^(\d{13}) .*event=health ') {
            continue
        }
        $timestamp = [DateTimeOffset]::FromUnixTimeMilliseconds([long]$Matches[1])
        if ($timestamp -lt $windowStart -or $timestamp -gt $now) {
            continue
        }
        $captured = Read-Metric $line 'captured_fps'
        $encoded = Read-Metric $line 'encoded_fps'
        if ($null -eq $captured -or $null -eq $encoded) {
            continue
        }
        $samples += [pscustomobject]@{
            Timestamp = $timestamp
            Captured = $captured
            Encoded = $encoded
            VideoDrops = [long](Read-Metric $line 'video_drops')
            CaptureReplacements = [long](Read-Metric $line 'capture_replacements')
            CadenceSkips = [long](Read-Metric $line 'cadence_skips')
        }
    }
}

$reason = 'five-minute capture and encoder averages meet the configured threshold'
$status = 'healthy'
if ($null -eq $gameProcess) {
    $status = 'breach'
    $reason = 'stream_town_game is not running'
} elseif (-not $gameProcess.Responding) {
    $status = 'breach'
    $reason = 'stream_town_game is not responding'
} elseif ($samples.Count -lt [Math]::Max(3, $WindowMinutes * 6)) {
    $status = 'breach'
    $reason = "only $($samples.Count) health samples were recorded in the last $WindowMinutes minutes"
} else {
    $averageCaptured = ($samples | Measure-Object -Property Captured -Average).Average
    $averageEncoded = ($samples | Measure-Object -Property Encoded -Average).Average
    if ($averageCaptured -lt $MinimumFps -or $averageEncoded -lt $MinimumFps) {
        $status = 'breach'
        $reason = "average FPS fell below $MinimumFps (capture=$([Math]::Round($averageCaptured, 2)), encoded=$([Math]::Round($averageEncoded, 2)))"
    }
}

$result = New-Result $status $reason $samples $gameProcess
$row = [pscustomobject]$result
if (-not (Test-Path -LiteralPath $historyPath -PathType Leaf)) {
    $row | Export-Csv -LiteralPath $historyPath -NoTypeInformation
} else {
    $row | Export-Csv -LiteralPath $historyPath -NoTypeInformation -Append
}
$row | ConvertTo-Json -Compress

if ($status -ne 'healthy') {
    exit 2
}
