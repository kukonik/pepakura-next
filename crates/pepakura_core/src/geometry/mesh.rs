//! Модуль для работы с мешами (3D-моделями).

use serde::{Deserialize, Serialize};

use crate::geometry::Vertex;

/// Грань меша (треугольник).
/// 
/// Содержит индексы трёх вершин и опциональный ID материала.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Face {
    /// Индексы вершин грани [v1, v2, v3]
    pub vertices: [usize; 3],
    /// ID материала (опционально)
    pub material_id: Option<usize>,
}

impl Face {
    /// Создаёт новую грань из трёх вершин.
    /// 
    /// # Аргументы
    /// * `v1` - индекс первой вершины
    /// * `v2` - индекс второй вершины
    /// * `v3` - индекс третьей вершины
    /// 
    /// # Примеры
    /// 
    /// ```
    /// use pepakura_core::geometry::Face;
    /// 
    /// let face = Face::new(0, 1, 2);
    /// assert_eq!(face.vertices, [0, 1, 2]);
    /// ```
    pub fn new(v1: usize, v2: usize, v3: usize) -> Self {
        Self {
            vertices: [v1, v2, v3],
            material_id: None,
        }
    }

    /// Создаёт грань с материалом.
    pub fn with_material(v1: usize, v2: usize, v3: usize, material_id: usize) -> Self {
        Self {
            vertices: [v1, v2, v3],
            material_id: Some(material_id),
        }
    }

    /// Возвращает индексы вершин как слайс.
    pub fn vertex_indices(&self) -> &[usize; 3] {
        &self.vertices
    }

    /// Вычисляет площадь грани (если известны позиции вершин).
    pub fn area(&self, vertices: &[Vertex]) -> Option<f64> {
        let max_idx = *self.vertices.iter().max()?;
        if vertices.len() <= max_idx {
            return None;
        }

        let v0 = &vertices[self.vertices[0]].position;
        let v1 = &vertices[self.vertices[1]].position;
        let v2 = &vertices[self.vertices[2]].position;

        // Векторы сторон
        let a = [
            v1[0] - v0[0],
            v1[1] - v0[1],
            v1[2] - v0[2],
        ];
        let b = [
            v2[0] - v0[0],
            v2[1] - v0[1],
            v2[2] - v0[2],
        ];

        // Векторное произведение
        let cross = [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ];

        // Площадь = половина длины векторного произведения
        Some((cross[0].powi(2) + cross[1].powi(2) + cross[2].powi(2)).sqrt() / 2.0)
    }
}

/// Ограничивающий прямоугольный параллелепипед (AABB).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoundingBox {
    /// Минимальная точка [min_x, min_y, min_z]
    pub min: [f64; 3],
    /// Максимальная точка [max_x, max_y, max_z]
    pub max: [f64; 3],
}

impl BoundingBox {
    /// Создаёт новый bounding box.
    pub fn new(min: [f64; 3], max: [f64; 3]) -> Self {
        Self { min, max }
    }

    /// Возвращает размер коробки [width, height, depth].
    pub fn size(&self) -> [f64; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    /// Возвращает центр коробки.
    pub fn center(&self) -> [f64; 3] {
        [
            (self.min[0] + self.max[0]) / 2.0,
            (self.min[1] + self.max[1]) / 2.0,
            (self.min[2] + self.max[2]) / 2.0,
        ]
    }

    /// Пустой bounding box.
    pub fn empty() -> Self {
        Self {
            min: [f64::INFINITY, f64::INFINITY, f64::INFINITY],
            max: [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY],
        }
    }
}

/// Метаданные меша.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeshMetadata {
    /// Имя меша
    pub name: Option<String>,
    /// Автор
    pub author: Option<String>,
    /// Описание
    pub description: Option<String>,
    /// Теги для поиска
    pub tags: Vec<String>,
    /// Дата создания
    pub created_at: Option<String>,
    /// Дата модификации
    pub modified_at: Option<String>,
}

/// 3D-меш для развёртки.
/// 
/// Содержит вершины, грани и метаданные модели.
/// 
/// # Примеры
/// 
/// ```
/// use pepakura_core::geometry::{Mesh, Vertex, Face};
/// 
/// let mut mesh = Mesh::new("Cube");
/// // Добавление вершин и граней...
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// Вершины меша
    pub vertices: Vec<Vertex>,
    /// Грани меша (треугольники)
    pub faces: Vec<Face>,
    /// Имя меша
    pub name: String,
    /// Метаданные
    pub metadata: MeshMetadata,
}

