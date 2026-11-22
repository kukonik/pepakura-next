<template>
  <div style="display: flex; height: 100vh; background: #ffffff; color: #ccc; font-family: sans-serif;">
    <div style="flex: 1; border-right: 1px solid #333;">
      <ModelViewer3D :meshData="objData" :mtlData="mtlData" :textureMap="textureMap" />
    </div>

    <div style="flex: 1; border-right: 1px solid #333; display: flex; flex-direction: column; background: #ffffff;">
      <div style="padding: 10px; border-bottom: 1px solid #333; display: flex; gap: 10px; align-items: center;">
        <span style="font-size: 14px; font-weight: bold;">2D Развёртка</span>
        <label style="font-size: 12px; display: flex; align-items: center; gap: 4px;">
          <input type="checkbox" v-model="showWireframe2D" @change="draw2D" checked> Сетка
        </label>
        <label style="font-size: 12px; display: flex; align-items: center; gap: 4px;">
          <input type="checkbox" v-model="showIslands2D" @change="draw2D" checked> Острова
        </label>
        <label style="font-size: 12px; display: flex; align-items: center; gap: 4px;">
          Части:
          <select v-model="partsMode" style="font-size: 12px; background: #333; color: #ccc; border: 1px solid #555;">
            <option value="auto">авто</option>
            <option value="materials">из файла</option>
            <option value="off">выкл</option>
          </select>
        </label>
      </div>
      <div style="flex: 1; position: relative;">
        <canvas ref="canvas2D" style="width: 100%; height: 100%;"></canvas>
      </div>
    </div>

    <div style="width: 300px; display: flex; flex-direction: column; padding: 20px; background: #252526;">
      <button @click="handleLoadObj" style="padding: 10px; margin-bottom: 10px; cursor: pointer; background: #0e639c; color: white; border: none; width: 100%;">
        Загрузить .obj файл
      </button>
      <button @click="doUnfold" :disabled="!objData || isUnfolding" style="padding: 10px; margin-bottom: 10px; cursor: pointer; background: #333; color: white; border: 1px solid #555; width: 100%;">
        {{ isUnfolding ? 'Обработка...' : 'Развернуть (MST-Seam)' }}
      </button>
      <button @click="callOrchestrator" :disabled="!objData || isAiThinking" style="padding: 10px; margin-bottom: 20px; cursor: pointer; background: #5a1e1e; color: white; border: 1px solid #771e1e; width: 100%;">
        {{ isAiThinking ? 'AI думает...' : '🤖 AI Анализ геометрии' }}
      </button>
<button @click="handleExport" style="padding: 10px; margin-bottom: 10px; cursor: pointer; background: #0e639c; color: white; border: none; width: 100%;">📄 Экспорт PDF</button>
<button @click="exportPdfPages55" style="padding: 10px; margin-bottom: 20px; cursor: pointer; background: #1e80ff; color: white; border: none; width: 100%;">🖨 PDF текстурный (A4)</button>

      <div style="flex: 1; overflow: auto; background: #ffffff; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 12px;">
        <div v-if="rustResult" style="color: #9cdcfe; margin-bottom: 10px; white-space: pre-wrap;">{{ rustResult }}</div>
        <div v-if="aiError" style="color: #f48771; margin-bottom: 10px; white-space: pre-wrap;">AI Ошибка: {{ aiError }}</div>
        <div v-if="aiResult" style="color: #b5cea8; margin-bottom: 10px; white-space: pre-wrap;">🤖 AI Ответ:\n{{ aiResult }}</div>
        <div v-if="!objData" style="color: #666;">Файл не загружен</div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { initCam, applyCam, autoFitCam } from '../cam.js';

// import { calcEdgeCounts, drawSeams } from '../seams.js';
import { unref,  ref, nextTick, onMounted, watch, reactive } from 'vue';
const unfoldData = ref(null);
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { readTextFile, readFile } from '@tauri-apps/plugin-fs';
import ModelViewer3D from './ModelViewer3D.vue';

