<template>
  <div class="app-root">
    <header class="app-header">
      <div class="header-left">
        <div class="logo">
          <span class="logo-icon">▢</span>
          <span class="logo-text">Pepakura Next</span>
          <span class="logo-sub">TXT2D3DPaper</span>
        </div>
      </div>

      <div class="header-center">
        <nav class="stage-tabs">
          <button
            class="stage-tab"
            :class="{ active: stage === 'text' }"
            @click="setStage('text')"
          >
            TXT
          </button>
          <button
            class="stage-tab"
            :class="{ active: stage === 'twod' }"
            @click="setStage('twod')"
          >
            2D
          </button>
          <button
            class="stage-tab"
            :class="{ active: stage === 'threed' }"
            @click="setStage('threed')"
          >
            3D
          </button>
          <button
            class="stage-tab"
            :class="{ active: stage === 'unfold' }"
            @click="setStage('unfold')"
          >
            Paper
          </button>
        </nav>
      </div>

      <div class="header-right">
        <span class="mode-chip">Режим: Авто (AI доступен)</span>
      </div>
    </header>

    <div class="app-body">
      <aside class="sidebar">
        <p class="sidebar-title">Проекты</p>
      </aside>

      <main class="main-area">
        <TxtStage v-if="stage === 'text'" />
        <TwoDStage v-else-if="stage === 'twod'" />
        <ThreeDStage v-else-if="stage === 'threed'" />
        <UnfoldStage v-else />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useProjectStore } from '@/stores/project'
import TxtStage from '@/components/stages/TxtStage.vue'
import TwoDStage from '@/components/stages/TwoDStage.vue'
import ThreeDStage from '@/components/stages/ThreeDStage.vue'
import UnfoldStage from '@/components/stages/UnfoldStage.vue'

const project = useProjectStore()

const stage = computed(() => project.stage)

function setStage(s: 'text' | 'twod' | 'threed' | 'unfold') {
  project.setStage(s)
}
</script>

<style scoped>
.app-root {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: radial-gradient(circle at top left, #0f172a 0, #020617 55%, #000 100%);
  color: #e5e7eb;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI',
    sans-serif;
}

.app-header {
  height: 52px;
  padding: 0 1rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.5);
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: rgba(15, 23, 42, 0.98);
}

.header-left,
.header-center,
.header-right {
  display: flex;
  align-items: center;
}

.header-center {
  flex: 1;
  justify-content: center;
}

.logo {
  display: flex;
  align-items: baseline;
  gap: 0.35rem;
}

.logo-icon {
  font-size: 1rem;
}

.logo-text {
  font-size: 0.95rem;
  font-weight: 600;
}

.logo-sub {
  font-size: 0.78rem;
  color: #9ca3af;
}

.stage-tabs {
  display: flex;
  gap: 0.35rem;
  padding: 0.2rem;
  border-radius: 999px;
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(148, 163, 184, 0.7);
}

.stage-tab {
  border-radius: 999px;
  border: 1px solid transparent;
  background: transparent;
  color: #cbd5e1;
  font-size: 0.8rem;
  padding: 0.18rem 0.9rem;
  cursor: pointer;
}

.stage-tab.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  border-color: rgba(191, 219, 254, 0.9);
  color: #f9fafb;
}

.mode-chip {
  font-size: 0.78rem;
  padding: 0.2rem 0.7rem;
  border-radius: 999px;
  border: 1px solid rgba(56, 189, 248, 0.9);
  color: #e0f2fe;
  background: radial-gradient(circle at top left, #0f172a, #0369a1);
}

.app-body {
  flex: 1;
  display: flex;
  min-height: 0;
}

.sidebar {
  width: 220px;
  border-right: 1px solid rgba(148, 163, 184, 0.5);
  padding: 0.7rem 0.6rem;
  background: linear-gradient(to bottom, rgba(15, 23, 42, 0.98), #020617);
}

.sidebar-title {
  font-size: 0.8rem;
  color: #9ca3af;
}

.main-area {
  flex: 1;
  min-width: 0;
  padding: 0.7rem;
  display: flex;
}
</style>
