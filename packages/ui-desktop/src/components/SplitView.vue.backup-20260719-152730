<template>
  <div style="display: flex; height: 100vh; background: #1e1e1e; color: #ccc; font-family: sans-serif;">
    <div style="flex: 2; border-right: 1px solid #333;">
      <ModelViewer3D :meshData="objData" />
    </div>
    <div style="flex: 1; display: flex; flex-direction: column; padding: 20px; background: #252526;">
      <button @click="loadFile" style="padding: 10px; margin-bottom: 10px; cursor: pointer; background: #0e639c; color: white; border: none;">
        Открыть OBJ файл
      </button>
      <button @click="doUnfold" :disabled="!objData" style="padding: 10px; margin-bottom: 10px; cursor: pointer; background: #333; color: white; border: 1px solid #555;">
        Вызвать Unfold (Rust)
      </button>
      <button @click="callOrchestrator" :disabled="!objData" style="padding: 10px; margin-bottom: 20px; cursor: pointer; background: #5a1e1e; color: white; border: 1px solid #771e1e;">
        AI Assist (Orchestrator)
      </button>
      <div style="flex: 1; overflow: auto; background: #1e1e1e; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 12px;">
        <div v-if="rustError" style="color: #f48771; margin-bottom: 10px;">Rust Ошибка: {{ rustError }}</div>
        <div v-if="rustResult" style="color: #9cdcfe; margin-bottom: 10px;">Rust Ответ: {{ rustResult }}</div>
        <div v-if="aiError" style="color: #f48771; margin-bottom: 10px;">AI Ошибка: {{ aiError }}</div>
        <div v-if="aiResult" style="color: #b5cea8; margin-bottom: 10px; white-space: pre-wrap;">AI Ответ: {{ aiResult }}</div>
        <div v-if="!objData" style="color: #666;">Файл не загружен</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import ModelViewer3D from './ModelViewer3D.vue';

const objData = ref('');
const rustResult = ref('');
const rustError = ref('');
const aiResult = ref('');
const aiError = ref('');

const loadFile = async () => {
  rustError.value = ''; rustResult.value = ''; aiError.value = ''; aiResult.value = '';
  try {
    const filePath = await open({ multiple: false, filters: [{ name: '3D Models', extensions: ['obj'] }] });
    if (!filePath) return;
    const text = await readTextFile(filePath as string);
    objData.value = text;
  } catch (e: any) { rustError.value = `Ошибка загрузки: ${e.toString()}`; }
};

const doUnfold = async () => {
  if (!objData.value) return;
  rustError.value = ''; rustResult.value = 'Вызов Rust...'; aiError.value = ''; aiResult.value = '';
  try {
    const result = await invoke<string>('unfold_lscm', { objData: objData.value });
    rustResult.value = result;
  } catch (e: any) { rustError.value = `Ошибка Rust: ${e.toString()}`; rustResult.value = ''; }
};

const callOrchestrator = async () => {
  if (!objData.value) return;
  aiError.value = ''; aiResult.value = 'Опрос AI Orchestrator (localhost:3000)...'; rustError.value = ''; rustResult.value = '';
  try {
    const response = await fetch('http://127.0.0.1:3000/fix', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        content: objData.value,
        description: "Analyze this OBJ mesh data for 3D paper unfolding. Find geometry errors like non-manifold edges.",
        strategies: ["rust", "code"]
      })
    });
    if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
    const data = await response.json();
    if (data.fixed_code) aiResult.value = data.fixed_code;
    else if (data.error) aiError.value = data.error;
    else aiResult.value = JSON.stringify(data, null, 2);
  } catch (e: any) { aiError.value = `Ошибка: ${e.toString()}`; aiResult.value = ''; }
};
</script>
