pub mod export_png;
pub mod export_jpg;
pub mod export_obj;
pub mod export_stl;

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
