
<template>
  <div class="ai-generator">
    <div class="header">
      <h2><i class="fas fa-robot"></i> AI Генерация 3D</h2>
      <p class="subtitle">Преобразуйте изображение в 3D модель с помощью TripoSR</p>
    </div>

    <div class="tabs">
      <button
        class="tab"
        :class="{ active: activeTab === 'image' }"
        @click="activeTab = 'image'"
      >
        <i class="fas fa-image"></i> Image → 3D
      </button>
      <button
        class="tab"
        :class="{ active: activeTab === 'text' }"
        @click="activeTab = 'text'"
      >
        <i class="fas fa-font"></i> Text → 3D
      </button>
      <button
        class="tab"
        :class="{ active: activeTab === 'settings' }"
        @click="activeTab = 'settings'"
      >
        <i class="fas fa-cog"></i> Настройки
      </button>
    </div>

    <div v-if="activeTab === 'image'" class="tab-content">
      <div class="upload-section">
        <div
          class="dropzone"
          @dragover.prevent="dragOver = true"
          @dragleave="dragOver = false"
          @drop="onDrop"
          :class="{ 'drag-over': dragOver }"
        >
          <i class="fas fa-cloud-upload-alt"></i>
          <h3>Перетащите изображение сюда</h3>
          <p>или</p>
          <button class="btn-primary" @click="selectImage">
            Выберите файл
          </button>
          <input
            ref="fileInput"
            type="file"
            accept="image/*"
            @change="onFileSelected"
            style="display: none"
          />
          <p class="hint">Поддерживаются JPG, PNG, WEBP. Максимум 10 МБ.</p>
        </div>

        <div v-if="selectedImage" class="preview">
          <img :src="selectedImage" alt="Preview" />
          <button class="btn-icon" @click="clearImage" title="Удалить">
            <i class="fas fa-times"></i>
          </button>
        </div>
      </div>

      <div class="settings-section">
        <div class="form-group">
          <label>Качество</label>
          <select v-model="quality">
            <option value="fast">Быстрое (0.1 сек)</option>
            <option value="balanced">Сбалансированное (1 сек)</option>
            <option value="high">Высокое (5 сек)</option>
          </select>
        </div>
        <div class="form-group">
          <label>Формат вывода</label>
          <select v-model="format">
            <option value="obj">OBJ</option>
            <option value="glb">GLB</option>
            <option value="stl">STL</option>
            <option value="ply">PLY</option>
          </select>
        </div>
        <div class="form-group">
          <label>
            <input type="checkbox" v-model="useCache" />
            Использовать кэш
          </label>
        </div>
      </div>

      <div class="actions">
        <button
          class="btn-generate"
          :disabled="!selectedImage || generating"
          @click="generate"
        >
          <i class="fas fa-bolt"></i>
          {{ generating ? 'Генерация...' : 'Сгенерировать 3D' }}
        </button>
      </div>

      <div v-if="generating" class="progress-section">
        <div class="progress-bar">
          <div
            class="progress-fill"
            :style="{ width: progress + '%' }"
          ></div>
        </div>
        <p class="progress-text">{{ progressText }}</p>
      </div>

      <div v-if="result" class="result-section">
        <h3><i class="fas fa-check-circle"></i> Модель сгенерирована!</h3>
        <div class="result-info">
          <div class="info-row">
            <span>Файл:</span>
            <span>{{ result.mesh_path }}</span>
          </div>
          <div class="info-row">
            <span>Вершин:</span>
            <span>{{ result.vertices }}</span>
          </div>
          <div class="info-row">
            <span>Граней:</span>
            <span>{{ result.faces }}</span>
          </div>
          <div class="info-row">
            <span>Устройство:</span>
            <span>{{ result.device }}</span>
          </div>
          <div class="info-row" v-if="result.cached">
            <span><i class="fas fa-history"></i> Из кэша</span>
          </div>
        </div>
        <div class="result-actions">
          <button class="btn-secondary" @click="openInViewer">
            <i class="fas fa-eye"></i> Просмотреть
          </button>
          <button class="btn-secondary" @click="downloadModel">
            <i class="fas fa-download"></i> Скачать
          </button>
          <button class="btn-secondary" @click="importToProject">
            <i class="fas fa-folder-plus"></i> Импортировать в проект
          </button>
        </div>
      </div>

      <div v-if="error" class="error-section">
        <h3><i class="fas fa-exclamation-triangle"></i> Ошибка</h3>
        <p>{{ error }}</p>
        <button class="btn-secondary" @click="error = ''">Закрыть</button>
      </div>
    </div>

    <div v-if="activeTab === 'text'" class="tab-content">
      <div class="text-input-section">
        <div class="form-group">
          <label>Текстовый промпт</label>
          <textarea
            v-model="textPrompt"
            placeholder="Опишите 3D модель, например: 'реалистичный дракон с крыльями'"
            rows="4"
          ></textarea>
        </div>
        <div class="hint">
          Чем детальнее описание, тем лучше результат. Используйте английский для лучшей совместимости.
        </div>
      </div>

      <div class="settings-section">
        <div class="form-group">
          <label>Качество</label>
          <select v-model="textQuality">
            <option value="low">Низкое (быстрее, меньше деталей)</option>
            <option value="medium">Среднее (баланс)</option>
            <option value="high">Высокое (медленнее, больше деталей)</option>
          </select>
        </div>
        <div class="form-group">
          <label>Формат вывода</label>
          <select v-model="textFormat">
            <option value="obj">OBJ</option>
            <option value="glb">GLB</option>
            <option value="stl">STL</option>
            <option value="ply">PLY</option>
          </select>
        </div>
        <div class="form-group">
          <label>
            <input type="checkbox" v-model="textUseCache" />
            Использовать кэш
          </label>
        </div>
      </div>

      <div class="actions">
        <button
          class="btn-generate"
          :disabled="!textPrompt || generatingText"
          @click="generateFromText"
        >
          <i class="fas fa-bolt"></i>
          {{ generatingText ? 'Генерация...' : 'Сгенерировать 3D из текста' }}
        </button>
      </div>

      <div v-if="generatingText" class="progress-section">
        <div class="progress-bar">
          <div
            class="progress-fill"
            :style="{ width: textProgress + '%' }"
          ></div>
        </div>
        <p class="progress-text">{{ textProgressText }}</p>
      </div>

      <div v-if="textResult" class="result-section">
        <h3><i class="fas fa-check-circle"></i> Модель сгенерирована и развернута!</h3>
        <div class="result-info">
          <div class="info-row">
            <span>Файл:</span>
            <span>{{ textResult.mesh_path }}</span>
          </div>
          <div class="info-row">
            <span>Вершин:</span>
            <span>{{ textResult.vertices }}</span>
          </div>
          <div class="info-row">
            <span>Граней:</span>
            <span>{{ textResult.faces }}</span>
          </div>
          <div class="info-row">
            <span>Устройство:</span>
            <span>{{ textResult.device }}</span>
          </div>
          <div class="info-row" v-if="textResult.cached">
            <span><i class="fas fa-history"></i> Из кэша</span>
          </div>
          <div class="info-row" v-if="generationProgress.step">
            <span>Последний шаг:</span>
            <span>{{ generationProgress.step }} ({{ generationProgress.percentage }}%)</span>
          </div>
        </div>
        
        <!-- SVG Preview -->
        <div v-if="svgResult" class="svg-preview-section">
          <h4><i class="fas fa-vector-square"></i> SVG Развертка</h4>
          <div class="svg-preview">
            <div v-html="svgResult" class="svg-container"></div>
          </div>
          <div class="result-actions">
            <button class="btn-secondary" @click="openTextInViewer">
              <i class="fas fa-eye"></i> Просмотреть 3D
            </button>
            <button class="btn-secondary" @click="downloadTextModel">
              <i class="fas fa-download"></i> Скачать модель
            </button>
            <button class="btn-secondary" @click="downloadSvg">
              <i class="fas fa-file-download"></i> Скачать SVG
            </button>
            <button class="btn-secondary" @click="importTextToProject">
              <i class="fas fa-folder-plus"></i> Импортировать в проект
            </button>
          </div>
        </div>
        <div v-else class="result-actions">
          <button class="btn-secondary" @click="openTextInViewer">
            <i class="fas fa-eye"></i> Просмотреть
          </button>
          <button class="btn-secondary" @click="downloadTextModel">
            <i class="fas fa-download"></i> Скачать
          </button>
          <button class="btn-secondary" @click="importTextToProject">
            <i class="fas fa-folder-plus"></i> Импортировать в проект
          </button>
        </div>
      </div>

      <div v-if="textError" class="error-section">
        <h3><i class="fas fa-exclamation-triangle"></i> Ошибка</h3>
        <p>{{ textError }}</p>
        <button class="btn-secondary" @click="textError = ''">Закрыть</button>
      </div>
    </div>

    <div v-if="activeTab === 'settings'" class="tab-content">
      <div class="settings-tab">
        <h3>Настройки TripoSR</h3>
        <div class="form-group">
          <label>Путь к Python</label>
          <input type="text" v-model="pythonPath" placeholder="python" />
        </div>
        <div class="form-group">
          <label>Размер изображения</label>
          <input type="number" v-model="imageSize" min="256" max="1024" />
        </div>
        <div class="form-group">
          <label>Чанк размер</label>
          <input type="number" v-model="chunkSize" min="1024" max="32768" />
        </div>
        <button class="btn-secondary" @click="saveSettings">
          Сохранить настройки
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useProjectStore } from '../stores/projectStore'

