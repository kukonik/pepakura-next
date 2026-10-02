$src = "D:\Dev\pepakura-next\platform\desktop\ui-desktop\src-tauri\src\commands.rs"
$lines = Get-Content $src
$out = [System.Collections.ArrayList]::new()
$done = $false
foreach ($l in $lines) {
    if (-not $done -and $l -match "degenerate_islands.len\(\)") {
        $out.Add($l) | Out-Null
        $out.Add("            // POST-PROCESS: 2D Shelf Packing + Rotation") | Out-Null
        $out.Add("            let mut bboxes: std::collections::HashMap<usize, (f64, f64, f64, f64)> = std::collections::HashMap::new();") | Out-Null
        $out.Add("            for (fi, face) in faces_dto.iter().enumerate() {") | Out-Null
        $out.Add("                let id = island_ids[fi];") | Out-Null
        $out.Add("                for &vi in &face.vertices {") | Out-Null
        $out.Add("                    let v = all_vertices_2d[vi];") | Out-Null
        $out.Add("                    let e = bboxes.entry(id).or_insert((f64::MAX, f64::MIN, f64::MAX, f64::MIN));") | Out-Null
        $out.Add("                    if v[0] < e.0 { e.0 = v[0]; } if v[0] > e.1 { e.1 = v[0]; }") | Out-Null
        $out.Add("                    if v[1] < e.2 { e.2 = v[1]; } if v[1] > e.3 { e.3 = v[1]; }") | Out-Null
        $out.Add("                }") | Out-Null
        $out.Add("            }") | Out-Null
        $out.Add("            let mut pack_data: Vec<(usize, f64, f64)> = Vec::new();") | Out-Null
        $out.Add("            for (&id, (min_x, max_x, min_y, max_y)) in &bboxes {") | Out-Null
        $out.Add("                let w = max_x - min_x; let h = max_y - min_y;") | Out-Null
        $out.Add("                let cx = min_x + w / 2.0; let cy = min_y + h / 2.0;") | Out-Null
        $out.Add("                if h > w {") | Out-Null
        $out.Add("                    for (fi, face) in faces_dto.iter().enumerate() {") | Out-Null
        $out.Add("                        if island_ids[fi] == id {") | Out-Null
        $out.Add("                            for &vi in &face.vertices {") | Out-Null
        $out.Add("                                let v = &mut all_vertices_2d[vi];") | Out-Null
        $out.Add("                                let dx = v[0] - cx; let dy = v[1] - cy;") | Out-Null
        $out.Add("                                v[0] = cx - dy; v[1] = cy + dx;") | Out-Null
        $out.Add("                            }") | Out-Null
        $out.Add("                        }") | Out-Null
        $out.Add("                    }") | Out-Null
        $out.Add("                    pack_data.push((id, h, w));") | Out-Null
        $out.Add("                } else {") | Out-Null
        $out.Add("                    pack_data.push((id, w, h));") | Out-Null
        $out.Add("                }") | Out-Null
        $out.Add("            }") | Out-Null
        $out.Add("            pack_data.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));") | Out-Null
        $out.Add("            let pack_w = 300.0; let mut cur_x = 0.0; let mut cur_y = 0.0; let mut max_h = 0.0;") | Out-Null
        $out.Add("            for (id, i_w, i_h) in &pack_data {") | Out-Null
        $out.Add("                if cur_x + i_w > pack_w && cur_x > 0.0 { cur_x = 0.0; cur_y += max_h; max_h = 0.0; }") | Out-Null
        $out.Add("                let mut lx = f64::MAX; let mut ly = f64::MAX;") | Out-Null
        $out.Add("                for (fi, face) in faces_dto.iter().enumerate() {") | Out-Null
        $out.Add("                    if island_ids[fi] == *id {") | Out-Null
        $out.Add("                        for &vi in &face.vertices {") | Out-Null
        $out.Add("                            let v = all_vertices_2d[vi];") | Out-Null
        $out.Add("                            if v[0] < lx { lx = v[0]; } if v[1] < ly { ly = v[1]; }") | Out-Null
        $out.Add("                        }") | Out-Null
        $out.Add("                    }") | Out-Null
        $out.Add("                }") | Out-Null
        $out.Add("                let ox = cur_x - lx; let oy = cur_y - ly;") | Out-Null
        $out.Add("                for (fi, face) in faces_dto.iter().enumerate() {") | Out-Null
        $out.Add("                    if island_ids[fi] == *id {") | Out-Null
        $out.Add("                        for &vi in &face.vertices {") | Out-Null
        $out.Add("                            let v = &mut all_vertices_2d[vi];") | Out-Null
        $out.Add("                            v[0] += ox; v[1] += oy;") | Out-Null
        $out.Add("                        }") | Out-Null
        $out.Add("                    }") | Out-Null
        $out.Add("                }") | Out-Null
        $out.Add("                cur_x += i_w; if *i_h > max_h { max_h = *i_h; }") | Out-Null
        $out.Add("            }") | Out-Null
        $done = $true
        continue
    }
    $out.Add($l) | Out-Null
}
Set-Content -Path $src -Value $out -Encoding UTF8
