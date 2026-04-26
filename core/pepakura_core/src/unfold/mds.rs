//! Multidimensional Scaling (MDS) based mesh unfolding.

use nalgebra::{DMatrix, DVector, SymmetricEigen};
use std::time::Instant;

use crate::geometry::mesh::Mesh;
use crate::errors::Result;

/// Configuration for MDS unfolding.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnfoldConfig {
    pub preserve_detail: bool,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl Default for UnfoldConfig {
    fn default() -> Self {
        UnfoldConfig {
            preserve_detail: true,
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }
}

/// Result of unfolding a mesh into 2D.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnfoldedMesh {
    pub vertices_2d: Vec<[f64; 2]>,
    pub faces: Vec<[usize; 3]>, // same indices as original mesh
    pub source_mesh: Mesh,      // reference to original mesh (clone)
    pub metadata: UnfoldMetadata,
}

/// Metadata about the unfolding process.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnfoldMetadata {
    pub iterations: usize,
    pub stress: f64,
    pub elapsed_ms: u64,
    pub config: UnfoldConfig,
}

/// Compute squared Euclidean distance matrix between vertices.
fn squared_distance_matrix(vertices: &[[f64; 3]]) -> DMatrix<f64> {
    let n = vertices.len();
    let mut d = DMatrix::zeros(n, n);
    for i in 0..n {
        let vi = &vertices[i];
        for j in i + 1..n {
            let vj = &vertices[j];
            let dist2 = (vi[0] - vj[0]).powi(2)
                + (vi[1] - vj[1]).powi(2)
                + (vi[2] - vj[2]).powi(2);
            d[(i, j)] = dist2;
            d[(j, i)] = dist2;
        }
    }
    d
}

/// Perform classical metric MDS (double centering).
fn classical_mds(dist_matrix: &DMatrix<f64>, dim: usize) -> (DMatrix<f64>, f64) {
    let n = dist_matrix.nrows();
    // B = -0.5 * J * D * J, where J = I - (1/n) * 1*1^T
    let j = DMatrix::identity(n, n) - DMatrix::repeat(n, n, 1.0 / n as f64);
    let b = -0.5 * &j * dist_matrix * &j;

    // Eigen decomposition of symmetric matrix B
    let eigen = SymmetricEigen::new(b);
    let eigenvalues = eigen.eigenvalues;
    let eigenvectors = eigen.eigenvectors;

    // Sort eigenvalues descending
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| eigenvalues[b].partial_cmp(&eigenvalues[a]).unwrap());

    // Take top 'dim' eigenvalues and eigenvectors
    let mut selected_values = DVector::zeros(dim);
    let mut selected_vectors = DMatrix::zeros(n, dim);
    for (k, &idx) in indices.iter().take(dim).enumerate() {
        selected_values[k] = eigenvalues[idx].max(0.0); // ensure non-negative
        selected_vectors.column_mut(k).copy_from(&eigenvectors.column(idx));
    }

    // Coordinates X = V * sqrt(Λ)
    let sqrt_lambda = selected_values.map(|v| v.sqrt());
    let coordinates = selected_vectors * DMatrix::from_diagonal(&sqrt_lambda);

    // Stress (raw stress) = sum of squared differences between original distances and embedded distances
    let mut stress = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            let orig = dist_matrix[(i, j)].sqrt();
            let emb = ((coordinates.row(i) - coordinates.row(j)).norm_squared()).sqrt();
            stress += (orig - emb).powi(2);
        }
    }

    (coordinates, stress)
}

