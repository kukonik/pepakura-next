// ===== Заглушки экспорта (реальная реализация — Этап 2/3) =====

#[derive(Debug, Clone)] pub enum PageSize { A4, A3, A2, A1, Letter }
#[derive(Debug, Clone)] pub enum PdfOrientation { Portrait, Landscape }

#[derive(Debug, Clone)]
pub struct PdfExportConfig {
    pub orientation: PdfOrientation,
    pub page_size: PageSize,
    pub scale: f64,
    pub show_fold_lines: bool,
    pub show_cut_lines: bool,
    pub show_part_numbers: bool,
    pub show_grid: bool,
    pub grid_size_mm: f64,
}

#[derive(Debug, Clone)] pub enum DxfUnits { Mm, Millimeters, Centimeters, Meters, Inches }

#[derive(Debug, Clone)]
pub struct DxfExportConfig {
    pub units: DxfUnits,
    pub page_size: PageSize,
    pub scale: f64,
    pub export_cut_lines: bool,
    pub export_fold_lines: bool,
    pub export_part_numbers: bool,
}

#[derive(Debug, Clone)]
pub struct TextureExportConfig {
    pub texture_width: u32,
    pub texture_height: u32,
    pub format: String,
    pub quality: u8,
}

// Обобщённые заглушки: принимают любой Serialize, чтобы сходиться с вызовами commands.rs
pub fn export_nest_result_to_svgs<T: serde::Serialize, U: serde::Serialize>(_result: &T, _unfold: &U) -> Vec<String> { vec![] }
pub fn export_sheet_to_svg<T: serde::Serialize, U: serde::Serialize>(_parts: &T, _unfold: &U) -> String { String::new() }
pub fn export_pdf<T: serde::Serialize>(_result: &T, _config: &PdfExportConfig) -> Result<Vec<u8>, String> { Ok(vec![]) }
pub fn export_pdf_to_file<T: serde::Serialize>(_result: &T, _config: &PdfExportConfig, _path: &str) -> Result<(), String> { Ok(()) }
pub fn export_dxf<T: serde::Serialize>(_result: &T, _config: &DxfExportConfig) -> Result<String, String> { Ok(String::new()) }
pub fn export_dxf_to_file<T: serde::Serialize>(_result: &T, _config: &DxfExportConfig, _path: &str) -> Result<(), String> { Ok(()) }
pub fn export_textures<T: serde::Serialize>(_mesh: &T, _config: &TextureExportConfig, _out_dir: &str) -> Result<(), String> { Ok(()) }
pub fn generate_uv_from_position<T: serde::Serialize>(_mesh: &T) -> Result<Vec<[f32; 2]>, String> { Ok(vec![]) }
