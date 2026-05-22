//! Экспорт текстур для развёртки.
//!
//! Сохраняет UV-развёртку и текстуры отдельными файлами.
//!
//! ## Форматы
//!
//! - PNG — растровые текстуры
//! - JSON — UV-координаты
//! - SVG — векторная развёртка с текстурами

use crate::geometry::Mesh;
use crate::unfold::UnfoldedMesh;
use image::{ImageBuffer, Rgba, codecs::jpeg::JpegEncoder, ImageEncoder};
use image::EncodableLayout;
use pepakura_platform::fs::{FileSystem, FileError};
use serde::{Deserialize, Serialize};

/// Результат экспорта текстур.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureExportResult {
    /// Путь к файлу текстуры
    pub texture_path: String,
    /// Путь к файлу UV-координат
    pub uv_path: String,
    /// Путь к SVG файлу развёртки
    pub svg_path: String,
    /// Ширина текстуры
    pub texture_width: u32,
    /// Высота текстуры
    pub texture_height: u32,
}

/// Конфигурация экспорта текстур.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureExportConfig {
    /// Ширина текстуры
    pub texture_width: u32,
    /// Высота текстуры
    pub texture_height: u32,
    /// Формат текстуры (png, jpg)
    pub format: String,
    /// Качество (для JPG, 0-100)
    pub quality: u8,
}

impl Default for TextureExportConfig {
    fn default() -> Self {
        Self {
            texture_width: 1024,
            texture_height: 1024,
            format: "png".to_string(),
            quality: 90,
        }
    }
}

/// UV-координаты для экспорта.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UvData {
    /// Вершины с UV-координатами
    pub vertices: Vec<UvVertex>,
    /// Грани
    pub faces: Vec<UvFace>,
}

/// Вершина с UV-координатами.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UvVertex {
    /// 3D позиция
    pub position: [f64; 3],
    /// 2D UV-координаты
    pub uv: [f64; 2],
    /// 2D позиция на развёртке
    pub position_2d: [f64; 2],
}

/// Грань с UV.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UvFace {
    /// Индексы вершин
    pub vertices: [usize; 3],
    /// ID текстуры
    pub texture_id: Option<u32>,
}

