param(
    [string]$BlenderPath,
    [string]$OutputRoot,
    [string[]]$Only
)

$ErrorActionPreference = 'Stop'

$repositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$requiredVersion = (Get-Content (Join-Path $repositoryRoot 'bevy-port\blender-version.txt') -Raw).Trim()
if ($requiredVersion -notmatch '^(\d+)\.(\d+)\.(\d+)$') {
    throw "Invalid pinned Blender version: $requiredVersion"
}
if ([string]::IsNullOrWhiteSpace($BlenderPath)) {
    $majorMinor = "$($Matches[1]).$($Matches[2])"
    $BlenderPath = "C:\Program Files\Blender Foundation\Blender $majorMinor\blender.exe"
}
if (-not (Test-Path -LiteralPath $BlenderPath -PathType Leaf)) {
    throw "Pinned Blender $requiredVersion was not found at $BlenderPath"
}
$reportedVersion = (& $BlenderPath --version | Select-Object -First 1).Trim()
if ($LASTEXITCODE -ne 0 -or $reportedVersion -ne "Blender $requiredVersion") {
    throw "Expected Blender $requiredVersion but found '$reportedVersion' at $BlenderPath"
}

if ([string]::IsNullOrWhiteSpace($OutputRoot)) {
    $OutputRoot = Join-Path $repositoryRoot 'bevy-port\assets\migrated\models'
} elseif (-not [IO.Path]::IsPathRooted($OutputRoot)) {
    $OutputRoot = Join-Path $repositoryRoot $OutputRoot
}
$OutputRoot = [IO.Path]::GetFullPath($OutputRoot)
New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null
$reportPath = Join-Path $OutputRoot 'model-conversion.json'
$stdoutPath = Join-Path $OutputRoot 'blender.stdout.log'
$stderrPath = Join-Path $OutputRoot 'blender.stderr.log'
$pythonScript = Join-Path $PSScriptRoot 'blender\convert_fbx_to_glb.py'

$blenderArguments = @(
    '--background',
    '--factory-startup',
    '--python', $pythonScript,
    '--',
    '--repo-root', $repositoryRoot,
    '--output-root', $OutputRoot,
    '--report', $reportPath
)
foreach ($source in $Only) {
    $blenderArguments += @('--only', $source)
}

$blenderProcess = Start-Process -FilePath $BlenderPath -ArgumentList $blenderArguments -WindowStyle Hidden -Wait -PassThru -RedirectStandardOutput $stdoutPath -RedirectStandardError $stderrPath
if ($blenderProcess.ExitCode -ne 0) {
    Get-Content $stdoutPath -Tail 20
    Get-Content $stderrPath -Tail 20
    throw "Blender conversion failed with exit code $($blenderProcess.ExitCode). See $stdoutPath"
}
if (-not (Test-Path -LiteralPath $reportPath -PathType Leaf)) {
    throw "Blender exited without creating $reportPath"
}

Write-Host "Model conversion report: $reportPath"
Write-Host "Blender log: $stdoutPath"
