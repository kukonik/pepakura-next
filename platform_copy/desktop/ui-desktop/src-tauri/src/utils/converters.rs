//! Конвертеры между форматами 3D моделей

use gltf::Gltf;
use std::error::Error;
use std::fmt;

/// Ошибка конвертации GLB
#[derive(Debug)]
pub enum GlbConversionError {
    ParseError(String),
    NoGeometry,
    InvalidData,
}

impl fmt::Display for GlbConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlbConversionError::ParseError(msg) => write!(f, "Ошибка парсинга GLB: {}", msg),
            GlbConversionError::NoGeometry => write!(f, "GLB файл не содержит геометрии"),
            GlbConversionError::InvalidData => write!(f, "Невалидные данные в GLB файле"),
        }
    }
}

impl Error for GlbConversionError {}

/// Конвертирует байты GLB файла в строку формата OBJ
/// Игнорирует текстуры и нормали, берет только геометрию (позиции) и индексы
pub fn glb_to_obj_string(glb_bytes: &[u8]) -> Result<String, GlbConversionError> {
    let doc = Gltf::from_slice(glb_bytes)
        .map_err(|e| GlbConversionError::ParseError(e.to_string()))?;

    let mut obj_string = String::new();
    let mut vertex_offset = 1; // OBJ индексы с 1
    let mut total_vertices = 0;
    let mut total_faces = 0;

    // Добавляем заголовок
    obj_string.push_str("# Converted from GLB by Pepakura Next\n");
    obj_string.push_str("# https://github.com/pepakura-next/pepakura-next\n\n");

    // Проходим по всем сценам и нодам
    for scene in doc.scenes() {
        for node in scene.nodes() {
            traverse_node(&node, &mut obj_string, &mut vertex_offset, &mut total_vertices, &mut total_faces)?;
        }
    }

    if total_vertices == 0 {
        return Err(GlbConversionError::NoGeometry);
    }

    // Добавляем статистику
    obj_string.push_str(&format!("\n# Total vertices: {}\n", total_vertices));
    obj_string.push_str(&format!("# Total faces: {}\n", total_faces));

    Ok(obj_string)
}

fn traverse_node(
    node: &gltf::Node,
    obj: &mut String,
    vertex_offset: &mut usize,
    total_vertices: &mut usize,
    total_faces: &mut usize,
) -> Result<(), GlbConversionError> {
    if let Some(mesh) = node.mesh() {
        for (primitive_index, primitive) in mesh.primitives().enumerate() {
            // Получаем читателя для атрибутов
            let reader = primitive.reader(|_| None);
            
            // Читаем позиции вершин
            let positions = reader.read_positions()
                .ok_or(GlbConversionError::InvalidData)?;
            
            // Записываем вершины
            for pos in positions {
                obj.push_str(&format!("v {} {} {}\n", pos[0], pos[1], pos[2]));
                *total_vertices += 1;
            }
            
            // Читаем индексы
            let indices = reader.read_indices()
                .ok_or(GlbConversionError::InvalidData)?;
            
            // Формируем грани (треугольники)
            match primitive.mode() {
                gltf::mesh::Mode::Triangles => {
                    let indices_vec: Vec<u32> = indices.into_u32().collect();
                    for chunk in indices_vec.chunks(3) {
                        if chunk.len() == 3 {
                            // OBJ индексы начинаются с 1, и мы добавляем смещение
                            obj.push_str(&format!("f {} {} {}\n", 
                                chunk[0] as usize + *vertex_offset,
                                chunk[1] as usize + *vertex_offset,
                                chunk[2] as usize + *vertex_offset
                            ));
                            *total_faces += 1;
                        }
                    }
                }
                gltf::mesh::Mode::TriangleStrip => {
                    let indices_vec: Vec<u32> = indices.into_u32().collect();
                    for i in 0..indices_vec.len().saturating_sub(2) {
                        if i % 2 == 0 {
                            obj.push_str(&format!("f {} {} {}\n",
                                indices_vec[i] as usize + *vertex_offset,
                                indices_vec[i+1] as usize + *vertex_offset,
                                indices_vec[i+2] as usize + *vertex_offset
                            ));
                        } else {
                            // Для нечетных индексов порядок меняется
                            obj.push_str(&format!("f {} {} {}\n",
                                indices_vec[i] as usize + *vertex_offset,
                                indices_vec[i+2] as usize + *vertex_offset,
                                indices_vec[i+1] as usize + *vertex_offset
                            ));
                        }
                        *total_faces += 1;
                    }
                }
                gltf::mesh::Mode::TriangleFan => {
                    let indices_vec: Vec<u32> = indices.into_u32().collect();
                    if indices_vec.len() >= 3 {
                        let first = indices_vec[0];
                        for i in 1..indices_vec.len().saturating_sub(1) {
                            obj.push_str(&format!("f {} {} {}\n",
                                first as usize + *vertex_offset,
                                indices_vec[i] as usize + *vertex_offset,
                                indices_vec[i+1] as usize + *vertex_offset
                            ));
                            *total_faces += 1;
                        }
                    }
                }
                _ => {
                    // Пропускаем линии, точки и другие режимы
                    continue;
                }
            }
            
            // Обновляем смещение для следующего примитива
            *vertex_offset += positions.len();
        }
    }
    
    // Рекурсивно обрабатываем дочерние ноды
    for child in node.children() {
        traverse_node(&child, obj, vertex_offset, total_vertices, total_faces)?;
    }
    
    Ok(())
}

