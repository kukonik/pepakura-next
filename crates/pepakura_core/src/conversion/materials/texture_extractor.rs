//! Экстрактор текстур из PDO

use crate::pdo_parser::PdoTexture;
use image::{RgbaImage, ImageFormat};
use std::io::Cursor;

/// Экстрактор текстур из PDO
pub struct TextureExtractor;

impl TextureExtractor {
    /// Конвертирует PDO текстуру в изображение
    pub fn extract_texture(pdo_texture: &PdoTexture) -> Option<RgbaImage> {
        if pdo_texture.data.is_empty() {
            return None;
        }

        // PDO текстуры обычно в формате RGBA
        RgbaImage::from_raw(pdo_texture.width, pdo_texture.height, pdo_texture.data.clone())
    }

    /// Экспортирует текстуру в PNG формат
    pub fn export_to_png(pdo_texture: &PdoTexture) -> Option<Vec<u8>> {
        let image = Self::extract_texture(pdo_texture)?;

        let mut png_data = Vec::new();
        let mut cursor = Cursor::new(&mut png_data);
        image
            .write_to(&mut cursor, ImageFormat::Png)
            .ok()?;

        Some(png_data)
    }

    /// Экспортирует текстуру в JPEG формат
    pub fn export_to_jpeg(pdo_texture: &PdoTexture, __quality: u8) -> Option<Vec<u8>> {
        let image = Self::extract_texture(pdo_texture)?;

        // Конвертируем в RGB для JPEG
        let rgb_image = image::DynamicImage::ImageRgba8(image).to_rgb8();

        let mut jpeg_data = Vec::new();
        let mut cursor = Cursor::new(&mut jpeg_data);
        rgb_image
            .write_to(&mut cursor, ImageFormat::Jpeg)
            .ok()?;

        Some(jpeg_data)
    }

    /// Создает текстуры атлас из множества текстур
    pub fn create_texture_atlas(
        textures: &[PdoTexture],
    ) -> Option<(RgbaImage, Vec<TextureAtlasRegion>)> {
        if textures.is_empty() {
            return None;
        }

        // Простая упаковка в ряд (можно улучшить через bin packing)
        let total_width: u32 = textures.iter().map(|t| t.width).sum();
        let max_height = textures.iter().map(|t| t.height).max()?;

        let mut atlas = RgbaImage::new(total_width, max_height);
        let mut regions = Vec::new();
        let mut current_x = 0u32;

        for texture in textures {
            if let Some(img) = Self::extract_texture(texture) {
                let region = TextureAtlasRegion {
                    texture_id: texture.id,
                    x: current_x,
                    y: 0,
                    width: texture.width,
                    height: texture.height,
                    uv_min: [
                        current_x as f32 / total_width as f32,
                        0.0,
                    ],
                    uv_max: [
                        (current_x + texture.width) as f32 / total_width as f32,
                        1.0,
                    ],
                };
                regions.push(region);

                // Копируем пиксели в атлас
                for y in 0..texture.height.min(max_height) {
                    for x in 0..texture.width {
                        let pixel = img.get_pixel(x, y);
                        atlas.put_pixel(current_x + x, y, *pixel);
                    }
                }

                current_x += texture.width;
            }
        }

        Some((atlas, regions))
    }

