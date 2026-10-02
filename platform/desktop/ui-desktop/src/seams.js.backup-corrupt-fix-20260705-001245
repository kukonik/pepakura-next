export function calcEdgeCounts(faces2D) {
  const edgeCounts = new Map();
  for (const f of faces2D) {
    if (f.length < 3) continue;
    const ev = [[f[0], f[1]], [f[1], f[2]], [f[2], f[0]]];
    for (const [a, b] of ev) {
      const key = Math.min(a, b) + '-' + Math.max(a, b);
      edgeCounts.set(key, (edgeCounts.get(key) || 0) + 1);
    }
  }
  return edgeCounts;
}

export function drawSeams(ctx, vertices2D, face, edgeCounts, mapX, mapY) {
  const ev = [[face[0], face[1]], [face[1], face[2]], [face[2], face[0]]];
  for (const [a, b] of ev) {
    const key = Math.min(a, b) + '-' + Math.max(a, b);
    const isSeam = (edgeCounts.get(key) || 0) === 1;
    ctx.beginPath();
    ctx.moveTo(mapX(vertices2D[a][0]), mapY(vertices2D[a][1]));
    ctx.lineTo(mapX(vertices2D[b][0]), mapY(vertices2D[b][1]));
    ctx.strokeStyle = isSeam ? '#000000' : '#bbbbbb';
    ctx.lineWidth = isSeam ? 2.5 : 0.5;
    ctx.stroke();
  }
}
