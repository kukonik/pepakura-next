<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { useProjectStore } from '@/stores/projectStore'
import ModelViewer3D from '@/components/ModelViewer3D.vue'
import SheetPreview from '@/components/SheetPreview.vue'
// Если lucide-vue-next не установлен, используйте классы fas из FontAwesome, как в вашем оригинальном HTML.
// import { ChevronLeft, ChevronRight, Layers, FileText, Box, Scissors, Download, Upload, Settings, HelpCircle, Search, Sparkles, Globe, Save, Palette, Maximize2, RefreshCw } from 'lucide-vue-next'

const store = useProjectStore()

// === UI State ===
const activeView = ref<'3d' | '2d' | 'settings'>('3d')
const activeTool = ref<string>('import')
const searchQuery = ref<string>('')
const isLoading = ref(false)
const isRightPanelCollapsed = ref(false)
const isLeftPanelCollapsed = ref(false)
const isFullscreen = ref(false)
const materialSize = ref<string>('A4')
const flapWidth = ref<number>(10)
const showNumbers = ref<boolean>(true)
const showFolds = ref<boolean>(true)
const aiTip = ref<string>('Наведите на деталь для подсказки')

const showMessage = (text: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') => {
  console.log(`[${type.toUpperCase()}] ${text}`)
}

// === Import 3D ===
const import3D = async () => {
  isLoading.value = true
  showMessage('Выберите 3D модель...', 'info')
  try {
    const selected = await open({
      filters: [{ name: '3D Models', extensions: ['obj', 'stl', 'ply', 'fbx', 'gltf', 'glb', 'pdo'] }],
      multiple: false,
    })
    if (!selected) return
    const filePath = Array.isArray(selected) ? selected : selected

    const result = await invoke<{
      success: boolean
      modelPath: string
      info?: unknown
    }>('import_3d_model', { filePath })

    if (result.success) {
      store.modelPath = result.modelPath
      // @ts-expect-error модель инфо может расширяться
      store.modelInfo = result.info ?? null
      store.projectName =
        filePath.split(/[\\/]/).pop()?.replace(/\.[^.]+$/, '') ?? 'Без названия'
      activeView.value = '3d'
      showMessage('3D модель загружена', 'success')
    }
  } catch (e) {
    showMessage(`Ошибка: ${e instanceof Error ? e.message : String(e)}`, 'error')
  } finally {
    isLoading.value = false
  }
}

// === Import 2D / Text ===
const import2D = async () => { showMessage('Импорт 2D...', 'info') }
const importFromText = async () => { showMessage('Текстовый импорт...', 'info') }
const importFromWeb = async () => { showMessage('Веб-поиск...', 'info') }

// === Unfold ===
const unfoldModel = async () => {
  if (!store.modelPath) { showMessage('Сначала загрузите модель', 'warning'); return }
  isLoading.value = true
  showMessage('Создаём развёртку...', 'info')
  try {
    const svgContent = await invoke<string>('unfold_3d_model', { objPath: store.modelPath })
    store.svgContent = svgContent
    activeView.value = '2d'
    showMessage('Развёртка создана', 'success')
  } catch (e) {
    showMessage(`Ошибка: ${e instanceof Error ? e.message : String(e)}`, 'error')
  } finally {
    isLoading.value = false
  }
}

// === Export ===
const exportResult = async (format: string) => {
  if (!store.svgContent && format === 'svg') { showMessage('Нет данных', 'warning'); return }
  try {
    const path = await save({ filters: [{ name: format.toUpperCase(), extensions: [format] }], defaultPath: `export.${format}` })
    if (path) {
      if (format === 'svg') await writeTextFile(path, store.svgContent)
      else await invoke('export_unfold', { svgContent: store.svgContent, format, outputPath: path })
      showMessage(`Экспортировано: ${path.split(/[\\/]/).pop()}`, 'success')
    }
  } catch (e) { showMessage(`Ошибка экспорта: ${e}`, 'error') }
}

const exportSVG = async () => await exportResult('svg')
const exportPDF = async () => await exportResult('pdf')
const exportDXF = async () => await exportResult('dxf')
const exportSTL = async () => await exportResult('stl')
const exportOBJ = async () => await exportResult('obj')
const exportPNG = async () => await exportResult('png')