/// Unfold a 3D mesh into 2D using Multidimensional Scaling.
pub fn unfold_mds(mesh: &Mesh, config: &UnfoldConfig) -> Result<UnfoldedMesh> {
    let start = Instant::now();

    let n = mesh.vertices.len();
    if n < 3 {
        return Err(crate::errors::PepakuraError::InvalidMesh(
            "Mesh must have at least 3 vertices".to_string(),
        ));
    }

    // Extract vertex positions
    let positions: Vec<[f64; 3]> = mesh.vertices.iter().map(|v| v.position).collect();

    // Compute squared distance matrix
    let d = squared_distance_matrix(&positions);

    // Perform MDS to 2D
    let (coordinates, stress) = classical_mds(&d, 2);

    // Convert to Vec<[f64; 2]>
    let vertices_2d: Vec<[f64; 2]> = (0..n)
        .map(|i| [coordinates[(i, 0)], coordinates[(i, 1)]])
        .collect();

    // Faces remain the same (copy indices)
    let faces: Vec<[usize; 3]> = mesh.faces.iter().map(|f| f.vertices).collect();

    let elapsed = start.elapsed();

    Ok(UnfoldedMesh {
        vertices_2d,
        faces,
        source_mesh: mesh.clone(),
        metadata: UnfoldMetadata {
            iterations: 0, // classical MDS is non‑iterative
            stress,
            elapsed_ms: elapsed.as_millis() as u64,
            config: config.clone(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vertex::Vertex;
    use crate::geometry::mesh::Face;

    fn create_cube_mesh() -> Mesh {
        // 8 vertices of a unit cube
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [1.0, 1.0, 0.0]),
            Vertex::new(3, [0.0, 1.0, 0.0]),
            Vertex::new(4, [0.0, 0.0, 1.0]),
            Vertex::new(5, [1.0, 0.0, 1.0]),
            Vertex::new(6, [1.0, 1.0, 1.0]),
            Vertex::new(7, [0.0, 1.0, 1.0]),
        ];
        // 12 faces (two triangles per cube face)
        let faces = vec![
            // bottom
            Face::new(0, 1, 2),
            Face::new(2, 3, 0),
            // top
            Face::new(4, 5, 6),
            Face::new(6, 7, 4),
            // front
            Face::new(0, 1, 5),
            Face::new(5, 4, 0),
            // back
            Face::new(2, 3, 7),
            Face::new(7, 6, 2),
            // left
            Face::new(0, 3, 7),
            Face::new(7, 4, 0),
            // right
            Face::new(1, 2, 6),
            Face::new(6, 5, 1),
        ];
        Mesh::new(vertices, faces).unwrap()
    }

    fn create_pyramid_mesh() -> Mesh {
        // square pyramid: base square + apex
        let vertices = vec![
            Vertex::new(0, [-1.0, -1.0, 0.0]),
            Vertex::new(1, [1.0, -1.0, 0.0]),
            Vertex::new(2, [1.0, 1.0, 0.0]),
            Vertex::new(3, [-1.0, 1.0, 0.0]),
            Vertex::new(4, [0.0, 0.0, 2.0]), // apex
        ];
        let faces = vec![
            // base (two triangles)
            Face::new(0, 1, 2),
            Face::new(2, 3, 0),
            // sides
            Face::new(0, 1, 4),
            Face::new(1, 2, 4),
            Face::new(2, 3, 4),
            Face::new(3, 0, 4),
        ];
        Mesh::new(vertices, faces).unwrap()
    }

    fn create_flat_square_mesh() -> Mesh {
        // single square (two triangles) lying in XY plane
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [1.0, 1.0, 0.0]),
            Vertex::new(3, [0.0, 1.0, 0.0]),
        ];
        let faces = vec![
            Face::new(0, 1, 2),
            Face::new(2, 3, 0),
        ];
        Mesh::new(vertices, faces).unwrap()
    }

    #[test]
    fn test_unfold_cube() {
        let mesh = create_cube_mesh();
        let config = UnfoldConfig::default();
        let unfolded = unfold_mds(&mesh, &config).unwrap();
        assert_eq!(unfolded.vertices_2d.len(), 8);
        assert_eq!(unfolded.faces.len(), 12);
        // Stress should be relatively low for a convex shape
        assert!(unfolded.metadata.stress < 10.0);
    }

    #[test]
    fn test_unfold_pyramid() {
        let mesh = create_pyramid_mesh();
        let config = UnfoldConfig::default();
        let unfolded = unfold_mds(&mesh, &config).unwrap();
        assert_eq!(unfolded.vertices_2d.len(), 5);
        assert_eq!(unfolded.faces.len(), 6);
        assert!(unfolded.metadata.stress < 10.0);
    }

    #[test]
    fn test_unfold_flat_square() {
        let mesh = create_flat_square_mesh();
        let config = UnfoldConfig::default();
        let unfolded = unfold_mds(&mesh, &config).unwrap();
        // Flat square should unfold with near‑zero stress
        assert!(unfolded.metadata.stress < 1e-6);
        // Vertices should keep their XY coordinates (up to rotation/reflection)
        for (i, v2d) in unfolded.vertices_2d.iter().enumerate() {
            let v3d = &mesh.vertices[i].position;
            // Distance between 2D point and original XY should be small
            let dx = v2d[0] - v3d[0];
            let dy = v2d[1] - v3d[1];
            assert!(dx.hypot(dy) < 0.1);
        }
    }

    #[test]
    fn test_unfold_empty_mesh() {
        let vertices = vec![];
        let faces = vec![];
        let mesh = Mesh::new(vertices, faces).unwrap();
        let config = UnfoldConfig::default();
        let result = unfold_mds(&mesh, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_unfold_small_mesh() {
        // Two vertices – cannot embed in 2D with MDS (requires at least 3)
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
        ];
        let faces = vec![Face::new(0, 1, 0)]; // degenerate face
        let mesh = Mesh::new(vertices, faces).unwrap();
        let config = UnfoldConfig::default();
        let result = unfold_mds(&mesh, &config);
        // Should error because n < 3
        assert!(result.is_err());
    }
}