impl Mesh {
    /// Создаёт новый пустой меш с именем.
    /// 
    /// # Примеры
    /// 
    /// ```
    /// use pepakura_core::geometry::Mesh;
    /// 
    /// let mesh = Mesh::new("MyModel");
    /// assert_eq!(mesh.name, "MyModel");
    /// assert!(mesh.vertices.is_empty());
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            name: name.to_string(),
            metadata: MeshMetadata {
                name: Some(name.to_string()),
                ..Default::default()
            },
        }
    }

    /// Создаёт меш с вершинами и гранями.
    pub fn with_data(name: &str, vertices: Vec<Vertex>, faces: Vec<Face>) -> Self {
        Self {
            vertices,
            faces,
            name: name.to_string(),
            metadata: MeshMetadata {
                name: Some(name.to_string()),
                ..Default::default()
            },
        }
    }

    /// Добавляет вершину в меш.
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    /// Добавляет грань в меш.
    pub fn add_face(&mut self, face: Face) {
        self.faces.push(face);
    }

    /// Вычисляет ограничивающий короб меша.
    /// 
    /// # Примеры
    /// 
    /// ```
    /// use pepakura_core::geometry::{Mesh, Vertex, Face};
    /// 
    /// let mut mesh = Mesh::new("Test");
    /// mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
    /// mesh.add_vertex(Vertex::new(1, [1.0, 1.0, 1.0]));
    /// let bbox = mesh.bounding_box();
    /// assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
    /// assert_eq!(bbox.max, [1.0, 1.0, 1.0]);
    /// ```
    pub fn bounding_box(&self) -> BoundingBox {
        if self.vertices.is_empty() {
            return BoundingBox::empty();
        }

        let mut bbox = BoundingBox::empty();

        for vertex in &self.vertices {
            bbox.min[0] = bbox.min[0].min(vertex.position[0]);
            bbox.min[1] = bbox.min[1].min(vertex.position[1]);
            bbox.min[2] = bbox.min[2].min(vertex.position[2]);
            bbox.max[0] = bbox.max[0].max(vertex.position[0]);
            bbox.max[1] = bbox.max[1].max(vertex.position[1]);
            bbox.max[2] = bbox.max[2].max(vertex.position[2]);
        }

        bbox
    }

    /// Вычисляет центроид (среднюю точку) меша.
    /// 
    /// # Примеры
    /// 
    /// ```
    /// use pepakura_core::geometry::{Mesh, Vertex};
    /// 
    /// let mut mesh = Mesh::new("Test");
    /// mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
    /// mesh.add_vertex(Vertex::new(1, [2.0, 2.0, 2.0]));
    /// assert_eq!(mesh.centroid(), [1.0, 1.0, 1.0]);
    /// ```
    pub fn centroid(&self) -> [f64; 3] {
        if self.vertices.is_empty() {
            return [0.0, 0.0, 0.0];
        }

        let mut sum = [0.0, 0.0, 0.0];
        for vertex in &self.vertices {
            sum[0] += vertex.position[0];
            sum[1] += vertex.position[1];
            sum[2] += vertex.position[2];
        }

        let count = self.vertices.len() as f64;
        [sum[0] / count, sum[1] / count, sum[2] / count]
    }

    /// Масштабирует меш на заданный фактор.
    /// 
    /// # Примеры
    /// 
    /// ```
    /// use pepakura_core::geometry::{Mesh, Vertex};
    /// 
    /// let mut mesh = Mesh::new("Test");
    /// mesh.add_vertex(Vertex::new(0, [1.0, 2.0, 3.0]));
    /// mesh.scale(2.0);
    /// assert_eq!(mesh.vertices[0].position, [2.0, 4.0, 6.0]);
    /// ```
    pub fn scale(&mut self, factor: f64) {
        for vertex in &mut self.vertices {
            vertex.position[0] *= factor;
            vertex.position[1] *= factor;
            vertex.position[2] *= factor;
        }
    }

    /// Транслирует (сдвигает) меш на заданный оффсет.
    /// 
    /// # Примеры
    /// 
    /// ```
    /// use pepakura_core::geometry::{Mesh, Vertex};
    /// 
    /// let mut mesh = Mesh::new("Test");
    /// mesh.add_vertex(Vertex::new(0, [1.0, 2.0, 3.0]));
    /// mesh.translate([1.0, 1.0, 1.0]);
    /// assert_eq!(mesh.vertices[0].position, [2.0, 3.0, 4.0]);
    /// ```
    pub fn translate(&mut self, offset: [f64; 3]) {
        for vertex in &mut self.vertices {
            vertex.position[0] += offset[0];
            vertex.position[1] += offset[1];
            vertex.position[2] += offset[2];
        }
    }

    /// Центрирует меш в начале координат.
    pub fn center(&mut self) {
        let c = self.centroid();
        self.translate([-c[0], -c[1], -c[2]]);
    }

    /// Возвращает количество вершин.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Возвращает количество граней.
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Вычисляет общую площадь поверхности.
    pub fn total_area(&self) -> f64 {
        self.faces
            .iter()
            .filter_map(|face| face.area(&self.vertices))
            .sum()
    }

    /// Валидирует меш на корректность.
    /// 
    /// Проверяет:
    /// - Все индексы вершин в гранях существуют
    /// - Нет вырожденных граней (все вершины разные)
    pub fn validate(&self) -> Result<(), MeshError> {
        for (i, face) in self.faces.iter().enumerate() {
            // Проверка индексов
            for &idx in &face.vertices {
                if idx >= self.vertices.len() {
                    return Err(MeshError::InvalidVertexIndex(idx));
                }
            }

            // Проверка на вырожденность
            if face.vertices[0] == face.vertices[1]
                || face.vertices[1] == face.vertices[2]
                || face.vertices[0] == face.vertices[2]
            {
                return Err(MeshError::DegenerateFace(i));
            }
        }

        Ok(())
    }
}

