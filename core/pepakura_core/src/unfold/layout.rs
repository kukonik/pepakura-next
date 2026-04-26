use crate::unfold::{UnfoldedFace, PlacedFace, LayoutResult, Point2D};

pub fn arrange_faces(unfolded_faces: Vec<UnfoldedFace>) -> LayoutResult {
    // Пока используем простую укладку - все элементы в ряд
    let mut placed_faces = Vec::new();
    let mut current_x = 0.0;
    let face_width = 50.0; // Примерная ширина грани
    let face_height = 50.0; // Примерная высота грани
    
    for (_index, face) in unfolded_faces.into_iter().enumerate() {
        let placed_face = PlacedFace {
            face,
            position: Point2D { x: current_x, y: 0.0 },
            rotation: 0.0,
        };
        
        placed_faces.push(placed_face);
        current_x += face_width;
    }
    
    LayoutResult {
        faces: placed_faces,
        width: current_x,
        height: face_height,
    }
}