function arrayBufferToBase64(buffer) {
  let binary = '';
  const bytes = new Uint8Array(buffer);
  for (let i = 0; i < bytes.byteLength; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
}


const objData = ref('');
const rustResult = ref('');
const aiResult = ref('');
const aiError = ref('');
const isUnfolding = ref(false);
const isAiThinking = ref(false);

const canvas2D = ref<HTMLCanvasElement | null>(null);
const mtlData = ref<string | null>(null)
const textureMap = ref<Record<string, string>>({})
const showWireframe2D = ref(true);
const showIslands2D = ref(true);
const partsMode = ref('auto'); // [82-A] режим разбиения на части

let vertices2D: number[][] = [];
let faces2D: number[][] = [];
let islandIds: number[] = [];
let degenerateIslands: number[] = [];

let lastMetrics = {
  vertex_count: 0,
  face_count: 0,
  islands: 0,
  total_area: 0
};

const handleLoadObj = async () => {
  objData.value = '';
  rustResult.value = '';
  aiError.value = '';
  aiResult.value = '';
  try {
    const filePath = await open({
      multiple: false,
      filters: [{ name: '3D Models', extensions: ['obj'] }]
    });
    if (!filePath) return;
    const text = await readTextFile(filePath as string);
    objData.value = text;
      console.log("[VUE LOAD] readTextFile returned. Length:", text?.length, "Snippet:", text?.substring(0, 100));
    const mtlMatch = text.match(/mtllib\s+(.+)/);
    if (mtlMatch) {
        const objDir = (filePath as string).substring(
          0,
          Math.max(
            (filePath as string).lastIndexOf('\\'),
            (filePath as string).lastIndexOf('/')
          ) + 1
        );
      const mtlPath = objDir + mtlMatch[1].trim();
      try {
        const mtlText = await readTextFile(mtlPath);
        const lines = mtlText.split(/\r?\n/);
        let currentMtl: string | null = null;
        const texCache: Record<string, string>  = reactive({});
        for (const rawLine of lines) {
          const line = rawLine.trim();
          if (!line || line.startsWith('#')) continue;
          const newMtlMatch = line.match(/^newmtl\s+(.+)/i);
          if (newMtlMatch) {
            currentMtl = newMtlMatch[1].trim();
            continue;
          }
          const mapMatch = line.match(/^map_Kd\s+(.+)/i);
          if (mapMatch && currentMtl) {
            const texName = mapMatch[1].trim();
            if (!texCache[texName]) {
              const texPath = objDir + texName;
              const texBytes = await readFile(texPath);
              const tb = texBytes as Uint8Array;
              const mime = (tb.length > 2 && tb[0] === 0xFF && tb[1] === 0xD8) ? 'image/jpeg' : 'image/png'; // FIX40
              texCache[texName] = 'data:' + mime + ';base64,' + arrayBufferToBase64(texBytes);
            }
            textureMap.value[currentMtl] = texCache[texName];
          }
        }
        mtlData.value = mtlText;

      } catch (e) {
        console.error('Ошибка загрузки MTL/текстур:', e);
      }
    }
    rustResult.value = 'Файл загружен. Длина: ' + text.length + ' символов';
  } catch (e: any) {
      rustResult.value = 'Ошибка загрузки: ' + e.toString();
  }
};

const doUnfold = async () => {
  if (!objData.value) return;
  isUnfolding.value = true;
  rustResult.value = '';
  aiError.value = '';
  aiResult.value = '';

  try {
    rustResult.value = 'Выполняю MST-Seam развёртку...\n';

    const currentObjData = String(objData.value || '');

    console.log('[VUE DEBUG] objData type:', typeof objData.value, "len:", objData.value?.length, "val:", objData.value?.substring(0, 50));
  (window as any).__unfoldT055 = performance.now(); (window as any).__unfoldMs55 = 0; // [SPD-1] таймер развёртки
    const r = await invoke('unfold_mesh', {
  objData: currentObjData,
  config: { algorithm: 'smart', parts: { mode: partsMode.value, concave_deg: 45 } } // [82-A]
});
unfoldData.value = r;
    console.log("[RUST RAW RESPONSE]", JSON.stringify(r)?.substring(0, 1500));
    if (true) {
      // lastMetrics deprecated
      rustResult.value += '✅ Развёртка выполнена\nДвижок: ' + r.engine_used + '\nДеталей: ' + (r.pieces?.length || 0) + '\nЧасти: ' + (((r.parts || []).map(pp => pp.name + ' [' + pp.face_count + 'гр]').join(', ')) || '—') + '\nВершин 2D: ' + (r.vertices_2d?.length || 0) + '\nГраней: ' + (r.faces?.length || 0);
      vertices2D = r.vertices_2d || [];
        faces2D = r.faces_2d || [];
        islandIds = r.face_pieces || [];
        (r.vertices_2d || []).forEach((v: any) => { v[1] = -v[1]; }); // [TEX-M] ядро отдаёт y вверх (unfold_hinge n2=[-uy,ux]), рендер рисует y вниз — разворачиваем данные один раз при загрузке
        (r.pieces || []).forEach((p: any) => { (p.outline_2d || []).forEach((v: any) => { v[1] = -v[1]; }); (p.cut_lines || []).forEach((s: any) => { s[0][1] = -s[0][1]; s[1][1] = -s[1][1]; }); (p.fold_lines || []).forEach((s: any) => { s[0][1] = -s[0][1]; s[1][1] = -s[1][1]; }); }); // [TEX-M]
        buildTexFaces(unref(currentObjData)); // [TEX-A] UV из OBJ (до раскладки)
        pieces2D = layoutPieces78(r.pieces || []); // [80-A] раскладка полками
        loadTexImages(); // [TEX-A] картинки + текстурный слой
        lastMetrics = { vertex_count: (r.vertices_2d || []).length, face_count: (r.faces || []).length, islands: new Set(r.face_pieces || []).size, total_area: 0 }; // FIX40
        view2D.scale = 1; view2D.offsetX = 0; view2D.offsetY = 0; view2D.autoFitDone = true;
      // islandIds deprecated
      // degenerateIslands deprecated
      await nextTick();
draw2D();
    } else {
        rustResult.value = r.message;
    }
  } catch (e: any) {
      rustResult.value = 'Ошибка Rust:\n' + e.toString();
  } finally { isUnfolding.value = false; }
};

const islandColors = [
  'rgba(0, 122, 204, 0.5)', 'rgba(204, 0, 122, 0.5)', 'rgba(122, 204, 0, 0.5)',
  'rgba(204, 122, 0, 0.5)', 'rgba(0, 204, 204, 0.5)', 'rgba(204, 204, 0, 0.5)',
  'rgba(122, 0, 204, 0.5)', 'rgba(0, 204, 122, 0.5)', 'rgba(204, 80, 80, 0.5)',
  'rgba(80, 204, 80, 0.5)'
];

let pieces2D = [];
let view2D = { scale: 1, offsetX: 0, offsetY: 0, isDragging: false, lastX: 0, lastY: 0 };
let is2DInteractive = false;

const handle2DWheel = (e) => {
  e.preventDefault();
  e.stopPropagation();
  const canvas = canvas2D.value as HTMLCanvasElement;
  if (!canvas) return;
  const rect = canvas.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;
  const delta = e.deltaY > 0 ? 0.9 : 1.1;
  const newScale = Math.max(0.1, Math.min(view2D.scale * delta, 50));
  const scaleRatio = newScale / view2D.scale;
  view2D.offsetX = mouseX - (mouseX - view2D.offsetX) * scaleRatio;
  view2D.offsetY = mouseY - (mouseY - view2D.offsetY) * scaleRatio;
  view2D.scale = newScale;
  draw2D();
};

const handle2DMouseDown = (e) => {
  view2D.isDragging = true;
  view2D.lastX = e.clientX;
  view2D.lastY = e.clientY;
};

const handle2DMouseMove = (e) => {
  if (!view2D.isDragging) return;
  view2D.offsetX += e.clientX - view2D.lastX;
  view2D.offsetY += e.clientY - view2D.lastY;
  view2D.lastX = e.clientX;
  view2D.lastY = e.clientY;
  draw2D();
};

const handle2DMouseUp = () => {
  view2D.isDragging = false;
};

// ===== [TEX-A] текстурирование 2D-развёртки по UV исходного OBJ =====
let texFaces: any[] = [];
let texReady: any = false;
let texImgs: any = new Map();
let texLayer: any = null;
let texMtlMap: any = new Map();
let texMtlParsed: any = false;
const texMapGet = (): any => { try { const v: any = unref(textureMap); return (v && typeof v === "object") ? v : null; } catch (e) { return null; } };
const texNormKey = (s: any) => String(s || "").toLowerCase().replace(/^.*[\\\/]/, "").replace(/\.[a-z0-9]+$/, "").trim();
function parseMtlMapKd(mtlText: any): any {
  const map: any = new Map(); let cur: any = "";
  if (typeof mtlText !== "string") { return map; }
  mtlText.split(/\r?\n/).forEach((raw: any) => { const line: any = String(raw).trim(); if (line.startsWith("newmtl ")) { cur = line.slice(7).trim(); } else if (line.startsWith("map_Kd") && cur) { const f: any = line.slice(6).trim().split(/\s+/).pop(); if (f) { map.set(cur, String(f).replace(/^.*[\\\/]/, "")); } } });
  return map;
}
function resolveTexUrl(mat: any): any {
  const tm: any = texMapGet();
  if (!tm || !mat) { return null; }
  if (!texMtlParsed) { texMtlParsed = true; texMtlMap = parseMtlMapKd(unref(mtlData)); }
  const keys: any = Object.keys(tm);
  if (tm[mat]) { return tm[mat]; }
  const file: any = texMtlMap.get(mat);
  if (file) { if (tm[file]) { return tm[file]; } const f2: any = keys.find((k: any) => texNormKey(k) === texNormKey(file)); if (f2) { return tm[f2]; } }
  const m: any = texNormKey(mat);
  let hit: any = keys.find((k: any) => texNormKey(k) === m);
  if (!hit) { hit = keys.find((k: any) => { const nk: any = texNormKey(k); return nk && (m.indexOf(nk) >= 0 || nk.indexOf(m) >= 0); }); }
  return hit ? tm[hit] : null;
}
function buildTexFaces(objText: any) {
  texFaces = []; texReady = false;
  try {
    if (typeof objText !== "string" || !objText.length) { console.warn("[TEX-A] нет текста OBJ"); return; }
    if (!faces2D || !faces2D.length) { console.warn("[TEX-A] faces2D пуст"); return; }
    const vt: any[] = []; const out: any[] = []; let mat: any = "";
    objText.split(/\r?\n/).forEach((raw: any) => {
      const line: any = String(raw).trim();
      if (line.startsWith("vt ")) { const p: any = line.slice(3).trim().split(/\s+/); vt.push([parseFloat(p[0]) || 0, parseFloat(p[1]) || 0]); }
      else if (line.startsWith("usemtl ")) { mat = line.slice(7).trim(); }
      else if (line.startsWith("f ")) {
        const vv: any = line.slice(2).trim().split(/\s+/).map((tok: any) => { const q: any = tok.split("/"); if (q.length < 2 || !q[1]) { return null; } let t: any = parseInt(q[1], 10); if (isNaN(t)) { return null; } if (t < 0) { t = vt.length + t; } return t - 1; });
        for (let k = 1; k + 1 < vv.length; k++) { const ix: any = [vv[0], vv[k], vv[k + 1]]; const okq: any = ix.every((z: any) => z !== null && z >= 0 && z < vt.length); out.push(okq ? { uv: ix.map((z: any) => [vt[z][0], vt[z][1]]), mat: mat } : { uv: null, mat: mat }); }
      }
    });
    texFaces = out;
    texReady = vt.length > 0 && out.length === faces2D.length && out.some((t: any) => !!t.uv);
    if (!texReady) { console.warn("[TEX-A] UV отключены: vt=" + vt.length + " objFaces=" + out.length + " unfoldFaces=" + faces2D.length); } else { console.log("[TEX-A] UV готовы: граней=" + out.length + " vt=" + vt.length); }
  } catch (e) { console.warn("[TEX-A] ошибка разбора OBJ: " + e); }
}
function drawTexTri(g: any, img: any, src: any, dst: any) {
  const dx1: any = src[1][0] - src[0][0]; const dy1: any = src[1][1] - src[0][1]; const dx2: any = src[2][0] - src[0][0]; const dy2: any = src[2][1] - src[0][1];
  const det: any = dx1 * dy2 - dx2 * dy1;
  if (!isFinite(det) || Math.abs(det) < 1e-9) { return; }
  const du1: any = dst[1][0] - dst[0][0]; const dv1: any = dst[1][1] - dst[0][1]; const du2: any = dst[2][0] - dst[0][0]; const dv2: any = dst[2][1] - dst[0][1];
  const a: any = (du1 * dy2 - du2 * dy1) / det; const c: any = (dx1 * du2 - dx2 * du1) / det;
  const b: any = (dv1 * dy2 - dv2 * dy1) / det; const d: any = (dx1 * dv2 - dx2 * dv1) / det;
  const e: any = dst[0][0] - a * src[0][0] - c * src[0][1]; const f: any = dst[0][1] - b * src[0][0] - d * src[0][1];
  g.save(); g.beginPath(); g.moveTo(dst[0][0], dst[0][1]); g.lineTo(dst[1][0], dst[1][1]); g.lineTo(dst[2][0], dst[2][1]); g.closePath(); g.clip();
  g.transform(a, b, c, d, e, f); g.drawImage(img, 0, 0); g.restore();
}
function buildTexLayer() {
  texLayer = null;
  if (!texReady || !faces2D || !faces2D.length || !vertices2D || !vertices2D.length) { return; }
  let minX: any = Infinity; let minY: any = Infinity; let maxX: any = -Infinity; let maxY: any = -Infinity;
  faces2D.forEach((f: any) => { if (!f) { return; } f.forEach((vi: any) => { const v: any = vertices2D[vi]; if (!v || !isFinite(v[0]) || !isFinite(v[1])) { return; } if (v[0] < minX) { minX = v[0]; } if (v[0] > maxX) { maxX = v[0]; } if (v[1] < minY) { minY = v[1]; } if (v[1] > maxY) { maxY = v[1]; } }); });
  if (!isFinite(minX) || !isFinite(minY)) { return; }
  const spanX: any = Math.max(1e-9, maxX - minX); const spanY: any = Math.max(1e-9, maxY - minY);
  const ppmm: any = Math.min(1024, 4096 / spanX, 4096 / spanY); // [TEX-L] cap 1024 px/юнит — старый cap 16 давал слой 90x45px
  const cnv: any = document.createElement("canvas");
  cnv.width = Math.max(1, Math.min(4096, Math.ceil(spanX * ppmm))); cnv.height = Math.max(1, Math.min(4096, Math.ceil(spanY * ppmm))); // [TEX-R] слой 4K
  const g: any = cnv.getContext("2d"); g.imageSmoothingEnabled = true; g.imageSmoothingQuality = "high";
  const lx: any = (x: any) => (x - minX) * ppmm; const ly: any = (y: any) => (y - minY) * ppmm;
  const inflate = (tri: any, px: any) => { const cx: any = (tri[0][0] + tri[1][0] + tri[2][0]) / 3; const cy: any = (tri[0][1] + tri[1][1] + tri[2][1]) / 3; return tri.map((p: any) => { const vx: any = p[0] - cx; const vy: any = p[1] - cy; const len: any = Math.hypot(vx, vy) || 1; const k: any = (len + px) / len; return [cx + vx * k, cy + vy * k]; }); };
  let drawn: any = 0;
  let mir: any = 0; // [TEX-E1] счётчик зеркальных граней (det(src)*det(dst)<0)
  for (let i = 0; i < faces2D.length; i++) {
    const tf: any = texFaces[i];
    if (!tf || !tf.uv || !tf.mat) { continue; }
    const url: any = texImgs.get("m:" + tf.mat); const img: any = url ? texImgs.get("u:" + url) : null;
    if (!img || !img.complete || !img.naturalWidth) { continue; }
    const f: any = faces2D[i];
    const va: any = vertices2D[f[0]]; const vb: any = vertices2D[f[1]]; const vc: any = vertices2D[f[2]];
    if (!va || !vb || !vc) { continue; }
    const iw: any = img.naturalWidth; const ih: any = img.naturalHeight;
    const src: any = tf.uv.map((p: any) => [p[0] * iw, (1 - p[1]) * ih]);
    const dst: any = inflate([[lx(va[0]), ly(va[1])], [lx(vb[0]), ly(vb[1])], [lx(vc[0]), ly(vc[1])]], 0.6);
    const s55: any = (src[1][0] - src[0][0]) * (src[2][1] - src[0][1]) - (src[2][0] - src[0][0]) * (src[1][1] - src[0][1]); const d55: any = (dst[1][0] - dst[0][0]) * (dst[2][1] - dst[0][1]) - (dst[2][0] - dst[0][0]) * (dst[1][1] - dst[0][1]); if (s55 * d55 < 0) { mir++; } // [TEX-E2] отражение UV->2D
    drawTexTri(g, img, src, dst); drawn++;
  }
  texLayer = { canvas: cnv, minX: minX, minY: minY, w: spanX, h: spanY, drawn: drawn, mir: mir }; // [TEX-E3a]
  console.log("[TEX-A] слой: " + drawn + "/" + faces2D.length + " граней, " + cnv.width + "x" + cnv.height + "px");
  console.log("[TEX-E3b] зеркальных граней: " + mir + "/" + drawn + " (если > 0 — в модели зеркальные UV-копии, текстура на них отражена)"); // [TEX-E3b]
}
function scheduleTex55() { if ((window as any).__texQ55) { return; } (window as any).__texQ55 = 1; setTimeout(() => { (window as any).__texQ55 = 0; buildTexLayer(); draw2D(); }, 120); } // [TEX-Q] один прогон слоя на пачку onload вместо N+1
function loadTexImages() {
  if (!texReady) { buildTexLayer(); return; }
  const mats: any = new Map();
  texFaces.forEach((t: any) => { if (t && t.mat && !mats.has(t.mat)) { mats.set(t.mat, true); } });
  mats.forEach((_v: any, mat: any) => {
    let url: any = resolveTexUrl(mat); // [TEX-D] let — для fallback атласа
    if (!url) { const tk: any = texMapGet(); const kk: any = tk ? Object.keys(tk) : []; if (kk.length === 1) { url = tk[kk[0]]; console.warn("[TEX-D] материал " + mat + " без своей текстуры — беру единственную (атлас): " + kk[0]); } else { console.warn("[TEX-A] материал без текстуры: " + mat); return; } }
    texImgs.set("m:" + mat, url);
    if (texImgs.get("u:" + url) === undefined) {
      const im: any = new Image(); texImgs.set("u:" + url, im);
      im.onload = () => { scheduleTex55(); };
      im.onerror = () => { console.warn("[TEX-A] картинка не загрузилась: " + url); };
      im.src = url;
    }
  });
  const noTex55: any = []; const seen55: any = new Map(); texFaces.forEach((t: any) => { if (t && t.mat && !seen55.has(t.mat)) { seen55.set(t.mat, true); if (!texImgs.has("m:" + t.mat)) { noTex55.push(t.mat); } } }); // [TEX-F] материалы без текстуры
  console.log("[TEX-F] без текстуры: " + noTex55.length + " " + JSON.stringify(noTex55) + " | ключи textureMap: " + JSON.stringify(Object.keys(texMapGet() || {})) + " | map_Kd: " + JSON.stringify(Array.from(texMtlMap.entries()))); // [TEX-F]
  scheduleTex55(); // [TEX-Q]
}
// ===== [PDF-B] текстурированный многостраничный PDF (A4, 200 dpi) =====
const PDF55_DPI: any = 300; // [PDF-D] 300 DPI — резче линии и текстура на печати (страница 2480x3508)
const PDF55_PXMM: any = PDF55_DPI / 25.4;
const PDF55_PATH: any = "G:\\Dev\\pepakura-next\\unfold_tex.pdf";
function pdf55PackPieces(): any {
  const items: any[] = [];
  (pieces2D || []).forEach((p: any, pi: any) => {
    let minX: any = Infinity; let minY: any = Infinity; let maxX: any = -Infinity; let maxY: any = -Infinity;
    const up = (x: any, y: any) => { if (typeof x !== "number" || typeof y !== "number" || !isFinite(x) || !isFinite(y)) { return; } if (x < minX) { minX = x; } if (x > maxX) { maxX = x; } if (y < minY) { minY = y; } if (y > maxY) { maxY = y; } };
    (p.outline_2d || []).forEach((v: any) => { up(v[0], v[1]); });
    (p.cut_lines || []).forEach((s: any) => { up(s[0][0], s[0][1]); up(s[1][0], s[1][1]); });
    (p.fold_lines || []).forEach((s: any) => { up(s[0][0], s[0][1]); up(s[1][0], s[1][1]); });
    if (!isFinite(minX)) { return; }
    items.push({ p: p, pi: pi, minX: minX, minY: minY, w: Math.max(1e-6, maxX - minX), h: Math.max(1e-6, maxY - minY), wmm: 0, hmm: 0, x: 0, y: 0 });
  });
  if (!items.length) { return null; }
  let maxW: any = 0; let maxH: any = 0;
  items.forEach((it: any) => { if (it.w > maxW) { maxW = it.w; } if (it.h > maxH) { maxH = it.h; } });
  const mmPerUnit: any = Math.max(2, Math.min(400, Math.min(170 / maxW, 270 / maxH)));
  items.forEach((it: any) => { it.wmm = it.w * mmPerUnit; it.hmm = it.h * mmPerUnit; });
  items.sort((a: any, b: any) => b.hmm - a.hmm);
  const PW: any = 190; const PH: any = 277;
  const pgs: any[] = [];
  let cur: any = null; let cx: any = 0; let cy: any = 0; let rowH: any = 0;
  items.forEach((it: any) => {
    if (it.wmm > PW) { it.wmm = PW; }
    if (cur && cx > 0 && cx + it.wmm > PW) { cx = 0; cy += rowH + 4; rowH = 0; }
    if (cur && cur.items.length > 0 && cy + it.hmm > PH) { cur = null; }
    if (!cur) { cur = { items: [] }; pgs.push(cur); cx = 0; cy = 0; rowH = 0; }
    it.x = cx; it.y = cy;
    cur.items.push(it);
    cx += it.wmm + 4; if (it.hmm > rowH) { rowH = it.hmm; }
  });
  return { pgs: pgs, mmPerUnit: mmPerUnit };
}
function pdf55FaceGroups(): any {
  // [TEX-G] face groups from islandIds+faces2D (same source as screen): fixes white eye pieces in PDF
  const groups: any = new Map();
  if (islandIds && islandIds.length && faces2D && faces2D.length) {
    for (let fi = 0; fi < Math.min(islandIds.length, faces2D.length); fi++) {
      const k: any = islandIds[fi];
      if (!groups.has(k)) { groups.set(k, []); }
      groups.get(k).push(fi);
    }
  }
  const idVals: any = new Set(islandIds || []);
  (pieces2D || []).forEach((p: any, i: any) => {
    p.__k55 = idVals.has(p.id) ? p.id : (idVals.has(p.part_id) ? p.part_id : i);
    if (!groups.has(p.__k55)) { groups.set(p.__k55, []); }
    const g: any = groups.get(p.__k55);
    if (g.length === 0 && p.face_ids && p.face_ids.length) { p.face_ids.forEach((f2: any) => g.push(f2)); }
  });
  let gsum55 = 0;
  groups.forEach((v: any) => { gsum55 += v.length; });
  console.log("[TEX-G] граней в группах PDF: " + gsum55 + "/" + (faces2D ? faces2D.length : 0) + " | деталей с пустыми группами: " + (pieces2D || []).filter((p: any) => !groups.get(p.__k55) || groups.get(p.__k55).length === 0).length);
  return groups;
}
function pdf55DrawPiece(g: any, it: any, ox: any, oy: any, mmpu: any, groups: any) {
  const X: any = (x: any) => ox + (x - it.minX) * mmpu * PDF55_PXMM;
  const Y: any = (y: any) => oy + (y - it.minY) * mmpu * PDF55_PXMM;
  const outline: any = it.p.outline_2d || [];
  if (outline.length >= 3) {
    g.beginPath(); g.moveTo(X(outline[0][0]), Y(outline[0][1]));
    for (let i = 1; i < outline.length; i++) { g.lineTo(X(outline[i][0]), Y(outline[i][1])); }
    g.closePath(); g.fillStyle = "#ffffff"; g.fill(); g.strokeStyle = "#b0b0b0"; g.lineWidth = 1; g.stroke();
  }
  g.save(); if (outline.length >= 3) { g.beginPath(); g.moveTo(X(outline[0][0]), Y(outline[0][1])); for (let i2 = 1; i2 < outline.length; i2++) { g.lineTo(X(outline[i2][0]), Y(outline[i2][1])); } g.closePath(); g.clip(); } // [TEX-C] текстура клипится по контуру детали — не заходит на клапаны
  const arr: any = (it.p.__k55 === null || it.p.__k55 === undefined) ? null : groups.get(it.p.__k55);
  if (arr) {
    arr.forEach((fi: any) => {
      const tf: any = texFaces[fi];
      if (!tf || !tf.uv || !tf.mat) { return; }
      const url: any = texImgs.get("m:" + tf.mat); const img: any = url ? texImgs.get("u:" + url) : null;
      if (!img || !img.complete || !img.naturalWidth) { return; }
      const f: any = faces2D[fi]; if (!f) { return; }
      const va: any = vertices2D[f[0]]; const vb: any = vertices2D[f[1]]; const vc: any = vertices2D[f[2]];
      if (!va || !vb || !vc) { return; }
      const iw: any = img.naturalWidth; const ih: any = img.naturalHeight;
      const src: any = tf.uv.map((q: any) => [q[0] * iw, (1 - q[1]) * ih]);
      const dst: any = [[X(va[0]), Y(va[1])], [X(vb[0]), Y(vb[1])], [X(vc[0]), Y(vc[1])]];
      drawTexTri(g, img, src, dst);
    });
  }
  g.restore(); // [TEX-C] конец клипа текстуры
  g.strokeStyle = "#e02020"; g.lineWidth = 2.2; g.setLineDash([]); g.beginPath();
  (it.p.cut_lines || []).forEach((s: any) => { g.moveTo(X(s[0][0]), Y(s[0][1])); g.lineTo(X(s[1][0]), Y(s[1][1])); });
  g.stroke();
  g.strokeStyle = "#2060e0"; g.lineWidth = 1.6; g.setLineDash([6, 4]); g.beginPath();
  (it.p.fold_lines || []).forEach((s: any) => { g.moveTo(X(s[0][0]), Y(s[0][1])); g.lineTo(X(s[1][0]), Y(s[1][1])); });
  g.stroke(); g.setLineDash([]);
}
async function exportPdfPages55() {
  try {
    if (!pieces2D || !pieces2D.length) { alert("PDF: \u0441\u043D\u0430\u0447\u0430\u043B\u0430 \u0432\u044B\u043F\u043E\u043B\u043D\u0438\u0442\u0435 \u0440\u0430\u0437\u0432\u0451\u0440\u0442\u043A\u0443"); return; }
    const groups: any = pdf55FaceGroups();
    const MARG: any = 10;
    const jpgs: any[] = [];
    const pi55: any = (window as any).__pageInfo55;
    if (pi55 && pi55.list && pi55.list.length && pi55.mmpu && pi55.w && pi55.h) {
      // [PDF-D] snapshot of each preview sheet: page = sheet (WYSIWYG), fill% in page label
      const ti55: any = (window as any).__tileInfo55;
      const fillTxt: any = (ti55 && typeof ti55.fill === "number") ? ("  |  \u0437\u0430\u043F\u043E\u043B\u043D\u0435\u043D\u0438\u0435 " + ti55.fill + "%") : "";
      for (let k = 0; k < pi55.list.length; k++) {
        const ox: any = pi55.list[k].x, oy: any = pi55.list[k].y;
        const cnv: any = document.createElement("canvas");
        cnv.width = Math.round(210 * PDF55_PXMM); cnv.height = Math.round(297 * PDF55_PXMM);
        const g: any = cnv.getContext("2d");
        g.fillStyle = "#ffffff"; g.fillRect(0, 0, cnv.width, cnv.height);
        g.imageSmoothingEnabled = true; g.imageSmoothingQuality = "high";
        const X: any = (x: any) => (MARG + (x - ox) * pi55.mmpu) * PDF55_PXMM;
        const Y: any = (y: any) => (MARG + (y - oy) * pi55.mmpu) * PDF55_PXMM;
        // [PDF-D] frames: sheet border solid + pieces zone 170x270 dashed
        g.strokeStyle = "#c8c8c8"; g.lineWidth = 1; g.setLineDash([5, 4]);
        g.strokeRect(MARG * PDF55_PXMM, MARG * PDF55_PXMM, 170 * PDF55_PXMM, 270 * PDF55_PXMM);
        g.setLineDash([]);
        g.strokeRect(0.5, 0.5, cnv.width - 1, cnv.height - 1);
        // [PDF-D] ruler 100 mm drawn BEFORE pieces (never covers them)
        g.strokeStyle = "#808080"; g.lineWidth = 2; g.beginPath();
        const ry55: any = cnv.height - Math.round(16 * PDF55_PXMM); const rx55: any = cnv.width / 2 - 50 * PDF55_PXMM;
        g.moveTo(rx55, ry55); g.lineTo(rx55 + 100 * PDF55_PXMM, ry55);
        g.moveTo(rx55, ry55 - 6); g.lineTo(rx55, ry55 + 6);
        g.moveTo(rx55 + 100 * PDF55_PXMM, ry55 - 6); g.lineTo(rx55 + 100 * PDF55_PXMM, ry55 + 6);
        g.stroke(); g.fillStyle = "#808080"; g.font = "20px sans-serif"; g.textAlign = "left";
        g.fillText("100 \u043C\u043C", rx55 + 100 * PDF55_PXMM + 10, ry55 + 7);
        pieces2D.forEach((p: any) => {
          let mnx = Infinity, mny = Infinity, mxx = -Infinity, mxy = -Infinity;
          const up9: any = (x: any, y: any) => { if (x < mnx) mnx = x; if (x > mxx) mxx = x; if (y < mny) mny = y; if (y > mxy) mxy = y; };
          (p.outline_2d || []).forEach((v: any) => up9(v[0], v[1]));
          (p.cut_lines || []).forEach((s: any) => { up9(s[0][0], s[0][1]); up9(s[1][0], s[1][1]); });
          (p.fold_lines || []).forEach((s: any) => { up9(s[0][0], s[0][1]); up9(s[1][0], s[1][1]); });
          if (!isFinite(mnx)) { return; }
          if (mxx < ox - 1e-9 || mnx > ox + pi55.w + 1e-9 || mxy < oy - 1e-9 || mny > oy + pi55.h + 1e-9) { return; }
          pdf55DrawPieceWorld(g, p, X, Y, groups);
          // [PDF-D] piece label at top-left of its bbox
          g.fillStyle = "#707070"; g.font = "14px sans-serif"; g.textAlign = "left";
          g.fillText("#" + p.id + (p.part ? " " + p.part : ""), X(mnx), Y(mny) - 4);
        });
        g.fillStyle = "#909090"; g.font = "22px sans-serif"; g.textAlign = "center";
        g.fillText("\u0441\u0442\u0440. " + (k + 1) + "/" + pi55.list.length + "  |  \u043C\u0430\u0441\u0448\u0442\u0430\u0431 " + pi55.mmpu.toFixed(2) + " \u043C\u043C \u043D\u0430 \u0435\u0434\u0438\u043D\u0438\u0446\u0443 \u043C\u043E\u0434\u0435\u043B\u0438" + fillTxt, cnv.width / 2, cnv.height - Math.round(6 * PDF55_PXMM));
        jpgs.push(cnv.toDataURL("image/jpeg", 0.95)); // [PDF-D] quality 0.95
      }
    } else {
      // [PDF-D] fallback: old packing if __pageInfo55 not found
      const pack: any = pdf55PackPieces();
      if (!pack) { alert("PDF: \u043D\u0435\u0442 \u0434\u0435\u0442\u0430\u043B\u0435\u0439"); return; }
      for (let pg = 0; pg < pack.pgs.length; pg++) {
        const cnv: any = document.createElement("canvas");
        cnv.width = Math.round(210 * PDF55_PXMM); cnv.height = Math.round(297 * PDF55_PXMM);
        const g: any = cnv.getContext("2d");
        g.fillStyle = "#ffffff"; g.fillRect(0, 0, cnv.width, cnv.height);
        g.imageSmoothingEnabled = true; g.imageSmoothingQuality = "high";
        pack.pgs[pg].items.forEach((it: any) => {
          pdf55DrawPiece(g, it, (MARG + it.x) * PDF55_PXMM, (MARG + it.y) * PDF55_PXMM, pack.mmPerUnit, groups);
        });
        g.fillStyle = "#909090"; g.font = "22px sans-serif"; g.textAlign = "center";
        g.fillText("\u0441\u0442\u0440. " + (pg + 1) + "/" + pack.pgs.length + "  |  \u043C\u0430\u0441\u0448\u0442\u0430\u0431 " + pack.mmPerUnit.toFixed(2) + " \u043C\u043C \u043D\u0430 \u0435\u0434\u0438\u043D\u0438\u0446\u0443 \u043C\u043E\u0434\u0435\u043B\u0438", cnv.width / 2, cnv.height - Math.round(6 * PDF55_PXMM));
        jpgs.push(cnv.toDataURL("image/jpeg", 0.95));
      }
      console.log("[PDF-D] fallback packing (no __pageInfo55)");
    }
    const r: any = await invoke("export_pdf", { vertices: [], faces: [], islandIds: null, path: PDF55_PATH, pages: jpgs });
    console.log("[PDF-B] " + r);
    alert("PDF \u0441\u043E\u0445\u0440\u0430\u043D\u0451\u043D: " + PDF55_PATH + "\n\u0441\u0442\u0440\u0430\u043D\u0438\u0446: " + jpgs.length);
  } catch (e) { console.error("[PDF-B] \u043E\u0448\u0438\u0431\u043A\u0430: " + e); alert("PDF \u043E\u0448\u0438\u0431\u043A\u0430: " + e); }
}
(window as any).__pdf55export = exportPdfPages55;
if (false) { // [PDF-C] плавающая кнопка отключена — кнопка перенесена в панель
  (window as any).__pdf55wired = true;
  window.addEventListener("keydown", (ev: any) => { if (ev.ctrlKey && (ev.code === "KeyP" || ev.key === "p" || ev.key === "P")) { ev.preventDefault(); exportPdfPages55(); } });
  const b: any = document.createElement("button");
  b.textContent = "PDF текстурный [Ctrl+P]";
  b.setAttribute("style", "position:fixed;top:8px;right:8px;z-index:99999;padding:6px 10px;background:#1e80ff;color:#ffffff;border:none;border-radius:6px;font:13px sans-serif;cursor:pointer;opacity:0.9;");
  b.onclick = () => { exportPdfPages55(); };
  document.body.appendChild(b);
}
// [80-A] раскладка деталей полками (без пересечений): сортировка по высоте, укладка рядами
const layoutPieces78 = (pieces) => {
  if (!pieces || pieces.length < 2) return pieces;
  const boxes = pieces.map(p => {
    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
    const up = (x, y) => {
      if (typeof x !== 'number' || typeof y !== 'number' || !isFinite(x) || !isFinite(y)) return;
      if (x < minX) minX = x;
      if (x > maxX) maxX = x;
      if (y < minY) minY = y;
      if (y > maxY) maxY = y;
    };
    (p.outline_2d || []).forEach(v => up(v[0], v[1]));
    (p.cut_lines || []).forEach(s => { up(s[0][0], s[0][1]); up(s[1][0], s[1][1]); });
    (p.fold_lines || []).forEach(s => { up(s[0][0], s[0][1]); up(s[1][0], s[1][1]); });
    if (!isFinite(minX)) { minX = 0; minY = 0; maxX = 0; maxY = 0; }
    return { p, minX, minY, w: maxX - minX, h: maxY - minY };
  });
  boxes.sort((a, b) => b.h - a.h);
  const gap = boxes[0].h * 0.12 + 1e-9;
  const perRow = Math.ceil(Math.sqrt(boxes.length));
  const shift = (p, dx, dy) => {
    (p.outline_2d || []).forEach(v => { v[0] += dx; v[1] += dy; });
    (p.cut_lines || []).forEach(s => { s[0][0] += dx; s[0][1] += dy; s[1][0] += dx; s[1][1] += dy; });
    (p.fold_lines || []).forEach(s => { s[0][0] += dx; s[0][1] += dy; s[1][0] += dx; s[1][1] += dy; });
  };
  // [TEX-A] сдвигаем 2D-грани вместе с деталями (текстуры совпадают с раскладкой)
  const idSetT: any = new Map();
  if (islandIds && islandIds.length) { islandIds.forEach((v: any) => { idSetT.set(v, true); }); }
  const pieceKeyOf = (p: any, i: any) => { if (idSetT.has(p.id)) { return p.id; } if (idSetT.has(p.part_id)) { return p.part_id; } if (idSetT.has(i)) { return i; } return null; };
  let texShiftHits: any = 0;
  const shiftFacesOf = (pid: any, dx: any, dy: any) => {
    if (!islandIds || !islandIds.length || !faces2D || !faces2D.length || !vertices2D || !vertices2D.length) { return; }
    const doneV: any = new Map();
    for (let fi = 0; fi < Math.min(islandIds.length, faces2D.length); fi++) {
      if (islandIds[fi] !== pid) { continue; }
      const f: any = faces2D[fi];
      if (!f) { continue; }
      texShiftHits++;
      f.forEach((vi: any) => { if (doneV.has(vi)) { return; } doneV.set(vi, true); const v: any = vertices2D[vi]; if (v) { v[0] += dx; v[1] += dy; } });
    }
  };
  // [PAG-1] постраничная раскладка A4 (WYSIWYG с PDF55): масштаб по крупнейшей детали, страницы 190x277мм, сетка страниц
  let maxW55: any = 0; let maxH55: any = 0;
  boxes.forEach((b: any) => { if (b.w > maxW55) { maxW55 = b.w; } if (b.h > maxH55) { maxH55 = b.h; } });
  const mmpu55: any = Math.max(2, Math.min(400, Math.min(170 / maxW55, 270 / maxH55)));
  // [PAG-5] аспект-подгонка отключена (давала полосу); аспект области пишется при отрисовке
  // [PAG-8] A4 page layout: sheet 190x277mm (A4 minus printer margins), pieces zone 170x270mm (tab reserve)
  // [PAG-8] zoom knob: __a4Zoom55 in app console (0.5 = half scale, fewer sheets)
  const zoomW55: any = Math.max(0.05, Math.min(1, (window as any).__a4Zoom55 || 1));
  const mmpuP8: any = mmpu55 * zoomW55;
  const PAGEW8: any = 190 / mmpuP8, PAGEH8: any = 277 / mmpuP8; // sheet frame, units
  const CW8: any = 170 / mmpuP8, CH8: any = 270 / mmpuP8; // pieces zone, units
  // [PAG-9] compact packing: MaxRects (BSSF) + optional 90-deg rotation; knob __a4Rotate55=false
  // [PAG-9] compact packing: MaxRects (BSSF) + optional 90-deg rotation; knob __a4Rotate55=false
  const gapP8: any = 8 / mmpuP8; // [PAG-9b] gap 8 mm in units (bylo 0.5 UNITA = 72 mm - glavnyy bag)
  const rotOn9: any = (window as any).__a4Rotate55 !== false;
  const pick9 = (rects: any, dw: any, dh: any) => {
    let lb: any = null;
    for (const r of rects) for (let o = 0; o < 2; o++) {
      if (o === 1 && (!rotOn9 || Math.abs(dw - dh) < 1e-9)) continue;
      const w = (o === 0 ? dw : dh) + gapP8, h = (o === 0 ? dh : dw) + gapP8;
      if (w > r.w + 1e-9 || h > r.h + 1e-9) continue;
      const c: any = { x: r.x, y: r.y, w: o ? dh : dw, h: o ? dw : dh, rot: !!o, s0: Math.min(r.w - w, r.h - h), s1: Math.max(r.w - w, r.h - h), ry: r.y, rx: r.x };
      if (!lb || c.s0 < lb.s0 - 1e-9 || (Math.abs(c.s0 - lb.s0) <= 1e-9 && (c.s1 < lb.s1 - 1e-9 || (Math.abs(c.s1 - lb.s1) <= 1e-9 && (c.ry < lb.ry - 1e-9 || (Math.abs(c.ry - lb.ry) <= 1e-9 && c.rx < lb.rx)))))) lb = c;
    }
    return lb;
  };
  const pagesR9: any = [], placeP9: any = [];
  boxes.map((_b: any, i: any) => i).sort((i: any, j: any) => { const a = boxes[i].w * boxes[i].h, b = boxes[j].w * boxes[j].h; return a !== b ? b - a : boxes[j].h - boxes[i].h; }).forEach((bi: any) => {
    const b: any = boxes[bi];
    let best: any = null, pg = -1;
    for (let p = 0; p < pagesR9.length && !best; p++) { const t = pick9(pagesR9[p], b.w, b.h); if (t) { best = t; pg = p; } }
    if (!best) { pagesR9.push([{ x: 0, y: 0, w: CW8, h: CH8 }]); pg = pagesR9.length - 1; best = pick9(pagesR9[pg], b.w, b.h); }
    if (!best) best = { x: 0, y: 0, w: b.w, h: b.h, rot: false };
    const hx0 = best.x, hy0 = best.y, hx1 = best.x + best.w + gapP8, hy1 = best.y + best.h + gapP8, keep9: any = [];
    for (const f of pagesR9[pg]) {
      const ix0 = Math.max(f.x, hx0), iy0 = Math.max(f.y, hy0), ix1 = Math.min(f.x + f.w, hx1), iy1 = Math.min(f.y + f.h, hy1);
      if (ix1 <= ix0 + 1e-9 || iy1 <= iy0 + 1e-9) { keep9.push(f); continue; }
      if (hx0 - f.x > 1e-9) keep9.push({ x: f.x, y: f.y, w: hx0 - f.x, h: f.h });
      if (f.x + f.w - hx1 > 1e-9) keep9.push({ x: hx1, y: f.y, w: f.x + f.w - hx1, h: f.h });
      if (hy0 - f.y > 1e-9) keep9.push({ x: f.x, y: f.y, w: f.w, h: hy0 - f.y });
      if (f.y + f.h - hy1 > 1e-9) keep9.push({ x: f.x, y: hy1, w: f.w, h: f.y + f.h - hy1 });
    }
    pagesR9[pg] = keep9.filter((a: any) => !keep9.some((c: any) => c !== a && c.x <= a.x + 1e-9 && c.y <= a.y + 1e-9 && c.x + c.w >= a.x + a.w - 1e-9 && c.y + c.h >= a.y + a.h - 1e-9 && (c.w * c.h > a.w * a.h + 1e-9 || (Math.abs(c.w - a.w) < 1e-9 && Math.abs(c.h - a.h) < 1e-9 && keep9.indexOf(c) < keep9.indexOf(a)))));
    placeP9.push({ bi, page: pg, x: best.x, y: best.y, rot: best.rot });
  });
  const nP8: any = pagesR9.length, colsP8: any = Math.max(1, Math.ceil(Math.sqrt(nP8))), PGAP8: any = Math.max(0.05 * maxW55, 0.5);
  let areaP9 = 0; boxes.forEach((b: any) => { areaP9 += b.w * b.h; });
  const fillP9: any = nP8 > 0 ? Math.round(100 * areaP9 / (nP8 * CW8 * CH8)) : 0;
  // [PAG-9] apply: rotate geometry+texture faces together (UV untouched), then shift to slot
  texShiftHits = 0;
  const rot90cw = (p: any) => { const rp = (t: any) => { const x = t[0], y = t[1]; t[0] = y; t[1] = -x; }; (p.outline_2d || []).forEach(rp); (p.cut_lines || []).forEach((s: any) => { rp(s[0]); rp(s[1]); }); (p.fold_lines || []).forEach((s: any) => { rp(s[0]); rp(s[1]); }); };
  const rotFacesOf = (pid: any) => {
    if (!islandIds || !islandIds.length || !faces2D || !faces2D.length || !vertices2D || !vertices2D.length) return;
    const doneR: any = new Map();
    for (let fi = 0; fi < Math.min(islandIds.length, faces2D.length); fi++) { if (islandIds[fi] !== pid) continue; const f: any = faces2D[fi]; if (!f) continue; f.forEach((vi: any) => { if (doneR.has(vi)) return; doneR.set(vi, true); const v: any = vertices2D[vi]; if (v) { const x = v[0], y = v[1]; v[0] = y; v[1] = -x; } }); }
  };
  const bb9 = (p: any) => { let mnx = Infinity, mny = Infinity, mxx = -Infinity, mxy = -Infinity; const u = (x: any, y: any) => { if (x < mnx) mnx = x; if (x > mxx) mxx = x; if (y < mny) mny = y; if (y > mxy) mxy = y; }; (p.outline_2d || []).forEach((v: any) => u(v[0], v[1])); (p.cut_lines || []).forEach((s: any) => { u(s[0][0], s[0][1]); u(s[1][0], s[1][1]); }); (p.fold_lines || []).forEach((s: any) => { u(s[0][0], s[0][1]); u(s[1][0], s[1][1]); }); return isFinite(mnx) ? [mnx, mny, mxx, mxy] : [0, 0, 0, 0]; };
  const listP8: any = [];
  placeP9.forEach((pl: any) => {
    const ox: any = (pl.page % colsP8) * (PAGEW8 + PGAP8), oy: any = Math.floor(pl.page / colsP8) * (PAGEH8 + PGAP8), b: any = boxes[pl.bi];
    if (pl.rot) { rot90cw(b.p); const pk0 = pieceKeyOf(b.p, pl.bi); if (pk0 !== null) rotFacesOf(pk0); }
    const q = bb9(b.p), dx: any = ox + pl.x - q[0], dy: any = oy + pl.y - q[1];
    shift(b.p, dx, dy);
    const pk = pieceKeyOf(b.p, pl.bi); if (pk !== null) shiftFacesOf(pk, dx, dy);
  });
  for (let k = 0; k < nP8; k++) listP8.push({ x: (k % colsP8) * (PAGEW8 + PGAP8), y: Math.floor(k / colsP8) * (PAGEH8 + PGAP8), no: k + 1 });
  (window as any).__pageInfo55 = { count: nP8, cols: colsP8, rows: Math.ceil(nP8 / colsP8), w: PAGEW8, h: PAGEH8, mmpu: mmpuP8, list: listP8 }; // [PAG-8] sheet frames for draw2D78
  console.log("[PAG-9] \u043B\u0438\u0441\u0442\u043E\u0432 " + nP8 + ", \u0434\u0435\u0442\u0430\u043B\u0435\u0439 " + boxes.length + ", \u0437\u0430\u043F\u043E\u043B\u043D\u0435\u043D\u0438\u0435 " + fillP9 + "%, \u043C\u0430\u0441\u0448\u0442\u0430\u0431 " + mmpuP8.toFixed(2) + " \u043C\u043C/\u044E\u043D\u0438\u0442"); // [PAG-9]
  const rowsT55: any = nP8; const maxXT55: any = colsP8 * (PAGEW8 + PGAP8); const maxYT55: any = Math.ceil(nP8 / colsP8) * (PAGEH8 + PGAP8); // [PAG-8] legacy aliases
  (window as any).__tileInfo55 = { n: boxes.length, pages: nP8, w: PAGEW8, h: PAGEH8, mmpu: mmpuP8, fill: fillP9 }; // [PAG-8] info line data
  console.log("[TEX-A] сдвинутых граней: " + texShiftHits + "/" + (faces2D ? faces2D.length : 0)); // [FIX71B] перенесён из loadTexImages: там texShiftHits вне scope (ReferenceError)
  let cx = 0, cy = 0, rowH = 0, placed = 0; // [PAG-1] переменные старой раскладки (цикл отключён ниже)
  boxes.slice(0, 0).forEach((b: any, bi: any) => { // [PAG-1] старая раскладка полками отключена
    shift(b.p, cx - b.minX, cy - b.minY);
    const pk = pieceKeyOf(b.p, bi); // [TEX-A]
    if (pk !== null) { shiftFacesOf(pk, cx - b.minX, cy - b.minY); }
    cx += b.w + gap;
    if (b.h > rowH) rowH = b.h;
    placed++;
    if (placed % perRow === 0) { cx = 0; cy += rowH + gap; rowH = 0; }
  });
  return pieces;
};

const draw2D78 = () => {
  const canvas = canvas2D.value;
  if (!canvas || !canvas.getContext) return;
  const ctx = canvas.getContext('2d');
  const W = canvas.width, H = canvas.height;
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, W, H);
  const pieces = pieces2D || [];
  if (!pieces.length) return;

  // [79-A] габариты по реальным полям Rust: outline_2d / cut_lines / fold_lines
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  const updB = (x, y) => {
    if (typeof x !== 'number' || typeof y !== 'number' || !isFinite(x) || !isFinite(y)) return;
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
  };
  pieces.forEach(p => {
    (p.outline_2d || []).forEach(v => updB(v[0], v[1]));
    (p.cut_lines || []).forEach(seg => { updB(seg[0][0], seg[0][1]); updB(seg[1][0], seg[1][1]); });
    (p.fold_lines || []).forEach(seg => { updB(seg[0][0], seg[0][1]); updB(seg[1][0], seg[1][1]); });
  });
  if (!isFinite(minX)) return;

  // вписываем развёртку в канвас, поверх действует зум/панорама пользователя
  const pad = 24;
  const baseScale = Math.min(
    (W - 2 * pad) / Math.max(1e-9, maxX - minX),
    (H - 2 * pad) / Math.max(1e-9, maxY - minY)
  );
  const totalScale = baseScale * (view2D.scale || 1);
  const tx = (x) => (x - minX) * totalScale + (view2D.offsetX || 0) + pad;
  const ty = (y) => (y - minY) * totalScale + (view2D.offsetY || 0) + pad;

  // заливка деталей белым (читаемость)
  ctx.fillStyle = 'rgba(255,255,255,0.9)';
  pieces.forEach(p => {
    const outline = p.outline_2d || [];
    if (outline.length < 3) return;
    ctx.beginPath();
    ctx.moveTo(tx(outline[0][0]), ty(outline[0][1]));
    for (let i = 1; i < outline.length; i++) ctx.lineTo(tx(outline[i][0]), ty(outline[i][1]));
    ctx.closePath();
    ctx.fill();
  });

  // [TEX-A] текстурный слой (под линиями реза/сгибов)
  if (texLayer && texLayer.canvas && texLayer.drawn > 0) {
    ctx.drawImage(texLayer.canvas, tx(texLayer.minX), ty(texLayer.minY), texLayer.w * totalScale, texLayer.h * totalScale);
  }
  // [PAG-2] рамки страниц A4 + номера страниц (постраничный вид)
  { const wasMs55: any = (window as any).__unfoldMs55; if ((window as any).__unfoldT055 > 0 && !wasMs55) { (window as any).__unfoldMs55 = Math.round(performance.now() - (window as any).__unfoldT055); console.log("[SPD-2] развёртка: " + ((window as any).__unfoldMs55 / 1000).toFixed(1) + " с"); } } // [SPD-1]+[FIX72]
  (window as any).__viewAspect55 = ctx.canvas.width / Math.max(1, ctx.canvas.height); // [PAG-4] аспект области 2D для раскладки
  { const ti55: any = (window as any).__tileInfo55; if (ti55) { ctx.fillStyle = "#707070"; ctx.font = "12px sans-serif"; ctx.fillText("\u0434\u0435\u0442\u0430\u043B\u0435\u0439 " + ti55.n + " | \u043B\u0438\u0441\u0442\u043E\u0432 A4 " + ti55.pages + " | \u043B\u0438\u0441\u0442 190x277 \u043C\u043C | " + ti55.mmpu.toFixed(1) + " \u043C\u043C/\u044E\u043D\u0438\u0442", 12, H - 12); } }
  // [PAG-8b] A4 sheet frames from pi55.list in world coords (same tx/ty as pieces)
  {
    const pf55: any = (window as any).__pageInfo55;
    if (pf55 && pf55.list && pf55.list.length) {
      ctx.save();
      ctx.strokeStyle = "#c0c0c0"; ctx.lineWidth = 1; ctx.setLineDash([8, 6]);
      ctx.font = "11px sans-serif";
      pf55.list.forEach((pg: any) => {
        const x0: any = tx(pg.x), y0: any = ty(pg.y), x1: any = tx(pg.x + pf55.w), y1: any = ty(pg.y + pf55.h);
        ctx.fillStyle = "rgba(255,255,255,0.05)";
        ctx.fillRect(x0, y0, x1 - x0, y1 - y0);
        ctx.strokeRect(x0, y0, x1 - x0, y1 - y0);
      });
      ctx.fillStyle = "#909090";
      pf55.list.forEach((pg: any) => {
        const x0: any = tx(pg.x), y0: any = ty(pg.y);
        ctx.fillText("\u0441\u0442\u0440. " + pg.no + " / " + pf55.count, x0 + 4, y0 + 14);
      });
      ctx.restore();
    }
  }
  const pi55: any = (window as any).__pageInfo55;
  if (false) { // [PAG-8b] legacy frame drawer disabled (own grid, mismatched pieces)
    ctx.strokeStyle = "#c0c0c0"; ctx.lineWidth = 1; ctx.setLineDash([8, 6]);
    for (let k55 = 0; k55 < pi55.count; k55++) {
      const cc55: any = (k55 >= (pi55.lastRow || 0) * pi55.cols) ? ((pi55.cols - (pi55.lastLen || pi55.cols)) / 2) : 0;
      const gx55: any = ((k55 % pi55.cols) + cc55) * (196 / pi55.mmpu) - 10 / pi55.mmpu; // [PAG-4] центрирование последнего ряда
      const gy55: any = Math.floor(k55 / pi55.cols) * (283 / pi55.mmpu) - 10 / pi55.mmpu;
      ctx.strokeRect(tx(gx55), ty(gy55), (210 / pi55.mmpu) * totalScale, (297 / pi55.mmpu) * totalScale);
      ctx.fillStyle = "#a0a0a0"; ctx.font = "11px sans-serif";
      ctx.fillText("стр. " + (k55 + 1) + "/" + pi55.count + " | масштаб " + pi55.mmpu.toFixed(1) + " мм/юнит" + ((window as any).__unfoldMs55 ? " | развёртка " + ((window as any).__unfoldMs55 / 1000).toFixed(1) + "с" : ""), tx(gx55) + 6, ty(gy55) + 14); // [SPD-1]
    }
    ctx.setLineDash([]);
  }
  // [82-A] резы по частям: каждая часть своим цветом (внутри части - один путь)
  const PART_COLORS = ['#e02020', '#e08a20', '#20a020', '#a020c0', '#20c0a0', '#c02080', '#806020', '#2050c0', '#c05020', '#508030'];
  const partColorOf = (pid) => {
    const ids = [];
    const seen = {};
    pieces.forEach(p => { const k = p.part_id || 0; if (!seen[k]) { seen[k] = true; ids.push(k); } });
    ids.sort((a, b) => a - b);
    const idx = ids.indexOf(pid);
    return PART_COLORS[(idx < 0 ? 0 : idx) % PART_COLORS.length];
  };
  ctx.lineWidth = 1.4;
  ctx.setLineDash([]);
  ctx.lineJoin = 'round';
  ctx.lineCap = 'round';
  const byPart = {};
  pieces.forEach(p => {
    const k = p.part_id || 0;
    if (!byPart[k]) byPart[k] = [];
    (p.cut_lines || []).forEach(seg => byPart[k].push(seg));
  });
  Object.keys(byPart).forEach(k => {
    ctx.strokeStyle = partColorOf(Number(k));
    ctx.beginPath();
    byPart[k].forEach(seg => {
      ctx.moveTo(tx(seg[0][0]), ty(seg[0][1]));
      ctx.lineTo(tx(seg[1][0]), ty(seg[1][1]));
    });
    ctx.stroke();
  });

  // сгибы — синим пунктиром
  ctx.strokeStyle = '#2060e0';
  ctx.lineWidth = 1.1;
  ctx.setLineDash([6, 4]);
  ctx.beginPath();
  pieces.forEach(p => {
    (p.fold_lines || []).forEach(seg => {
      ctx.moveTo(tx(seg[0][0]), ty(seg[0][1]));
      ctx.lineTo(tx(seg[1][0]), ty(seg[1][1]));
    });
  });
  ctx.stroke();
  ctx.setLineDash([]);

  // номер детали в центре её габарита
  ctx.fillStyle = '#909090';
  ctx.font = '11px sans-serif';
  pieces.forEach(p => {
    const outline = p.outline_2d || [];
    if (!outline.length) return;
    let sx = 0, sy = 0;
    outline.forEach(v => { sx += v[0]; sy += v[1]; });
    ctx.fillText('#' + p.id + ' ' + (p.part || ''), tx(sx / outline.length), ty(sy / outline.length));
  });
};

