//! Генетический алгоритм для оптимизации раскладки (nesting).
//!
//! Генетический алгоритм минимизирует отходы бумаги за счёт:
//! - Оптимального размещения деталей
//! - Поворота деталей на лучшие углы
//! - Минимизации количества листов
//!
//! ## Алгоритм
//!
//! 1. Инициализация популяции случайными раскладками
//! 2. Оценка пригодности (fitness) каждой раскладки
//! 3. Селекция (отбор лучших)
//! 4. Кроссовер (скрещивание)
//! 5. Мутация
//! 6. Повторение шагов 2-5 N поколений

use crate::nesting::{NestPart, NestSheet, NestResult, NestMetrics, PaperSettings};
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// Конфигурация генетического алгоритма.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticConfig {
    /// Размер популяции
    pub population_size: usize,
    /// Количество поколений
    pub generations: usize,
    /// Вероятность мутации (0.0 - 1.0)
    pub mutation_rate: f64,
    /// Вероятность кроссовера (0.0 - 1.0)
    pub crossover_rate: f64,
    /// Количество элит (лучших особей)
    pub elite_count: usize,
    /// Шаги поворота в градусах
    pub rotation_steps: Vec<f32>,
    /// Настройки бумаги
    pub paper: PaperSettings,
}

impl Default for GeneticConfig {
    fn default() -> Self {
        Self {
            population_size: 50,
            generations: 100,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            elite_count: 5,
            rotation_steps: vec![0.0, 90.0, 180.0, 270.0],
            paper: PaperSettings::from_format("A4"),
        }
    }
}

/// Особь (раскладка) в популяции.
#[derive(Debug, Clone)]
pub struct Individual {
    /// Раскладка деталей
    pub parts: Vec<NestPart>,
    /// Пригодность (fitness)
    pub fitness: f64,
    /// Количество использованных листов
    pub sheet_count: usize,
}

/// Генетический оптимизатор раскладки.
pub struct GeneticNesting {
    config: GeneticConfig,
    rng: StdRng,
}

impl GeneticNesting {
    /// Создаёт новый оптимизатор.
    pub fn new(config: GeneticConfig) -> Self {
        Self {
            config,
            rng: StdRng::seed_from_u64(42), // Детерминированность для тестов
        }
    }

    /// Оптимизирует раскладку деталей.
    ///
    /// # Аргументы
    /// * `parts` - детали для размещения
    ///
    /// # Возвращает
    /// * `NestResult` - оптимальная раскладка
    pub fn optimize(&mut self, parts: &[NestPart]) -> NestResult {
        // Инициализация популяции
        let mut population = self.initialize_population(parts);

        // Эволюция
        for generation in 0..self.config.generations {
            // Оценка пригодности
            self.evaluate_fitness(&mut population);

            // Сортировка по fitness (лучшие первые)
            population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

            // Логирование прогресса
            if generation % 10 == 0 {
                log::debug!(
                    "Generation {}: best fitness = {:.4}, sheets = {}",
                    generation,
                    population[0].fitness,
                    population[0].sheet_count
                );
            }

            // Создание нового поколения
            population = self.create_next_generation(&population);
        }

        // Финальная оценка
        self.evaluate_fitness(&mut population);
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        // Возврат лучшей особи
        let best = &population[0];
        self.create_nest_result(best)
    }

    /// Инициализирует популяцию случайными раскладками.
    fn initialize_population(&mut self, parts: &[NestPart]) -> Vec<Individual> {
        let mut population = Vec::with_capacity(self.config.population_size);

        for _ in 0..self.config.population_size {
            let individual = self.create_random_individual(parts);
            population.push(individual);
        }

        population
    }

    /// Создаёт случайную особь.
    fn create_random_individual(&mut self, parts: &[NestPart]) -> Individual {
        let mut placed_parts = Vec::new();

        for part in parts {
            let mut new_part = part.clone();

            // Случайная позиция
            let max_x = self.config.paper.width_mm - self.config.paper.margin_mm * 2.0;
            let max_y = self.config.paper.height_mm - self.config.paper.margin_mm * 2.0;

            new_part.x_mm = self.rng.gen_range(
                self.config.paper.margin_mm..max_x - part.width_mm
            );
            new_part.y_mm = self.rng.gen_range(
                self.config.paper.margin_mm..max_y - part.height_mm
            );

            // Случайный поворот
            if let Some(&rotation) = self.config.rotation_steps.choose(&mut self.rng) {
                new_part.rotation = rotation;
            }

            placed_parts.push(new_part);
        }

        Individual {
            parts: placed_parts,
            fitness: 0.0,
            sheet_count: 0,
        }
    }

