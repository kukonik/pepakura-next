<template>
  <div class="pepakura-editor">
    <div class="editor-header">
      <h2>Pepakura Next Editor</h2>
      <div class="project-info" v-if="projectStore.currentProject">
        <span>Проект: {{ projectStore.currentProject.name }}</span>
        <span v-if="projectStore.currentProject.path"> | Путь: {{ projectStore.currentProject.path }}</span>
      </div>
    </div>
    
    <div class="editor-content">
      <!-- Панель инструментов -->
      <div class="toolbar">
        <button 
          @click="loadPDOFile" 
          class="tool-button"
          :disabled="isProcessing"
        >
          Загрузить PDO
        </button>
        <button 
          @click="performNesting" 
          class="tool-button"
          :disabled="isProcessing || !hasUnfolds"
        >
          Вложение
        </button>
        <button 
          @click="exportToSVG" 
          class="tool-button"
          :disabled="isProcessing || !canExport"
        >
          Экспорт SVG
        </button>
        <button 
          @click="generate3DModel" 
          class="tool-button"
          :disabled="isProcessing"
        >
          3D изображение
        </button>
      </div>
      
      <!-- Основная рабочая область -->
      <div class="main-content">
        <!-- Левая панель - компоненты управления -->
        <div class="control-panel">
          <div class="panel-section">
            <h3>Загрузка модели</h3>
            <PDOFileLoader />
          </div>
          
          <div class="panel-section">
            <h3>Вложение</h3>
            <NestingController />
          </div>
          
          <div class="panel-section">
            <h3>Экспорт</h3>
            <SVGExporter />
          </div>
          
          <div class="panel-section">
            <h3>Генерация 3D</h3>
            <ImageTo3DGenerator />
          </div>
        </div>
        
        <!-- Центральная область - визуализация -->
        <div class="visualization-area">
          <div class="visualization-content">
            <div v-if="projectStore.currentProject" class="project-visualization">
              <h3>Визуализация проекта</h3>
              <div class="visualization-placeholder">
                <div class="placeholder-content">
                  <div class="placeholder-icon">📐</div>
                  <p>Визуализация проекта будет отображаться здесь</p>
                </div>
              </div>
            </div>
            
            <div v-else class="no-project">
              <div class="placeholder-content">
                <div class="placeholder-icon">📁</div>
                <p>Загрузите PDO файл для начала работы</p>
                <button @click="loadPDOFile" class="primary-button">Загрузить PDO</button>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Правая панель - свойства и информация -->
        <div class="properties-panel">
          <div class="panel-section">
            <h3>Свойства проекта</h3>
            <div class="project-properties" v-if="projectStore.currentProject">
              <div class="property-item">
                <label>Название:</label>
                <span>{{ projectStore.currentProject.projectMeta.name }}</span>
              </div>
              <div class="property-item">
                <label>Путь:</label>
                <span>{{ projectStore.projectPath || 'Не сохранен' }}</span>
              </div>
              <div class="property-item">
                <label>Создан:</label>
                <span>{{ new Date(projectStore.currentProject.projectMeta.createdAt).toLocaleDateString() }}</span>
              </div>
              <div class="property-item">
                <label>Изменен:</label>
                <span>{{ new Date(projectStore.currentProject.projectMeta.updatedAt).toLocaleDateString() }}</span>
              </div>
            </div>
            <div class="no-properties" v-else>
              <p>Нет загруженного проекта</p>
            </div>
          </div>
          
          <div class="panel-section">
            <h3>Информация о вложении</h3>
            <div class="nesting-info" v-if="projectStore.nestingResult">
              <div class="property-item">
                <label>Листов:</label>
                <span>{{ projectStore.nestingResult.sheetCount }}</span>
              </div>
              <div class="property-item">
                <label>Деталей:</label>
                <span>{{ projectStore.nestingResult.totalParts }}</span>
              </div>
              <div class="property-item">
                <label>Заполнение:</label>
                <span>{{ (projectStore.nestingResult.utilization * 100).toFixed(2) }}%</span>
              </div>
            </div>
            <div class="no-properties" v-else>
              <p>Вложение не выполнено</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Статусная строка -->
    <div class="status-bar">
      <div class="status-item">
        <span v-if="isProcessing" class="processing">Обработка...</span>
        <span v-else class="ready">Готов</span>
      </div>
      <div class="status-item">
        <span v-if="error" class="error">{{ error }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useProjectStore } from '@/stores/projectStore'
import PDOFileLoader from './PDOFileLoader.vue'
import NestingController from './NestingController.vue'
import SVGExporter from './SVGExporter.vue'
import ImageTo3DGenerator from './ImageTo3DGenerator.vue'

const projectStore = useProjectStore()

// Проверяем, есть ли развертки для вложения
const hasUnfolds = computed(() => {
  return !!projectStore.currentProject
})

// Проверяем, можно ли выполнить экспорт
const canExport = computed(() => {
  return !!projectStore.currentProject && !!projectStore.nestResult
})
</script>

<style scoped>
.pepakura-editor {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-family: var(--font-family);
}

.editor-header {
  padding: 16px 24px;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
}

.editor-header h2 {
  margin: 0 0 8px 0;
  color: var(--text-primary);
}

.project-info {
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.editor-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.toolbar {
  padding: 12px 24px;
  background-color: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  gap: 12px;
}

.tool-button {
  padding: 8px 16px;
  background-color: var(--primary-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.tool-button:hover:not(:disabled) {
  background-color: var(--primary-color-dark);
}

.tool-button:disabled {
  background-color: var(--bg-disabled);
  cursor: not-allowed;
}

.main-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.control-panel {
  width: 300px;
  overflow-y: auto;
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.panel-section {
  margin-bottom: 24px;
}

.panel-section h3 {
  margin: 0 0 16px 0;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-color);
  color: var(--text-primary);
}

.visualization-area {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.visualization-content {
  flex: 1;
  overflow: auto;
  padding: 24px;
}

.project-visualization,
.no-project {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.visualization-placeholder,
.no-project .placeholder-content {
  text-align: center;
  padding: 40px;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.placeholder-icon {
  font-size: 4rem;
  margin-bottom: 16px;
}

.placeholder-content p {
  margin: 0 0 24px 0;
  font-size: 1.1rem;
  color: var(--text-secondary);
}

.primary-button {
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

.primary-button:hover {
  background-color: var(--primary-color-dark);
}

.properties-panel {
  width: 300px;
  overflow-y: auto;
  background-color: var(--bg-secondary);
  border-left: 1px solid var(--border-color);
  padding: 16px;
}

.property-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color-light);
}

.property-item label {
  font-weight: 500;
  color: var(--text-primary);
}

.property-item span {
  color: var(--text-secondary);
}

.no-properties {
  padding: 16px;
  text-align: center;
  color: var(--text-secondary);
}

.status-bar {
  padding: 8px 24px;
  background-color: var(--bg-tertiary);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  font-size: 0.9rem;
}

.status-item {
  display: flex;
  align-items: center;
}

.processing {
  color: var(--primary-color);
}

.ready {
  color: var(--success-color);
}

.error {
  color: var(--error-color);
}
</style>