// === Project ===
const saveProject = async () => {
  try {
    const path = await save({ filters: [{ name: 'Pepakura Project', extensions: ['pepa'] }], defaultPath: `${store.projectName || 'project'}.pepa` })
    if (path) {
      await invoke('save_project', { path, data: { modelPath: store.modelPath, svgContent: store.svgContent } })
      store.markSaved()
      showMessage('Проект сохранён', 'success')
    }
  } catch (e) { showMessage(`Ошибка сохранения: ${e}`, 'error') }
}

const openSettings = () => { activeView.value = 'settings' }
const switchTool = (tool: 'import' | 'edit' | 'unfold' | 'texture' | 'arrange') => { activeTool.value = tool }

// Добавляем вычисляемые свойства для статуса и количества деталей
const statusMessage = computed(() => store.statusMessage)
const detailCount = computed(() => store.detailCount)
const toggleFullscreen = () => { document.documentElement.requestFullscreen?.() }
const resetView = () => { showMessage('Вид сброшен', 'info') }
const toggleTheme = () => { store.theme = store.theme === 'dark' ? 'light' : 'dark' }

// Обработчик события колесика мыши для 2D просмотра
const handleWheel = (event: WheelEvent) => {
  // TODO: Добавить логику обработки колесика мыши для масштабирования
  console.log('Wheel event:', event)
}

const aiSearch = async () => { showMessage('AI поиск...', 'info') }
const webSearch = () => { 
  const q = encodeURIComponent(searchQuery.value + ' papercraft 3d model')
  window.open(`https://duckduckgo.com/?q=${q}`, '_blank') 
}
const askAI = async () => { showMessage('AI запрос...', 'info') }

const modelPath = computed(() => store.modelPath)
const svgContent = computed(() => store.svgContent)

onMounted(async () => { try { await invoke('health_check') } catch { } })
</script>

