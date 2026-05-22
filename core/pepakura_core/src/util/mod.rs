//! Модуль для вспомогательных утилит

pub mod math;

/// Преобразует градусы в радианы
/// 
/// # Аргументы
/// * `degrees` - Угол в градусах
/// 
/// # Возвращает
/// Угол в радианах
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

/// Преобразует радианы в градусы
/// 
/// # Аргументы
/// * `radians` - Угол в радианах
/// 
/// # Возвращает
/// Угол в градусах
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_degrees_to_radians() {
        let degrees = 180.0;
        let radians = degrees_to_radians(degrees);
        assert!((radians - std::f64::consts::PI).abs() < 1e-10);
    }
    
    #[test]
    fn test_radians_to_degrees() {
        let radians = std::f64::consts::PI;
        let degrees = radians_to_degrees(radians);
        assert!((degrees - 180.0).abs() < 1e-10);
    }
}