param(
    [string]$UnityPath,
    [string]$OutputPath
)

$ErrorActionPreference = 'Stop'

$repositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$projectVersionFile = Join-Path $repositoryRoot 'ProjectSettings\ProjectVersion.txt'
$versionMatch = Select-String -Path $projectVersionFile -Pattern '^m_EditorVersion:\s*(.+)$'
if (-not $versionMatch) {
    throw "Could not read the Unity editor version from $projectVersionFile"
}

$editorVersion = $versionMatch.Matches[0].Groups[1].Value.Trim()
if ([string]::IsNullOrWhiteSpace($UnityPath)) {
    $UnityPath = Join-Path $env:ProgramFiles "Unity\Hub\Editor\$editorVersion\Editor\Unity.exe"
}
if (-not (Test-Path -LiteralPath $UnityPath -PathType Leaf)) {
    throw "Unity $editorVersion was not found at $UnityPath"
}

if ([string]::IsNullOrWhiteSpace($OutputPath)) {
    $OutputPath = Join-Path $repositoryRoot 'bevy-port\generated\unity-export.json'
} elseif (-not [IO.Path]::IsPathRooted($OutputPath)) {
    $OutputPath = Join-Path $repositoryRoot $OutputPath
}
$OutputPath = [IO.Path]::GetFullPath($OutputPath)
$outputDirectory = Split-Path -Parent $OutputPath
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
$logPath = [IO.Path]::ChangeExtension($OutputPath, '.log')

$unityArguments = @(
    '-batchmode',
    '-nographics',
    '-quit',
    '-projectPath', $repositoryRoot,
    '-executeMethod', 'StreamTown.Migration.BevyMigrationExporter.ExportForBatch',
    '-streamTownExport', $OutputPath,
    '-logFile', $logPath
)
$unityProcess = Start-Process -FilePath $UnityPath -ArgumentList $unityArguments -WindowStyle Hidden -Wait -PassThru
if ($unityProcess.ExitCode -ne 0) {
    throw "Unity exporter failed with exit code $($unityProcess.ExitCode). See $logPath"
}
if (-not (Test-Path -LiteralPath $OutputPath -PathType Leaf)) {
    throw "Unity reported success but did not create $OutputPath"
}
if (-not (Select-String -Path $logPath -SimpleMatch 'STREAM_TOWN_EXPORT_OK=' -Quiet)) {
    throw "Unity exited without the exporter success marker. See $logPath"
}

Write-Host "Unity $editorVersion migration export: $OutputPath"
Write-Host "Exporter log: $logPath"