<template>
  <div class="editor-layout">
    <!-- ВЕРХНЯЯ ПАНЕЛЬ С ПОИСКОМ -->
    <div class="top-bar">
      <div class="logo">
        <!-- Если lucide-vue-next не установлен, раскомментируйте и используйте fas -->
        <i class="fas fa-cube logo-icon"></i>
        <span class="logo-text">Pepakura Next</span>
      </div>
      
      <div class="search-container">
        <i class="fas fa-search search-icon"></i>
        <input 
          type="text" 
          class="search-box" 
          v-model="searchQuery" 
          placeholder="Опишите модель, задайте вопрос AI или введите веб-адрес..."
          @keyup.enter="aiSearch"
        >
        <div class="search-actions">
          <button class="search-btn secondary" @click="webSearch">
            <i class="fas fa-globe"></i> Веб
          </button>
          <button class="search-btn primary" @click="aiSearch">
            <i class="fas fa-search"></i>
            AI Поиск
          </button>
        </div>
      </div>
      
      <div class="action-buttons">
        <button class="action-btn" @click="saveProject">
          <i class="fas fa-save"></i> Сохранить
        </button>
        <button class="action-btn" @click="openSettings">
          <i class="fas fa-cog"></i>
        </button>
      </div>
    </div>

    <!-- ОСНОВНАЯ РАBOЧАЯ ОБЛАСТЬ -->
    <div class="main-content">
      <!-- ЛЕВАЯ ПАНЕЛЬ ИНСТРУМЕНТОВ -->
      <div class="left-toolbar">
        <div 
          class="tool-btn" 
          :class="{ active: activeTool === 'import' }"
          @click="switchTool('import')"
          title="Импорт"
        >
          <i class="fas fa-file-import"></i>
          <span>Импорт</span>
        </div>
        <div 
          class="tool-btn" 
          :class="{ active: activeTool === 'edit' }"
          @click="switchTool('edit')"
          title="Редактирование"
        >
          <i class="fas fa-edit"></i>
          <span>Правка</span>
        </div>
        <div 
          class="tool-btn" 
          :class="{ active: activeTool === 'unfold' }"
          @click="unfoldModel"
          title="Развёртка"
        >
          <i class="fas fa-cut"></i>
          <span>Развертка</span>
        </div>
        <div 
          class="tool-btn" 
          :class="{ active: activeTool === 'texture' }"
          @click="switchTool('texture')"
          title="Текстуры"
        >
          <i class="fas fa-paint-brush"></i>
          <span>Текстуры</span>
        </div>
        <div 
          class="tool-btn" 
          :class="{ active: activeTool === 'arrange' }"
          @click="switchTool('arrange')"
          title="Компоновка"
        >
          <i class="fas fa-th"></i>
          <span>Компоновка</span>
        </div>
        <div style="margin-top: auto;">
          <div 
            class="tool-btn" 
            @click="toggleTheme"
            title="Тема"
          >
            <i class="fas fa-moon"></i>
            <span>Тема</span>
          </div>
        </div>
      </div>

      <!-- ЦЕНТР: УВЕЛИЧЕННЫЙ ПРОСМОТРЩИК -->
      <div class="viewer-section">
        <div class="viewer-header">
          <h2 class="viewer-title">3D Редактор и Просмотрщик</h2>
          <div class="viewer-actions">
            <div class="view-tabs">
              <button 
                class="view-tab-btn" 
                :class="{ active: activeView === '3d' }" 
                @click="activeView = '3d'"
              >
                3D Модель
              </button>
              <button 
                class="view-tab-btn" 
                :class="{ active: activeView === '2d' }" 
                @click="activeView = '2d'"
              >
                2D Развёртка
              </button>
            </div>
            <button class="action-btn" @click="toggleFullscreen">
              <i class="fas fa-expand"></i> Полный экран
            </button>
            <button class="action-btn" @click="resetView">
              <i class="fas fa-sync-alt"></i> Сбросить вид
            </button>
            <div class="badge">GPU Ускорение</div>
          </div>
        </div>
        
        <div class="viewer-container">
          <!-- Placeholder или Загрузчик -->
          <div v-if="!modelPath && activeView === '3d'" class="viewer-placeholder">
            <div class="placeholder-content">
              <div class="placeholder-icon">📂</div>
              <h3 style="margin-bottom: 0.5rem; color: #cbd5e1;">Загрузите модель для начала работы</h3>
              <p style="font-size: 0.95rem; max-width: 500px; margin: 0 auto 1.5rem;">
                Перетащите файл сюда или используйте кнопки импорта.<br>
                Поддерживаемые форматы: OBJ, STL, GLTF, SVG, PNG, JPG
              </p>
              <div style="display: flex; gap: 1rem; justify-content: center;">
                <button class="search-btn primary" @click="import3D">
                  <i class="fas fa-cube"></i> Импорт 3D
                </button>
              </div>
            </div>
          </div>

          <!-- 3D Viewer -->
          <ModelViewer3D 
            v-else-if="activeView === '3d'" 
            :model-path="modelPath" 
            style="width: 100%; height: 100%;"
          />

          <!-- 2D Viewer -->
          <template v-else-if="activeView === '2d'">
            <SheetPreview
              v-if="svgContent"
              :svg-content="svgContent"
              style="width: 100%; height: 100%;"
              @wheel.passive="handleWheel"
            />
            <div v-else class="viewer-placeholder">
              <div class="placeholder-content">
                <div class="placeholder-icon">📐</div>
                <h3 style="margin-bottom: 0.5rem; color: #cbd5e1;">Развёртка не создана</h3>
                <p style="font-size: 0.95rem; max-width: 500px; margin: 0 auto 1.5rem;">
                  Используйте инструмент "Развертка" для создания 2D развёртки из 3D модели.
                </p>
                <button class="search-btn primary" @click="unfoldModel" :disabled="!modelPath">
                  <i class="fas fa-cut"></i> Создать развёртку
                </button>
              </div>
            </div>
          </template>
          <div class="viewer-stats" v-if="modelPath || svgContent">
            <span v-if="activeView === '3d' && modelPath">3D Модель: {{ modelPath }}</span>
            <span v-else-if="activeView === '2d' && svgContent">2D Развёртка загружена</span>
          </div>
        </div>
      </div>

      <!-- ПРАВАЯ ПАНЕЛЬ: ИНСТРУМЕНТЫ И ЭКСПОРТ -->
      <div class="right-panel">
        <!-- СЕКЦИЯ ИМПОРТА -->
        <div class="panel-section">
          <div class="section-header">
            <h3 class="section-title"><i class="fas fa-download"></i> Импорт файлов</h3>
            <div class="badge">5+ форматов</div>
          </div>
          <div style="display: flex; flex-direction: column; gap: 0.8rem;">
            <button class="export-btn" @click="import3D">
              <i class="fas fa-cube"></i>
              <span>3D Модель</span>
              <small style="font-size: 0.75rem; color: #94a3b8;">OBJ, STL, GLTF</small>
            </button>
            <button class="export-btn" @click="import2D">
              <i class="fas fa-image"></i>
              <span>2D Изображение</span>
              <small style="font-size: 0.75rem; color: #94a3b8;">PNG, JPG, SVG</small>
            </button>
            <button class="export-btn" @click="importFromText">
              <i class="fas fa-font"></i>
              <span>Из текста (AI)</span>
              <small style="font-size: 0.75rem; color: #94a3b8;">Создать из описания</small>
            </button>
            <button class="export-btn" @click="importFromWeb">
              <i class="fas fa-cloud-download-alt"></i>
              <span>Из интернета</span>
              <small style="font-size: 0.75rem; color: #94a3b8;">Загрузить по ссылке</small>
            </button>
          </div>
        </div>

        <!-- СЕКЦИЯ ЭКСПОРТА -->
        <div class="panel-section">
          <div class="section-header">
            <h3 class="section-title"><i class="fas fa-share-square"></i> Экспорт развёрток</h3>
            <div class="badge">Профессиональный</div>
          </div>
          <div class="export-grid">
            <button class="export-btn" @click="exportPDF">
              <i class="fas fa-file-pdf"></i>
              <span>PDF</span>
            </button>
            <button class="export-btn" @click="exportSVG">
              <i class="fas fa-drafting-compass"></i>
              <span>SVG</span>
            </button>
            <button class="export-btn" @click="exportDXF">
              <i class="fas fa-ruler-combined"></i>
              <span>DXF</span>
            </button>
            <button class="export-btn" @click="exportSTL">
              <i class="fas fa-cube"></i>
              <span>STL</span>
            </button>
            <button class="export-btn" @click="exportOBJ">
              <i class="fas fa-shapes"></i>
              <span>OBJ</span>
            </button>
            <button class="export-btn" @click="exportPNG">
              <i class="fas fa-image"></i>
              <span>PNG</span>
            </button>
          </div>
        </div>

        <!-- СЕКЦИЯ НАСТРОЕК -->
        <div class="panel-section">
          <div class="section-header">
            <h3 class="section-title"><i class="fas fa-sliders-h"></i> Настройки развёртки</h3>
          </div>
          <div class="form-group">
            <label><i class="fas fa-ruler"></i> Размер материала</label>
            <select class="form-control" v-model="materialSize">
              <option value="A4">A4 (210×297 мм)</option>
              <option value="A3" selected>A3 (297×420 мм)</option>
              <option value="Letter">Letter (216×279 мм)</option>
              <option value="Custom">Custom...</option>
            </select>
          </div>
          <div class="form-group">
            <label><i class="fas fa-expand-alt"></i> Ширина клапанов: <span id="flapValue">{{ flapWidth }} мм</span></label>
            <input type="range" v-model.number="flapWidth" min="5" max="25" step="1" class="form-control" />
          </div>
          <div style="display: flex; gap: 1.5rem; margin-top: 1rem;">
            <label style="display: flex; align-items: center; gap: 0.5rem;">
              <div class="toggle">
                <input type="checkbox" v-model="showNumbers">
                <span class="slider"></span>
              </div>
              <span>Нумерация</span>
            </label>
            <label style="display: flex; align-items: center; gap: 0.5rem;">
              <div class="toggle">
                <input type="checkbox" v-model="showFolds">
                  <span class="slider"></span>
              </div>
              <span>Линии сгиба</span>
            </label>
          </div>
        </div>

        <!-- СЕКЦИЯ AI ПОМОЩНИКА -->
        <div class="panel-section" style="background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.9));">
          <div class="section-header">
            <h3 class="section-title"><i class="fas fa-robot"></i> AI Ассистент</h3>
            <div class="badge" style="background: rgba(16, 185, 129, 0.2); color: #10b981;">ОНЛАЙН</div>
          </div>
          <p style="font-size: 0.9rem; color: #94a3b8; margin-bottom: 1rem;">
            <i class="fas fa-lightbulb"></i> 
            <span id="aiTip">{{ aiTip }}</span>
          </p>
          <button class="search-btn primary" @click="askAI" style="width: 100%;">
            <i class="fas fa-comment-dots"></i> Задать вопрос AI
          </button>
        </div>
      </div>
    </div>

    <!-- НИЖНИЙ СТАТУС-БАР -->
    <div class="status-bar">
      <div class="status-item">
        <div class="status-dot"></div>
        <span id="statusMessage">Готово</span>
      </div>
      <div class="status-item">
        <i class="fas fa-microchip"></i>
        <span id="performanceStatus">Режим: Авто (AI доступен)</span>
        <div class="progress-bar">
          <div class="progress-fill"></div>
        </div>
      </div>
      <div class="status-item">
        <i class="fas fa-hdd"></i>
        <span>Память: <span id="memoryUsage">124 МБ</span></span>
        <i class="fas fa-layer-group" style="margin-left: 1rem;"></i>
        <span id="detailCount">Деталей: {{ store.detailCount || 0 }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Основной контейнер */
.editor-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: linear-gradient(155deg, #0b1120 0%, #1a202c 100%);
  color: #e2e8f0;
  font-family: 'Inter', 'Segoe UI', sans-serif;
  overflow: hidden;
}

/* ВЕРХНЯЯ ПАНЕЛЬ: Поиск и основные действия */
.top-bar {
  background: rgba(15, 23, 42, 0.92);
  backdrop-filter: blur(15px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  padding: 0.8rem 1.5rem;
  display: flex;
  align-items: center;
  gap: 1.5rem;
  flex-shrink: 0;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.25);
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  font-weight: 700;
}

.logo-icon {
  color: #60a5fa;
  font-size: 1.5rem;
}

.logo-text {
  font-size: 1.3rem;
  background: linear-gradient(90deg, #60a5fa, #38bdf8);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.search-container {
  flex: 1;
  max-width: 700px;
  position: relative;
}

.search-box {
  width: 100%;
  padding: 0.85rem 1rem 0.85rem 3rem;
  background: rgba(30, 41, 59, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 12px;
  color: #e2e8f0;
  font-size: 0.95rem;
  transition: all 0.3s ease;
}

.search-box:focus {
  outline: none;
  border-color: #60a5fa;
  box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
}

.search-icon {
  position: absolute;
  left: 1rem;
  top: 0.95rem;
  transform: none;
  color: #94a3b8;
}

.search-actions {
  position: absolute;
  right: 0.5rem;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  gap: 0.5rem;
}

.search-btn {
  padding: 0.7rem 1.2rem;
  border-radius: 10px;
  border: none;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
  white-space: nowrap;
}

.search-btn.primary {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.search-btn.secondary {
  background: rgba(71, 85, 105, 0.6);
  color: #e2e8f0;
}

.search-btn:hover {
  transform: translateY(-2px);
}

.search-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}

.search-btn .icon {
  width: 1.2em;
  height: 1.2em;
}

.action-btn {
  padding: 0.6rem 1.2rem;
  background: rgba(30, 41, 59, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 500;
  transition: all 0.2s;
}

.action-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.action-btn .icon {
  width: 1.2em;
  height: 1.2em;
}

/* ОСНОВНАЯ РАБОЧАЯ ОБЛАСТЬ */
.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
  position: relative;
}

/* ЛЕВАЯ ПАНЕЛЬ: Компактные инструменты */
.left-toolbar {
  width: 70px;
  background: rgba(15, 23, 42, 0.7);
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  padding: 1.5rem 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.8rem;
  flex-shrink: 0;
}

.tool-btn {
  width: 50px;
  height: 50px;
  background: rgba(30, 41, 59, 0.8);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  font-size: 0.75rem;
  transition: all 0.2s;
}

.tool-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  border-color: #60a5fa;
  color: #fff;
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.tool-btn.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  border: none;
  box-shadow: 0 0 15px rgba(59, 130, 246, 0.5);
}

/* ЦЕНТРАЛЬНАЯ ОБЛАСТЬ: Увеличенный просмотрщик */
.viewer-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 1.5rem;
  min-width: 0;
}

.viewer-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.2rem;
}

