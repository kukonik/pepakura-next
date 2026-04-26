//! Модуль для загрузки и сохранения OBJ файлов

use super::{Model, Vertex, Face};
use std::io::{BufRead, Write};

/// Загрузка модели из OBJ потока
/// 
/// # Аргументы
/// * `reader` - Буферизированный читатель (например, файл, память)
/// 
/// # Возвращает
/// Результат загрузки модели или ошибку
pub fn load_obj<R: BufRead>(reader: R) -> Result<Model, Box<dyn std::error::Error>> {
    let mut model = Model::new();
    let mut vertices: Vec<Vertex> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "v" => {
                // Вершина
                if parts.len() >= 4 {
                    let x = parts[1].parse::<f64>()?;
                    let y = parts[2].parse::<f64>()?;
                    let z = parts[3].parse::<f64>()?;
                    vertices.push(Vertex { x, y, z });
                }
            }
            "f" => {
                // Грань
                if parts.len() >= 4 {
                    // Поддерживаем только треугольники
                    if parts.len() == 4 {
                        let v1_idx = parts[1].parse::<usize>()? - 1;
                        let v2_idx = parts[2].parse::<usize>()? - 1;
                        let v3_idx = parts[3].parse::<usize>()? - 1;
                        
                        if v1_idx < vertices.len() && v2_idx < vertices.len() && v3_idx < vertices.len() {
                            let face = Face {
                                vertices: vec![v1_idx, v2_idx, v3_idx]
                            };
                            model.faces.push(face);
                        }
                    }
                }
            }
            _ => {
                // Игнорируем другие типы строк
            }
        }
    }
    
    // Копируем вершины в модель
    model.vertices = vertices;
    
    Ok(model)
}

/// Сохранение модели в OBJ поток
/// 
/// # Аргументы
/// * `model` - Модель для сохранения
/// * `writer` - Писатель (например, файл, память)
/// 
/// # Возвращает
/// Результат сохранения или ошибку
pub fn save_obj<W: Write>(model: &Model, writer: &mut W) -> Result<(), Box<dyn std::error::Error>> {
    // Записываем вершины
    for vertex in &model.vertices {
        writeln!(writer, "v {} {} {}", vertex.x, vertex.y, vertex.z)?;
    }
    
    // Записываем грани
    for face in &model.faces {
        // Индексы в OBJ начинаются с 1
        let v1_idx = face.vertices[0] + 1;
        let v2_idx = face.vertices[1] + 1;
        let v3_idx = face.vertices[2] + 1;
        
        writeln!(writer, "f {} {} {}", v1_idx, v2_idx, v3_idx)?;
    }
    
    Ok(())
}

/// Загрузка модели из файла (только для нативных платформ)
#[cfg(not(target_family = "wasm"))]
pub fn load_obj_file(path: &str) -> Result<Model, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    load_obj(reader)
}

/// Сохранение модели в файл (только для нативных платформ)
#[cfg(not(target_family = "wasm"))]
pub fn save_obj_file(model: &Model, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufWriter;
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    save_obj(model, &mut writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_load_simple_obj() {
        let obj_content = r#"v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.0 1.0 0.0
f 1 2 3"#;
        
        let cursor = Cursor::new(obj_content);
        let model = load_obj(cursor).unwrap();
        
        // Проверяем количество вершин и граней
        assert_eq!(model.vertices.len(), 3);
        assert_eq!(model.faces.len(), 1);
    }
    
    #[test]
    fn test_save_obj() {
        let mut model = Model::new();
        model.vertices = vec![
            Vertex { x: 0.0, y: 0.0, z: 0.0 },
            Vertex { x: 1.0, y: 0.0, z: 0.0 },
            Vertex { x: 0.0, y: 1.0, z: 0.0 },
        ];
        model.faces = vec![
            Face { vertices: vec![0, 1, 2] },
        ];
        
        let mut buffer = Vec::new();
        save_obj(&model, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        
        assert!(output.contains("v 0 0 0"));
        assert!(output.contains("v 1 0 0"));
        assert!(output.contains("v 0 1 0"));
        assert!(output.contains("f 1 2 3"));
    }
    
    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_load_save_file() {
        use tempfile::NamedTempFile;
        
        let mut model = Model::new();
        model.vertices = vec![
            Vertex { x: 0.0, y: 0.0, z: 0.0 },
            Vertex { x: 1.0, y: 0.0, z: 0.0 },
            Vertex { x: 0.0, y: 1.0, z: 0.0 },
        ];
        model.faces = vec![
            Face { vertices: vec![0, 1, 2] },
        ];
        
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        save_obj_file(&model, path).unwrap();
        let loaded = load_obj_file(path).unwrap();
        
        assert_eq!(loaded.vertices.len(), 3);
        assert_eq!(loaded.faces.len(), 1);
    }
}