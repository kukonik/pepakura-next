pub mod export_png;
pub mod export_jpg;
pub mod export_obj;
pub mod export_stl;
pub mod svg;

// Re-export SVG types
pub use svg::{PageSize, SvgExportConfig, export_svg};

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub format: String,
    pub quality: u8,
}

pub fn export_to_pdf(_layout: &crate::unfold::LayoutResult, _options: &ExportOptions) -> Result<(), String> {
    Ok(())
}

pub fn export_to_svg(_layout: &crate::unfold::LayoutResult, _options: &ExportOptions) -> Result<(), String> {
    Ok(())
}