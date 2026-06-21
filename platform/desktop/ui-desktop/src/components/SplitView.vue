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
      <button @click="handleExport"  style="z-index: 9999; pointer-events: auto; padding: 10px; margin-bottom: 20px; cursor: pointer; background: #0e639c; color: white; border: none; width: 100%;">📄 Экспорт PDF</button>

      <div style="flex: 1; overflow: auto; background: #ffffff; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 12px;">
        <div v-if="rustError" style="color: #f48771; margin-bottom: 10px; white-space: pre-wrap;">Rust Ошибка: {{ rustError }}</div>
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

import { calcEdgeCounts, drawSeams } from '../seams.js';
import { ref, nextTick } from 'vue';
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
const rustError = ref('');
const aiResult = ref('');
const aiError = ref('');
const isUnfolding = ref(false);
const isAiThinking = ref(false);

const canvas2D = ref<HTMLCanvasElement | null>(null);
const mtlData = ref<string | null>(null)
const textureMap = ref<Record<string, string>>({})
const showWireframe2D = ref(true);
const showIslands2D = ref(true);

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
  rustError.value = ''; rustResult.value = ''; aiError.value = ''; aiResult.value = '';
  try {
    const filePath = await open({
      multiple: false,
      filters: [{ name: '3D Models', extensions: ['obj'] }]
    });
    if (!filePath) return;
    const text = await readTextFile(filePath as string);
    objData.value = text;
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
        const texCache: Record<string, string> = {};
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
              texCache[texName] = 'data:image/png;base64,' + arrayBufferToBase64(texBytes);
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
    rustError.value = 'Ошибка загрузки: ' + e.toString();
  }
};

const doUnfold = async () => {
  if (!objData.value) return;
  isUnfolding.value = true;
  rustError.value = ''; rustResult.value = ''; aiError.value = ''; aiResult.value = '';

  try {
    rustResult.value = 'Выполняю MST-Seam развёртку...\n';

    const resultStr = await invoke<string>('unfold_mesh', {
      objData: objData.value,
      config: { algorithm: 'smart' }
    });

    const r = JSON.parse(resultStr);
    if (r.success) {
      lastMetrics = { vertex_count: r.vertex_count, face_count: r.face_count, islands: r.islands, total_area: r.total_area };
      rustResult.value += '✅ ' + r.message + '\nАлгоритм: ' + r.algorithm_used + '\nВершин: ' + r.vertex_count + '\nГраней: ' + r.face_count + '\nОстровов: ' + r.islands + '\nПлощадь: ' + r.total_area + ' кв. ед.\nДеградированных: ' + (r.degenerate_islands ? r.degenerate_islands.length : 0);
      vertices2D = r.vertices_2d;
      faces2D = r.faces.map((f: any) => f.vertices);
      islandIds = r.island_ids;
      degenerateIslands = r.degenerate_islands || [];
      await nextTick();
draw2D();
    } else {
      rustError.value = r.message;
    }
  } catch (e: any) {
    rustError.value = 'Ошибка Rust:\n' + e.toString();
    rustResult.value = '';
  } finally { isUnfolding.value = false; }
};

const islandColors = [
  'rgba(0, 122, 204, 0.5)', 'rgba(204, 0, 122, 0.5)', 'rgba(122, 204, 0, 0.5)',
  'rgba(204, 122, 0, 0.5)', 'rgba(0, 204, 204, 0.5)', 'rgba(204, 204, 0, 0.5)',
  'rgba(122, 0, 204, 0.5)', 'rgba(0, 204, 122, 0.5)', 'rgba(204, 80, 80, 0.5)',
  'rgba(80, 204, 80, 0.5)'
];

