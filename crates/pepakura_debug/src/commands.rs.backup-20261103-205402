//! # CLI Commands
//!
//! Реализация команд CLI для pepakura-debug.

use pepakura_core::analysis::mesh_analyzer::MeshAnalyzer;
use pepakura_core::geometry::Mesh;
use std::path::Path;

/// Проверка статуса LLM
pub fn cmd_llm_status() -> Result<(), String> {
    println!("╔════════════════════════════════════════╗");
    println!("║       LLM Backend Status Check        ║");
    println!("╚════════════════════════════════════════╝\n");

    let analyzer = MeshAnalyzer::new();

    println!("Бэкенд: {}", analyzer.backend_name());
    println!("Модель: qwen2.5:7b");
    println!();

    let status = analyzer.llm_status();

    if status.available {
        println!("✓ LLM доступен");

        if let Some(name) = &status.backend_name {
            println!("  Бэкенд: {}", name);
        }

        if let Some(version) = &status.version {
            println!("  Версия: {}", version);
        }

        if !status.models.is_empty() {
            println!("  Модели:");
            for model in &status.models {
                println!("    - {}", model);
            }
        }
    } else {
        println!("✗ LLM недоступен");

        if let Some(error) = &status.error {
            println!("  Ошибка: {}", error);
        }

        println!();
        println!("Убедитесь, что Ollama запущен:");
        println!("  ollama serve");
        println!();
        println!("Или установите модель:");
        println!("  ollama pull qwen2.5:7b");
    }

    Ok(())
}

/// Анализ модели из файла
pub fn cmd_analyze(path: &str, explain: bool) -> Result<(), String> {
    println!("╔════════════════════════════════════════╗");
    println!("║        Mesh Analysis Report          ║");
    println!("╚════════════════════════════════════════╝\n");

    // Проверяем существование файла
    if !Path::new(path).exists() {
        return Err(format!("Файл не найден: {}", path));
    }

    // Загружаем меш
    println!("Загрузка модели: {}", path);
    let mesh = load_mesh(path)?;
    println!("✓ Загружено: {} вершин, {} граней\n", mesh.vertices.len(), mesh.faces.len());

    // Создаём анализатор
    let analyzer = MeshAnalyzer::new();

    // Проверяем LLM
    let llm_available = analyzer.is_llm_available();
    if llm_available {
        println!("LLM бэкенд: {}", analyzer.backend_name());
    } else {
        println!("LLM недоступен — будет выполнен только числовой анализ");
    }
    println!();

    // Анализируем
    println!("Анализ...");
    let result = analyzer.analyze(&mesh);

    // Выводим статистику
    println!("\n{}", "═".repeat(40));
    println!("СТАТИСТИКА МОДЕЛИ");
    println!("{}", "═".repeat(40));
    println!("{}", result.stats.detailed());

    // Выводим LLM-анализ
    if let Some(ai) = &result.ai_analysis {
        println!("\n{}", "═".repeat(40));
        println!("LLM-АНАЛИЗ");
        println!("{}", "═".repeat(40));

        println!(
            "\nСложность: {} (оценка: {:.0}%)",
            ai.difficulty.to_uppercase(),
            ai.overall_score * 100.0
        );

        println!(
            "\nОценки:
  Развёртка:   {:.0}%
  Детализация: {:.0}%
  Печать:      {:.0}%",
            ai.unfoldability_score * 100.0,
            ai.detail_score * 100.0,
            ai.printability_score * 100.0
        );

        println!("\nОбоснование:");
        println!("  {}", ai.reasoning);

        if !ai.issues.is_empty() {
            println!("\nПроблемы:");
            for issue in &ai.issues {
                let icon = match issue.severity.as_str() {
                    "critical" => "🔴",
                    "warning" => "🟡",
                    "info" => "ℹ️",
                    _ => "•",
                };
                println!("  {} [{}] {}: {} ({} шт.)",
                    icon, issue.severity, issue.code, issue.message, issue.count);
            }
        }

        if !ai.recommendations.is_empty() {
            println!("\nРекомендации:");
            for rec in &ai.recommendations {
                let icon = match rec.priority.as_str() {
                    "high" => "🔴",
                    "medium" => "🟡",
                    "low" => "🟢",
                    _ => "•",
                };
                println!("  {} {}", icon, rec.text);
            }
        }
    }

    // Объяснение проблем
    if explain {
        println!("\n{}", "═".repeat(40));
        println!("ОБЪЯСНЕНИЕ ПРОБЛЕМ");
        println!("{}", "═".repeat(40));

        if let Some(ai) = &result.ai_analysis {
            if ai.issues.is_empty() {
                println!("\nПроблем не обнаружено ✓");
            } else {
                println!("\nДля каждой проблемы:");
                println!("  1. Найдите проблемную зону в 3D редакторе");
                println!("  2. Используйте инструменты ретопологии");
                println!("  3. Упростите геометрию в проблемных местах");
                println!();
                println!("Полезные команды:");
                println!("  - Blender: Decimate modifier");
                println!("  - MeshLab: Filters → Remeshing");
                println!("  - Pepakura: Tools → Divide Edge");
            }
        } else {
            println!("\nLLM недоступен — объяснения невозможны");
        }
    }

    println!("\n{}", "═".repeat(40));
    println!("Время анализа: {} мс", result.analysis_time_ms);
    println!("{}", "═".repeat(40));

    Ok(())
}