// [79-A] draw2D теперь рисует по реальным полям Rust (pieces: outline_2d/cut_lines/fold_lines).
const draw2D = () => { draw2D78(); };

const draw2D_legacy_unused = () => {
  const canvas = canvas2D.value;
  if (!canvas || !canvas.getContext) return;
  const ctx = canvas.getContext('2d');
  const width = canvas.width;
  const height = canvas.height;

  ctx.clearRect(0, 0, width, height);
  ctx.fillStyle = '#f0f0f0';
  ctx.fillRect(0, 0, width, height);

  if (!unfoldData.value) {
    ctx.fillStyle = 'red';
    ctx.font = '16px Arial';
    ctx.fillText('Ошибка: нет данных от Rust (unfoldData пуст).', 20, 30);
    return;
  }

  let res = unfoldData.value;
  if (typeof res === 'string') {
    try {
      res = JSON.parse(res);
    } catch (e) {
      ctx.fillStyle = 'red';
      ctx.font = '16px Arial';
      ctx.fillText('Ошибка: unfoldData is string but not JSON. Смотреть консоль.', 20, 30);
      console.error('Failed to parse JSON string in draw2D:', e, res);
      return;
    }
  }

  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
  const points = [];

  if (res.pieces && res.pieces.length > 0) {
    res.pieces.forEach(p => {
      if (p.outline && p.outline.points) {
        p.outline.points.forEach(pt => points.push(pt));
      }
    });
  } else if (res.vertices_2d && res.faces) {
    res.vertices_2d.forEach(v => points.push({ x: v[0], y: v[1] }));
  } else {
    ctx.fillStyle = 'red';
    ctx.font = '16px Arial';
    ctx.fillText('Ошибка: в объекте ответа нет pieces или vertices_2d/faces.', 20, 30);
    console.error('Некорректный ответ Rust (нет данных для отрисовки):', res);
    return;
  }

  points.forEach(p => {
    minX = Math.min(minX, p.x);
    maxX = Math.max(maxX, p.x);
    minY = Math.min(minY, p.y);
    maxY = Math.max(maxY, p.y);
  });

  if (minX === Infinity || maxX === -Infinity || minY === Infinity || maxY === -Infinity) {
    ctx.fillStyle = 'red';
    ctx.font = '16px Arial';
    ctx.fillText('Ошибка: Не удалось вычислить границы 2D-проекции.', 20, 30);
    return;
  }

  const padding = 50;
  const meshWidth = maxX - minX;
  const meshHeight = maxY - minY;
  const scaleX = (width - padding * 2) / meshWidth;
  const scaleY = (height - padding * 2) / meshHeight;
  // FIX40: зум/пан через view2D
  const fitScale = Math.min(scaleX, scaleY);
  if (!view2D.autoFitDone) { view2D.scale = 1; view2D.offsetX = 0; view2D.offsetY = 0; view2D.autoFitDone = true; }
  const scale = fitScale * view2D.scale;
  
  const fitOffsetX = (width - meshWidth * fitScale) / 2 - minX * fitScale;
  const offsetX = fitOffsetX * view2D.scale + view2D.offsetX;
  const fitOffsetY = (height - meshHeight * fitScale) / 2 - minY * fitScale;
  const offsetY = fitOffsetY * view2D.scale + view2D.offsetY;

  const tx = (x) => x * scale + offsetX;
  const ty = (y) => y * scale + offsetY;

  if (res.pieces && res.pieces.length > 0) {
    res.pieces.forEach((piece) => {
      if (piece.outline && piece.outline.points.length > 2) {
        ctx.beginPath();
        const p0 = piece.outline.points[0];
        ctx.moveTo(tx(p0.x), ty(p0.y));
        for (let i = 1; i < piece.outline.points.length; i++) {
          const p = piece.outline.points[i];
          ctx.lineTo(tx(p.x), ty(p.y));
        }
        ctx.closePath();
        
        ctx.fillStyle = 'rgba(255, 255, 255, 0.9)';
        ctx.fill();
        
        ctx.strokeStyle = '#000000';
        ctx.lineWidth = 2;
        ctx.setLineDash([]);
        ctx.stroke();
      }

      if (piece.folds && piece.folds.length > 0) {
        ctx.strokeStyle = '#ff0000';
        ctx.lineWidth = 1.5;
        ctx.setLineDash([6, 4]); 
        ctx.beginPath();
        
        piece.folds.forEach(fold => {
          if (fold.start && fold.end) {
            ctx.moveTo(tx(fold.start.x), ty(fold.start.y));
            ctx.lineTo(tx(fold.end.x), ty(fold.end.y));
          } else if (fold.points && fold.points.length === 2) {
            ctx.moveTo(tx(fold.points[0].x), ty(fold.points[0].y));
            ctx.lineTo(tx(fold.points[1].x), ty(fold.points[1].y));
          }
        });
        
        ctx.stroke();
        ctx.setLineDash([]); 
      }
    });
  } else if (res.vertices_2d && res.faces) {
    ctx.strokeStyle = '#0055ff';
    ctx.lineWidth = 1;
    ctx.setLineDash([]);
    
    res.faces.forEach(face => {
      ctx.beginPath();
      face.forEach((idx, i) => {
        const v = res.vertices_2d[idx];
        if (v) {
          if (i === 0) ctx.moveTo(tx(v[0]), ty(v[1]));
          else ctx.lineTo(tx(v[0]), ty(v[1]));
        }
      });
      ctx.closePath();
      ctx.stroke();
    });
  }
  
  ctx.setLineDash([]);
};;;;;;;;;;;;;;