    /// Создает текстуры атлас с улучшенной упаковкой (grid-based)
    pub fn create_texture_atlas_grid(
        textures: &[PdoTexture],
        padding: u32,
    ) -> Option<(RgbaImage, Vec<TextureAtlasRegion>)> {
        if textures.is_empty() {
            return None;
        }

        // Сортируем текстуры по высоте (для лучшей упаковки)
        let mut sorted_indices: Vec<usize> = (0..textures.len()).collect();
        sorted_indices.sort_by(|&a, &b| {
            textures[b].height.cmp(&textures[a].height)
        });

        // Оцениваем размер атласа (степень двойки)
        let total_area: u32 = textures
            .iter()
            .map(|t| (t.width + padding) * (t.height + padding))
            .sum();
        let atlas_size = Self::next_power_of_two((total_area as f32).sqrt() as u32).max(256);

        let mut atlas = RgbaImage::new(atlas_size, atlas_size);
        let mut regions = Vec::new();

        // Простая grid упаковка
        let mut current_x = padding;
        let mut current_y = padding;
        let mut row_height = 0;

        for &idx in &sorted_indices {
            let texture = &textures[idx];

            if let Some(img) = Self::extract_texture(texture) {
                // Проверяем, помещается ли в текущий ряд
                if current_x + texture.width + padding > atlas_size {
                    // Переходим к следующему ряду
                    current_x = padding;
                    current_y += row_height + padding;
                    row_height = 0;
                }

                // Проверяем, помещается ли по вертикали
                if current_y + texture.height + padding > atlas_size {
                    // Нужно увеличить атлас (упрощенно - возвращаем ошибку)
                    // В реальной реализации нужно динамическое изменение размера
                    continue;
                }

                let region = TextureAtlasRegion {
                    texture_id: texture.id,
                    x: current_x,
                    y: current_y,
                    width: texture.width,
                    height: texture.height,
                    uv_min: [
                        current_x as f32 / atlas_size as f32,
                        current_y as f32 / atlas_size as f32,
                    ],
                    uv_max: [
                        (current_x + texture.width) as f32 / atlas_size as f32,
                        (current_y + texture.height) as f32 / atlas_size as f32,
                    ],
                };
                regions.push(region);

                // Копируем пиксели в атлас
                for y in 0..texture.height {
                    for x in 0..texture.width {
                        let pixel = img.get_pixel(x, y);
                        atlas.put_pixel(current_x + x, current_y + y, *pixel);
                    }
                }

                current_x += texture.width + padding;
                row_height = row_height.max(texture.height);
            }
        }

        Some((atlas, regions))
    }

    /// Находит следующую степень двойки
    fn next_power_of_two(mut n: u32) -> u32 {
        if n == 0 {
            return 1;
        }
        n -= 1;
        n |= n >> 1;
        n |= n >> 2;
        n |= n >> 4;
        n |= n >> 8;
        n |= n >> 16;
        n + 1
    }

    /// Вычисляет средний цвет текстуры
    pub fn compute_average_color(pdo_texture: &PdoTexture) -> Option<[f32; 4]> {
        let image = Self::extract_texture(pdo_texture)?;

        let mut sum_r = 0u64;
        let mut sum_g = 0u64;
        let mut sum_b = 0u64;
        let mut sum_a = 0u64;
        let mut count = 0u64;

        for pixel in image.pixels() {
            sum_r += pixel[0] as u64;
            sum_g += pixel[1] as u64;
            sum_b += pixel[2] as u64;
            sum_a += pixel[3] as u64;
            count += 1;
        }

        if count == 0 {
            return None;
        }

        Some([
            (sum_r / count) as f32 / 255.0,
            (sum_g / count) as f32 / 255.0,
            (sum_b / count) as f32 / 255.0,
            (sum_a / count) as f32 / 255.0,
        ])
    }

    /// Проверяет, является ли текстура полупрозрачной
    pub fn is_transparent(pdo_texture: &PdoTexture) -> bool {
        if let Some(image) = Self::extract_texture(pdo_texture) {
            image.pixels().any(|p| p[3] < 255)
        } else {
            false
        }
    }

    /// Вычисляет dimensions текстуры
    pub fn get_dimensions(pdo_texture: &PdoTexture) -> (u32, u32) {
        (pdo_texture.width, pdo_texture.height)
    }

    /// Проверяет, является ли размер текстуры степенью двойки
    pub fn is_power_of_two(pdo_texture: &PdoTexture) -> bool {
        Self::is_power_of_two_u32(pdo_texture.width)
            && Self::is_power_of_two_u32(pdo_texture.height)
    }

