$src = "D:\Dev\pepakura-next\crates\pepakura_core\src\unfold\seam_cut.rs"
$lines = Get-Content $src
$out = [System.Collections.ArrayList]::new()
$state = 0
foreach ($l in $lines) {
    if ($state -eq 0 -and $l -match "for \(f_idx, face\) in mesh\.faces") {
        $out.Add("        // Alpha 0.4: Centroid calculation for compactness") | Out-Null
        $out.Add("        let mut cx = 0.0; let mut cy = 0.0; let mut cz = 0.0;") | Out-Null
        $out.Add("        for v in &mesh.vertices { cx += v.position[0]; cy += v.position[1]; cz += v.position[2]; }") | Out-Null
        $out.Add("        let n_v = mesh.vertices.len() as f64;") | Out-Null
        $out.Add("        let centroid = [cx / n_v, cy / n_v, cz / n_v];") | Out-Null
        $out.Add("        let mut max_dist = 0.0;") | Out-Null
        $out.Add("        for v in &mesh.vertices {") | Out-Null
        $out.Add("            let dx = v.position[0] - centroid[0]; let dy = v.position[1] - centroid[1]; let dz = v.position[2] - centroid[2];") | Out-Null
        $out.Add("            let d = (dx*dx + dy*dy + dz*dz).sqrt();") | Out-Null
        $out.Add("            if d > max_dist { max_dist = d; }") | Out-Null
        $out.Add("        }") | Out-Null
        $out.Add("        if max_dist < 1e-5 { max_dist = 1.0; }") | Out-Null
        $state = 1
    }
    if ($state -eq 1 -and $l -match "let weight = compute_edge_weight") {
        $out.Add("                let weight = compute_edge_weight(mesh, f_idx, other_f_idx, v1, v2, &centroid, max_dist);") | Out-Null
        $state = 2
        continue
    }
    if ($state -eq 2 -and $l -match "fn compute_edge_weight\(mesh: &Mesh, f1: usize, f2: usize, v1: usize, v2: usize\)") {
        $out.Add("fn compute_edge_weight(mesh: &Mesh, f1: usize, f2: usize, v1: usize, v2: usize, centroid: &[f64; 3], max_dist: f64) -> f64 {") | Out-Null
        $state = 3
        continue
    }
    if ($state -eq 3 -and $l -match "w_dihedral \+ w_length") {
        $out.Add("    let mid_x = (p1[0] + p2[0]) * 0.5 - centroid[0];") | Out-Null
        $out.Add("    let mid_y = (p1[1] + p2[1]) * 0.5 - centroid[1];") | Out-Null
        $out.Add("    let mid_z = (p1[2] + p2[2]) * 0.5 - centroid[2];") | Out-Null
        $out.Add("    let dist = ((mid_x*mid_x + mid_y*mid_y + mid_z*mid_z).sqrt() / max_dist).clamp(0.0, 1.0);") | Out-Null
        $out.Add("    let w_centroid = 2.0 * (1.0 - dist);") | Out-Null
        $out.Add("    w_dihedral + w_length + w_centroid") | Out-Null
        $state = 4
        continue
    }
    $out.Add($l) | Out-Null
}
Set-Content -Path $src -Value $out -Encoding UTF8