/// Развёртка модели с LLM-объяснением
pub fn cmd_unfold(path: &str, explain: bool) -> Result<(), String> {
    println!("╔════════════════════════════════════════╗");
    println!("║        Unfold with LLM Help          ║");
    println!("╚════════════════════════════════════════╝\n");

    // Проверяем существование файла
    if !Path::new(path).exists() {
        return Err(format!("Файл не найден: {}", path));
    }

    // Загружаем меш
    println!("Загрузка модели: {}", path);
    let mesh = load_mesh(path)?;
    println!("✓ Загружено: {} вершин, {} граней\n", mesh.vertices.len(), mesh.faces.len());

    // Анализируем перед развёрткой
    let analyzer = MeshAnalyzer::new();
    let result = analyzer.analyze(&mesh);

    println!("Статистика: {}", result.stats.summary());

    if let Some(ai) = &result.ai_analysis {
        println!("\nСложность развёртки: {}", ai.difficulty.to_uppercase());

        if !ai.issues.is_empty() {
            println!("\nНайдены проблемы, которые могут осложнить развёртку:");
            for issue in &ai.issues {
                println!("  - [{}] {}", issue.severity, issue.message);
            }

            if explain {
                println!("\nРекомендации перед развёрткой:");
                for rec in &ai.recommendations {
                    println!("  → {}", rec.text);
                }
            }
        } else {
            println!("\n✓ Проблем не обнаружено, модель готова к развёртке");
        }
    } else {
        println!("\nLLM недоступен — анализ проблем невозможен");
    }

    println!("\nПримечание: Для фактической развёртки используйте:");
    println!("  pepakura-next unfold {}", path);

    Ok(())
}

/// Загрузить меш из файла
fn load_mesh(path: &str) -> Result<Mesh, String> {
    let path_lower = path.to_lowercase();

    if path_lower.ends_with(".obj") {
        load_obj(path)
    } else {
        Err(format!("Неподдерживаемый формат. Используйте .obj"))
    }
}

/// Загрузить OBJ файл
fn load_obj(path: &str) -> Result<Mesh, String> {
    // Используем obj crate для парсинга
    let obj_model = obj::Obj::load(path)
        .map_err(|e| format!("Ошибка парсинга OBJ: {}", e))?;

    // Создаём меш из первой модели
    if obj_model.data.objects.is_empty() {
        return Err("OBJ файл не содержит моделей".to_string());
    }

    let obj_data = &obj_model.data;
    let mut mesh = Mesh::new(Path::new(path).file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("model"));

    // Добавляем вершины
    for (i, position) in obj_data.position.iter().enumerate() {
        mesh.add_vertex(pepakura_core::geometry::Vertex::new(
            i,
            [position[0] as f64, position[1] as f64, position[2] as f64],
        ));
    }

    // Добавляем грани (треугольные) из объектов
    for object in &obj_data.objects {
        for group in &object.groups {
            for poly in &group.polys {
                // Полигон может иметь более 3 вершин, берём первые три для треугольника
                // poly является SimplePolygon (кортежная структура с одним полем Vec<IndexTuple>)
                let vertices = &poly.0;
                if vertices.len() >= 3 {
                    let indices: Vec<usize> = vertices.iter()
                        .take(3)
                        .map(|v| (v.0 - 1) as usize) // OBJ использует 1-based indexing, v.0 - индекс позиции
                        .collect();
                    if indices.len() == 3 {
                        mesh.add_face(pepakura_core::geometry::Face::new(
                            indices[0],
                            indices[1],
                            indices[2],
                        ));
                    }
                }
            }
        }
    }

    Ok(mesh)
}