    fn is_power_of_two_u32(n: u32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

/// Регион в текстурном атласе
#[derive(Debug, Clone)]
pub struct TextureAtlasRegion {
    pub texture_id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
}

impl TextureAtlasRegion {
    /// Возвращает ширину региона
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Возвращает высоту региона
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Возвращает UV размер
    pub fn uv_size(&self) -> [f32; 2] {
        [
            self.uv_max[0] - self.uv_min[0],
            self.uv_max[1] - self.uv_min[1],
        ]
    }

    /// Возвращает UV центр
    pub fn uv_center(&self) -> [f32; 2] {
        [
            (self.uv_min[0] + self.uv_max[0]) / 2.0,
            (self.uv_min[1] + self.uv_max[1]) / 2.0,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_texture(width: u32, height: u32) -> PdoTexture {
        let data = (0..width * height * 4)
            .map(|i| (i % 256) as u8)
            .collect();

        PdoTexture {
            id: 0,
            width,
            height,
            data,
            name: "Test".to_string(),
        }
    }

    #[test]
    fn test_extract_texture() {
        let texture = create_test_texture(4, 4);
        let image = TextureExtractor::extract_texture(&texture);

        assert!(image.is_some());
        let image = image.unwrap();
        assert_eq!(image.width(), 4);
        assert_eq!(image.height(), 4);
    }

    #[test]
    fn test_extract_texture_empty_data() {
        let texture = PdoTexture {
            id: 0,
            width: 4,
            height: 4,
            data: vec![],
            name: "Test".to_string(),
        };

        let image = TextureExtractor::extract_texture(&texture);
        assert!(image.is_none());
    }

    #[test]
    fn test_export_to_png() {
        let texture = create_test_texture(4, 4);
        let png_data = TextureExtractor::export_to_png(&texture);

        assert!(png_data.is_some());
        assert!(!png_data.unwrap().is_empty());
    }

    #[test]
    fn test_create_texture_atlas() {
        let textures = vec![
            create_test_texture(64, 64),
            create_test_texture(32, 32),
        ];

        let result = TextureExtractor::create_texture_atlas(&textures);
        assert!(result.is_some());

        let (atlas, regions) = result.unwrap();
        assert_eq!(regions.len(), 2);
        assert_eq!(atlas.width(), 96); // 64 + 32
        assert_eq!(atlas.height(), 64); // max(64, 32)
    }

    #[test]
    fn test_create_texture_atlas_empty() {
        let textures: Vec<PdoTexture> = vec![];
        let result = TextureExtractor::create_texture_atlas(&textures);
        assert!(result.is_none());
    }

    #[test]
    fn test_compute_average_color() {
        // Создаем текстуру с одинаковыми пикселями
        let mut data = vec![128u8; 4 * 4 * 4]; // RGBA = 128, 128, 128, 255
        let texture = PdoTexture {
            id: 0,
            width: 4,
            height: 4,
            data,
            name: "Test".to_string(),
        };

        let avg_color = TextureExtractor::compute_average_color(&texture);
        assert!(avg_color.is_some());

        let color = avg_color.unwrap();
        assert!((color[0] - 0.5).abs() < 0.01); // ~128/255
        assert!((color[1] - 0.5).abs() < 0.01);
        assert!((color[2] - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_is_power_of_two() {
        let texture_pot = create_test_texture(64, 64);
        assert!(TextureExtractor::is_power_of_two(&texture_pot));

        let texture_non_pot = create_test_texture(63, 64);
        assert!(!TextureExtractor::is_power_of_two(&texture_non_pot));
    }

    #[test]
    fn test_texture_atlas_region() {
        let region = TextureAtlasRegion {
            texture_id: 0,
            x: 10,
            y: 20,
            width: 64,
            height: 32,
            uv_min: [0.1, 0.2],
            uv_max: [0.5, 0.6],
        };

        assert_eq!(region.width(), 64);
        assert_eq!(region.height(), 32);
        assert_eq!(region.uv_size(), [0.4, 0.4]);
        assert_eq!(region.uv_center(), [0.3, 0.4]);
    }
}