/// Упрощенная конвертация GLB в OBJ (для MVP)
/// Возвращает строку OBJ с базовой геометрией
pub fn glb_to_obj_simple(glb_bytes: &[u8]) -> Result<String, GlbConversionError> {
    let doc = Gltf::from_slice(glb_bytes)
        .map_err(|e| GlbConversionError::ParseError(e.to_string()))?;

    let mut obj = String::new();
    obj.push_str("# Simple GLB to OBJ conversion\n");
    
    let mut vertex_count = 0;
    let mut face_count = 0;
    
    for scene in doc.scenes() {
        for node in scene.nodes() {
            if let Some(mesh) = node.mesh() {
                for primitive in mesh.primitives() {
                    if let Some(reader) = primitive.reader(|_| None) {
                        // Вершины
                        if let Some(positions) = reader.read_positions() {
                            for pos in positions {
                                obj.push_str(&format!("v {} {} {}\n", pos[0], pos[1], pos[2]));
                                vertex_count += 1;
                            }
                        }
                        
                        // Грани (только треугольники)
                        if primitive.mode() == gltf::mesh::Mode::Triangles {
                            if let Some(indices) = reader.read_indices() {
                                let indices_vec: Vec<u32> = indices.into_u32().collect();
                                for chunk in indices_vec.chunks(3) {
                                    if chunk.len() == 3 {
                                        // Индексы в OBJ начинаются с 1
                                        obj.push_str(&format!("f {} {} {}\n", 
                                            chunk[0] + 1, 
                                            chunk[1] + 1, 
                                            chunk[2] + 1
                                        ));
                                        face_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    if vertex_count == 0 {
        return Err(GlbConversionError::NoGeometry);
    }
    
    obj.push_str(&format!("\n# Vertices: {}, Faces: {}\n", vertex_count, face_count));
    Ok(obj)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glb_to_obj_simple_with_mock_data() {
        // Минимальный GLB заголовок (невалидный, но для теста)
        let empty_glb = vec![0x67, 0x6C, 0x54, 0x46]; // "glTF" в ASCII
        let result = glb_to_obj_simple(&empty_glb);
        assert!(result.is_err());
        
        // Проверяем, что ошибка - ParseError
        match result {
            Err(GlbConversionError::ParseError(_)) => (),
            _ => panic!("Expected ParseError"),
        }
    }
}