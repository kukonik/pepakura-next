<template>
  <div class="split-container">
    <div class="toolbar">
      <button @click="handleLoadObj">Загрузить .obj файл</button>
      <button @click="handleUnfold" :disabled="!objText">Развернуть</button>
    </div>
    <div class="viewer-wrapper" v-if="objText">
      <ModelViewer3D :obj-text="objText" />
    </div>
    <div v-else class="placeholder">
      <p>Загрузите OBJ-файл для просмотра</p>
    </div>
    <div v-if="error" class="error">{{ error }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'
import ModelViewer3D from './ModelViewer3D.vue'

const objText = ref<string>('')
const error = ref<string>('')

async function handleLoadObj() {
  try {
    error.value = ''
    const selected = await open({
      multiple: false,
      filters: [{ name: 'OBJ Files', extensions: ['obj'] }]
    })
    if (selected) {
      const filePath = selected as string
      const content = await readTextFile(filePath)
      objText.value = content
      console.log('📂 File loaded:', content.length, 'bytes')
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Ошибка загрузки'
    console.error('Load OBJ Error:', err)
  }
}

function handleUnfold() {
  // TODO: реализовать развёртку
}
</script>

<style scoped>
.split-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
}
.toolbar {
  padding: 10px;
  background: #2a2a3e;
  display: flex;
  gap: 10px;
}
.viewer-wrapper {
  flex: 1;
  min-height: 0;
}
.placeholder {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: gray;
}
.error {
  padding: 10px;
  background: #440000;
  color: #ff8888;
}
</style>