const draw2D = () => {
  if (!canvas2D.value || vertices2D.length === 0 || faces2D.length === 0) {
    return;
  }

  const canvas = canvas2D.value as HTMLCanvasElement;
  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  const rect = canvas.getBoundingClientRect();
  canvas.width = rect.width * window.devicePixelRatio;
  canvas.height = rect.height * window.devicePixelRatio;
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

  const width = rect.width;
  const height = rect.height;

  ctx.clearRect(0, 0, width, height);
  ctx.fillStyle = '#ffffff';
  ctx.fillRect(0, 0, width, height);

  // BBox только по вершинам, входящим в грани (исключаем висячие [0,0])
  const validVerts = new Set<number>();
  for (const face of faces2D) {
    for (const vi of face) validVerts.add(vi);
  }

  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
  for (const vi of validVerts) {
    const v = vertices2D[vi];
    if (!v) continue;
    const x = Array.isArray(v) ? v[0] : (v as any)?.x ?? NaN;
    const y = Array.isArray(v) ? v[1] : (v as any)?.y ?? NaN;
    if (!Number.isNaN(x)) {
      if (x < minX) minX = x;
      if (x > maxX) maxX = x;
    }
    if (!Number.isNaN(y)) {
      if (y < minY) minY = y;
      if (y > maxY) maxY = y;
    }
  }

  if (!Number.isFinite(minX) || !Number.isFinite(maxX) || !Number.isFinite(minY) || !Number.isFinite(maxY)) {
    return;
  }

  const rangeX = maxX - minX || 1;
  const rangeY = maxY - minY || 1;

  // Камера/центрирование (используем cam.js)
  initCam(ctx.canvas, () => requestAnimationFrame(draw2D));
  applyCam(ctx, width, height, minX, minY, rangeX, rangeY);

  const mapX = (x: number) => x;
  const mapY = (y: number) => y;

  // Построение карты рёбер: ключ - "minIndex_maxIndex"
  const edgeMap: Record<string, { faceIndex: number; islandId: number }[]> = {};

  for (let fi = 0; fi < faces2D.length; fi++) {
    const face = faces2D[fi];
    const islandId = islandIds[fi] ?? 0;

    if (!face || face.length < 3) continue;

    // Face.vertices: [usize; 3] -> рёбра (0-1, 1-2, 2-0)
    for (let e = 0; e < 3; e++) {
      const i0 = face[e];
      const i1 = face[(e + 1) % 3];
      const minIndex = Math.min(i0, i1);
      const maxIndex = Math.max(i0, i1);
      const key = `${minIndex}_${maxIndex}`;

      if (!edgeMap[key]) {
        edgeMap[key] = [];
      }
      edgeMap[key].push({ faceIndex: fi, islandId });
    }
  }

  type EdgeSegment = {
    v0x: number;
    v0y: number;
    v1x: number;
    v1y: number;
  };

  const foldEdges: EdgeSegment[] = [];
  const boundaryEdges: EdgeSegment[] = [];
  const glueEdges: EdgeSegment[] = [];

  for (const key in edgeMap) {
    const refs = edgeMap[key];
    if (!refs || refs.length === 0) continue;

    const [minIndexStr, maxIndexStr] = key.split('_');
    const v0Index = parseInt(minIndexStr, 10);
    const v1Index = parseInt(maxIndexStr, 10);
    const v0 = vertices2D[v0Index];
    const v1 = vertices2D[v1Index];
    if (!v0 || !v1) continue;

    if (refs.length === 1) {
      // boundary: ребро в одной грани
      boundaryEdges.push({
        v0x: mapX(v0[0]),
        v0y: mapY(v0[1]),
        v1x: mapX(v1[0]),
        v1y: mapY(v1[1]),
      });
    } else if (refs.length === 2) {
      const a = refs[0];
      const b = refs[1];

      if (a.islandId === b.islandId) {
        // fold: две грани в одном острове
        foldEdges.push({
          v0x: mapX(v0[0]),
          v0y: mapY(v0[1]),
          v1x: mapX(v1[0]),
          v1y: mapY(v1[1]),
        });
      } else {
        // glue: две грани в разных островах
        glueEdges.push({
          v0x: mapX(v0[0]),
          v0y: mapY(v0[1]),
          v1x: mapX(v1[0]),
          v1y: mapY(v1[1]),
        });
      }
    } else {
      // Дегенеративный случай — игнорируем
      continue;
    }
  }

  // Заливка островов (по islandIds / degenerateIslands), без рисования всех треугольников как шубы
  if (showIslands2D.value) {
    for (let fi = 0; fi < faces2D.length; fi++) {
      const face = faces2D[fi];
      if (!face || face.length < 3) continue;

      const islId = islandIds[fi] ?? 0;
      if (degenerateIslands.includes(islId)) {
        ctx.fillStyle = 'rgba(255, 50, 50, 0.6)';
      } else {
        ctx.fillStyle = islandColors[islId % islandColors.length];
      }

      ctx.beginPath();
      const v0 = vertices2D[face[0]];
      ctx.moveTo(mapX(v0[0]), mapY(v0[1]));
      for (let k = 1; k < face.length; k++) {
        const vk = vertices2D[face[k]];
        ctx.lineTo(mapX(vk[0]), mapY(vk[1]));
      }
      ctx.closePath();
      ctx.fill();
    }
  }

  // 1. Fold-edges (пунктир, тонкие, серые)
  ctx.save();
  ctx.strokeStyle = '#808080';
  ctx.lineWidth = 0.5;
  ctx.setLineDash([4, 4]);
  ctx.beginPath();
  for (const e of foldEdges) {
    ctx.moveTo(e.v0x, e.v0y);
    ctx.lineTo(e.v1x, e.v1y);
  }
  ctx.stroke();
  ctx.restore();

  // 2. Boundary + Glue (сплошные, толстые, чёрные)
  ctx.save();
  ctx.strokeStyle = '#000000';
  ctx.lineWidth = 1.5;
  ctx.setLineDash([]);
  ctx.beginPath();
  for (const e of boundaryEdges) {
    ctx.moveTo(e.v0x, e.v0y);
    ctx.lineTo(e.v1x, e.v1y);
  }
  for (const e of glueEdges) {
    ctx.moveTo(e.v0x, e.v0y);
    ctx.lineTo(e.v1x, e.v1y);
  }
  ctx.stroke();
  ctx.restore();

  // 3. Опциональная подсветка glue, если включён showWireframe2D
  if (showWireframe2D.value) {
    ctx.save();
    ctx.strokeStyle = '#ff0000';
    ctx.lineWidth = 1.0;
    ctx.setLineDash([]);
    ctx.beginPath();
    for (const e of glueEdges) {
      ctx.moveTo(e.v0x, e.v0y);
      ctx.lineTo(e.v1x, e.v1y);
    }
    ctx.stroke();
    ctx.restore();
  }
};

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
    console.log('[Export] vertices2D length:', vertices2D.length, 'faces2D length:', faces2D.length);
    if (vertices2D.length === 0 || faces2D.length === 0) {
      alert('Нет данных развёртки для экспорта.');
      return;
    }
    console.log('[Export] sample vertices2D[0..2]:', vertices2D.slice(0,3));
    console.log('[Export] sample faces2D[0..2]:', faces2D.slice(0,3));
    const filePath = await save({
      filters: [{ name: 'PDF', extensions: ['pdf'] }],
      defaultPath: 'unfold.pdf'
    });
    if (!filePath) return;
    console.log('[Export] invoking export_pdf with path:', filePath);
    await invoke('export_pdf', { 
      vertices: vertices2D, 
      faces: faces2D, 
      islandIds: islandIds,
      path: filePath 
    });
    console.log('[Export] invoke succeeded');
    alert('PDF сохранён: ' + filePath);
  } catch (e) {
    console.error('[Export] invoke error:', e);
    alert('Ошибка экспорта: ' + e);
  }
};
</script>

<style scoped>
canvas { display: block; }
</style>



























