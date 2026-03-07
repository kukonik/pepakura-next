<template>
  <div class="image-to-3d-generator">
    <h2 class="title">Генерация 3D модели из изображения</h2>
    
    <div class="upload-section">
      <label class="upload-label">Выберите изображение:</label>
      <div 
        class="drop-zone"
        :class="{ 'drag-over': isDragOver }"
        @dragover.prevent="handleDragOver"
        @dragleave.prevent="handleDragLeave"
        @drop.prevent="handleDrop"
        @click="triggerFileInput"
      >
        <input
          ref="fileInput"
          type="file"
          accept="image/*"
          @change="handleFileSelect"
          class="file-input"
        />
        
        <div class="drop-zone-content">
          <div v-if="selectedImage" class="image-preview">
            <img :src="selectedImage" alt="Предпросмотр" class="preview-image" />
          </div>
          <div v-else class="drop-zone-placeholder">
            <div class="upload-icon">📁</div>
            <p class="upload-text">Перетащите изображение сюда или нажмите для выбора</p>
            <p class="upload-hint">Поддерживаются форматы: JPG, PNG, WEBP</p>
          </div>
        </div>
      </div>
    </div>
    
    <div class="settings-section">
      <div class="setting-group">
        <label for="quality-mode" class="setting-label">Качество:</label>
        <select id="quality-mode" v-model="qualityMode" class="setting-select">
          <option value="preview">Предварительный просмотр</option>
          <option value="high_quality">Высокое качество</option>
        </select>
      </div>
      
      <div class="setting-group">
        <label for="steps" class="setting-label">Количество шагов:</label>
        <input
          id="steps"
          type="number"
          v-model.number="steps"
          class="setting-input"
          min="10"
          max="100"
        />
      </div>
      
      <div class="setting-group">
        <label for="resolution" class="setting-label">Разрешение:</label>
        <select id="resolution" v-model="resolution" class="setting-select">
          <option value="256">256x256</option>
          <option value="512">512x512</option>
          <option value="1024">1024x1024</option>
        </select>
      </div>
    </div>
    
    <div class="actions-section">
      <button
        @click="generateModel"
        :disabled="isGenerating || !selectedImage"
        class="generate-button"
      >
        {{ isGenerating ? 'Генерация...' : 'Создать 3D модель' }}
      </button>
      
      <button
        @click="cancelGeneration"
        v-if="isGenerating"
        class="cancel-button"
      >
        Отмена
      </button>
    </div>
    
    <div v-if="generationResult" class="result-section">
      <h3 class="result-title">Результат генерации:</h3>
      <div class="artifacts-list">
        <div
          v-for="artifact in generationResult.artifacts"
          :key="artifact.path"
          class="artifact-item"
        >
          <div class="artifact-info">
            <span class="artifact-type">{{ getArtifactTypeLabel(artifact.kind) }}</span>
            <span class="artifact-format">{{ artifact.format }}</span>
          </div>
          <div class="artifact-actions">
            <button @click="viewArtifact(artifact)" class="view-button">
              Просмотр
            </button>
            <button @click="downloadArtifact(artifact)" class="download-button">
              Скачать
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <div v-if="error" class="error-section">
      <p class="error-message">Ошибка: {{ error }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { ImageTo3dClient } from '@/modules/ai-service/textTo3dClient'
import { ImageTo3dRequest, ImageTo3dResponse } from '@/shared/types/textTo3d.types'

// Состояние компонента
const selectedImage = ref<string | null>(null)
const isDragOver = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)
const qualityMode = ref<'preview' | 'high_quality'>('preview')
const steps = ref(30)
const resolution = ref(512)
const isGenerating = ref(false)
const generationResult = ref<ImageTo3dResponse | null>(null)
const error = ref<string | null>(null)

// Клиент для генерации 3D моделей из изображений
const client = new ImageTo3dClient()

// Функции для работы с файлами
const triggerFileInput = () => {
  if (fileInput.value) {
    fileInput.value.click()
  }
}

const handleDragOver = () => {
  isDragOver.value = true
}

const handleDragLeave = () => {
  isDragOver.value = false
}

const handleDrop = (event: DragEvent) => {
  isDragOver.value = false
  const files = event.dataTransfer?.files
  if (files && files.length > 0) {
    processFile(files[0])
  }
}

const handleFileSelect = (event: Event) => {
  const input = event.target as HTMLInputElement
  const files = input.files
  if (files && files.length > 0) {
    processFile(files[0])
  }
}

const processFile = (file: File) => {
  // Проверяем тип файла
  if (!file.type.startsWith('image/')) {
    error.value = 'Пожалуйста, выберите изображение'
    return
  }
  
  // Проверяем размер файла (максимум 10 МБ)
  if (file.size > 10 * 1024 * 1024) {
    error.value = 'Размер файла не должен превышать 10 МБ'
    return
  }
  
  // Создаем предпросмотр изображения
  const reader = new FileReader()
  reader.onload = (e) => {
    selectedImage.value = e.target?.result as string
    error.value = null
  }
  reader.onerror = () => {
    error.value = 'Не удалось загрузить изображение'
  }
  reader.readAsDataURL(file)
}