const store = useProjectStore()

// Проверка доступности Tauri
const isTauriAvailable = computed(() => {
  return typeof window !== 'undefined' && '__TAURI__' in window
})

const activeTab = ref('image')
const dragOver = ref(false)
const selectedImage = ref('')
const selectedFile = ref<File | null>(null)
const quality = ref('balanced')
const format = ref('obj')
const useCache = ref(true)
const generating = ref(false)
const progress = ref(0)
const progressText = ref('')
const result = ref<any>(null)
const error = ref('')
const pythonPath = ref('python')
const imageSize = ref(384)
const chunkSize = ref(8192)

const fileInput = ref<HTMLInputElement | null>(null)

// Text-to-3D variables
const textPrompt = ref('')
const textQuality = ref('medium')
const textFormat = ref('obj')
const textUseCache = ref(true)
const generatingText = ref(false)
const textProgress = ref(0)
const textProgressText = ref('')
const textResult = ref<any>(null)
const textError = ref('')

// New Replicate API integration variables
const apiKeyChecked = ref(false)
const apiKeyMissing = ref(false)
const svgResult = ref<string>('')
const generationProgress = ref({
  step: '',
  percentage: 0,
  message: ''
})

function selectImage() {
  fileInput.value?.click()
}

function onFileSelected(e: Event) {
  const target = e.target as HTMLInputElement
  if (target.files && target.files[0]) {
    const file = target.files[0]
    handleFile(file)
  }
}

