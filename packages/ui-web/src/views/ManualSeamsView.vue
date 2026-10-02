<template>
  <div class="manual-seams-view">
    <div class="header">
      <h2>Ручное редактирование швов</h2>
      <p>Выберите ребра модели для добавления или удаления швов</p>
    </div>
    
    <div class="content">
      <div class="toolbar">
        <button 
          :class="['tool-btn', { active: activeTool === 'add' }]"
          @click="setActiveTool('add')"
          title="Добавить шов"
        >
          <i class="icon-add"></i>
          Добавить
        </button>
        
        <button 
          :class="['tool-btn', { active: activeTool === 'remove' }]"
          @click="setActiveTool('remove')"
          title="Удалить шов"
        >
          <i class="icon-remove"></i>
          Удалить
        </button>
        
        <button 
          :class="['tool-btn', { active: activeTool === 'select' }]"
          @click="setActiveTool('select')"
          title="Выбрать шов"
        >
          <i class="icon-select"></i>
          Выбрать
        </button>
        
        <div class="divider"></div>
        
        <button 
          class="tool-btn"
          @click="clearAllSeams"
          title="Очистить все швы"
        >
          <i class="icon-clear"></i>
          Очистить
        </button>
      </div>
      
      <div class="main-area">
        <div class="model-viewer">
          <!-- 3D визуализация модели с швами -->
          <div class="viewer-placeholder">
            <p>3D визуализация модели</p>
            <p v-if="activeTool">Активный инструмент: {{ getToolName(activeTool) }}</p>
          </div>
        </div>
        
        <div class="seam-list">
          <h3>Список швов</h3>
          <div class="seam-items">
            <div 
              v-for="(seam, index) in seamItems" 
              :key="index"
              class="seam-item"
            >
              <div class="seam-info">
                <span>Шов {{ index + 1 }}</span>
                <span>Длина: {{ seam.length }} мм</span>
              </div>
              <button 
                class="remove-seam-btn"
                @click="removeSeam(index)"
                title="Удалить шов"
              >
                <i class="icon-delete"></i>
              </button>
            </div>
            
            <div v-if="seamItems.length === 0" class="no-seams">
              <p>Швы не добавлены</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

// Активный инструмент
const activeTool = ref<'add' | 'remove' | 'select' | null>(null);

// Список швов (имитация)
const seamItems = ref([
  { length: 120 },
  { length: 85 },
  { length: 210 },
  { length: 155 },
]);

/**
 * Установка активного инструмента
 * @param tool Инструмент
 */
function setActiveTool(tool: 'add' | 'remove' | 'select') {
  activeTool.value = tool;
}

/**
 * Получение названия инструмента
 * @param tool Инструмент
 * @returns Название инструмента
 */
function getToolName(tool: string | null): string {
  switch (tool) {
    case 'add': return 'Добавление швов';
    case 'remove': return 'Удаление швов';
    case 'select': return 'Выбор швов';
    default: return 'Нет активного инструмента';
  }
}

/**
 * Удаление шва
 * @param index Индекс шва
 */
function removeSeam(index: number) {
  seamItems.value.splice(index, 1);
}

/**
 * Очистка всех швов
 */
function clearAllSeams() {
  seamItems.value = [];
}
</script>

<style scoped>
.manual-seams-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 20px;
  background-color: var(--bg-primary);
}

.header {
  margin-bottom: 20px;
}

.header h2 {
  margin: 0 0 10px 0;
  font-size: 24px;
  font-weight: 600;
}

.header p {
  margin: 0;
  color: var(--text-secondary);
}

.content {
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: 20px;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.tool-btn {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 8px 12px;
  background-color: var(--btn-secondary-bg);
  color: var(--btn-secondary-text);
  border: 1px solid var(--btn-secondary-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.tool-btn:hover {
  background-color: var(--btn-secondary-hover);
}

.tool-btn.active {
  background-color: var(--primary-color);
  color: white;
  border-color: var(--primary-color);
}

.divider {
  width: 1px;
  height: 24px;
  background-color: var(--border-color);
  margin: 0 5px;
}

.main-area {
  display: flex;
  flex: 1;
  gap: 20px;
}

.model-viewer {
  flex: 1;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.viewer-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary);
}

.seam-list {
  width: 300px;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.seam-list h3 {
  padding: 15px;
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  border-bottom: 1px solid var(--border-color);
}

.seam-items {
  flex: 1;
  overflow-y: auto;
  padding: 10px;
}

.seam-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  margin-bottom: 10px;
  background-color: var(--bg-primary);
  border-radius: 4px;
  border: 1px solid var(--border-color);
}

.seam-info {
  display: flex;
  flex-direction: column;
}

.seam-info span:first-child {
  font-weight: 500;
}

.seam-info span:last-child {
  font-size: 12px;
  color: var(--text-secondary);
}

.remove-seam-btn {
  padding: 5px;
  background-color: transparent;
  border: none;
  cursor: pointer;
  color: var(--text-secondary);
}

.remove-seam-btn:hover {
  color: var(--danger-color);
}

.no-seams {
  text-align: center;
  padding: 20px;
  color: var(--text-secondary);
}

/* Иконки (имитация) */
.icon-add::before,
.icon-remove::before,
.icon-select::before,
.icon-clear::before,
.icon-delete::before {
  content: "□";
  margin-right: 5px;
}
</style>