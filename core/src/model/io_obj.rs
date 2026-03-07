//! Модуль для загрузки и сохранения OBJ файлов

use super::{Model, Vertex, Face};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Загрузка модели из OBJ файла
/// 
/// # Аргументы
/// * `path` - Путь к OBJ файлу
/// 
/// # Возвращает
/// Результат загрузки модели или ошибку
pub fn load_obj(path: &str) -> Result<Model, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
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

/// Сохранение модели в OBJ файл
/// 
/// # Аргументы
/// * `model` - Модель для сохранения
/// * `path` - Путь к OBJ файлу
/// 
/// # Возвращает
/// Результат сохранения или ошибку
pub fn save_obj(model: &Model, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    
    // Записываем вершины
    for vertex in &model.vertices {
        writeln!(file, "v {} {} {}", vertex.x, vertex.y, vertex.z)?;
    }
    
    // Записываем грани
    for face in &model.faces {
        // Индексы в OBJ начинаются с 1
        let v1_idx = face.vertices[0] + 1;
        let v2_idx = face.vertices[1] + 1;
        let v3_idx = face.vertices[2] + 1;
        
        writeln!(file, "f {} {} {}", v1_idx, v2_idx, v3_idx)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_simple_obj() {
        let obj_content = r#"v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.0 1.0 0.0
f 1 2 3"#;
        
        // Создаем временный файл
        let mut temp_file = std::env::temp_dir();
        temp_file.push("test.obj");
        std::fs::write(&temp_file, obj_content).unwrap();
        
        // Загружаем модель
        let model = load_obj(temp_file.to_str().unwrap()).unwrap();
        
        // Проверяем количество вершин и граней
        assert_eq!(model.vertices.len(), 3);
        assert_eq!(model.faces.len(), 1);
        
        // Удаляем временный файл
        std::fs::remove_file(temp_file).unwrap();
    }
}
