use crate::geometry::Mesh;

#[derive(Debug, Clone, Default)]
pub struct SanitizeOptions {
    pub remove_degenerate: bool,
}

pub fn simplify_mesh(_mesh: &Mesh, _opts: &SanitizeOptions) -> Mesh {
    Mesh::new("sanitized")
}
// [55-C1] sanitize_mesh: min-stub for plugins::builtin (real logic - later block)
pub fn sanitize_mesh(mesh: &Mesh) -> Result<Mesh, String> {
    Ok(mesh.clone())
}