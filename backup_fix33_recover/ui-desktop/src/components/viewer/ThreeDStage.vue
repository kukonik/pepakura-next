<script setup lang="ts">
import ThreeDViewerCanvas from "./ThreeDViewerCanvas.vue"
import { ref } from "vue"

const showLeftPanel = ref(false)
const showRightPanel = ref(false)
const showTopPanel = ref(false)

const toggleLeft = () => (showLeftPanel.value = !showLeftPanel.value)
const toggleRight = () => (showRightPanel.value = !showRightPanel.value)
const toggleTop = () => (showTopPanel.value = !showTopPanel.value)
</script>

<template>
  <div class="viewer-page">
    <!-- Полноэкранный viewer -->
    <ThreeDViewerCanvas class="viewer-page__canvas" />

    <!-- Кнопки поверх viewer -->
    <div class="viewer-page__buttons">
      <button class="vp-btn" @click="toggleLeft">≡ Проект</button>
      <button class="vp-btn" @click="toggleTop">⚙ Режим</button>
      <button class="vp-btn" @click="toggleRight">ⓘ Инфо</button>
    </div>

    <!-- Левая панель -->
    <div
      class="vp-panel vp-panel--left"
      v-if="showLeftPanel"
    >
      <div class="vp-panel__header">
        <span>Проекты</span>
        <button class="vp-panel__close" @click="toggleLeft">✕</button>
      </div>
      <div class="vp-panel__body">
        <p>Список проектов и сцен (пока заглушка).</p>
      </div>
    </div>

    <!-- Правая панель -->
    <div
      class="vp-panel vp-panel--right"
      v-if="showRightPanel"
    >
      <div class="vp-panel__header">
        <span>Информация</span>
        <button class="vp-panel__close" @click="toggleRight">✕</button>
      </div>
      <div class="vp-panel__body">
        <p>Статистика модели, подсказки AI и т.п. (заглушка).</p>
      </div>
    </div>

    <!-- Верхняя панель -->
    <div
      class="vp-panel vp-panel--top"
      v-if="showTopPanel"
    >
      <div class="vp-panel__header">
        <span>Режимы</span>
        <button class="vp-panel__close" @click="toggleTop">✕</button>
      </div>
      <div class="vp-panel__body">
        <p>Переключение TXT / 2D / 3D / Paper (заглушка).</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.viewer-page {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: #111;
}

.viewer-page__canvas {
  width: 100%;
  height: 100%;
}

/* Кнопки поверх viewer */
.viewer-page__buttons {
  position: absolute;
  top: 12px;
  left: 12px;
  display: flex;
  gap: 8px;
  z-index: 10;
}

.vp-btn {
  padding: 4px 10px;
  font-size: 12px;
  background: rgba(20, 20, 20, 0.9);
  color: #eee;
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 4px;
  cursor: pointer;
}
.vp-btn:hover {
  background: rgba(40, 40, 40, 0.95);
}

/* Общий стиль панелей */
.vp-panel {
  position: absolute;
  z-index: 20;
  background: rgba(10, 10, 10, 0.96);
  color: #eee;
  border: 1px solid rgba(255, 255, 255, 0.15);
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(6px);
}

/* Левая панель */
.vp-panel--left {
  top: 50px;
  bottom: 20px;
  left: 20px;
  width: 280px;
}

/* Правая панель */
.vp-panel--right {
  top: 50px;
  bottom: 20px;
  right: 20px;
  width: 280px;
}

/* Верхняя панель */
.vp-panel--top {
  top: 50px;
  left: 80px;
  right: 80px;
  height: 160px;
}

.vp-panel__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 10px;
  font-size: 13px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(30, 30, 30, 0.95);
}

.vp-panel__body {
  padding: 8px 10px;
  font-size: 12px;
  overflow: auto;
  max-height: calc(100% - 32px);
}

.vp-panel__close {
  border: none;
  background: transparent;
  color: #aaa;
  cursor: pointer;
}
.vp-panel__close:hover {
  color: #fff;
}
</style>