    /// Оценивает пригодность особей.
    fn evaluate_fitness(&self, population: &mut Vec<Individual>) {
        for individual in population {
            let (fitness, sheet_count) = self.calculate_fitness(&individual.parts);
            individual.fitness = fitness;
            individual.sheet_count = sheet_count;
        }
    }

    /// Вычисляет пригодность особи.
    ///
    /// Fitness функция учитывает:
    /// - Количество листов (меньше = лучше)
    /// - Процент заполнения листов (больше = лучше)
    /// - Перекрытия (штраф)
    fn calculate_fitness(&self, parts: &[NestPart]) -> (f64, usize) {
        let sheet_area = (self.config.paper.width_mm - 2.0 * self.config.paper.margin_mm)
            * (self.config.paper.height_mm - 2.0 * self.config.paper.margin_mm);

        // Размещение частей по листам
        let mut sheets: Vec<Vec<&NestPart>> = Vec::new();
        let mut current_sheet: Vec<&NestPart> = Vec::new();

        for part in parts {
            // Проверяем, помещается ли часть на текущий лист
            if self.can_place_on_sheet(&current_sheet, part) {
                current_sheet.push(part);
            } else {
                // Новый лист
                if !current_sheet.is_empty() {
                    sheets.push(current_sheet);
                }
                current_sheet = vec![part];
            }
        }

        if !current_sheet.is_empty() {
            sheets.push(current_sheet);
        }

        let sheet_count = sheets.len();

        // Вычисляем общую площадь частей
        let total_part_area: f64 = parts.iter()
            .map(|p| p.width_mm as f64 * p.height_mm as f64)
            .sum();

        // Вычисляем процент заполнения
        let total_sheet_area = sheet_count as f64 * sheet_area as f64;
        let fill_rate = if total_sheet_area > 0.0 {
            total_part_area / total_sheet_area
        } else {
            0.0
        };

        // Проверяем перекрытия
        let overlap_penalty = self.calculate_overlap_penalty(parts);

        // Fitness: больше = лучше
        // Минимизируем листы, максимизируем заполнение
        let fitness = (fill_rate * 100.0) / (sheet_count as f64 + 1.0) - overlap_penalty;

        (fitness, sheet_count)
    }

    /// Проверяет, можно ли разместить часть на листе.
    fn can_place_on_sheet(&self, placed: &[&NestPart], new_part: &NestPart) -> bool {
        let margin = self.config.paper.margin_mm;
        let max_x = self.config.paper.width_mm - margin * 2.0;
        let max_y = self.config.paper.height_mm - margin * 2.0;

        // Проверяем границы листа
        if new_part.x_mm < margin || new_part.y_mm < margin {
            return false;
        }
        if new_part.x_mm + new_part.width_mm > margin + max_x {
            return false;
        }
        if new_part.y_mm + new_part.height_mm > margin + max_y {
            return false;
        }

        // Проверяем перекрытия с другими частями
        for placed_part in placed {
            if self.parts_overlap(placed_part, new_part) {
                return false;
            }
        }

        true
    }

    /// Проверяет перекрытие двух частей.
    fn parts_overlap(&self, a: &NestPart, b: &NestPart) -> bool {
        // Простая проверка AABB (axis-aligned bounding box)
        let margin = 1.0; // Небольшой зазор

        a.x_mm < b.x_mm + b.width_mm + margin
            && a.x_mm + a.width_mm + margin > b.x_mm
            && a.y_mm < b.y_mm + b.height_mm + margin
            && a.y_mm + a.height_mm + margin > b.y_mm
    }

    /// Вычисляет штраф за перекрытия.
    fn calculate_overlap_penalty(&self, parts: &[NestPart]) -> f64 {
        let mut penalty = 0.0;

        for i in 0..parts.len() {
            for j in (i + 1)..parts.len() {
                if self.parts_overlap(&parts[i], &parts[j]) {
                    penalty += 10.0; // Большой штраф за каждое перекрытие
                }
            }
        }

        penalty
    }

