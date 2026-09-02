[CmdletBinding()]
param(
    [switch]$Debug
)

$ErrorActionPreference = 'Stop'
$workspaceRoot = Resolve-Path (Join-Path $PSScriptRoot '..')
$cargoArguments = @('run', '--release', '-p', 'stream_town_game')
if ($Debug) {
    $cargoArguments = @('run', '-p', 'stream_town_game')
}

Push-Location $workspaceRoot
try {
    & cargo @cargoArguments
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}
