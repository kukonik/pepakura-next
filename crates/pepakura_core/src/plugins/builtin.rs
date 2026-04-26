//! Встроенные плагины.
//! 
//! Предоставляет стандартные плагины для распространённых форматов.

use super::traits::*;
use crate::geometry::{Face, Mesh, Vertex};
use crate::export::{SvgExportConfig, export_svg_to_file};
use crate::unfold::UnfoldConfig;
use crate::sanitize::{sanitize_mesh, SanitizeOptions};
use crate::PepakuraError;
use std::path::Path;

/// Импортёр Wavefront OBJ.
pub struct ObjImporter;

impl ImportPlugin for ObjImporter {
    fn name(&self) -> &str {
        "Wavefront OBJ"
    }
    
    fn supported_extensions(&self) -> &[&str] {
        &["obj"]
    }
    
    fn import(&self, path: &Path) -> Result<Mesh, PepakuraError> {
        // Используем существующий obj парсер
        let content = std::fs::read_to_string(path)
            .map_err(|e| PepakuraError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read OBJ file: {}", e),
            )))?;
        
        parse_obj_content(&content, path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unnamed"))
    }
}

/// Парсит содержимое OBJ файла.
fn parse_obj_content(content: &str, name: &str) -> Result<Mesh, PepakuraError> {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut vertex_id = 0;
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "v" => {
                // Вершина: v x y z
                if parts.len() >= 4 {
                    let x: f64 = parts[1].parse().map_err(|_| PepakuraError::ParseError("Invalid vertex x".to_string()))?;
                    let y: f64 = parts[2].parse().map_err(|_| PepakuraError::ParseError("Invalid vertex y".to_string()))?;
                    let z: f64 = parts[3].parse().map_err(|_| PepakuraError::ParseError("Invalid vertex z".to_string()))?;
                    
                    vertices.push(Vertex::new(vertex_id, [x, y, z]));
                    vertex_id += 1;
                }
            }
            "f" => {
                // Грань: f v1 v2 v3 или f v1/vt1/vn1 v2/vt2/vn2 ...
                let mut face_vertices = Vec::new();
                
                for part in &parts[1..] {
                    // Берём только индекс вершины (до первого /)
                    let vertex_idx = part.split('/').next().ok_or_else(|| {
                        PepakuraError::ParseError("Invalid face format".to_string())
                    })?;
                    
                    let idx: usize = vertex_idx.parse().map_err(|_| {
                        PepakuraError::ParseError(format!("Invalid vertex index: {}", vertex_idx))
                    })?;
                    
                    // OBJ использует 1-based индексы
                    face_vertices.push(idx - 1);
                }
                
                // Триангулируем полигон (fan triangulation)
                if face_vertices.len() >= 3 {
                    for i in 1..face_vertices.len() - 1 {
                        faces.push(Face::new(
                            face_vertices[0],
                            face_vertices[i],
                            face_vertices[i + 1],
                        ));
                    }
                }
            }
            _ => {}
        }
    }
    
    if vertices.is_empty() {
        return Err(PepakuraError::EmptyMesh);
    }
    
    let mut mesh = Mesh::with_data(name, vertices, faces);
    mesh.metadata.description = Some("Imported from OBJ".to_string());
    
    // АВТОМАТИЧЕСКАЯ ОЧИСТКА И УПРОЩЕНИЕ
    // Упрощаем меш до безопасного количества граней (по умолчанию 5000)
    let sanitized_mesh = sanitize_mesh(&mesh)
        .map_err(|e| PepakuraError::ParseError(format!("Sanitization failed: {}", e)))?;
    
    Ok(sanitized_mesh)
}

/// Экспортёр SVG.
pub struct SvgExporter;

impl ExportPlugin for SvgExporter {
    fn name(&self) -> &str {
        "SVG Exporter"
    }
    
    fn supported_extensions(&self) -> &[&str] {
        &["svg"]
    }
    
    fn export(&self, unfolded: &crate::unfold::UnfoldedMesh, path: &Path) -> Result<(), PepakuraError> {
        let config = SvgExportConfig::default();
        export_svg_to_file(unfolded, &config, path.to_str().unwrap())
            .map_err(|e| PepakuraError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("SVG export failed: {}", e),
            )))
    }
}

/// Простой развёртыватель проекцией.
pub struct SimpleUnfolder;

impl UnfoldPlugin for SimpleUnfolder {
    fn name(&self) -> &str {
        "Simple Projection"
    }
    
    fn description(&self) -> &str {
        "Базовая развёртка через проекцию на плоскость"
    }
    
    fn unfold(&self, mesh: &Mesh, _config: &UnfoldConfig) -> Result<crate::unfold::UnfoldedMesh, PepakuraError> {
        crate::unfold::unfold_simple_projection(mesh)
            .map(|u| crate::unfold::UnfoldedMesh {
                vertices_2d: u.vertices_2d,
                uv_coords: u.uv_coords,
                faces: u.faces,
                source_mesh: u.source_mesh,
                metadata: Default::default(),
            })
            .map_err(|e| PepakuraError::UnfoldError(e))
    }
}

/// Создаёт реестр со встроенными плагинами.
pub fn create_builtin_registry() -> super::registry::PluginRegistry {
    use super::registry::PluginRegistry;
    
    let mut registry = PluginRegistry::new();
    
    registry.register_importer(Box::new(ObjImporter));
    registry.register_exporter(Box::new(SvgExporter));
    registry.register_unfolder(Box::new(SimpleUnfolder));
    
    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_obj_importer_extensions() {
        let importer = ObjImporter;
        assert!(importer.supports_extension("obj"));
        assert!(importer.supports_extension("OBJ"));
        assert!(!importer.supports_extension("stl"));
    }
    
    #[test]
    fn test_svg_exporter_extensions() {
        let exporter = SvgExporter;
        assert!(exporter.supports_extension("svg"));
        assert!(!exporter.supports_extension("png"));
    }
    
    #[test]
    fn test_parse_obj_content() {
        let obj_content = r#"# Test cube
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 1.0 1.0 0.0
v 0.0 1.0 0.0
f 1 2 3
f 1 3 4
"#;
        
        let mesh = parse_obj_content(obj_content, "Test").unwrap();
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.faces.len(), 2);
    }
    
    #[test]
    fn test_parse_obj_empty() {
        let result = parse_obj_content("", "Test");
        assert!(matches!(result, Err(PepakuraError::EmptyMesh)));
    }
    
    #[test]
    fn test_create_builtin_registry() {
        let registry = create_builtin_registry();
        
        assert!(registry.has_importer("obj"));
        assert!(registry.has_exporter("svg"));
        assert!(registry.has_unfolder("Simple Projection"));
    }
}