// Функция генерации модели
const generateModel = async () => {
  if (!selectedImage.value) {
    error.value = 'Пожалуйста, выберите изображение'
    return
  }
  
  try {
    isGenerating.value = true
    error.value = null
    
    // Подготовка запроса
    const request: ImageTo3dRequest = {
      model_id: 'default-image-to-3d-model',
      preset_id: 'default',
      mode: qualityMode.value,
      reference_image: selectedImage.value,
      params: {
        steps: steps.value,
        resolution: resolution.value,
        seed: Date.now() // Используем текущее время как сид
      }
    }
    
    // Отправка запроса на генерацию
    const result = await client.generateFromImage(request)
    generationResult.value = result
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Неизвестная ошибка'
    console.error('Ошибка генерации 3D модели из изображения:', err)
  } finally {
    isGenerating.value = false
  }
}

// Функция отмены генерации
const cancelGeneration = () => {
  isGenerating.value = false
  error.value = 'Генерация отменена пользователем'
}

// Функция для получения метки типа артефакта
const getArtifactTypeLabel = (kind: string) => {
  switch (kind) {
    case 'mesh':
      return '3D Меш'
    case 'preview':
      return 'Предпросмотр'
    case 'texture':
      return 'Текстура'
    default:
      return kind
  }
}

// Функция просмотра артефакта
const viewArtifact = (artifact: any) => {
  // TODO: Реализовать просмотр артефакта
  console.log('Просмотр артефакта:', artifact)
  alert(`Просмотр артефакта: ${artifact.path}`)
}

// Функция скачивания артефакта
const downloadArtifact = (artifact: any) => {
  // TODO: Реализовать скачивание артефакта
  console.log('Скачивание артефакта:', artifact)
  alert(`Скачивание артефакта: ${artifact.path}`)
}
</script>

<style scoped>
.image-to-3d-generator {
  padding: 20px;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.title {
  margin-top: 0;
  margin-bottom: 20px;
  color: var(--text-primary);
  font-size: 1.5rem;
}

.upload-section {
  margin-bottom: 20px;
}

.upload-label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: var(--text-primary);
}

.drop-zone {
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  padding: 20px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s;
  background-color: var(--bg-input);
}

.drop-zone.drag-over {
  border-color: var(--primary-color);
  background-color: var(--primary-color-light);
}

.file-input {
  display: none;
}

.drop-zone-content {
  min-height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.image-preview {
  max-width: 100%;
  max-height: 300px;
}

.preview-image {
  max-width: 100%;
  max-height: 300px;
  object-fit: contain;
}

.drop-zone-placeholder {
  color: var(--text-secondary);
}

.upload-icon {
  font-size: 3rem;
  margin-bottom: 12px;
}

.upload-text {
  margin: 0 0 8px 0;
  font-size: 1.1rem;
}

.upload-hint {
  margin: 0;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.settings-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  margin-bottom: 20px;
}

.setting-group {
  display: flex;
  flex-direction: column;
}

.setting-label {
  margin-bottom: 8px;
  font-weight: 500;
  color: var(--text-primary);
}

.setting-select,
.setting-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-input);
  color: var(--text-primary);
  font-size: 1rem;
}

.actions-section {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
}

.generate-button {
  padding: 12px 24px;
  background-color: var(--primary-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.generate-button:hover:not(:disabled) {
  background-color: var(--primary-color-dark);
}

.generate-button:disabled {
  background-color: var(--bg-disabled);
  cursor: not-allowed;
}

.cancel-button {
  padding: 12px 24px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.cancel-button:hover {
  background-color: var(--bg-hover);
}

.result-section {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid var(--border-color);
}

.result-title {
  margin-top: 0;
  margin-bottom: 16px;
  color: var(--text-primary);
  font-size: 1.25rem;
}

.artifacts-list {
  display: grid;
  gap: 12px;
}

.artifact-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
}

.artifact-info {
  display: flex;
  gap: 12px;
}

.artifact-type {
  font-weight: 500;
  color: var(--text-primary);
}

.artifact-format {
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.artifact-actions {
  display: flex;
  gap: 8px;
}

.view-button,
.download-button {
  padding: 6px 12px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.view-button:hover,
.download-button:hover {
  background-color: var(--bg-hover);
}

.error-section {
  margin-top: 20px;
  padding: 12px;
  background-color: var(--error-bg);
  border: 1px solid var(--error-border);
  border-radius: 4px;
}

.error-message {
  margin: 0;
  color: var(--error-text);
}
</style>