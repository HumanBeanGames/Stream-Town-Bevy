[CmdletBinding()]
param(
    [switch]$Release,
    [switch]$ValidateOnly
)

$ErrorActionPreference = 'Stop'
$workspaceRoot = Resolve-Path (Join-Path $PSScriptRoot '..')

Push-Location $workspaceRoot
try {
    if ($ValidateOnly) {
        cargo run -p stream_town_tools -- --validate-authoring
        exit $LASTEXITCODE
    }

    if ($Release) {
        cargo run --release -p stream_town_tools
    }
    else {
        cargo run -p stream_town_tools
    }
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}
