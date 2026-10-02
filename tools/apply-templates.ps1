param(
    [switch]$Backup = $true
)

if ($PSScriptRoot) {
    $projectRoot = Split-Path $PSScriptRoot -Parent
} else {
    $projectRoot = "D:\Dev\pepakura-next"
}

$templatesDir = Join-Path $projectRoot "templates"

if ($Backup) {
    $backupDir = Join-Path $projectRoot "backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    if (Test-Path (Join-Path $projectRoot "crates")) {
        Copy-Item -Path (Join-Path $projectRoot "crates") -Destination $backupDir -Recurse -Force
    }
    if (Test-Path (Join-Path $projectRoot "src-tauri")) {
        Copy-Item -Path (Join-Path $projectRoot "src-tauri") -Destination $backupDir -Recurse -Force
    }
}

$coreLibDest  = Join-Path $projectRoot "crates\pepakura_core\src\lib.rs"
$commandsDest = Join-Path $projectRoot "src-tauri\src\commands.rs"
$typesDest    = Join-Path $projectRoot "packages\ui-desktop\src\types\pepakura.ts"

$destDirs = @($coreLibDest, $commandsDest, $typesDest) | ForEach-Object { Split-Path $_ -Parent }
$destDirs | ForEach-Object {
    if (-not (Test-Path $_)) { New-Item -ItemType Directory -Path $_ -Force | Out-Null }
}

Copy-Item (Join-Path $templatesDir "core-lib.rs.tmpl")        $coreLibDest    -Force
Copy-Item (Join-Path $templatesDir "commands.rs.tmpl")        $commandsDest   -Force
if (Test-Path (Join-Path $templatesDir "pepakura-types.ts.tmpl")) {
    Copy-Item (Join-Path $templatesDir "pepakura-types.ts.tmpl") $typesDest -Force
}

$cargoPath = Join-Path $projectRoot "src-tauri\Cargo.toml"
if (Test-Path $cargoPath) {
    $cargoContent = Get-Content $cargoPath -Raw
    $depsTemplate = Get-Content (Join-Path $templatesDir "tauri-deps.toml.tmpl") -Raw

    $newCargo = [regex]::Replace(
        $cargoContent,
        '\[dependencies\][\s\S]*?(?=\[\w|\z)',
        $depsTemplate,
        [System.Text.RegularExpressions.RegexOptions]::Multiline
    )

    Set-Content -Path $cargoPath -Value $newCargo -Encoding UTF8
}
