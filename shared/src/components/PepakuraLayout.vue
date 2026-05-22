<template>
  <div class="pepakura-layout">
    <!-- ВЕРХНЯЯ ПАНЕЛЬ С ПОИСКОМ -->
    <div class="top-bar">
      <div class="logo">
        <i class="fas fa-cube logo-icon"></i>
        <span class="logo-text">Pepakura Next</span>
      </div>

      <div class="search-container">
        <div class="search-input-wrapper">
          <i class="fas fa-search search-icon"></i>
          <input type="text" class="search-box" id="searchInput" placeholder="Опишите модель, задайте вопрос AI или введите веб-адрес...">
        </div>
        <div class="search-actions">
          <button class="search-btn secondary" @click="webSearch()">
            <i class="fas fa-globe"></i> Веб
          </button>
          <button class="search-btn primary" @click="aiSearch()">
            <i class="fas fa-robot"></i> AI Поиск
          </button>
        </div>
      </div>

      <div style="display: flex; gap: 0.8rem;">
        <button class="action-btn" @click="saveProject()">
          <i class="fas fa-save"></i> Сохранить
        </button>
        <button class="action-btn" @click="openSettings()">
          <i class="fas fa-cog"></i>
        </button>
      </div>
    </div>

    <!-- ОСНОВНАЯ РАБОЧАЯ ОБЛАСТЬ -->
    <div class="main-content">
      <!-- ЛЕВАЯ ПАНЕЛЬ ИНСТРУМЕНТОВ -->
      <div class="left-toolbar">
        <div class="tool-btn" :class="{ active: currentTool === 'import' }" @click="switchTool('import')" title="Импорт">
          <i class="fas fa-file-import"></i>
          <span>Импорт</span>
        </div>
        <div class="tool-btn" :class="{ active: currentTool === 'edit' }" @click="switchTool('edit')" title="Редактирование">
          <i class="fas fa-edit"></i>
          <span>Правка</span>
        </div>
        <div class="tool-btn" :class="{ active: currentTool === 'unfold' }" @click="switchTool('unfold')" title="Развёртка">
          <i class="fas fa-cut"></i>
          <span>Развертка</span>
        </div>
        <div class="tool-btn" :class="{ active: currentTool === 'texture' }" @click="switchTool('texture')" title="Текстуры">
          <i class="fas fa-paint-brush"></i>
          <span>Текстуры</span>
        </div>
        <div class="tool-btn" :class="{ active: currentTool === 'arrange' }" @click="switchTool('arrange')" title="Компоновка">
          <i class="fas fa-th"></i>
          <span>Компоновка</span>
        </div>
        <div style="margin-top: auto;">
          <div class="tool-btn" @click="toggleTheme()" title="Тема">
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
            <button class="action-btn" @click="toggleFullscreen()">
              <i class="fas fa-expand"></i> Полный экран
            </button>
            <button class="action-btn" @click="resetView()">
              <i class="fas fa-sync-alt"></i> Сбросить вид
            </button>
            <div class="badge">GPU Ускорение</div>
          </div>
        </div>

        <div class="viewer-container">
          <!-- Слот для центрального контента (ModelViewer) -->
          <slot name="center">
            <!-- Fallback, если слот не передан -->
            <ModelViewer :mesh-data="meshData" @request-import3-d="import3DWrapper" @request-import2-d="import2DWrapper" />
          </slot>
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
            <button class="export-btn" @click="import3DWrapper()">
              <i class="fas fa-cube"></i>
              <span>3D Модель</span>
              <small style="font-size: 0.75rem; color: #94a3b8;">OBJ, STL, GLTF</small>
            </button>
            <button class="export-btn" @click="import2DWrapper()">
              <i class="fas fa-image"></i>
              <span>2D Изображение</span>
              <small style="font-size: 0.75rem; color: #94a3b8;">PNG, JPG, SVG</small>
            </button>
            <button class="export-btn" @click="importFromText()">
              <i class="fas fa-font"></i>
              <span>Из текста (AI)</span>
              <small style="font-size: 0.75rem; color: #94a3b8;">Создать из описания</small>
            </button>
            <button class="export-btn" @click="importFromWeb()">
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
            <button class="export-btn" @click="exportPDF()">
              <i class="fas fa-file-pdf"></i>
              <span>PDF</span>
            </button>
            <button class="export-btn" @click="exportSVG()">
              <i class="fas fa-drafting-compass"></i>
              <span>SVG</span>
            </button>
            <button class="export-btn" @click="exportDXF()">
              <i class="fas fa-ruler-combined"></i>
              <span>DXF</span>
            </button>
            <button class="export-btn" @click="exportSTL()">
              <i class="fas fa-cube"></i>
              <span>STL</span>
            </button>
            <button class="export-btn" @click="exportOBJ()">
              <i class="fas fa-shapes"></i>
              <span>OBJ</span>
            </button>
            <button class="export-btn" @click="exportPNG()">
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
            <select class="form-control" id="materialSize">
              <option>A4 (210×297 мм)</option>
              <option selected>A3 (297×420 мм)</option>
              <option>Letter (216×279 мм)</option>
              <option>Custom...</option>
            </select>
          </div>
          <div class="form-group">
            <label><i class="fas fa-expand-alt"></i> Ширина клапанов: <span id="flapValue">12 мм</span></label>
            <input type="range" class="form-control" id="flapWidth" min="5" max="25" value="12" step="1">
          </div>
          <div style="display: flex; gap: 1.5rem; margin-top: 1rem;">
            <label style="display: flex; align-items: center; gap: 0.5rem;">
              <div class="toggle">
                <input type="checkbox" id="toggleNumbers" checked>
                <span class="slider"></span>
              </div>
              <span>Нумерация</span>
            </label>
            <label style="display: flex; align-items: center; gap: 0.5rem;">
              <div class="toggle">
                <input type="checkbox" id="toggleFolds">
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
            <span id="aiTip">Совет: Используйте "Импорт из текста" для создания моделей по описанию.</span>
          </p>
          <button class="search-btn primary" @click="askAI()" style="width: 100%;">
            <i class="fas fa-comment-dots"></i> Задать вопрос AI
          </button>
        </div>
      </div>
    </div>

    <!-- НИЖНИЙ СТАТУС-БАР -->
    <div class="status-bar">
      <div class="status-item">
        <div class="status-dot" :style="{ backgroundColor: statusColor }"></div>
        <span id="statusMessage">{{ statusMessage }}</span>
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
        <span id="detailCount">Деталей: {{ currentModel?.parts || 0 }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ModelViewer from './ModelViewer.vue';
import { useNotifications } from '../composables/useNotifications';
import { useModelStats } from '../composables/useModelStats';
import { useUIControls } from '../composables/useUIControls';
import { useImportExport } from '../composables/useImportExport';
import { useSearch } from '../composables/useSearch';
import { useModelLoader } from '../composables/useModelLoader';

// Используем composables
const { currentTool, switchTool, statusMessage, statusColor } = useNotifications();
const { currentModel } = useModelStats();
const { saveProject, openSettings, toggleTheme, toggleFullscreen, resetView } = useUIControls();
const { import3D, import2D, importFromText, importFromWeb, exportPDF, exportSVG, exportDXF, exportSTL, exportOBJ, exportPNG } = useImportExport();
const { aiSearch, webSearch, askAI, updateAITip } = useSearch();

// Используем loader для получения meshData
const { meshData, loadModel } = useModelLoader();

// Функции-обертки для обработки событий от ModelViewer и кнопок
const import3DWrapper = async () => {
  import3D(); // Показываем уведомление
  // Здесь должна быть логика получения пути файла (например, через tauri dialog)
  // Пока что вызываем заглушку с фиктивным путем
  await loadModel('dummy_path_for_demo.obj');
};

const import2DWrapper = () => {
  import2D();
  // loadModel('dummy_path_for_2d_demo.png'); // Пример для 2D, если поддерживается
};
</script>

<style scoped>
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
.logo { display: flex; align-items: center; gap: 0.8rem; font-weight: 700; }
.logo-icon { color: #60a5fa; font-size: 1.5rem; }
.logo-text { font-size: 1.3rem; background: linear-gradient(90deg, #60a5fa, #38bdf8); -webkit-background-clip: text; -webkit-text-fill-color: transparent; }

.search-container { 
    flex: 1; 
    max-width: 700px; 
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.search-input-wrapper {
    position: relative;
    flex: 1;
    display: flex;
    align-items: center;
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
    top: 50%; 
    transform: translateY(-50%); 
    color: #94a3b8;
    pointer-events: none;
    z-index: 1;
    line-height: 1;
    font-size: 1rem;
}

.search-actions { 
    display: flex; 
    gap: 0.5rem; 
    flex-shrink: 0;
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
}

.search-btn.primary { background: linear-gradient(135deg, #3b82f6, #6366f1); color: white; }
.search-btn.secondary { background: rgba(71, 85, 105, 0.6); color: #e2e8f0; }
.search-btn:hover { transform: translateY(-2px); }

/* ОСНОВНАЯ РАБОЧАЯ ОБЛАСТЬ */
.pepakura-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
}

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
    width: 50px; height: 50px;
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
.tool-btn:hover { background: rgba(56, 70, 95, 0.9); border-color: #60a5fa; color: #fff; }
.tool-btn.active { background: linear-gradient(135deg, #3b82f6, #6366f1); color: white; border: none; }

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
.viewer-title { font-size: 1.4rem; font-weight: 600; color: #f1f5f9; }
.viewer-actions { display: flex; gap: 0.8rem; }
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
.action-btn:hover { background: rgba(56, 70, 95, 0.9); }
.viewer-container {
    flex: 1;
    background: rgba(15, 23, 42, 0.9);
    border-radius: 16px;
    border: 2px solid rgba(255, 255, 255, 0.1);
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    box-shadow: inset 0 0 30px rgba(0, 0, 0, 0.3);
    min-height: 400px;
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
.panel-section { background: rgba(30, 41, 59, 0.6); border-radius: 14px; padding: 1.5rem; border: 1px solid rgba(255, 255, 255, 0.08); }
.section-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.2rem; }
.section-title { font-size: 1.1rem; font-weight: 600; color: #e2e8f0; display: flex; align-items: center; gap: 0.7rem; }
.section-title i { color: #60a5fa; }
.export-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 0.8rem; margin-top: 1rem; }
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
.export-btn:hover { background: rgba(56, 70, 95, 0.9); transform: translateY(-3px); }
.export-btn i { font-size: 1.3rem; }
.form-group { margin-bottom: 1.2rem; }
label { display: block; margin-bottom: 0.5rem; font-size: 0.9rem; color: #cbd5e1; }
.form-control { width: 100%; padding: 0.8rem; background: rgba(15, 23, 42, 0.8); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 10px; color: #e2e8f0; }

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
.status-item { display: flex; align-items: center; gap: 0.7rem; }
.status-dot { width: 10px; height: 10px; border-radius: 50%; }
.progress-bar { width: 180px; height: 6px; background: rgba(255, 255, 255, 0.1); border-radius: 3px; overflow: hidden; }
.progress-fill { height: 100%; background: linear-gradient(90deg, #3b82f6, #60a5fa); width: 65%; }

/* Утилиты */
.badge { background: rgba(96, 165, 250, 0.2); color: #60a5fa; padding: 0.2rem 0.6rem; border-radius: 10px; font-size: 0.75rem; }
.toggle { position: relative; display: inline-block; width: 50px; height: 24px; }
.toggle input { opacity: 0; width: 0; height: 0; }
.slider { position: absolute; cursor: pointer; top: 0; left: 0; right: 0; bottom: 0; background-color: #475569; transition: .4s; border-radius: 34px; }
.slider:before { position: absolute; content: ""; height: 16px; width: 16px; left: 4px; bottom: 4px; background-color: white; transition: .4s; border-radius: 50%; }
input:checked + .slider { background: linear-gradient(135deg, #3b82f6, #6366f1); }
input:checked + .slider:before { transform: translateX(26px); }
</style>