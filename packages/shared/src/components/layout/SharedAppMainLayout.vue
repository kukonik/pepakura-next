<template>
  <div v-if="isFullscreen3D" class="app-root-fullscreen">
    <router-view />
  </div>
  <div v-else class="app-root-regular">
    <header class="app-header">
      <div class="app-header-left">
        <div class="app-logo">
          Pepakura Next TXT 2D 3D Paper
        </div>
        <nav class="app-nav">
          <button class="app-nav-item">
            Проекты
          </button>
          <button class="app-nav-item">
            Редактор
          </button>
          <button class="app-nav-item">
            Настройки
          </button>
          <button class="app-nav-item">
            Экспериментальный AI-режим. Интерфейс и поведение могут меняться.
          </button>
        </nav>
      </div>
    </header>

    <main class="app-main">
      <section class="app-main-header">
        <h1 class="app-main-title">
          3D модель · Полноэкранный просмотр с базовыми действиями. Редактирование пока в разработке.
        </h1>
      </section>
      <section class="app-main-content">
        <router-view />
      </section>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const isFullscreen3D = computed(() => {
  return route.meta && route.meta.fullscreen3d === true
})
</script>

<style scoped>
.app-root-fullscreen {
  position: fixed;
  inset: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

.app-root-regular {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background: #111111;
  color: #f0f0f0;
}

.app-header {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  background: #181818;
  border-bottom: 1px solid #2a2a2a;
}

.app-header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.app-logo {
  font-weight: 600;
  font-size: 14px;
}

.app-nav {
  display: flex;
  gap: 8px;
}

.app-nav-item {
  padding: 4px 8px;
  font-size: 12px;
  border-radius: 4px;
  border: none;
  background: #262626;
  color: #f0f0f0;
  cursor: pointer;
}

.app-nav-item:hover {
  background: #333333;
}

.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.app-main-header {
  padding: 12px 16px;
  border-bottom: 1px solid #2a2a2a;
}

.app-main-title {
  margin: 0;
  font-size: 13px;
  font-weight: 500;
}

.app-main-content {
  flex: 1;
  position: relative;
  overflow: hidden;
}
</style>
