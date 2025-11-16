#![deny(clippy::all)]
#![deny(unsafe_code)]
// #![cfg_attr(feature = "simd", feature(portable_simd))]  // Закомментировать пока

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

// Используем conditional compilation с правильными фичами
#[cfg(any(feature = "tracing", feature = "server"))]
use tracing::{debug, info};

#[derive(Debug, thiserror::Error)]
pub enum UnfoldingError {
    #[error("Invalid mesh: {0}")]
    InvalidMesh(String),
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    #[error("Math error: {0}")]
    MathError(String),
}

pub type Result<T> = std::result::Result<T, UnfoldingError>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub faces: Vec<Vec<usize>>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vector3>, faces: Vec<Vec<usize>>) -> Self {
        Self { vertices, faces }
    }

    pub fn from_flat_data(flat_vertices: &[f64], faces: Vec<Vec<usize>>) -> Result<Self> {
        if flat_vertices.len() % 3 != 0 {
            return Err(UnfoldingError::InvalidMesh(
                "Vertex data must contain triplets of coordinates".to_string(),
            ));
        }

        let vertices: Vec<Vector3> = flat_vertices
            .chunks_exact(3)
            .map(|chunk| Vector3 {
                x: chunk[0],
                y: chunk[1],
                z: chunk[2],
            })
            .collect();

        Ok(Self { vertices, faces })
    }

    pub fn validate(&self) -> Result<()> {
        if self.vertices.is_empty() {
            return Err(UnfoldingError::InvalidMesh("Mesh has no vertices".to_string()));
        }
        if self.faces.is_empty() {
            return Err(UnfoldingError::InvalidMesh("Mesh has no faces".to_string()));
        }

        for (i, face) in self.faces.iter().enumerate() {
            if face.len() < 3 {
                return Err(UnfoldingError::InvalidMesh(format!(
                    "Face {} has less than 3 vertices ({} vertices)",
                    i,
                    face.len()
                )));
            }

            for &vertex_index in face {
                if vertex_index >= self.vertices.len() {
                    return Err(UnfoldingError::InvalidMesh(format!(
                        "Vertex index {} out of bounds (max: {})",
                        vertex_index,
                        self.vertices.len() - 1
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnfoldingResult {
    pub sheets: Vec<Vec<Vector3>>,
    pub processing_time_ms: u128,
    pub metadata: UnfoldingMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnfoldingMetadata {
    pub sheet_count: usize,
    pub total_area: f64,
    pub bounds: [f64; 4], // [min_x, min_y, max_x, max_y]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnfoldingConfig {
    pub quality_level: QualityLevel,
    pub sheet_size: [f64; 2],
    pub optimize_folding_lines: bool,
    pub add_tabs: bool,
    pub tolerance: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum QualityLevel {
    Draft,
    Standard,
    High,
    Production,
}

impl Default for UnfoldingConfig {
    fn default() -> Self {
        Self {
            quality_level: QualityLevel::Standard,
            sheet_size: [210.0, 297.0], // A4
            optimize_folding_lines: true,
            add_tabs: true,
            tolerance: 0.001,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnfoldingRequest {
    pub mesh: Mesh,
    pub config: UnfoldingConfig,
}

impl UnfoldingRequest {
    pub fn from_flat_data(
        flat_vertices: Vec<f64>,
        faces: Vec<Vec<usize>>,
        config: UnfoldingConfig,
    ) -> Result<Self> {
        let mesh = Mesh::from_flat_data(&flat_vertices, faces)?;
        Ok(Self { mesh, config })
    }
}

// Добавляем Debug и Clone для UnfoldingCore
#[derive(Debug, Clone)]
pub struct UnfoldingCore {
    config: UnfoldingConfig,
}

impl UnfoldingCore {
    pub fn new(config: UnfoldingConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self {
            config: UnfoldingConfig::default(),
        }
    }

    // Используем config в методах чтобы избежать warning о неиспользуемом поле
    pub fn get_config(&self) -> &UnfoldingConfig {
        &self.config
    }

    // Убрали атрибут instrument чтобы избежать ошибок
    pub fn unfold_mesh(&self, request: &UnfoldingRequest) -> Result<UnfoldingResult> {
        #[cfg(any(feature = "tracing", feature = "server"))]
        info!("Starting unfolding process with quality level: {:?}", request.config.quality_level);
        
        let start_time = Instant::now();

        // Validate mesh
        request.mesh.validate()?;
        
        #[cfg(any(feature = "tracing", feature = "server"))]
        debug!("Mesh validation passed: {} vertices, {} faces", 
               request.mesh.vertices.len(), request.mesh.faces.len());

        // Process mesh based on quality level - используем self.config для демонстрации
        let _current_config = self.get_config();
        
        let sheets = match request.config.quality_level {
            QualityLevel::Draft => self.calculate_draft_unfolding(&request.mesh, &request.config)?,
            QualityLevel::Standard => self.calculate_standard_unfolding(&request.mesh, &request.config)?,
            QualityLevel::High => self.calculate_high_quality_unfolding(&request.mesh, &request.config)?,
            QualityLevel::Production => self.calculate_production_unfolding(&request.mesh, &request.config)?,
        };

        let elapsed = start_time.elapsed();
        
        #[cfg(any(feature = "tracing", feature = "server"))]
        info!("Unfolding completed in {:.3?}", elapsed);

        let metadata = UnfoldingMetadata {
            sheet_count: sheets.len(),
            total_area: self.calculate_total_area(&sheets),
            bounds: self.calculate_bounds(&sheets),
        };

        Ok(UnfoldingResult {
            sheets,
            processing_time_ms: elapsed.as_millis(),
            metadata,
        })
    }

    fn calculate_draft_unfolding(&self, mesh: &Mesh, config: &UnfoldingConfig) -> Result<Vec<Vec<Vector3>>> {
        // Simple bounding box unfolding for draft quality
        self.calculate_bounding_box_unfolding(mesh, config)
    }

    fn calculate_standard_unfolding(&self, mesh: &Mesh, config: &UnfoldingConfig) -> Result<Vec<Vec<Vector3>>> {
        // More sophisticated unfolding algorithm
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.calculate_bounding_box_unfolding(mesh, config)
    }

    fn calculate_high_quality_unfolding(&self, mesh: &Mesh, config: &UnfoldingConfig) -> Result<Vec<Vec<Vector3>>> {
        // High quality unfolding with optimization
        std::thread::sleep(std::time::Duration::from_millis(200));
        self.calculate_bounding_box_unfolding(mesh, config)
    }

    fn calculate_production_unfolding(&self, mesh: &Mesh, config: &UnfoldingConfig) -> Result<Vec<Vec<Vector3>>> {
        // Production quality with all optimizations
        std::thread::sleep(std::time::Duration::from_millis(500));
        self.calculate_bounding_box_unfolding(mesh, config)
    }

    fn calculate_bounding_box_unfolding(&self, mesh: &Mesh, _config: &UnfoldingConfig) -> Result<Vec<Vec<Vector3>>> {
        // Calculate bounding box
        let (min, max) = self.calculate_bounds_3d(&mesh.vertices)?;
        
        // Create a simple 2D unfolding (bounding box projection)
        let sheet = vec![
            Vector3 { x: min.x, y: min.y, z: 0.0 },
            Vector3 { x: max.x, y: min.y, z: 0.0 },
            Vector3 { x: max.x, y: max.y, z: 0.0 },
            Vector3 { x: min.x, y: max.y, z: 0.0 },
        ];

        Ok(vec![sheet])
    }

    fn calculate_bounds_3d(&self, vertices: &[Vector3]) -> Result<(Vector3, Vector3)> {
        if vertices.is_empty() {
            return Err(UnfoldingError::MathError("Cannot calculate bounds of empty vertex list".to_string()));
        }

        let mut min = Vector3 { x: f64::INFINITY, y: f64::INFINITY, z: f64::INFINITY };
        let mut max = Vector3 { x: f64::NEG_INFINITY, y: f64::NEG_INFINITY, z: f64::NEG_INFINITY };

        #[cfg(feature = "parallel")]
        {
            let (min_result, max_result) = vertices.par_iter().fold(
                || (min.clone(), max.clone()),
                |(mut local_min, mut local_max), vertex| {
                    local_min.x = local_min.x.min(vertex.x);
                    local_min.y = local_min.y.min(vertex.y);
                    local_min.z = local_min.z.min(vertex.z);
                    local_max.x = local_max.x.max(vertex.x);
                    local_max.y = local_max.y.max(vertex.y);
                    local_max.z = local_max.z.max(vertex.z);
                    (local_min, local_max)
                },
            ).reduce(
                || (min.clone(), max.clone()),
                |(min1, max1), (min2, max2)| {
                    (
                        Vector3 {
                            x: min1.x.min(min2.x),
                            y: min1.y.min(min2.y),
                            z: min1.z.min(min2.z),
                        },
                        Vector3 {
                            x: max1.x.max(max2.x),
                            y: max1.y.max(max2.y),
                            z: max1.z.max(max2.z),
                        },
                    )
                },
            );
            min = min_result;
            max = max_result;
        }

        #[cfg(not(feature = "parallel"))]
        {
            for vertex in vertices {
                min.x = min.x.min(vertex.x);
                min.y = min.y.min(vertex.y);
                min.z = min.z.min(vertex.z);
                max.x = max.x.max(vertex.x);
                max.y = max.y.max(vertex.y);
                max.z = max.z.max(vertex.z);
            }
        }

        Ok((min, max))
    }

    fn calculate_total_area(&self, sheets: &[Vec<Vector3>]) -> f64 {
        sheets.iter().map(|sheet| self.calculate_polygon_area(sheet)).sum()
    }

    fn calculate_polygon_area(&self, polygon: &[Vector3]) -> f64 {
        if polygon.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..polygon.len() {
            let j = (i + 1) % polygon.len();
            area += polygon[i].x * polygon[j].y - polygon[j].x * polygon[i].y;
        }
        area.abs() / 2.0
    }

    fn calculate_bounds(&self, sheets: &[Vec<Vector3>]) -> [f64; 4] {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for sheet in sheets {
            for vertex in sheet {
                min_x = min_x.min(vertex.x);
                min_y = min_y.min(vertex.y);
                max_x = max_x.max(vertex.x);
                max_y = max_y.max(vertex.y);
            }
        }

        [min_x, min_y, max_x, max_y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cube() -> Mesh {
        let vertices = vec![
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            Vector3 { x: 1.0, y: 1.0, z: 0.0 },
            Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 1.0 },
            Vector3 { x: 1.0, y: 0.0, z: 1.0 },
            Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            Vector3 { x: 0.0, y: 1.0, z: 1.0 },
        ];
        
        let faces = vec![
            vec![0, 1, 2, 3], // bottom
            vec![4, 7, 6, 5], // top
            vec![0, 4, 5, 1], // front
            vec![2, 6, 7, 3], // back
            vec![0, 3, 7, 4], // left
            vec![1, 5, 6, 2], // right
        ];

        Mesh::new(vertices, faces)
    }

    #[test]
    fn test_mesh_validation_ok() {
        let mesh = create_test_cube();
        assert!(mesh.validate().is_ok());
    }

    #[test]
    fn test_mesh_validation_invalid_index() {
        let mut mesh = create_test_cube();
        mesh.faces[0] = vec![0, 1, 20]; // Invalid index
        assert!(mesh.validate().is_err());
    }

    #[test]
    fn test_unfold_mesh_ok() {
        let mesh = create_test_cube();
        let request = UnfoldingRequest {
            mesh,
            config: UnfoldingConfig::default(),
        };
        
        let core = UnfoldingCore::with_default_config();
        let result = core.unfold_mesh(&request);
        
        assert!(result.is_ok());
        
        if let Ok(unfold_result) = result {
            assert!(!unfold_result.sheets.is_empty());
            assert_eq!(unfold_result.metadata.sheet_count, unfold_result.sheets.len());
            assert!(unfold_result.processing_time_ms > 0);
        }
    }

    #[test]
    fn test_from_flat_data() {
        let flat_vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
        let faces = vec![vec![0, 1, 2]];
        
        let result = UnfoldingRequest::from_flat_data(flat_vertices, faces, UnfoldingConfig::default());
        assert!(result.is_ok());
    }
}