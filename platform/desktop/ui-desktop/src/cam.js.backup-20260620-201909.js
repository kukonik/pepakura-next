export function initCam(canvas, triggerDraw) {
    if (canvas._camInit) return;
    canvas._camInit = true;
    canvas.style.cursor = 'grab';
    const cam = { x: 0, y: 0, zoom: 1, pan: false, sx: 0, sy: 0, init: false };
    canvas._cam = cam;
    canvas.addEventListener('mousedown', (e) => {
        cam.pan = true;
        cam.sx = e.clientX - cam.x;
        cam.sy = e.clientY - cam.y;
        canvas.style.cursor = 'grabbing';
    });
    window.addEventListener('mousemove', (e) => {
        if (!cam.pan) return;
        cam.x = e.clientX - cam.sx;
        cam.y = e.clientY - cam.sy;
        triggerDraw();
    });
    window.addEventListener('mouseup', () => {
        cam.pan = false;
        canvas.style.cursor = 'grab';
    });
    canvas.addEventListener('wheel', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const rect = canvas.getBoundingClientRect();
        const mx = e.clientX - rect.left; 
        const my = e.clientY - rect.top;
        const z = e.deltaY < 0 ? 1.1 : 1/1.1;
        const nz = cam.zoom * z;
        cam.x = mx - (mx - cam.x) * (nz / cam.zoom);
        cam.y = my - (my - cam.y) * (nz / cam.zoom);
        cam.zoom = nz;
        triggerDraw();
    }, { passive: false });
}

export function applyCam(ctx, width, height, minX, minY, rangeX, rangeY) {
    const cam = ctx.canvas._cam;
    if (!cam.init) {
        const fitX = width / Math.max(1e-5, rangeX);
        const fitY = height / Math.max(1e-5, rangeY);
        cam.zoom = Math.min(fitX, fitY) * 0.9;
        cam.x = width / 2 - (minX + rangeX / 2) * cam.zoom;
        cam.y = height / 2 + (minY + rangeY / 2) * cam.zoom;
        cam.init = true;
    }
    ctx.translate(cam.x, cam.y);
    ctx.scale(cam.zoom, -cam.zoom);
}