.viewer-title {
  font-size: 1.4rem;
  font-weight: 600;
  color: #f1f5f9;
}

.viewer-actions {
  display: flex;
  align-items: center;
  gap: 0.8rem;
}

/* Стили для переключателя вкладок */
.view-tabs {
  display: flex;
  background: rgba(30, 41, 59, 0.8);
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  overflow: hidden;
}

.view-tab-btn {
  padding: 0.6rem 1.2rem;
  background: transparent;
  border: none;
  color: #cbd5e1;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
}

.view-tab-btn.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  box-shadow: 0 0 10px rgba(59, 130, 246, 0.3);
}

.view-tab-btn:hover:not(.active) {
  background: rgba(56, 70, 95, 0.9);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.viewer-container {
  flex: 1;
  background: rgba(15, 23, 42, 0.9);
  border-radius: 16px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  position: relative;
  box-shadow: inset 0 0 30px rgba(0, 0, 0, 0.3);
}

.viewer-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #94a3b8;
  text-align: center;
  padding: 2rem;
}

.placeholder-icon {
  font-size: 4rem;
  margin-bottom: 1.5rem;
  opacity: 0.3;
}

.viewer-stats {
  position: absolute;
  bottom: 1rem;
  left: 1rem;
  background: rgba(0, 0, 0, 0.5);
  padding: 0.5rem 1rem;
  border-radius: 8px;
  font-size: 0.85rem;
  color: #cbd5e1;
}

