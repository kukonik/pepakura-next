// Diagnostics #22: what JSON does serde actually produce for UnwrapResult?
// Expected by SplitView.vue draw2D:
//   top-level keys: pieces, vertices_2d, faces, engine_used
//   piece.outline.points[i] must be {"x":..,"y":..} object, NOT [x,y] array
use pepakura_core::import::parse_obj_str;
use pepakura_core::papercraft_adapter::unfold_with_papercraft;

const CUBE_OBJ: &str = concat!(
    "v -1.0 -1.0 -1.0\n",
    "v  1.0 -1.0 -1.0\n",
    "v  1.0  1.0 -1.0\n",
    "v -1.0  1.0 -1.0\n",
    "v -1.0 -1.0  1.0\n",
    "v  1.0 -1.0  1.0\n",
    "v  1.0  1.0  1.0\n",
    "v -1.0  1.0  1.0\n",
    "f 1 2 6\n",
    "f 1 6 5\n",
    "f 2 3 7\n",
    "f 2 7 6\n",
    "f 5 6 7\n",
    "f 5 7 8\n",
    "f 1 4 3\n",
    "f 1 3 2\n",
    "f 4 8 7\n",
    "f 4 7 3\n",
    "f 1 5 8\n",
    "f 1 8 4\n",
);

fn main() {
    let mesh = match parse_obj_str(CUBE_OBJ) {
        Ok(m) => m,
        Err(e) => { println!("PARSE ERROR: {}", e); return; }
    };
    println!("mesh: vertices={}, faces={}", mesh.vertices.len(), mesh.faces.len());
    if mesh.vertices.is_empty() || mesh.faces.is_empty() {
        println!("EMPTY MESH: parse_obj_str did not understand the OBJ");
        return;
    }
    let res = match unfold_with_papercraft(&mesh) {
        Ok(r) => r,
        Err(e) => { println!("UNFOLD ERROR: {}", e); return; }
    };
    let json = match serde_json::to_value(&res) {
        Ok(j) => j,
        Err(e) => { println!("SERDE ERROR: {}", e); return; }
    };
    let keys: Vec<String> = json.as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    println!("TOP-LEVEL KEYS: {:?}", keys);
    match json.get("pieces").and_then(|v| v.as_array()) {
        Some(pieces) => {
            println!("pieces count: {}", pieces.len());
            if let Some(p0) = pieces.first() {
                let pk: Vec<String> = p0.as_object()
                    .map(|o| o.keys().cloned().collect())
                    .unwrap_or_default();
                println!("piece[0] keys: {:?}", pk);
                match p0.get("outline") {
                    Some(o) => {
                        match o.get("points").and_then(|v| v.as_array()) {
                            Some(pts) => {
                                let first = pts.first().map(|p| p.to_string())
                                    .unwrap_or_else(|| "empty".to_string());
                                println!("points[0] = {}", first);
                                println!("GOOD if it looks like {{\"x\":..,\"y\":..}}, BAD if [x,y]");
                            }
                            None => println!("BAD: outline has no points key"),
                        }
                    }
                    None => println!("BAD: piece has no outline key"),
                }
            }
        }
        None => println!("BAD: no pieces key at top level"),
    }
    println!("---- FULL JSON ----");
    println!("{}", serde_json::to_string_pretty(&json).unwrap_or_default());
}