function onDrop(e: DragEvent) {
  dragOver.value = false
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    const file = files[0]
    if (file && file.type.startsWith('image/')) {
      handleFile(file)
    } else {
      error.value = 'Пожалуйста, выберите изображение'
    }
  }
}

function handleFile(file: File) {
  if (file.size > 10 * 1024 * 1024) {
    error.value = 'Файл слишком большой (максимум 10 МБ)'
    return
  }
  selectedFile.value = file
  const reader = new FileReader()
  reader.onload = (e) => {
    selectedImage.value = e.target?.result as string
  }
  reader.readAsDataURL(file)
}

function clearImage() {
  selectedImage.value = ''
  selectedFile.value = null
  if (fileInput.value) {
    fileInput.value.value = ''
  }
}

async function generate() {
  if (!selectedFile.value) return

  if (!isTauriAvailable.value) {
    error.value = 'AI генерация доступна только в десктопном приложении Tauri'
    return
  }

  generating.value = true
  progress.value = 10
  progressText.value = 'Подготовка...'
  error.value = ''
  result.value = null

  try {
    // Сохраняем временный файл изображения
    const tempPath = await store.saveTempImage(selectedFile.value)
    progress.value = 30
    progressText.value = 'Запуск TripoSR...'

    // Динамический импорт invoke, чтобы избежать ошибок в браузере
    const { invoke } = await import('@tauri-apps/api/core')
    // Вызываем Tauri команду
    const response = await invoke('ai_generate_from_image', {
      payload: {
        image_path: tempPath,
        format: format.value,
        quality: quality.value,
      }
    })

    progress.value = 90
    progressText.value = 'Обработка результата...'

    // Предполагаем, что ответ соответствует структуре triposr_generator.py
    result.value = response as any

    progress.value = 100
    progressText.value = 'Готово!'
  } catch (err: any) {
    error.value = err.toString()
    console.error('Generation error:', err)
  } finally {
    generating.value = false
  }
}

