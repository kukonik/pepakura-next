$srcPath = 'D:\Dev\pepakura-next\platform\desktop\ui-desktop\src-tauri\src\commands.rs'
$lines = Get-Content $srcPath
$newLines = [System.Collections.ArrayList]::new()
$rotLogic = @(
    'let mut min_x = f64::MAX; let mut max_x = f64::MIN;',
    'let mut min_y = f64::MAX; let mut max_y = f64::MIN;',
    'let mut island_vert_indices = std::collections::HashSet::new();',
    'for (fi, face) in unfolded.faces.iter().enumerate() {',
    '    if unfolded.island_ids[fi] == island_id {',
    '        for &vi in &face.vertices {',
    '            island_vert_indices.insert(vi);',
    '            let v = &unfolded.vertices_2d[vi];',
    '            if v[0] < min_x { min_x = v[0]; } if v[0] > max_x { max_x = v[0]; }',
    '            if v[1] < min_y { min_y = v[1]; } if v[1] > max_y { max_y = v[1]; }',
    '        }',
    '    }',
    '}',
    'let w = max_x - min_x;',
    'let h = max_y - min_y;',
    'let cx = min_x + w / 2.0;',
    'let cy = min_y + h / 2.0;',
    'if h > w {',
    '    for &vi in &island_vert_indices {',
    '        let v = &mut unfolded.vertices_2d[vi];',
    '        let dx = v[0] - cx;',
    '        let dy = v[1] - cy;',
    '        v[0] = cx - dy;',
    '        v[1] = cy + dx;',
    '    }',
    '    island_bboxes.push((island_id, h, w));',
    '} else {',
    '    island_bboxes.push((island_id, w, h));',
    '}'
)
$packLogic = @(
    'let pack_w = 300.0;'
    'island_bboxes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));'
    'let mut current_x = 0.0;'
    'let mut current_y = 0.0;'
    'let mut max_h_in_row = 0.0;'
    'for (target_island_id, island_w, island_h) in &island_bboxes {',
    '    if current_x + island_w > pack_w && current_x > 0.0 {',
    '        current_x = 0.0;'
    '        current_y += max_h_in_row;'
    '        max_h_in_row = 0.0;'
    '    }'
    '    let mut local_min_x = f64::MAX; let mut local_min_y = f64::MAX;'
    '    let mut island_verts_set = std::collections::HashSet::new();'
    '    for (fi, face) in unfolded.faces.iter().enumerate() {',
    '        if unfolded.island_ids[fi] == *target_island_id {',
    '            for &vi in &face.vertices {',
    '                island_verts_set.insert(vi);'
    '                let v = unfolded.vertices_2d[vi];'
    '                if v[0] < local_min_x { local_min_x = v[0]; }'
    '                if v[1] < local_min_y { local_min_y = v[1]; }'
    '            }'
    '        }'
    '    }'
    '    let offset_x = current_x - local_min_x;'
    '    let offset_y = current_y - local_min_y;'
    '    for &vi in &island_verts_set {',
    '        let v = &mut unfolded.vertices_2d[vi];'
    '        v[0] += offset_x;'
    '        v[1] += offset_y;'
    '    }'
    '    current_x += island_w;'
    '    if *island_h > max_h_in_row { max_h_in_row = *island_h; }'
    '}'
)
$state = 'NORMAL'
$declInjected = $false
$packInjected = $false
foreach ($line in $lines) {
    if ($state -eq 'SKIP_OLD_CALL') {
        if ($line -match '\);') { $state = 'NORMAL' }
        continue
    }
    if ($line -match 'fn process_and_pack_island\(') {
        $null = $newLines.Add('# [allow(dead_code)] fn _old_pack(')
        continue
    }
    if ($line -match 'process_and_pack_island\(') {
        $state = 'SKIP_OLD_CALL'
        foreach ($r in $rotLogic) { $null = $newLines.Add($r) }
        continue
    }
    if (-not $declInjected -and $line -match 'for island_id in') {
        $null = $newLines.Add('            let mut island_bboxes: Vec<(usize, f64, f64)> = Vec::new();')
        $declInjected = $true
    }
    if (-not $packInjected -and $line -match 'let elapsed\s*=') {
        foreach ($p in $packLogic) { $null = $newLines.Add($p) }
        $packInjected = $true
    }
    $null = $newLines.Add($line)
}
Set-Content -Path $srcPath -Value $newLines -Encoding UTF8
Write-Host '[INJECT] Файл собран.' -ForegroundColor Green
