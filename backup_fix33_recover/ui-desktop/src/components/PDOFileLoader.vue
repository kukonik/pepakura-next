<template>
  <div class="pdo-file-loader">
    <h3>Загрузка PDO файла</h3>
    <div class="upload-area" @dragover.prevent="handleDragOver" @drop.prevent="handleDrop">
      <div class="upload-content" v-if="!isUploading">
        <div class="upload-icon">📁</div>
        <p class="upload-text">Перетащите PDO файл сюда</p>
        <p class="upload-hint">или</p>
        <button @click="triggerFileInput" class="upload-button">Выбрать файл</button>
        <input
          ref="fileInput"
          type="file"
          accept=".pdo"
          @change="handleFileSelect"
          class="file-input"
        />
      </div>
      <div class="upload-progress" v-else>
        <div class="spinner"></div>
        <p>Загрузка файла...</p>
      </div>
    </div>
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
    <div v-if="successMessage" class="success-message">
      {{ successMessage }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useProjectStore } from '@/stores/projectStore'
import type { ParsePdoResult } from '@/types/pdo'

const fileInput = ref<HTMLInputElement | null>(null)
const isUploading = ref(false)
const error = ref<string | null>(null)
const successMessage = ref<string | null>(null)
const projectStore = useProjectStore()

const triggerFileInput = () => {
  if (fileInput.value) {
    fileInput.value.click()
  }
}

const handleDragOver = (event: DragEvent) => {
  event.preventDefault()
}

const handleDrop = async (event: DragEvent) => {
  event.preventDefault()
  if (event.dataTransfer?.files.length) {
    await processFile(event.dataTransfer.files[0])
  }
}

const handleFileSelect = async (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files?.length) {
    await processFile(input.files[0])
  }
}

const processFile = async (file: File) => {
  // Проверяем расширение файла
  if (!file.name.toLowerCase().endsWith('.pdo')) {
    error.value = 'Пожалуйста, выберите файл с расширением .pdo'
    return
  }

  // Проверяем размер файла (максимум 50 МБ)
  if (file.size > 50 * 1024 * 1024) {
    error.value = 'Размер файла не должен превышать 50 МБ'
    return
  }

  isUploading.value = true
  error.value = null
  successMessage.value = null

  try {
    // Читаем файл как ArrayBuffer
    const arrayBuffer = await file.arrayBuffer()
    const uint8Array = new Uint8Array(arrayBuffer)
    
    // Вызываем Tauri команду для парсинга PDO
    const result = await invoke<ParsePdoResult>('parse_pdo_to_pepa', { data: Array.from(uint8Array) })
    
    if (result.success) {
      successMessage.value = `Файл успешно загружен: ${file.name}`
      console.log('PDO parsing result:', result)
      
      // Обновляем состояние проекта
      if (result.scene) {
        // Создаем новый проект на основе сцены
        const now = new Date().toISOString()
        const newProject = {
          schemaVersion: result.scene.sceneVersion,
          projectMeta: {
            name: file.name.replace('.pdo', ''),
            createdAt: now,
            updatedAt: now
          },
          scene: result.scene,
          settings: {
            paperFormat: "A4",
            marginMm: 5,
            scale: 1,
            extensions: {}
          },
          extensions: {}
        }
        
        // Обновляем состояние проекта в store
        projectStore.currentProject = newProject
        projectStore.projectPath = null
        projectStore.isDirty = true
      }
    } else {
      error.value = result.error || 'Неизвестная ошибка при парсинге файла'
    }
  } catch (err: any) {
    error.value = err.message || 'Ошибка при загрузке файла'
    console.error('Failed to process PDO file:', err)
  } finally {
    isUploading.value = false
  }
}
</script>

<style scoped>
.pdo-file-loader {
  padding: 20px;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.upload-area {
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  padding: 40px 20px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s;
  background-color: var(--bg-input);
  margin-bottom: 20px;
}

.upload-area:hover {
  border-color: var(--primary-color);
  background-color: var(--primary-color-light);
}

.upload-content {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.upload-icon {
  font-size: 3rem;
  margin-bottom: 16px;
}

.upload-text {
  margin: 0 0 8px 0;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.upload-hint {
  margin: 0 0 16px 0;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.upload-button {
  padding: 10px 20px;
  background-color: var(--primary-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.upload-button:hover {
  background-color: var(--primary-color-dark);
}

.file-input {
  display: none;
}

.upload-progress {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid rgba(255, 255, 255, 0.3);
  border-top: 4px solid var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-message {
  padding: 12px;
  background-color: var(--error-bg);
  border: 1px solid var(--error-border);
  border-radius: 4px;
  color: var(--error-text);
  margin-top: 16px;
}

.success-message {
  padding: 12px;
  background-color: var(--success-bg);
  border: 1px solid var(--success-border);
  border-radius: 4px;
  color: var(--success-text);
  margin-top: 16px;
}
</style>