// New functions for Replicate API integration
async function checkAndSaveApiKey(): Promise<boolean> {
  if (!isTauriAvailable.value) {
    textError.value = 'API ключ доступен только в десктопном приложении Tauri'
    return false
  }

  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const hasKey = await invoke<boolean>('has_api_key')
    
    if (hasKey) {
      apiKeyChecked.value = true
      apiKeyMissing.value = false
      return true
    }
    
    // Ask for API key
    const key = prompt('Введите ваш API ключ Replicate (получите на replicate.com):')
    if (!key) {
      textError.value = 'API ключ не введен. Генерация невозможна.'
      return false
    }
    
    await invoke('save_api_key', { key })
    apiKeyChecked.value = true
    apiKeyMissing.value = false
    return true
  } catch (err: any) {
    textError.value = `Ошибка проверки API ключа: ${err.toString()}`
    console.error('API key check error:', err)
    return false
  }
}

async function generateFromText() {
  if (!textPrompt.value) return

  if (!isTauriAvailable.value) {
    textError.value = 'AI генерация доступна только в десктопном приложении Tauri'
    return
  }

  // Check API key
  const hasKey = await checkAndSaveApiKey()
  if (!hasKey) {
    apiKeyMissing.value = true
    generatingText.value = false
    return
  }

  generatingText.value = true
  textProgress.value = 10
  textProgressText.value = 'Подготовка...'
  textError.value = ''
  textResult.value = null
  svgResult.value = ''
  generationProgress.value = { step: '', percentage: 0, message: '' }

  try {
    // Динамический импорт необходимых модулей Tauri
    const { invoke } = await import('@tauri-apps/api/core')
    const { listen } = await import('@tauri-apps/api/event')

    // Слушаем события прогресса
    const unlisten = await listen<{ step: string, percentage: number, message: string }>('generate-progress', (event) => {
      generationProgress.value = event.payload
      textProgress.value = event.payload.percentage
      textProgressText.value = `${event.payload.step}: ${event.payload.message}`
    })

    // Вызываем новую команду generate_and_unfold
    const svgContent = await invoke<string>('generate_and_unfold', {
      prompt: textPrompt.value
    })

    // Успешное завершение
    svgResult.value = svgContent
    textProgress.value = 100
    textProgressText.value = 'Готово! SVG сгенерирован.'
    
    // Создаем объект результата для совместимости с существующим UI
    textResult.value = {
      mesh_path: 'generated_from_replicate.glb',
      vertices: 'N/A',
      faces: 'N/A',
      device: 'Replicate API',
      cached: false,
      svg: svgContent
    }

    // Отписываемся от событий
    unlisten()
  } catch (err: any) {
    textError.value = err.toString()
    console.error('Text generation error:', err)
  } finally {
    generatingText.value = false
  }
}

function openTextInViewer() {
  if (textResult.value?.mesh_path) {
    store.loadModel(textResult.value.mesh_path)
  }
}