    /// Создаёт новое поколение.
    fn create_next_generation(&mut self, population: &[Individual]) -> Vec<Individual> {
        let mut new_population = Vec::with_capacity(self.config.population_size);

        // Элиты (лучшие особи переходят без изменений)
        for i in 0..self.config.elite_count {
            if i < population.len() {
                new_population.push(population[i].clone());
            }
        }

        // Остальные особи через кроссовер и мутацию
        while new_population.len() < self.config.population_size {
            // Селекция (турнирная)
            let parent1 = self.tournament_selection(population);
            let parent2 = self.tournament_selection(population);

            // Кроссовер
            let mut child = if self.rng.gen_bool(self.config.crossover_rate) {
                self.crossover(parent1, parent2)
            } else {
                parent1.clone()
            };

            // Мутация
            if self.rng.gen_bool(self.config.mutation_rate) {
                self.mutate(&mut child);
            }

            new_population.push(child);
        }

        new_population
    }

    /// Турнирная селекция.
    fn tournament_selection<'a>(&mut self, population: &'a [Individual]) -> &'a Individual {
        let tournament_size = 5;
        let mut best = &population[0];

        for _ in 1..tournament_size {
            let idx = self.rng.gen_range(0..population.len());
            if population[idx].fitness > best.fitness {
                best = &population[idx];
            }
        }

        best
    }

    /// Кроссовер (скрещивание).
    fn crossover(&mut self, parent1: &Individual, parent2: &Individual) -> Individual {
        let mut child_parts = Vec::new();

        // Одноточечный кроссовер
        let crossover_point = self.rng.gen_range(0..parent1.parts.len());

        for i in 0..parent1.parts.len() {
            if i < crossover_point {
                child_parts.push(parent1.parts[i].clone());
            } else {
                child_parts.push(parent2.parts[i].clone());
            }
        }

        Individual {
            parts: child_parts,
            fitness: 0.0,
            sheet_count: 0,
        }
    }

    /// Мутация особи.
    fn mutate(&mut self, individual: &mut Individual) {
        for part in &mut individual.parts {
            if self.rng.gen_bool(self.config.mutation_rate) {
                // Мутация позиции
                let max_shift = 10.0;
                part.x_mm += self.rng.gen_range(-max_shift..max_shift);
                part.y_mm += self.rng.gen_range(-max_shift..max_shift);

                // Ограничение границами
                part.x_mm = part.x_mm.clamp(
                    self.config.paper.margin_mm,
                    self.config.paper.width_mm - self.config.paper.margin_mm - part.width_mm,
                );
                part.y_mm = part.y_mm.clamp(
                    self.config.paper.margin_mm,
                    self.config.paper.height_mm - self.config.paper.margin_mm - part.height_mm,
                );
            }

            if self.rng.gen_bool(self.config.mutation_rate * 0.5) {
                // Мутация поворота
                if let Some(&rotation) = self.config.rotation_steps.choose(&mut self.rng) {
                    part.rotation = rotation;
                }
            }
        }
    }

    /// Создаёт результат раскладки из лучшей особи.
    fn create_nest_result(&self, individual: &Individual) -> NestResult {
        let sheet_area = (self.config.paper.width_mm - 2.0 * self.config.paper.margin_mm)
            * (self.config.paper.height_mm - 2.0 * self.config.paper.margin_mm);

        let total_parts_area: f64 = individual.parts.iter()
            .map(|p| p.width_mm as f64 * p.height_mm as f64)
            .sum();

        let total_sheets_area = individual.sheet_count as f64 * sheet_area as f64;

        let avg_fill_rate = if total_sheets_area > 0.0 {
            (total_parts_area / total_sheets_area * 100.0) as f32
        } else {
            0.0
        };

        // Группируем части по листам
        let mut sheets = Vec::new();
        let mut current_sheet_parts = Vec::new();
        let mut current_sheet_id = 0;

        for part in &individual.parts {
            // Простая группировка по порядку
            if current_sheet_parts.len() >= individual.parts.len() / individual.sheet_count.max(1) {
                if !current_sheet_parts.is_empty() {
                    sheets.push(NestSheet {
                        id: current_sheet_id,
                        index: current_sheet_id,
                        width_mm: self.config.paper.width_mm,
                        height_mm: self.config.paper.height_mm,
                        margin_mm: self.config.paper.margin_mm,
                        parts: current_sheet_parts,
                    });
                    current_sheet_id += 1;
                }
                current_sheet_parts = Vec::new();
            }
            current_sheet_parts.push(part.clone());
        }

        if !current_sheet_parts.is_empty() {
            sheets.push(NestSheet {
                id: current_sheet_id,
                index: current_sheet_id,
                width_mm: self.config.paper.width_mm,
                height_mm: self.config.paper.height_mm,
                margin_mm: self.config.paper.margin_mm,
                parts: current_sheet_parts,
            });
        }

        NestResult {
            sheets,
            metrics: NestMetrics {
                total_sheets: individual.sheet_count as u32,
                total_parts: individual.parts.len() as u32,
                avg_fill_rate,
                total_parts_area: total_parts_area as f32,
                total_sheets_area: total_sheets_area as f32,
            },
            params_snapshot: Default::default(),
        }
    }
}