/// Ошибки экспорта текстур.
#[derive(Debug, thiserror::Error)]
pub enum TextureExportError {
    #[error("Ошибка файловой системы: {0}")]
    FileSystemError(#[from] FileError),

    #[error("Ошибка изображения: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Ошибка JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Нет UV-координат")]
    NoUvCoords,

    #[error("Пустой меш")]
    EmptyMesh,
}

/// Экспортирует текстуру и UV-координаты.
///
/// # Аргументы
/// * `unfolded` - развёрнутый меш с UV
/// * `config` - конфигурация экспорта
/// * `fs` - реализация FileSystem для записи файлов
/// * `output_dir` - директория для экспорта
///
/// # Возвращает
/// * `Ok(TextureExportResult)` - результат экспорта
/// * `Err(TextureExportError)` - ошибка
pub fn export_textures<F: FileSystem>(
    unfolded: &UnfoldedMesh,
    config: &TextureExportConfig,
    fs: &F,
    output_dir: &str,
) -> Result<TextureExportResult, TextureExportError> {
    if unfolded.vertices_2d.is_empty() {
        return Err(TextureExportError::EmptyMesh);
    }

    let uv_coords = unfolded.uv_coords
        .as_ref()
        .ok_or(TextureExportError::NoUvCoords)?;

    // Создаём директорию
    fs.create_dir_all(output_dir)?;

    // Экспортируем UV-координаты
    let uv_path = format!("{}/uv_coords.json", output_dir);
    let uv_data = create_uv_data(unfolded, uv_coords);
    let uv_json = serde_json::to_string_pretty(&uv_data)?;
    fs.write_text(&uv_path, &uv_json)?;

    // Экспортируем текстуру (заглушка - белая текстура)
    let texture_path = format!("{}/texture.{}", output_dir, config.format);
    let texture = create_placeholder_texture(config.texture_width, config.texture_height);

    match config.format.as_str() {
        "jpg" | "jpeg" => {
            // Для JPEG нужно использовать временный буфер
            let mut jpeg_data = Vec::new();
            let cursor = std::io::Cursor::new(&mut jpeg_data);
            let encoder = JpegEncoder::new_with_quality(cursor, config.quality);
            encoder.write_image(
                &texture,
                texture.width(),
                texture.height(),
                image::ColorType::Rgba8,
            )?;
            fs.write_file(&texture_path, &jpeg_data)?;
        }
        _ => {
            let png_data = texture.as_bytes();
            fs.write_file(&texture_path, png_data)?;
        }
    }

    // Экспортируем SVG с текстурами
    let svg_path = format!("{}/unfolded.svg", output_dir);
    let svg_content = export_svg_with_textures(unfolded, uv_coords, &texture_path);
    fs.write_text(&svg_path, &svg_content)?;

    Ok(TextureExportResult {
        texture_path,
        uv_path,
        svg_path,
        texture_width: config.texture_width,
        texture_height: config.texture_height,
    })
}

/// Создаёт UV-данные из развёртки.
fn create_uv_data(unfolded: &UnfoldedMesh, uv_coords: &[[f64; 2]]) -> UvData {
    let mut vertices = Vec::new();

    for (i, vertex) in unfolded.source_mesh.vertices.iter().enumerate() {
        let uv = uv_coords.get(i).copied().unwrap_or([0.0, 0.0]);
        let pos_2d = unfolded.vertices_2d.get(i).copied().unwrap_or([0.0, 0.0]);

        vertices.push(UvVertex {
            position: vertex.position,
            uv,
            position_2d: pos_2d,
        });
    }

    let faces = unfolded.faces
        .iter()
        .map(|face| UvFace {
            vertices: [face.vertices[0], face.vertices[1], face.vertices[2]],
            texture_id: None,
        })
        .collect();

    UvData { vertices, faces }
}

/// Создаёт тестовую текстуру (шахматная доска).
fn create_placeholder_texture(width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);

    let checker_size = (width / 16).max(8);

    for x in 0..width {
        for y in 0..height {
            let cx = x / checker_size;
            let cy = y / checker_size;
            
            let is_white = (cx + cy) % 2 == 0;
            
            let color = if is_white {
                Rgba([255, 255, 255, 255])
            } else {
                Rgba([200, 200, 200, 255])
            };

            img.put_pixel(x, y, color);
        }
    }

    img
}

/// Экспортирует SVG с текстурами.
fn export_svg_with_textures(
    unfolded: &UnfoldedMesh,
    _uv_coords: &[[f64; 2]],
    texture_path: &str,
) -> String {
    let mut svg = String::new();

    // Вычисляем bounding box
    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(unfolded);
    let width = max_x - min_x;
    let height = max_y - min_y;

    // SVG заголовок
    svg.push_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:xlink="http://www.w3.org/1999/xlink"
     width="{:.2}" height="{:.2}"
     viewBox="{:.2} {:.2} {:.2} {:.2}">
<!-- Pepakura Next SVG Export with Textures -->
"#,
        width, height, min_x, min_y, width, height
    ));

    // Определяем текстуру
    svg.push_str(&format!(
        r#"
<defs>
    <pattern id="texture" patternUnits="userSpaceOnUse" width="{:.2}" height="{:.2}">
        <image width="{:.2}" height="{:.2}" xlink:href="{}"/>
    </pattern>
</defs>
"#,
        width, height, width, height, texture_path
    ));

    // Группы для граней с текстурой
    svg.push_str("<g id=\"textured-faces\">\n");

    for face in &unfolded.faces {
        let v0 = &unfolded.vertices_2d[face.vertices[0]];
        let v1 = &unfolded.vertices_2d[face.vertices[1]];
        let v2 = &unfolded.vertices_2d[face.vertices[2]];

        // Создаём полигон с текстурой
        svg.push_str(&format!(
            r##"  <polygon points="{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}"
           fill="url(#texture)"
           stroke="#ff0000"
           stroke-width="0.5"
           opacity="0.8"/>
"##,
            v0[0], v0[1], v1[0], v1[1], v2[0], v2[1]
        ));
    }

    svg.push_str("</g>\n");
    svg.push_str("</svg>");

    svg
}