async function downloadTextModel() {
  if (!textResult.value?.mesh_path) return

  if (!isTauriAvailable.value) {
    textError.value = 'Скачивание моделей доступно только в десктопном приложении Tauri'
    return
  }

  try {
    const { save } = await import('@tauri-apps/plugin-dialog')
    const { readFile, writeFile } = await import('@tauri-apps/plugin-fs')
    const filePath = await save({
      defaultPath: `hunyuan_model.${textFormat.value}`,
      filters: [{
        name: '3D Model',
        extensions: [textFormat.value]
      }]
    })
    if (filePath) {
      // Используем Tauri readFile вместо fetch для локальных файлов
      const content = await readFile(textResult.value.mesh_path)
      await writeFile(filePath, content)
    }
  } catch (err: any) {
    textError.value = `Ошибка скачивания: ${err.toString()}`
    console.error('Text download error:', err)
  }
}

async function downloadSvg() {
  if (!svgResult.value) return

  if (!isTauriAvailable.value) {
    textError.value = 'Скачивание SVG доступно только в десктопном приложении Tauri'
    return
  }

  try {
    const { save } = await import('@tauri-apps/plugin-dialog')
    const { writeFile } = await import('@tauri-apps/plugin-fs')
    const filePath = await save({
      defaultPath: 'unfolded_model.svg',
      filters: [{
        name: 'SVG Image',
        extensions: ['svg']
      }]
    })
    if (filePath) {
      await writeFile(filePath, svgResult.value)
    }
  } catch (err: any) {
    textError.value = `Ошибка скачивания SVG: ${err.toString()}`
    console.error('SVG download error:', err)
  }
}

function importTextToProject() {
  if (textResult.value?.mesh_path) {
    store.importModel(textResult.value.mesh_path)
  }
}

function openInViewer() {
  if (result.value?.mesh_path) {
    store.loadModel(result.value.mesh_path)
  }
}

async function downloadModel() {
  if (!result.value?.mesh_path) return

  if (!isTauriAvailable.value) {
    error.value = 'Скачивание моделей доступно только в десктопном приложении Tauri'
    return
  }

  try {
    const { save } = await import('@tauri-apps/plugin-dialog')
    const { readFile, writeFile } = await import('@tauri-apps/plugin-fs')
    const filePath = await save({
      defaultPath: `triposr_model.${format.value}`,
      filters: [{
        name: '3D Model',
        extensions: [format.value]
      }]
    })
    if (filePath) {
      // Используем Tauri readFile вместо fetch для локальных файлов
      const content = await readFile(result.value.mesh_path)
      await writeFile(filePath, content)
    }
  } catch (err: any) {
    error.value = `Ошибка скачивания: ${err.toString()}`
    console.error('Download error:', err)
  }
}

function importToProject() {
  if (result.value?.mesh_path) {
    store.importModel(result.value.mesh_path)
  }
}

function saveSettings() {
  // Сохранение настроек в localStorage или конфиг
  localStorage.setItem('triposr_python', pythonPath.value)
  localStorage.setItem('triposr_image_size', imageSize.value.toString())
  localStorage.setItem('triposr_chunk_size', chunkSize.value.toString())
  alert('Настройки сохранены')
}

onMounted(() => {
  const savedPython = localStorage.getItem('triposr_python')
  if (savedPython) pythonPath.value = savedPython
  const savedSize = localStorage.getItem('triposr_image_size')
  if (savedSize) imageSize.value = parseInt(savedSize)
  const savedChunk = localStorage.getItem('triposr_chunk_size')
  if (savedChunk) chunkSize.value = parseInt(savedChunk)
})
</script>

<style scoped>
.ai-generator {
  padding: 1.5rem;
  background: rgba(15, 23, 42, 0.9);
  border-radius: 16px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  color: #e2e8f0;
  height: 100%;
  overflow-y: auto;
}

.header {
  margin-bottom: 1.5rem;
}

.header h2 {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  font-size: 1.5rem;
  color: #cbd5e1;
  margin: 0;
}

.subtitle {
  color: #94a3b8;
  margin-top: 0.25rem;
}

.tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  padding-bottom: 0.5rem;
}

.tab {
  padding: 0.5rem 1rem;
  background: transparent;
  border: none;
  color: #94a3b8;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
}

.tab:hover {
  background: rgba(255, 255, 255, 0.05);
}

.tab.active {
  background: rgba(59, 130, 246, 0.2);
  color: #60a5fa;
}

