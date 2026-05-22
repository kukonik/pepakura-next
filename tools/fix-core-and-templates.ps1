param(
    [switch]$RunCheck = $true
)

$ErrorActionPreference = "Stop"

$projectRoot  = "D:\Dev\pepakura-next"
$templatesDir = "$projectRoot\templates"
$toolsDir     = "$projectRoot\tools"

if (-not (Test-Path $templatesDir)) {
    New-Item -Path $templatesDir -ItemType Directory -Force | Out-Null
}
if (-not (Test-Path $toolsDir)) {
    New-Item -Path $toolsDir -ItemType Directory -Force | Out-Null
}

$coreLibTemplate = @'
mod pdo_parser;
mod pepa_scene_adapter;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsePdoResult {
    pub success: bool,
    pub error: Option<String>,
    pub scene: Option<pepa_scene_adapter::PepaScene>,
}

/// Parse a PDO file and convert to PepaScene (core function without Tauri)
pub fn parse_pdo_to_pepa_core(data: &[u8]) -> ParsePdoResult {
    match pdo_parser::PdoModel::parse_from_bytes(data) {
        Ok(pdo_model) => {
            let pepa_scene: pepa_scene_adapter::PepaScene = pdo_model.into();
            ParsePdoResult {
                success: true,
                error: None,
                scene: Some(pepa_scene),
            }
        }
        Err(e) => {
            ParsePdoResult {
                success: false,
                error: Some(e.to_string()),
                scene: None,
            }
        }
    }
}

// Re-export everything from submodules
pub use pdo_parser::*;
pub use pepa_scene_adapter::*;
'@

Set-Content -Path "$templatesDir\core-lib.rs.tmpl" -Value $coreLibTemplate -Encoding UTF8

$commandsTemplate = @'
// Импорт из ядра
use pepakura_core::{parse_pdo_to_pepa_core, ParsePdoResult};

/// Обёртка для вызова парсера PDO из Tauri
#[tauri::command]
pub fn parse_pdo_to_pepa(data: Vec<u8>) -> ParsePdoResult {
    // Передаем данные в ядро (ссылка на массив байт)
    parse_pdo_to_pepa_core(&data)
}
'@

Set-Content -Path "$templatesDir\commands.rs.tmpl" -Value $commandsTemplate -Encoding UTF8

$depsTemplate = @'
[dependencies]
tauri = { version = "1.5", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pepakura_core = { path = "../crates/pepakura_core" }
# ... остальные зависимости
'@

Set-Content -Path "$templatesDir\tauri-deps.toml.tmpl" -Value $depsTemplate -Encoding UTF8

$coreSrcDir = "$projectRoot\crates\pepakura_core\src"
if (-not (Test-Path "$coreSrcDir\pepa_scene_adapter.rs")) {
    $pepaSceneAdapter = @'
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PepaScene {
    // TODO: реальные поля PepaScene
}
'@
    Set-Content -Path "$coreSrcDir\pepa_scene_adapter.rs" -Value $pepaSceneAdapter -Encoding UTF8
}

$applyTemplatesScript = @'
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
'@

Set-Content -Path "$toolsDir\apply-templates.ps1" -Value $applyTemplatesScript -Encoding UTF8

if ($RunCheck) {
    & "$toolsDir\apply-templates.ps1" -Backup:$false

    Push-Location "$projectRoot\crates\pepakura_core"
    cargo check
    Pop-Location
}
