$src = "D:\Dev\pepakura-next\platform\desktop\ui-desktop\src\components\SplitView.vue"
$lines = Get-Content $src
$out = [System.Collections.ArrayList]::new()
$state = 0; $skip = 0
foreach ($l in $lines) {
    if ($skip -gt 0) { $skip--; continue }
    if ($state -eq 0 -and $l -match "ctx\.scale\(window\.devicePixelRatio") {
        $out.Add($l) | Out-Null
        $out.Add("        if (!ctx.canvas._cam) {") | Out-Null
        $out.Add("            ctx.canvas._cam = { x: 0, y: 0, zoom: 1, pan: false, sx: 0, sy: 0, init: false };") | Out-Null
        $out.Add("            ctx.canvas.addEventListener('mousedown', (e) => { ctx.canvas._cam.pan = true; ctx.canvas._cam.sx = e.clientX - ctx.canvas._cam.x; ctx.canvas._cam.sy = e.clientY - ctx.canvas._cam.y; });") | Out-Null
        $out.Add("            ctx.canvas.addEventListener('mousemove', (e) => { if (!ctx.canvas._cam.pan) return; ctx.canvas._cam.x = e.clientX - ctx.canvas._cam.sx; ctx.canvas._cam.y = e.clientY - ctx.canvas._cam.sy; });") | Out-Null
        $out.Add("            ctx.canvas.addEventListener('mouseup', () => { ctx.canvas._cam.pan = false; });") | Out-Null
        $out.Add("            ctx.canvas.addEventListener('mouseleave', () => { ctx.canvas._cam.pan = false; });") | Out-Null
        $state = 1
        continue
    }
    if ($state -eq 1 -and $l -match "ctx\.clearRect") {
        $out.Add($l) | Out-Null
        $out.Add("        ctx.canvas.addEventListener('wheel', (e) => {") | Out-Null
        $out.Add("            e.preventDefault();") | Out-Null
        $out.Add("            const c = ctx.canvas._cam;") | Out-Null
        $out.Add("            const rect = ctx.canvas.getBoundingClientRect();") | Out-Null
        $out.Add("            const mx = e.clientX - rect.left; const my = e.clientY - rect.top;") | Out-Null
        $out.Add("            const z = e.deltaY < 0 ? 1.1 : 1/1.1;") | Out-Null
        $out.Add("            const nz = c.zoom * z;") | Out-Null
        $out.Add("            c.x = mx - (mx - c.x) * (nz / c.zoom);") | Out-Null
        $out.Add("            c.y = my - (my - c.y) * (nz / c.zoom);") | Out-Null
        $out.Add("            c.zoom = nz;") | Out-Null
        $out.Add("        }, { passive: false });") | Out-Null
        $out.Add("        const cam = ctx.canvas._cam;") | Out-Null
        $out.Add("        ctx.translate(cam.x, cam.y);") | Out-Null
        $out.Add("        ctx.scale(cam.zoom, -cam.zoom);") | Out-Null
        $state = 2
        continue
    }
    if ($state -eq 2 -and $l -match "const scaleX =") {
        $out.Add("            if (!cam.init) {") | Out-Null
        $out.Add("                const fitX = (width - margin * 2) / Math.max(1e-5, rangeX);") | Out-Null
        $out.Add("                const fitY = (height - margin * 2) / Math.max(1e-5, rangeY);") | Out-Null
        $out.Add("                cam.zoom = Math.min(fitX, fitY);") | Out-Null
        $out.Add("                cam.x = width / 2 - (minX + rangeX / 2) * cam.zoom;") | Out-Null
        $out.Add("                cam.y = height / 2 + (minY + rangeY / 2) * cam.zoom;") | Out-Null
        $out.Add("                cam.init = true;") | Out-Null
        $out.Add("            }") | Out-Null
        $skip = 5
        $state = 3
        continue
    }
    $out.Add($l) | Out-Null
}
$c = $out -join "`r`n"
$c = $c -replace 'mapX\(', '('
$c = $c -replace 'mapY\(', '('
Set-Content -Path $src -Value $c -Encoding UTF8