/// Ошибки меша.
#[derive(Debug, thiserror::Error)]
pub enum MeshError {
    /// Некорректный индекс вершины
    #[error("Некорректный индекс вершины: {0}")]
    InvalidVertexIndex(usize),
    /// Вырожденная грань
    #[error("Вырожденная грань: индекс {0}")]
    DegenerateFace(usize),
    /// Невозможная грань (не треугольник)
    #[error("Невозможная грань: индекс {0}")]
    InvalidFace(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_mesh() -> Mesh {
        let mut mesh = Mesh::new("TestCube");
        
        // 8 вершин куба
        for i in 0..8 {
            let x = if i & 1 != 0 { 1.0 } else { 0.0 };
            let y = if i & 2 != 0 { 1.0 } else { 0.0 };
            let z = if i & 4 != 0 { 1.0 } else { 0.0 };
            mesh.add_vertex(Vertex::new(i, [x, y, z]));
        }

        // 12 граней куба (по 2 треугольника на грань)
        mesh.add_face(Face::new(0, 1, 2));
        mesh.add_face(Face::new(1, 3, 2));
        
        mesh
    }

    #[test]
    fn test_mesh_new() {
        let mesh = Mesh::new("Test");
        assert_eq!(mesh.name, "Test");
        assert!(mesh.vertices.is_empty());
        assert!(mesh.faces.is_empty());
    }

    #[test]
    fn test_mesh_bounding_box() {
        let mesh = create_test_mesh();
        let bbox = mesh.bounding_box();
        assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
        assert_eq!(bbox.max, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_mesh_centroid() {
        let mesh = create_test_mesh();
        let centroid = mesh.centroid();
        assert_eq!(centroid, [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_mesh_scale() {
        let mut mesh = create_test_mesh();
        mesh.scale(2.0);
        assert_eq!(mesh.vertices[0].position, [0.0, 0.0, 0.0]);
        assert_eq!(mesh.vertices[7].position, [2.0, 2.0, 2.0]);
    }

    #[test]
    fn test_mesh_translate() {
        let mut mesh = create_test_mesh();
        mesh.translate([1.0, 1.0, 1.0]);
        assert_eq!(mesh.vertices[0].position, [1.0, 1.0, 1.0]);
        assert_eq!(mesh.vertices[7].position, [2.0, 2.0, 2.0]);
    }

    #[test]
    fn test_mesh_center() {
        let mut mesh = create_test_mesh();
        mesh.center();
        let centroid = mesh.centroid();
        assert!(centroid.iter().all(|&v| v.abs() < 0.0001));
    }

    #[test]
    fn test_mesh_validate_success() {
        let mesh = create_test_mesh();
        assert!(mesh.validate().is_ok());
    }

    #[test]
    fn test_mesh_validate_invalid_index() {
        let mut mesh = Mesh::new("Invalid");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2)); // Индексы 1 и 2 не существуют
        assert!(matches!(mesh.validate(), Err(MeshError::InvalidVertexIndex(_))));
    }

    #[test]
    fn test_mesh_validate_degenerate() {
        let mut mesh = Mesh::new("Degenerate");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_face(Face::new(0, 0, 1)); // Вершина 0 повторяется
        assert!(matches!(mesh.validate(), Err(MeshError::DegenerateFace(_))));
    }

    #[test]
    fn test_face_area() {
        let mut mesh = Mesh::new("Triangle");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.0, 1.0, 0.0]));
        
        let face = Face::new(0, 1, 2);
        let area = face.area(&mesh.vertices).unwrap();
        assert!((area - 0.5).abs() < 0.0001);
    }
}