const callOrchestrator = async () => {
  if (!objData.value) return;
  isAiThinking.value = true;
  aiError.value = ''; aiResult.value = 'Запрос к AI Orchestrator (localhost:3000/api/analyze)...';

  try {
    const meshStats = 'Vertices: ' + lastMetrics.vertex_count + ', Faces: ' + lastMetrics.face_count + ', Islands: ' + lastMetrics.islands + ', Area: ' + lastMetrics.total_area;
    const userPrompt = "Analyze this papercraft unfolding. Why does it have " + lastMetrics.islands + " islands? How to optimize for paper crafting?";

    const response = await fetch('http://127.0.0.1:3000/api/analyze', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ mesh_stats: meshStats, user_prompt: userPrompt })
    });

    if (!response.ok) throw new Error('HTTP error! status: ' + response.status);
    const data = await response.json();

    if (data.ai_advice) {
      aiResult.value = data.ai_advice;
    } else if (data.error) {
      aiError.value = data.error;
    } else {
      aiResult.value = JSON.stringify(data, null, 2);
    }
  } catch (e: any) {
    aiError.value = 'Ошибка подключения:\n' + e.toString() + '\n\nУбедитесь, что запущен AI Orchestrator.';
    aiResult.value = '';
  } finally {
    isAiThinking.value = false;
  }
};


