use super::nesting::NestResult;

#[derive(Debug, Clone)] pub enum PageSize { A4 }
#[derive(Debug, Clone)] pub enum PdfOrientation { Portrait, Landscape }
#[derive(Debug, Clone)] pub struct PdfExportConfig { pub orientation: PdfOrientation, pub size: PageSize }
#[derive(Debug, Clone)] pub enum DxfUnits { Mm }
#[derive(Debug, Clone)] pub struct DxfExportConfig { pub units: DxfUnits }
#[derive(Debug, Clone)] pub struct TextureExportConfig { pub scale: f32 }

pub fn export_nest_result_to_svgs(_result: &NestResult, _out_dir: &str) -> Result<(), String> { Ok(()) }
pub fn export_sheet_to_svg(_parts: &[super::nesting::NestPart], _path: &str) -> Result<(), String> { Ok(()) }
pub fn export_pdf(_result: &NestResult, _config: &PdfExportConfig) -> Result<Vec<u8>, String> { Ok(vec![]) }
pub fn export_pdf_to_file(_result: &NestResult, _config: &PdfExportConfig, _path: &str) -> Result<(), String> { Ok(()) }
pub fn export_dxf(_result: &NestResult, _config: &DxfExportConfig) -> Result<String, String> { Ok(String::new()) }
pub fn export_dxf_to_file(_result: &NestResult, _config: &DxfExportConfig, _path: &str) -> Result<(), String> { Ok(()) }
pub fn export_textures(_mesh: &super::geometry::Mesh, _config: &TextureExportConfig, _out_dir: &str) -> Result<(), String> { Ok(()) }
pub fn generate_uv_from_position(_mesh: &super::geometry::Mesh) -> Result<Vec<[f32; 2]>, String> { Ok(vec![]) }
