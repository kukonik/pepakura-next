//! Export functionality for the desktop application

use pepakura_core::unfold::UnfoldResult;
use pepakura_core::export::{export_to_svg, export_to_png, export_to_jpg, export_to_obj, export_to_stl};

/// Export format enumeration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ExportFormat {
    SVG,
    PNG,
    JPG,
    OBJ,
    STL,
}

/// Export the unfold result to the specified format
/// 
/// # Arguments
/// * `result` - The unfold result to export
/// * `format` - The format to export to
/// 
/// # Returns
/// * `Vec<u8>` - The exported data
pub fn export_result(result: &UnfoldResult, format: ExportFormat) -> Vec<u8> {
    match format {
        ExportFormat::SVG => export_to_svg(result),
        ExportFormat::PNG => export_to_png(result),
        ExportFormat::JPG => export_to_jpg(result),
        ExportFormat::OBJ => export_to_obj(result),
        ExportFormat::STL => export_to_stl(result),
    }
}

/// Get the file extension for the specified format
/// 
/// # Arguments
/// * `format` - The format to get the extension for
/// 
/// # Returns
/// * `&'static str` - The file extension
pub fn get_file_extension(format: &ExportFormat) -> &'static str {
    match format {
        ExportFormat::SVG => "svg",
        ExportFormat::PNG => "png",
        ExportFormat::JPG => "jpg",
        ExportFormat::OBJ => "obj",
        ExportFormat::STL => "stl",
    }
}

/// Get the MIME type for the specified format
/// 
/// # Arguments
/// * `format` - The format to get the MIME type for
/// 
/// # Returns
/// * `&'static str` - The MIME type
pub fn get_mime_type(format: &ExportFormat) -> &'static str {
    match format {
        ExportFormat::SVG => "image/svg+xml",
        ExportFormat::PNG => "image/png",
        ExportFormat::JPG => "image/jpeg",
        ExportFormat::OBJ => "application/octet-stream",
        ExportFormat::STL => "model/stl",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pepakura_core::unfold::{Layout, Face2D, SeamInfo, UnfoldStats};
    
    #[test]
    fn test_export_format_extensions() {
        assert_eq!(get_file_extension(&ExportFormat::SVG), "svg");
        assert_eq!(get_file_extension(&ExportFormat::PNG), "png");
        assert_eq!(get_file_extension(&ExportFormat::JPG), "jpg");
        assert_eq!(get_file_extension(&ExportFormat::OBJ), "obj");
        assert_eq!(get_file_extension(&ExportFormat::STL), "stl");
    }
    
    #[test]
    fn test_export_format_mime_types() {
        assert_eq!(get_mime_type(&ExportFormat::SVG), "image/svg+xml");
        assert_eq!(get_mime_type(&ExportFormat::PNG), "image/png");
        assert_eq!(get_mime_type(&ExportFormat::JPG), "image/jpeg");
        assert_eq!(get_mime_type(&ExportFormat::OBJ), "application/octet-stream");
        assert_eq!(get_mime_type(&ExportFormat::STL), "model/stl");
    }
    
    #[test]
    fn test_export_functions() {
        let result = UnfoldResult {
            layouts: vec![
                Layout {
                    vertices: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                    faces: vec![Face2D {
                        vertex_indices: vec![0, 1, 2, 3],
                        original_face_index: 0,
                    }],
                    bounds: [0.0, 0.0, 1.0, 1.0],
                }
            ],
            seams: vec![
                SeamInfo {
                    face_indices: [0, 1],
                    length: 1.0,
                }
            ],
            stats: UnfoldStats {
                face_count: 1,
                layout_count: 1,
                seam_count: 1,
                processing_time: 0.001,
            },
        };
        
        // Test that export functions return non-empty data
        assert!(!export_result(&result, ExportFormat::SVG).is_empty());
        assert!(!export_result(&result, ExportFormat::PNG).is_empty());
        assert!(!export_result(&result, ExportFormat::JPG).is_empty());
        assert!(!export_result(&result, ExportFormat::OBJ).is_empty());
        assert!(!export_result(&result, ExportFormat::STL).is_empty());
    }
}