const handleExport = async () => {
  try {
    console.log('[Export] pieces2D:', pieces2D.length, 'vertices2D:', vertices2D.length, 'faces2D:', faces2D.length);
    if (pieces2D.length === 0 && (vertices2D.length === 0 || faces2D.length === 0)) {
      alert('Нет данных развёртки для экспорта.');
      return;
    }
    console.log('[Export] sample vertices2D[0..2]:', vertices2D.slice(0,3));
    console.log('[Export] sample faces2D[0..2]:', faces2D.slice(0,3));
    // FIX42: если системный диалог save() недоступен (нет плагина/прав) — пишем в фиксированный путь
    let filePath: string | null = null;
    try {
      filePath = await save({
        filters: [{ name: 'PDF', extensions: ['pdf'] }],
        defaultPath: 'unfold.pdf'
      });
    } catch (eDialog) {
      console.error('[Export] save() недоступен, пишу в фиксированный путь:', eDialog);
      filePath = 'G:\\Dev\\pepakura-next\\assets\\models\\unfold.pdf';
    }
    if (!filePath) { console.log('[Export] отменено пользователем'); return; }
    if (pieces2D.length > 0) {
      console.log('[Export] export_pieces_pdf, pieces:', pieces2D.length);
      const res78 = await invoke<string>('export_pieces_pdf', { pieces: pieces2D, path: filePath });
      console.log('[Export] invoke result:', res78);
      alert('[EXP78] ' + String(res78));
    } else {
      console.log('[Export] export_pdf_cmd fallback, vertices:', vertices2D.length);
      const res44 = await invoke<string>('export_pdf_cmd', {
        vertices: vertices2D,
        faces: faces2D,
        islandIds: islandIds,
        path: filePath
      });
      console.log('[Export] invoke result:', res44);
      alert('[EXP44] ' + String(res44));
    }
  } catch (e) {
    console.error('[Export] invoke error:', e);
    alert('Ошибка экспорта: ' + e);
  }
};

