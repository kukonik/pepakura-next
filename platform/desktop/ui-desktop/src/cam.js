export function initCam(cw, ch) {
  const pad = 40;
  return { x: 0, y: 0, zoom: Math.min((cw - 2 * pad), (ch - 2 * pad)) };
}

/**
 * Считает камеру под BBOX вершин и размер канвы.
 * Возвращает объект камеры, а НЕ вызывает initCam сам.
 */
export function autoFitCam(vertices, cw, ch) {
  if (!Array.isArray(vertices) || vertices.length === 0) {
    return null;
  }
  let minX = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  for (const v of vertices) {
    if (!v) continue;
    const x = v[0];
    const y = v[1];
    if (!Number.isFinite(x) || !Number.isFinite(y)) continue;
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
  }
  if (!isFinite(minX) || !isFinite(maxX) || !isFinite(minY) || !isFinite(maxY)) {
    return null;
  }

  const width = maxX - minX;
  const height = maxY - minY;
  const pad = Math.max(width, height) * 0.1 || 10;

  const cx = (minX + maxX) / 2;
  const cy = (minY + maxY) / 2;
  const scaleX = (cw - 2 * pad) / width;
  const scaleY = (ch - 2 * pad) / height;
  const zoom = Math.min(scaleX, scaleY);

  return { x: cx, y: cy, zoom };
}

export function applyCam(ctx, cam, cw, ch) {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, cw, ch);
  if (!cam) return;
  ctx.translate(cw / 2, ch / 2);
  ctx.scale(cam.zoom, cam.zoom);
  ctx.translate(-cam.x, -cam.y);
}