/// Вычисляет bounding box.
fn calculate_bounding_box(unfolded: &UnfoldedMesh) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for &[x, y] in &unfolded.vertices_2d {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    (min_x, min_y, max_x, max_y)
}

/// Извлекает UV-координаты из меша.
pub fn extract_uv_from_mesh(_mesh: &Mesh) -> Option<Vec<[f64; 2]>> {
    // Проверяем, есть ли UV-координаты в меше
    // Это зависит от формата файла (OBJ с MTL, FBX, и т.д.)

    // Заглушка: возвращаем None, если нет UV
    // В реальной реализации нужно парсить UV из файла
    None
}

/// Генерирует UV-координаты из 3D позиций.
pub fn generate_uv_from_position(mesh: &Mesh) -> Vec<[f64; 2]> {
    let mut uv_coords = Vec::with_capacity(mesh.vertices.len());

    // Простая проекция: используем X и Y координаты как UV
    let (mut min_x, mut min_y) = (f64::INFINITY, f64::INFINITY);
    let (mut max_x, mut max_y) = (f64::NEG_INFINITY, f64::NEG_INFINITY);

    // Находим bounding box
    for vertex in &mesh.vertices {
        min_x = min_x.min(vertex.position[0]);
        min_y = min_y.min(vertex.position[1]);
        max_x = max_x.max(vertex.position[0]);
        max_y = max_y.max(vertex.position[1]);
    }

    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);

    // Нормализуем к [0, 1]
    for vertex in &mesh.vertices {
        let u = (vertex.position[0] - min_x) / range_x;
        let v = (vertex.position[1] - min_y) / range_y;
        uv_coords.push([u, v]);
    }

    uv_coords
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Mesh, Vertex, Face};
    #[cfg(not(target_arch = "wasm32"))]
    use pepakura_platform::fs::native::NativeFileSystem;

    fn create_test_mesh() -> UnfoldedMesh {
        let mut mesh = Mesh::new("Test");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.5, 1.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));

        UnfoldedMesh {
            vertices_2d: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]],
            uv_coords: Some(vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]]),
            faces: mesh.faces.clone(),
            source_mesh: mesh,
            metadata: Default::default(),
        }
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_export_textures() {
        use pepakura_platform::fs::FileSystem;
        
        let unfolded = create_test_mesh();
        let config = TextureExportConfig::default();
        let fs = NativeFileSystem::new();

        let temp_dir = std::env::temp_dir().join("pepakura_test");
        let result = export_textures(&unfolded, &config, &fs, temp_dir.to_str().unwrap());

        assert!(result.is_ok());
        let export_result = result.unwrap();

        assert!(!export_result.texture_path.is_empty());
        assert!(!export_result.uv_path.is_empty());
        assert!(!export_result.svg_path.is_empty());
    }

    #[test]
    fn test_generate_uv_from_position() {
        let mut mesh = Mesh::new("Test");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.5, 1.0, 0.0]));

        let uv = generate_uv_from_position(&mesh);

        assert_eq!(uv.len(), 3);
        assert!(uv[0][0] >= 0.0 && uv[0][0] <= 1.0);
        assert!(uv[0][1] >= 0.0 && uv[0][1] <= 1.0);
    }

    #[test]
    fn test_create_placeholder_texture() {
        let texture = create_placeholder_texture(64, 64);

        assert_eq!(texture.width(), 64);
        assert_eq!(texture.height(), 64);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_export_no_uv_coords() {
        use pepakura_platform::fs::FileSystem;
        
        let mut unfolded = create_test_mesh();
        unfolded.uv_coords = None;

        let config = TextureExportConfig::default();
        let fs = NativeFileSystem::new();
        let temp_dir = std::env::temp_dir().join("pepakura_test_no_uv");

        let result = export_textures(&unfolded, &config, &fs, temp_dir.to_str().unwrap());

        assert!(matches!(result, Err(TextureExportError::NoUvCoords)));
    }
}