//
// === PATCH: интерактивность 2D canvas, добавлено PowerShell ===
onMounted(() => {
  const canvas = canvas2D?.value;
  if (!canvas) return;
  // FIX40: размер битмапа = размеру элемента (иначе рисуем в 300x150 и растягиваем)
  const resize2D = () => {
    const rect = canvas.getBoundingClientRect();
    canvas.width = Math.max(1, Math.floor(rect.width));
    canvas.height = Math.max(1, Math.floor(rect.height));
    draw2D();
  };
  resize2D();
  if (!(canvas as any).__fix40r) { (canvas as any).__fix40r = true; window.addEventListener('resize', resize2D); } // FIX41

  canvas.addEventListener('mousedown', (e) => {
    view2D.isDragging = true;
    view2D.lastX = e.clientX;
    view2D.lastY = e.clientY;
    canvas.style.cursor = 'grabbing';
  });

  window.addEventListener('mousemove', (e) => {
    if (!view2D.isDragging) return;
    view2D.offsetX += e.clientX - view2D.lastX;
    view2D.offsetY += e.clientY - view2D.lastY;
    view2D.lastX = e.clientX;
    view2D.lastY = e.clientY;
    draw2D();
  });

  window.addEventListener('mouseup', () => {
    view2D.isDragging = false;
    canvas.style.cursor = 'grab';
  });

  canvas.addEventListener('mouseleave', () => {
    view2D.isDragging = false;
    canvas.style.cursor = 'grab';
  });

  canvas.addEventListener('wheel', (e) => {
    e.preventDefault();
    const zoomFactor = 1.1;
    const delta = e.deltaY < 0 ? zoomFactor : 1 / zoomFactor;

    const rect = canvas.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;

    const newScale = view2D.scale * delta;
    const minScale = 0.1;
    const maxScale = 20.0;

    if (newScale < minScale || newScale > maxScale) return;

    view2D.offsetX = mx - (mx - view2D.offsetX) * (newScale / view2D.scale);
    view2D.offsetY = my - (my - view2D.offsetY) * (newScale / view2D.scale);
    view2D.scale = newScale;
    view2D.autoFitDone = true;
    draw2D();
  });

  draw2D();
});
// === END PATCH ===
;
// ===================== [TAB-F] PDF-экспорт: клапаны 5 мм + номера пар + текстуры + упаковка =====================
// Вставить ЦЕЛИКОМ в конец <script setup lang="ts"> перед последним </script>.
// Старый экспорт не трогает: перехватывает клики по кнопкам с "PDF" в надписи (capture-фаза).
;(function () {
  const W: any = window as any;
  if (W.__tabF55) { console.log('[TAB-F] уже установлен, повторная вставка пропущена'); return; }
  W.__tabF55 = true;

  const TABF = { PXMM: 8, PAGEW: 210, PAGEH: 297, ZX: 12, ZY: 12, ZW: 186, ZH: 270, TAB: 5.0, GAP: 3.0 };

  const hyp2 = (dx: number, dy: number) => Math.sqrt(dx * dx + dy * dy);
  const key2 = (p: any) => Math.round(p[0] * 1e5) + ',' + Math.round(p[1] * 1e5);
  const clamp1 = (x: number) => Math.max(-1, Math.min(1, x));

  // аффинное преобразование: Source-треугольник -> Target-треугольник
  function aff(T: any, S: any): any {
    const det = (S[1][0] - S[0][0]) * (S[2][1] - S[0][1]) - (S[2][0] - S[0][0]) * (S[1][1] - S[0][1]);
    if (Math.abs(det) < 1e-12) return null;
    const a = ((T[1][0] - T[0][0]) * (S[2][1] - S[0][1]) - (T[2][0] - T[0][0]) * (S[1][1] - S[0][1])) / det;
    const c = ((T[2][0] - T[0][0]) * (S[1][0] - S[0][0]) - (T[1][0] - T[0][0]) * (S[2][0] - S[0][0])) / det;
    const b = ((T[1][1] - T[0][1]) * (S[2][1] - S[0][1]) - (T[2][1] - T[0][1]) * (S[1][1] - S[0][1])) / det;
    const d = ((T[2][1] - T[0][1]) * (S[1][0] - S[0][0]) - (T[1][1] - T[0][1]) * (S[2][0] - S[0][0])) / det;
    return [a, b, c, d, T[0][0] - a * S[0][0] - c * S[0][1], T[0][1] - b * S[0][0] - d * S[0][1]];
  }

  // парсер OBJ: v / vt / usemtl / f (квады веером в треугольники)
  function parseObj(text: string): any {
    const verts: any[] = []; const uvs: any[] = []; const faces: any[] = [];
    let curMat: any = null;
    const lines = String(text || '').split(/\r?\n/);
    for (const raw of lines) {
      const L = raw.trim(); if (!L || L.charAt(0) === '#') continue;
      const t = L.split(/\s+/);
      if (t[0] === 'v' && t.length >= 4) verts.push([parseFloat(t[1]), parseFloat(t[2]), parseFloat(t[3])]);
      else if (t[0] === 'vt' && t.length >= 3) uvs.push([parseFloat(t[1]), parseFloat(t[2])]);
      else if (t[0] === 'usemtl' && t.length >= 2) curMat = t.slice(1).join(' ');
      else if (t[0] === 'f' && t.length >= 4) {
        const vi: any[] = []; const vti: any[] = []; let bad = false;
        for (let i = 1; i < t.length; i++) {
          const parts = t[i].split('/');
          const a = parseInt(parts[0], 10);
          if (!isFinite(a) || a === 0) { bad = true; break; }
          vi.push(a > 0 ? a - 1 : verts.length + a);
          const u = parseInt(parts[1] || '0', 10);
          vti.push(u > 0 ? u - 1 : (u < 0 ? uvs.length + u : -1));
        }
        if (bad || vi.length < 3) continue;
        for (let i = 1; i + 1 < vi.length; i++) faces.push({ vi: [vi[0], vi[i], vi[i + 1]], vti: [vti[0], vti[i], vti[i + 1]], mat: curMat });
      }
    }
    return { verts, uvs, faces };
  }

  // загрузка картинки: прямой URL -> convertFileSrc -> readBinaryFile (base64)
  async function loadTexFlex(url0: string): Promise<any> {
    const tries: string[] = [String(url0)];
    try {
      if (/^[a-zA-Z]:\\/.test(url0) || url0.indexOf('\\\\') === 0) {
        try { const m: any = await import('@tauri-apps/api/tauri'); tries.push(m.convertFileSrc(url0)); } catch (e) {}
        try {
          const fs: any = await import('@tauri-apps/api/fs');
          const bytes: Uint8Array = await fs.readBinaryFile(url0);
          let bin = '';
          for (let i = 0; i < bytes.length; i += 0x8000) bin += String.fromCharCode.apply(null, Array.from(bytes.subarray(i, i + 0x8000)) as any);
          tries.push('data:image/png;base64,' + btoa(bin));
        } catch (e) {}
      }
    } catch (e) {}
    for (const u of tries) {
      const img = await new Promise<any>((res) => { const im = new Image(); im.onload = () => res(im); im.onerror = () => res(null); im.src = u; });
      if (img) return img;
    }
    return null;
  }

  // упаковка MaxRects (полки не нужны): резерв = bbox + GAP, вырезы из свободных прямоугольников
  function packPieces(items: any[]): any {
    const pages: any[] = [[{ x: 0, y: 0, w: TABF.ZW, h: TABF.ZH }]];
    const placed: any[] = [];
    for (const it of items) {
      let pg = -1, best: any = null, bestScore = Infinity;
      for (let p = 0; p < pages.length && pg < 0; p++) {
        for (const r of pages[p]) {
          for (const rot of [false, true]) {
            const w = (rot ? it.h : it.w) + TABF.GAP, h = (rot ? it.w : it.h) + TABF.GAP;
            if (w > r.w + 1e-9 || h > r.h + 1e-9) continue;
            const score = Math.min(r.w - w, r.h - h);
            if (score < bestScore) { bestScore = score; best = { x: r.x, y: r.y, w: rot ? it.h : it.w, h: rot ? it.w : it.h, rot }; pg = p; }
          }
        }
      }
      if (!best) { pages.push([{ x: 0, y: 0, w: TABF.ZW, h: TABF.ZH }]); pg = pages.length - 1; best = { x: 0, y: 0, w: it.w, h: it.h, rot: false }; }
      const fx0 = best.x, fy0 = best.y, fx1 = best.x + best.w + TABF.GAP, fy1 = best.y + best.h + TABF.GAP;
      const out: any[] = [];
      for (const r of pages[pg]) {
        const ix0 = Math.max(r.x, fx0), iy0 = Math.max(r.y, fy0), ix1 = Math.min(r.x + r.w, fx1), iy1 = Math.min(r.y + r.h, fy1);
        if (ix1 <= ix0 + 1e-9 || iy1 <= iy0 + 1e-9) { out.push(r); continue; }
        if (r.y < fy0 - 1e-9) out.push({ x: r.x, y: r.y, w: r.w, h: fy0 - r.y });
        if (r.y + r.h > fy1 + 1e-9) out.push({ x: r.x, y: fy1, w: r.w, h: r.y + r.h - fy1 });
        if (r.x < fx0 - 1e-9) out.push({ x: r.x, y: Math.max(r.y, fy0), w: fx0 - r.x, h: Math.min(r.y + r.h, fy1) - Math.max(r.y, fy0) });
        if (r.x + r.w > fx1 + 1e-9) out.push({ x: fx1, y: Math.max(r.y, fy0), w: r.x + r.w - fx1, h: Math.min(r.y + r.h, fy1) - Math.max(r.y, fy0) });
      }
      pages[pg] = out;
      placed.push({ page: pg, x: best.x, y: best.y, rot: best.rot, w: it.w, h: it.h, g: it.g });
    }
    return { pages, placed };
  }

  // главный экспорт
  async function tabFExport(): Promise<void> {
    try {
      const UD: any = (typeof unfoldData !== 'undefined' && unfoldData && (unfoldData as any).value) ? (unfoldData as any).value : null;
      const P2D: any[] = (typeof pieces2D !== 'undefined' && pieces2D && (pieces2D as any).length) ? pieces2D as any : ((UD && UD.pieces) || []);
      const V2: any[] = (typeof vertices2D !== 'undefined' && vertices2D && (vertices2D as any).length) ? vertices2D as any : ((UD && UD.vertices_2d) || []);
      const F2: any[] = (typeof faces2D !== 'undefined' && faces2D && (faces2D as any).length) ? faces2D as any : ((UD && (UD.faces_2d || UD.faces)) || []);
      const ISO: any[] = (typeof islandIds !== 'undefined' && islandIds && (islandIds as any).length) ? islandIds as any : ((UD && UD.face_pieces) || []);
      if (!P2D.length || !V2.length || !F2.length || !ISO.length) { alert('[TAB-F] нет данных развёртки: сначала загрузите OBJ и нажмите "Развернуть"'); return; }

      const obj = parseObj(String((typeof objData !== 'undefined' && objData && (objData as any).value) || ''));
      if (obj.faces.length !== F2.length) console.warn('[TAB-F] OBJ-граней ' + obj.faces.length + ' != faces_2d ' + F2.length + ' (на квадах текстура может сместиться)');

      // масштаб: 2 итерации (bbox граней -> +клапаны -> снова масштаб)
      const faceBB = new Map<number, any>();
      for (let fi = 0; fi < F2.length; fi++) {
        const t = F2[fi]; if (!t || t[0] === t[1]) continue;
        const pid = ISO[fi]; if (pid === undefined || pid === null) continue;
        let bb = faceBB.get(pid); if (!bb) { bb = [1e18, 1e18, -1e18, -1e18]; faceBB.set(pid, bb); }
        for (const idx of t) { const v = V2[idx]; if (!v) continue; if (v[0] < bb[0]) bb[0] = v[0]; if (v[1] < bb[1]) bb[1] = v[1]; if (v[0] > bb[2]) bb[2] = v[0]; if (v[1] > bb[3]) bb[3] = v[1]; }
      }
      let mmpu = 1;
      { let mw = 1e-9, mh = 1e-9; faceBB.forEach((bb: any) => { mw = Math.max(mw, bb[2] - bb[0]); mh = Math.max(mh, bb[3] - bb[1]); }); mmpu = Math.min(TABF.ZW / mw, TABF.ZH / mh); }
      for (let it = 0; it < 2; it++) { const tw = TABF.TAB / mmpu; let mw = 1e-9, mh = 1e-9; faceBB.forEach((bb: any) => { mw = Math.max(mw, bb[2] - bb[0] + 2 * tw); mh = Math.max(mh, bb[3] - bb[1] + 2 * tw); }); mmpu = Math.min(TABF.ZW / mw, TABF.ZH / mh); }
      const twU = TABF.TAB / mmpu;

      // рёбра: экземпляры (деталь, грань, 2D-концы); сгибы = два экземпляра одной детали с равными 2D-концами
      const sameP = (p: any, q: any) => Math.abs(p[0] - q[0]) < 1e-7 && Math.abs(p[1] - q[1]) < 1e-7;
      const byKey = new Map<string, any[]>();
      for (let fi = 0; fi < F2.length; fi++) {
        const t = F2[fi]; if (!t || t[0] === t[1]) continue;
        const pid = ISO[fi]; if (pid === undefined || pid === null) continue;
        const P = [V2[t[0]], V2[t[1]], V2[t[2]]];
        if (!P[0] || !P[1] || !P[2]) continue;
        for (let k = 0; k < 3; k++) {
          const a = t[k], b = t[(k + 1) % 3];
          const key = a < b ? a + '_' + b : b + '_' + a;
          let arr = byKey.get(key); if (!arr) { arr = []; byKey.set(key, arr); }
          arr.push({ piece: pid, fi, a, b, pa: P[k], pb: P[(k + 1) % 3], d: null, n: null, tab: false, open: false, pair: 0, top1: null, top2: null });
        }
      }
      const consumed = new Map<string, Set<number>>();
      const foldRecsByPiece = new Map<number, any[]>();
      let foldCnt = 0;
      byKey.forEach((L: any[], key: string) => {
        for (let i = 0; i < L.length; i++) for (let j = i + 1; j < L.length; j++) {
          if (L[i].piece !== L[j].piece) continue;
          const eq = (sameP(L[i].pa, L[j].pa) && sameP(L[i].pb, L[j].pb)) || (sameP(L[i].pa, L[j].pb) && sameP(L[i].pb, L[j].pa));
          if (eq) {
            let s = consumed.get(key); if (!s) { s = new Set<number>(); consumed.set(key, s); }
            s.add(i); s.add(j);
            let fr = foldRecsByPiece.get(L[i].piece); if (!fr) { fr = []; foldRecsByPiece.set(L[i].piece, fr); }
            fr.push({ pa: L[i].pa, pb: L[i].pb, fi1: L[i].fi, fi2: L[j].fi });
            foldCnt++; j = L.length;
          }
        }
      });
      let pairCnt = 0;
      byKey.forEach((L: any[], key: string) => {
        const usedI = consumed.get(key) || new Set<number>();
        const rest = L.filter((x: any, idx: number) => !usedI.has(idx)).sort((x: any, y: any) => (x.piece - y.piece) || (x.fi - y.fi));
        if (!rest.length) return;
        if (rest.length >= 2) { pairCnt++; rest.forEach((r: any) => { r.pair = pairCnt; }); rest[0].tab = true; }
        else rest[0].open = true;
      });

      // границы деталей = все экземпляры, не входящие в сгибы
      const bnd = new Map<number, any[]>();
      byKey.forEach((L: any[], key: string) => {
        const usedI = consumed.get(key) || new Set<number>();
        L.forEach((inst: any, idx: number) => { if (usedI.has(idx)) return; let a = bnd.get(inst.piece); if (!a) { a = []; bnd.set(inst.piece, a); } a.push(inst); });
      });

      // клапаны: порт [FIX60B] из papercraft_adapter.rs (митры по углам границы)
      const pgData: any[] = [];
      bnd.forEach((segs: any[], pid: number) => {
        const pc: any = P2D[pid] || {};
        let cx = 0, cy = 0, cn = 0;
        for (let fi = 0; fi < F2.length; fi++) { if (ISO[fi] !== pid) continue; const t = F2[fi]; if (!t || t[0] === t[1]) continue; for (const idx of t) { const v = V2[idx]; if (!v) continue; cx += v[0]; cy += v[1]; cn++; } }
        const cen = cn ? [cx / cn, cy / cn] : [0, 0];
        for (const s of segs) {
          const dx = s.pb[0] - s.pa[0], dy = s.pb[1] - s.pa[1]; const el = hyp2(dx, dy);
          if (el < 1e-9) { s.d = [1, 0]; s.n = [0, 1]; continue; }
          s.d = [dx / el, dy / el]; s.n = [s.d[1], -s.d[0]];
          const mx = (s.pa[0] + s.pb[0]) / 2 - cen[0], my = (s.pa[1] + s.pb[1]) / 2 - cen[1];
          if (mx * s.n[0] + my * s.n[1] < 0) s.n = [-s.n[0], -s.n[1]];
        }
        const vtxE = new Map<number, number[]>();
        segs.forEach((s: any, i: number) => { let A = vtxE.get(s.a); if (!A) { A = []; vtxE.set(s.a, A); } A.push(i); let B = vtxE.get(s.b); if (!B) { B = []; vtxE.set(s.b, B); } B.push(i); });
        segs.forEach((s: any, ei: number) => {
          if (!s.tab) return;
          const el = hyp2(s.pb[0] - s.pa[0], s.pb[1] - s.pa[1]); if (el < 1e-9) return;
          let tw = Math.min(twU, 0.45 * el);
          const tFit = [0.3 * tw, 0.3 * tw]; const oms = [Math.PI, Math.PI];
          const ends: any[] = [[s.a, s.pa, 0], [s.b, s.pb, 1]];
          for (const ed of ends) {
            const v = ed[0], pV = ed[1], k = ed[2];
            const evs = vtxE.get(v); if (!evs || evs.length !== 2) continue;
            const oi = evs[0] === ei ? evs[1] : evs[0]; const o = segs[oi];
            let p2: any;
            if (o.a === v) p2 = o.pb; else if (o.b === v) p2 = o.pa; else continue;
            const dvx = p2[0] - pV[0], dvy = p2[1] - pV[1]; const lv = hyp2(dvx, dvy); if (lv < 1e-9) continue;
            const d2 = [dvx / lv, dvy / lv];
            const d1 = k === 0 ? s.d : [-s.d[0], -s.d[1]];
            const cr = d1[0] * d2[1] - d1[1] * d2[0], cs2 = d1[0] * d2[0] + d1[1] * d2[1];
            const sgn = s.d[0] * s.n[1] - s.d[1] * s.n[0];
            const ang = Math.atan2(cr, cs2); const TAU = Math.PI * 2;
            const om = sgn < 0 ? (ang < 0 ? -ang : TAU - ang) : (ang > 0 ? ang : TAU + ang);
            oms[k] = om;
            if (om < Math.PI && om > 1e-6) { const need = tw / Math.tan(om / 2); if (need > tFit[k]) tFit[k] = Math.min(need, 0.45 * el); }
          }
          for (let k = 0; k < 2; k++) { if (oms[k] < Math.PI && tFit[k] >= 0.45 * el - 1e-9) { const tw2 = tFit[k] * Math.tan(oms[k] / 2) * 0.95; if (tw2 < tw) tw = tw2; } }
          s.top1 = [s.pa[0] + s.n[0] * tw + s.d[0] * tFit[0], s.pa[1] + s.n[1] * tw + s.d[1] * tFit[0]];
          s.top2 = [s.pb[0] + s.n[0] * tw - s.d[0] * tFit[1], s.pb[1] + s.n[1] * tw - s.d[1] * tFit[1]];
        });

        // контур с клапанами: обход границы по 2D-точкам
        const byPt = new Map<string, number[]>();
        segs.forEach((s: any, i: number) => { const ka = key2(s.pa), kb = key2(s.pb); let A = byPt.get(ka); if (!A) { A = []; byPt.set(ka, A); } A.push(i); let B = byPt.get(kb); if (!B) { B = []; byPt.set(kb, B); } B.push(i); });
        const used = new Array(segs.length).fill(false);
        const loops: any[] = [];
        for (let i0 = 0; i0 < segs.length; i0++) {
          if (used[i0]) continue;
          const pts: any[] = []; const startKey = key2(segs[i0].pa);
          let cur = i0, flip = false, guard = 0;
          while (guard++ <= segs.length + 4) {
            used[cur] = true;
            const s = segs[cur];
            const pa = flip ? s.pb : s.pa, pb = flip ? s.pa : s.pb;
            const t1 = flip ? s.top2 : s.top1, t2 = flip ? s.top1 : s.top2;
            pts.push(pa); if (s.tab && s.top1 && s.top2) { pts.push(t1, t2); }
            pts.push(pb);
            const kb = key2(pb);
            if (kb === startKey && pts.length >= 3) break;
            const cands = (byPt.get(kb) || []).filter((j: number) => !used[j]);
            if (!cands.length) break;
            const sj = segs[cands[0]];
            flip = key2(sj.pa) === kb ? false : true;
            cur = cands[0];
          }
          if (pts.length >= 3) loops.push(pts);
        }

        // текстурные треугольники этой детали
        const tris: any[] = [];
        for (let fi = 0; fi < F2.length; fi++) {
          if (ISO[fi] !== pid) continue;
          const t = F2[fi]; if (!t || t[0] === t[1]) continue;
          const of = obj.faces[fi]; if (!of) continue;
          const T = [V2[t[0]], V2[t[1]], V2[t[2]]];
          if (!T[0] || !T[1] || !T[2]) continue;
          if (!of.vti || of.vti.length !== 3 || of.vti[0] < 0 || of.vti[1] < 0 || of.vti[2] < 0) continue;
          const uvT = [obj.uvs[of.vti[0]], obj.uvs[of.vti[1]], obj.uvs[of.vti[2]]];
          if (!uvT[0] || !uvT[1] || !uvT[2]) continue;
          tris.push({ T, uvT, mat: of.mat || '' });
        }

        // номера пар: на клапане и у ответной кромки
        const nums: any[] = [];
        for (const s of segs) {
          if (!s.pair) continue;
          const txt = String(s.pair);
          if (s.tab && s.top1 && s.top2) {
            const m = [(s.top1[0] + s.top2[0]) / 2, (s.top1[1] + s.top2[1]) / 2];
            let ang = Math.atan2(s.top2[1] - s.top1[1], s.top2[0] - s.top1[0]);
            if (ang > Math.PI / 2 || ang < -Math.PI / 2) ang += Math.PI;
            nums.push({ p: [m[0] + s.n[0] * 1.1, m[1] + s.n[1] * 1.1], ang, txt });
          } else {
            const m = [(s.pa[0] + s.pb[0]) / 2, (s.pa[1] + s.pb[1]) / 2];
            let ang = Math.atan2(s.pb[1] - s.pa[1], s.pb[0] - s.pa[0]);
            if (ang > Math.PI / 2 || ang < -Math.PI / 2) ang += Math.PI;
            nums.push({ p: [m[0] - s.n[0] * 2.8, m[1] - s.n[1] * 2.8], ang, txt });
          }
        }

        const fb = faceBB.get(pid) || [0, 0, 0, 0];
        const bb = [fb[0] - twU, fb[1] - twU, fb[2] + twU, fb[3] + twU];
        pgData.push({ pid, part: pc.part || ('Часть ' + ((pc.part_id !== undefined ? pc.part_id : 0) + 1)), loops, folds: foldRecsByPiece.get(pid) || [], nums, tris, bb });
      });
      console.log('[TAB-F] деталей: ' + pgData.length + ' | пар склейки: ' + pairCnt + ' | сгибов: ' + foldCnt + ' | масштаб ' + mmpu.toFixed(2) + ' мм/ед');

      // текстуры: material -> Image
      const tm: any = (typeof textureMap !== 'undefined' && textureMap && (textureMap as any).value) || {};
      const tmLow: any = {}; Object.keys(tm).forEach((k: string) => { tmLow[k.toLowerCase()] = tm[k]; });
      const needMats = new Set<string>();
      pgData.forEach((g: any) => g.tris.forEach((t: any) => { if (t.mat) needMats.add(t.mat); }));
      const texImgs: any = {};
      for (const m of Array.from(needMats)) {
        const u = tm[m] || tmLow[String(m).toLowerCase()];
        if (!u) continue;
        const img = await loadTexFlex(String(u));
        if (img) texImgs[m] = img;
      }
      console.log('[TAB-F] текстур загружено: ' + Object.keys(texImgs).length + '/' + needMats.size);

      // упаковка + поворот/сдвиг геометрии в слот
      const items = pgData.map((g: any) => ({ g, w: g.bb[2] - g.bb[0], h: g.bb[3] - g.bb[1], area: (g.bb[2] - g.bb[0]) * (g.bb[3] - g.bb[1]) }));
      items.sort((a: any, b: any) => b.area - a.area);
      const res = packPieces(items);
      const placedList: any[] = [];
      for (const pl of res.placed) {
        const g = pl.g;
        if (pl.rot) {
          const rotP = (q: any) => [-q[1], q[0]];
          g.loops = g.loops.map((lp: any) => lp.map(rotP));
          g.folds = g.folds.map((f: any) => ({ pa: rotP(f.pa), pb: rotP(f.pb), fi1: f.fi1, fi2: f.fi2 }));
          g.tris = g.tris.map((t: any) => ({ T: t.T.map(rotP), uvT: t.uvT, mat: t.mat }));
          g.nums = g.nums.map((nm: any) => ({ p: rotP(nm.p), ang: nm.ang + Math.PI / 2, txt: nm.txt }));
        }
        let mnx = 1e18, mny = 1e18;
        const acc = (q: any) => { if (q[0] < mnx) mnx = q[0]; if (q[1] < mny) mny = q[1]; };
        g.loops.forEach((lp: any) => lp.forEach(acc)); g.tris.forEach((t: any) => t.T.forEach(acc)); g.folds.forEach((f: any) => { acc(f.pa); acc(f.pb); });
        if (mnx > 1e17) { mnx = 0; mny = 0; }
        const dx = pl.x - mnx, dy = pl.y - mny;
        const sh = (q: any) => [q[0] + dx, q[1] + dy];
        g.loops = g.loops.map((lp: any) => lp.map(sh));
        g.folds = g.folds.map((f: any) => ({ pa: sh(f.pa), pb: sh(f.pb), fi1: f.fi1, fi2: f.fi2 }));
        g.tris = g.tris.map((t: any) => ({ T: t.T.map(sh), uvT: t.uvT, mat: t.mat }));
        g.nums = g.nums.map((nm: any) => ({ p: sh(nm.p), ang: nm.ang, txt: nm.txt }));
        g.labelPos = [mnx + dx + 0.4, mny + dy - 1.0];
        placedList.push({ page: pl.page, g });
      }
      let usedArea = 0; for (const pl of res.placed) usedArea += pl.w * pl.h;
      const fillPct = Math.round(100 * usedArea / (res.pages.length * TABF.ZW * TABF.ZH));

      // гора/долина по 3D-нормалям OBJ
      function n3(vi: any): any { const A = obj.verts[vi[0]], B = obj.verts[vi[1]], C = obj.verts[vi[2]]; const u = [B[0] - A[0], B[1] - A[1], B[2] - A[2]], v = [C[0] - A[0], C[1] - A[1], C[2] - A[2]]; const w = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]; const l = Math.sqrt(w[0] * w[0] + w[1] * w[1] + w[2] * w[2]) || 1; return [w[0] / l, w[1] / l, w[2] / l]; }
      function foldKind(fi1: number, fi2: number): string {
        const of1 = obj.faces[fi1], of2 = obj.faces[fi2]; if (!of1 || !of2) return 'flat';
        const n1 = n3(of1.vi), n2 = n3(of2.vi);
        const cs = clamp1(n1[0] * n2[0] + n1[1] * n2[1] + n1[2] * n2[2]);
        const ang = Math.acos(cs) * 180 / Math.PI;
        if (ang < 6) return 'flat';
        return ang <= 92 ? 'mountain' : 'valley';
      }

      function loopPath(ctx: any, loops: any) { ctx.beginPath(); for (const lp of loops) { ctx.moveTo(lp[0][0], lp[0][1]); for (let i = 1; i < lp.length; i++) ctx.lineTo(lp[i][0], lp[i][1]); ctx.closePath(); } }
      function drawPiece(ctx: any, g: any) {
        ctx.save();
        ctx.fillStyle = '#ffffff';
        loopPath(ctx, g.loops); ctx.fill('evenodd');
        ctx.save(); loopPath(ctx, g.loops); ctx.clip('evenodd');
        for (const t of g.tris) {
          const img = texImgs[t.mat]; if (!img) continue;
          const S = t.uvT.map((uv: any) => [uv[0] * img.width, (1 - uv[1]) * img.height]);
          const m = aff(t.T, S); if (!m) continue;
          ctx.save(); ctx.transform(m[0], m[1], m[2], m[3], m[4], m[5]); ctx.drawImage(img, 0, 0); ctx.restore();
        }
        ctx.restore();
        ctx.strokeStyle = '#1a1a1a'; ctx.lineWidth = 0.25; ctx.setLineDash([]);
        loopPath(ctx, g.loops); ctx.stroke();
        for (const f of g.folds) {
          const kind = foldKind(f.fi1, f.fi2); if (kind === 'flat') continue;
          ctx.beginPath(); ctx.moveTo(f.pa[0], f.pa[1]); ctx.lineTo(f.pb[0], f.pb[1]);
          if (kind === 'mountain') { ctx.strokeStyle = '#c62828'; ctx.setLineDash([]); } else { ctx.strokeStyle = '#1565c0'; ctx.setLineDash([2.2, 1.4]); }
          ctx.lineWidth = 0.25; ctx.stroke();
        }
        ctx.setLineDash([]);
        ctx.fillStyle = '#111'; ctx.font = 'bold 2.6px sans-serif'; ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
        for (const nm of g.nums) { ctx.save(); ctx.translate(nm.p[0], nm.p[1]); ctx.rotate(nm.ang); ctx.fillText(nm.txt, 0, 0); ctx.restore(); }
        ctx.fillStyle = '#9a9a9a'; ctx.font = '2.2px sans-serif'; ctx.textAlign = 'left'; ctx.textBaseline = 'alphabetic';
        ctx.fillText('#' + (g.pid + 1) + ' ' + g.part, g.labelPos[0], g.labelPos[1]);
        ctx.restore();
      }

      // страницы A4 -> JPEG
      const pagesJ: string[] = [];
      for (let p = 0; p < res.pages.length; p++) {
        const cv = document.createElement('canvas');
        cv.width = TABF.PAGEW * TABF.PXMM; cv.height = TABF.PAGEH * TABF.PXMM;
        const ctx: any = cv.getContext('2d');
        ctx.setTransform(TABF.PXMM, 0, 0, TABF.PXMM, 0, 0);
        ctx.fillStyle = '#fff'; ctx.fillRect(0, 0, TABF.PAGEW, TABF.PAGEH);
        ctx.strokeStyle = '#bbb'; ctx.lineWidth = 0.2; ctx.strokeRect(4, 4, TABF.PAGEW - 8, TABF.PAGEH - 8);
        ctx.fillStyle = '#444'; ctx.font = '2.8px sans-serif'; ctx.textAlign = 'left'; ctx.textBaseline = 'alphabetic';
        ctx.fillText('стр. ' + (p + 1) + '/' + res.pages.length + '  |  масштаб ' + mmpu.toFixed(2) + ' мм/ед  |  заполнение ' + fillPct + '%  |  клапан ' + TABF.TAB + ' мм', TABF.ZX, 7.5);
        const leg: any[] = [['#1a1a1a', 'рез', false], ['#c62828', 'гора', false], ['#1565c0', 'долина', true]];
        let lx = 128; ctx.font = '2.2px sans-serif'; ctx.textBaseline = 'middle';
        for (const [c, nm, dash] of leg) {
          ctx.strokeStyle = c; if (dash) ctx.setLineDash([1.8, 1.1]); ctx.lineWidth = 0.3;
          ctx.beginPath(); ctx.moveTo(lx, 6.2); ctx.lineTo(lx + 5, 6.2); ctx.stroke(); ctx.setLineDash([]);
          ctx.fillStyle = '#333'; ctx.fillText(nm, lx + 6, 6.2); lx += 6 + ctx.measureText(nm).width + 4;
        }
        ctx.textBaseline = 'alphabetic';
        for (const pl of placedList) if (pl.page === p) drawPiece(ctx, pl.g);
        ctx.strokeStyle = '#333'; ctx.lineWidth = 0.3; ctx.beginPath();
        ctx.moveTo(55, 289); ctx.lineTo(155, 289); ctx.moveTo(55, 287.6); ctx.lineTo(55, 290.4);
        ctx.moveTo(155, 287.6); ctx.lineTo(155, 290.4); ctx.moveTo(105, 288.2); ctx.lineTo(105, 289.8); ctx.stroke();
        ctx.fillStyle = '#333'; ctx.font = '2.5px sans-serif'; ctx.textAlign = 'center'; ctx.fillText('100 мм', 105, 293.4);
        pagesJ.push(cv.toDataURL('image/jpeg', 0.95));
      }
      console.log('[TAB-F] страниц: ' + pagesJ.length + ' | заполнение ' + fillPct + '%');

      const dlg: any = await import('@tauri-apps/api/dialog');
      const filePath = await dlg.save({ filters: [{ name: 'PDF', extensions: ['pdf'] }], defaultPath: 'unfold_tex.pdf' });
      if (!filePath) return;
      await invoke('export_pdf', { vertices: [], faces: [], islandIds: [], path: filePath, pages: pagesJ });
      alert('PDF сохранён: ' + filePath + '\nстраниц: ' + pagesJ.length + ', клапаны ' + TABF.TAB + ' мм');
    } catch (e: any) {
      console.error('[TAB-F]', e);
      alert('[TAB-F] ошибка экспорта: ' + (e && e.message ? e.message : String(e)));
    }
  }

  // перехват кнопок PDF (capture-фаза на document: сработает раньше обработчиков Vue)
  document.addEventListener('click', (e: any) => {
    const t = e.target;
    const b = t && t.closest ? t.closest('button') : null;
    if (!b) return;
    const txt = String(b.textContent || '').toLowerCase();
    if (txt.indexOf('pdf') === -1) return;
    e.preventDefault(); e.stopPropagation();
    tabFExport();
  }, true);
  W.__tabFexport = tabFExport;
  console.log('[TAB-F] готов: перехват кнопок PDF активен');
})();
</script>

<style scoped>
/* [STYLE-59] прежний блок вызывал SyntaxError в vite dev (lang.css); заменён минимальным. Оригинал: SplitView.vue.bak_style59 */
</style>




























