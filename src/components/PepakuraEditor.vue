<template>
  <div class="pepakura-editor">
    <div class="editor-layout">
      <!-- Левая панель с контроллерами -->
      <div class="left-panel">
        <h3>Контроллеры</h3>
        <!-- Здесь будут элементы управления -->
        <div class="controller-item">
          <button @click="loadModel">Загрузить модель</button>
        </div>
        <div class="controller-item">
          <button @click="performNesting">Выполнить нестинг</button>
        </div>
      </div>
      
      <!-- Центральная область с вьюверами -->
      <div class="center-area">
        <div class="viewer-container">
          <!-- 3D Вьювер -->
          <div class="viewer-section top">
            <h4>3D Модель</h4>
            <ModelViewer3D />
          </div>
          
          <!-- 2D Вьювер -->
          <div class="viewer-section bottom">
            <h4>2D Листы нестинга</h4>
            <SheetPreview />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ModelViewer3D from './ModelViewer3D.vue'
import SheetPreview from './SheetPreview.vue'

// Функции для кнопок (заглушки)
const loadModel = () => {
  console.log('Загрузка модели')
  // Здесь будет логика загрузки модели
}

const performNesting = () => {
  console.log('Выполнение нестинга')
  // Здесь будет логика нестинга
}
</script>

<style scoped>
.pepakura-editor {
  width: 100%;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.editor-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.left-panel {
  width: 250px;
  background-color: #f5f5f5;
  padding: 20px;
  border-right: 1px solid #ddd;
  overflow-y: auto;
}

.left-panel h3 {
  margin-top: 0;
}

.controller-item {
  margin-bottom: 15px;
}

.controller-item button {
  width: 100%;
  padding: 10px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.controller-item button:hover {
  background-color: #359c6d;
}

.center-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.viewer-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.viewer-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 10px;
}

.viewer-section.top {
  border-bottom: 1px solid #ddd;
}

.viewer-section h4 {
  margin: 0 0 10px 0;
  padding: 0;
}

.viewer-section :deep(.viewer-3d),
.viewer-section :deep(.sheet-preview) {
  flex: 1;
  min-height: 0;
}
</style>