/* ПРАВАЯ ПАНЕЛЬ: Расширенные настройки */
.right-panel {
  width: 380px;
  background: rgba(15, 23, 42, 0.7);
  border-left: 1px solid rgba(255, 255, 255, 0.08);
  padding: 1.5rem;
  overflow-y: auto;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 1.8rem;
}

.panel-section {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 14px;
  padding: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1.2rem;
}

.section-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #e2e8f0;
  display: flex;
  align-items: center;
  gap: 0.7rem;
}

.section-title i {
  color: #60a5fa;
}

.export-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.8rem;
  margin-top: 1rem;
}

.export-btn {
  padding: 0.9rem;
  background: rgba(30, 41, 59, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
}

.export-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  transform: translateY(-3px);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.export-btn i {
  font-size: 1.3rem;
}

.form-group {
  margin-bottom: 1.2rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  color: #cbd5e1;
}

.form-control {
  width: 100%;
  padding: 0.8rem;
  background: rgba(15, 23, 42, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: #e2e8f0;
}

.form-control:focus {
  outline: none;
  border-color: #60a5fa;
  box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
}

/* НИЖНИЙ СТАТУС-БАР */
.status-bar {
  background: rgba(15, 23, 42, 0.95);
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  padding: 0.8rem 1.5rem;
  display: flex;
  justify-content: space-between;
  font-size: 0.85rem;
  flex-shrink: 0;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 0.7rem;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #10b981;
}

.progress-bar {
  width: 180px;
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #3b82f6, #60a5fa);
  width: 65%;
}

/* Утилиты */
.badge {
  background: rgba(96, 165, 250, 0.2);
  color: #60a5fa;
  padding: 0.2rem 0.6rem;
  border-radius: 10px;
  font-size: 0.75rem;
}

.toggle {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #475569;
  transition: .4s;
  border-radius: 34px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
}

input:checked + .slider {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
}

input:checked + .slider:before {
  transform: translateX(26px);
}

/* Адаптивность */
@media (max-width: 1200px) {
  .right-panel {
    width: 320px;
  }
  .search-container {
    max-width: 500px;
  }
}

@media (max-width: 992px) {
  .right-panel {
    width: 280px;
  }
  .export-grid {
    grid-template-columns: 1fr;
  }
  .search-container {
    max-width: 400px;
  }
}

@media (max-width: 768px) {
  .top-bar {
    flex-wrap: wrap;
  }
  .search-container {
    order: 3;
    max-width: 100%;
    margin-top: 0.5rem;
  }
  .viewer-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  .viewer-actions {
    width: 100%;
    flex-wrap: wrap;
  }
  .view-tabs {
    width: 100%;
  }
}