.tab-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.upload-section {
  display: flex;
  gap: 1.5rem;
  align-items: flex-start;
}

.dropzone {
  flex: 1;
  border: 2px dashed rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  padding: 3rem 2rem;
  text-align: center;
  transition: all 0.3s;
  background: rgba(30, 41, 59, 0.5);
}

.dropzone.drag-over {
  border-color: #60a5fa;
  background: rgba(59, 130, 246, 0.1);
}

.dropzone i {
  font-size: 3rem;
  color: #94a3b8;
  margin-bottom: 1rem;
}

.dropzone h3 {
  margin: 0 0 0.5rem;
  color: #cbd5e1;
}

.dropzone p {
  margin: 0.5rem 0;
  color: #94a3b8;
}

.btn-primary {
  background: #3b82f6;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: #2563eb;
}

.hint {
  font-size: 0.875rem;
  margin-top: 1rem;
}

.preview {
  position: relative;
  width: 200px;
  height: 200px;
  border-radius: 12px;
  overflow: hidden;
  border: 2px solid rgba(255, 255, 255, 0.1);
}

.preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.btn-icon {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  background: rgba(0, 0, 0, 0.7);
  border: none;
  color: white;
  width: 2rem;
  height: 2rem;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.settings-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  background: rgba(30, 41, 59, 0.8);
  padding: 1.5rem;
  border-radius: 12px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  color: #cbd5e1;
  font-weight: 500;
}

select, input[type="text"], input[type="number"], textarea {
  background: rgba(15, 23, 42, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  padding: 0.5rem 0.75rem;
  color: #e2e8f0;
  outline: none;
  font-family: inherit;
  font-size: 0.95rem;
}

select:focus, input:focus, textarea:focus {
  border-color: #60a5fa;
}

textarea {
  resize: vertical;
  min-height: 80px;
}

.actions {
  display: flex;
  justify-content: center;
}

.btn-generate {
  background: linear-gradient(135deg, #8b5cf6, #3b82f6);
  color: white;
  border: none;
  padding: 1rem 2rem;
  border-radius: 10px;
  font-size: 1.125rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  transition: transform 0.2s, box-shadow 0.2s;
}

.btn-generate:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 10px 20px rgba(59, 130, 246, 0.3);
}

.btn-generate:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.progress-section {
  background: rgba(30, 41, 59, 0.8);
  padding: 1.5rem;
  border-radius: 12px;
}

.progress-bar {
  height: 8px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #8b5cf6, #3b82f6);
  border-radius: 4px;
  transition: width 0.3s;
}

.progress-text {
  text-align: center;
  color: #94a3b8;
  font-size: 0.875rem;
}

.result-section {
  background: rgba(30, 41, 59, 0.8);
  padding: 1.5rem;
  border-radius: 12px;
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.result-section h3 {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #10b981;
  margin-top: 0;
}

.result-info {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin: 1.5rem 0;
}

.info-row {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.info-row span:first-child {
  color: #cbd5e1;
}

.info-row span:last-child {
  color: #e2e8f0;
  font-weight: 500;
}

.result-actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.1);
  color: #e2e8f0;
  border: 1px solid rgba(255, 255, 255, 0.2);
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.2);
}

.error-section {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.3);
  padding: 1.5rem;
  border-radius: 12px;
}

.error-section h3 {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #ef4444;
  margin-top: 0;
}

.settings-tab {
  background: rgba(30, 41, 59, 0.8);
  padding: 1.5rem;
  border-radius: 12px;
}

.settings-tab h3 {
  margin-top: 0;
  color: #cbd5e1;
}

/* SVG Preview Styles */
.svg-preview-section {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.svg-preview-section h4 {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #60a5fa;
  margin-top: 0;
  margin-bottom: 1rem;
}

.svg-preview {
  background: rgba(15, 23, 42, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  padding: 1rem;
  max-height: 300px;
  overflow: auto;
}

.svg-container {
  display: flex;
  justify-content: center;
  align-items: center;
}

.svg-container svg {
  max-width: 100%;
  max-height: 250px;
}
</style>