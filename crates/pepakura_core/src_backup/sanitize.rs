use crate::geometry::Mesh;

#[derive(Debug, Clone, Default)]
pub struct SanitizeOptions {}

pub fn simplify_mesh(mesh: &Mesh, _opts: &SanitizeOptions) -> Result<Mesh, String> {
    // Заглушка: возвращаем копию
    Ok(mesh.clone())
}