/// Оптимизирует раскладку с помощью генетического алгоритма.
pub fn optimize_nesting_genetic(
    parts: &[NestPart],
    config: &GeneticConfig,
) -> NestResult {
    let mut optimizer = GeneticNesting::new(config.clone());
    optimizer.optimize(parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_parts() -> Vec<NestPart> {
        vec![
            NestPart {
                id: 1,
                name: Some("Part 1".to_string()),
                unfolded_face_index: 0,
                x_mm: 0.0,
                y_mm: 0.0,
                width_mm: 50.0,
                height_mm: 30.0,
                rotation: 0.0,
            },
            NestPart {
                id: 2,
                name: Some("Part 2".to_string()),
                unfolded_face_index: 1,
                x_mm: 0.0,
                y_mm: 0.0,
                width_mm: 40.0,
                height_mm: 25.0,
                rotation: 0.0,
            },
            NestPart {
                id: 3,
                name: Some("Part 3".to_string()),
                unfolded_face_index: 2,
                x_mm: 0.0,
                y_mm: 0.0,
                width_mm: 60.0,
                height_mm: 35.0,
                rotation: 0.0,
            },
        ]
    }

    #[test]
    fn test_genetic_nesting_basic() {
        let parts = create_test_parts();
        let config = GeneticConfig {
            population_size: 20,
            generations: 10,
            ..Default::default()
        };

        let mut optimizer = GeneticNesting::new(config);
        let result = optimizer.optimize(&parts);

        assert!(result.sheets.len() > 0);
        assert!(result.metrics.total_parts > 0);
    }

    #[test]
    fn test_fitness_calculation() {
        let parts = create_test_parts();
        let config = GeneticConfig::default();
        let optimizer = GeneticNesting::new(config);

        let individual = Individual {
            parts: parts.clone(),
            fitness: 0.0,
            sheet_count: 1,
        };

        let (fitness, sheets) = optimizer.calculate_fitness(&parts);
        assert!(fitness > 0.0);
        assert!(sheets >= 1);
    }

    #[test]
    fn test_parts_overlap() {
        let config = GeneticConfig::default();
        let optimizer = GeneticNesting::new(config);

        let part1 = NestPart {
            id: 1,
            name: None,
            unfolded_face_index: 0,
            x_mm: 0.0,
            y_mm: 0.0,
            width_mm: 50.0,
            height_mm: 30.0,
            rotation: 0.0,
        };

        let part2 = NestPart {
            id: 2,
            name: None,
            unfolded_face_index: 1,
            x_mm: 10.0, // Перекрывается
            y_mm: 10.0,
            width_mm: 50.0,
            height_mm: 30.0,
            rotation: 0.0,
        };

        assert!(optimizer.parts_overlap(&part1, &part2));

        let part3 = NestPart {
            id: 3,
            name: None,
            unfolded_face_index: 2,
            x_mm: 100.0, // Не перекрывается
            y_mm: 100.0,
            width_mm: 50.0,
            height_mm: 30.0,
            rotation: 0.0,
        };

        assert!(!optimizer.parts_overlap(&part1, &part3));
    }

    #[test]
    fn test_mutation() {
        let config = GeneticConfig {
            mutation_rate: 1.0, // Всегда мутируем
            ..Default::default()
        };

        let mut optimizer = GeneticNesting::new(config);
        let mut individual = Individual {
            parts: create_test_parts(),
            fitness: 0.0,
            sheet_count: 0,
        };

        let original_x = individual.parts[0].x_mm;
        optimizer.mutate(&mut individual);

        // Позиция должна измениться
        assert!((individual.parts[0].x_mm - original_x).abs() > 0.001);
    }
}
