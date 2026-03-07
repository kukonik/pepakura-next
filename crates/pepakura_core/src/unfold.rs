#[derive(Debug, Clone)]
pub struct UnfoldedFace {
    pub center: crate::nesting::Point2D,
    pub vertices_2d: Vec<crate::nesting::Point2D>,
}

#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub faces: Vec<UnfoldedFace>,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct UnfoldResult {
    pub faces: Vec<UnfoldedFace>,
    pub seams: Vec<()>,
    pub layout: LayoutResult,
}
