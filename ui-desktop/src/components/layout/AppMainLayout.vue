<template>
  <div class="app-root">
    <header
      v-if="fullscreenStage === 'none'"
      class="app-header"
    >
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

    <div class="app-body" :class="{ 'app-body-full': fullscreenStage !== 'none' }">
      <aside
        v-if="fullscreenStage === 'none'"
        class="sidebar"
      >
        <div class="sidebar-header">
          <p class="sidebar-title">Проекты</p>
          <span class="sidebar-status">скоро</span>
        </div>
        <p class="sidebar-empty">
          Здесь будут сохранённые проекты и истории генераций.
        </p>
      </aside>

      <main class="main-area" :class="{ 'main-area-full': fullscreenStage !== 'none' }">
        <TxtStage
          v-if="stage === 'text'"
          :fullscreen="fullscreenStage === 'text'"
          @toggle-fullscreen="toggleFullscreen('text')"
        />
        <TwoDStage
          v-else-if="stage === 'twod'"
          :fullscreen="fullscreenStage === 'twod'"
          @toggle-fullscreen="toggleFullscreen('twod')"
        />
        <ThreeDStage
          v-else-if="stage === 'threed'"
          :fullscreen="fullscreenStage === 'threed'"
          @toggle-fullscreen="toggleFullscreen('threed')"
        />
        <UnfoldStage
          v-else
          :fullscreen="fullscreenStage === 'unfold'"
          @toggle-fullscreen="toggleFullscreen('unfold')"
        />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useProjectStore } from '@/stores/project'
import TxtStage from '@/components/stages/TxtStage.vue'
import TwoDStage from '@/components/stages/TwoDStage.vue'
import ThreeDStage from '@/components/stages/ThreeDStage.vue'
import UnfoldStage from '@/components/stages/UnfoldStage.vue'

const project = useProjectStore()

const stage = computed(() => project.stage)

const fullscreenStage = ref<'none' | 'text' | 'twod' | 'threed' | 'unfold'>('none')

function setStage(s: 'text' | 'twod' | 'threed' | 'unfold') {
  project.setStage(s)
}

function toggleFullscreen(stageKey: 'text' | 'twod' | 'threed' | 'unfold') {
  fullscreenStage.value =
    fullscreenStage.value === stageKey ? 'none' : stageKey
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

.app-body-full {
  /* без хедера и сайдбара просто больше места */
}

.sidebar {
  width: 220px;
  border-right: 1px solid rgba(148, 163, 184, 0.5);
  padding: 0.7rem 0.6rem;
  background: linear-gradient(to bottom, rgba(15, 23, 42, 0.98), #020617);
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.25rem;
  margin-bottom: 0.25rem;
}

.sidebar-title {
  font-size: 0.8rem;
  color: #9ca3af;
  margin: 0;
}

.sidebar-status {
  font-size: 0.7rem;
  color: #6b7280;
  border-radius: 999px;
  border: 1px solid rgba(75, 85, 99, 0.9);
  padding: 0.05rem 0.35rem;
}

.sidebar-empty {
  font-size: 0.78rem;
  color: #6b7280;
  margin: 0.15rem 0 0;
}

.main-area {
  flex: 1;
  min-width: 0;
  padding: 0.7rem;
  display: flex;
}

.main-area-full {
  padding: 0; /* чтобы стейдж занял всё */
}
</style>
