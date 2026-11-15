<script setup lang="ts">
import { ref } from "vue";
import ThreeDViewerCanvas from "@/components/viewer/ThreeDViewerCanvas.vue";

const showLeftPanel = ref(false);
const showRightPanel = ref(false);
const showTopPanel = ref(false);
</script>

<template>
  <div class="stage-root">
    <ThreeDViewerCanvas />

    <div class="stage-overlay-topleft">
      <button class="stage-chip" @click="showLeftPanel = !showLeftPanel">
        Проекты
      </button>
      <button class="stage-chip" @click="showTopPanel = !showTopPanel">
        Режим
      </button>
      <button class="stage-chip" @click="showRightPanel = !showRightPanel">
        Инфо
      </button>
    </div>

    <transition name="panel-fade">
      <section v-if="showLeftPanel" class="stage-panel stage-panel-left">
        <header class="stage-panelheader">
          <div class="stage-paneltitle">Проекты</div>
          <button class="stage-panelclose" @click="showLeftPanel = false">✕</button>
        </header>
        <div class="stage-panelbody">
          <p>Список проектов (TODO).</p>
        </div>
      </section>
    </transition>

    <transition name="panel-fade">
      <section v-if="showRightPanel" class="stage-panel stage-panel-right">
        <header class="stage-panelheader">
          <div class="stage-paneltitle">Инфо</div>
          <button class="stage-panelclose" @click="showRightPanel = false">✕</button>
        </header>
        <div class="stage-panelbody">
          <p>Информация о модели (TODO).</p>
        </div>
      </section>
    </transition>

    <transition name="panel-fade">
      <section v-if="showTopPanel" class="stage-panel stage-panel-top">
        <header class="stage-panelheader">
          <div class="stage-paneltitle">Режимы</div>
          <button class="stage-panelclose" @click="showTopPanel = false">✕</button>
        </header>
        <div class="stage-panelbody">
          <p>Переключение TXT / 2D / 3D / Paper (TODO).</p>
        </div>
      </section>
    </transition>
  </div>
</template>

<style scoped>
.stage-root {
  position: fixed;
  inset: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: #020617;
}

.stage-overlay-topleft {
  position: absolute;
  top: 8px;
  left: 8px;
  display: flex;
  gap: 6px;
  z-index: 40;
}

.stage-chip {
  padding: 3px 9px;
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.85);
  background: rgba(15, 23, 42, 0.96);
  color: #e5e7eb;
  font-size: 11px;
  cursor: pointer;
}

.stage-chip:hover {
  background: rgba(30, 64, 175, 0.96);
  border-color: rgba(129, 140, 248, 0.96);
}

.stage-panel {
  position: absolute;
  z-index: 35;
  width: 260px;
  max-width: 32vw;
  max-height: 70vh;
  display: flex;
  flex-direction: column;
  border-radius: 6px;
  border: 1px solid rgba(148, 163, 184, 0.6);
  background: rgba(15, 23, 42, 0.98);
  box-shadow: 0 18px 45px rgba(0, 0, 0, 0.6);
  color: #e5e7eb;
  font-size: 13px;
}

.stage-panel-left {
  top: 40px;
  left: 8px;
}

.stage-panel-right {
  top: 40px;
  right: 8px;
}

.stage-panel-top {
  top: 40px;
  left: 50%;
  transform: translateX(-50%);
  width: 420px;
  max-width: 90vw;
}

.stage-panelheader {
  flex: 0 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 10px;
  border-bottom: 1px solid rgba(55, 65, 81, 0.9);
  background: rgba(17, 24, 39, 0.98);
}

.stage-paneltitle {
  font-size: 12px;
  font-weight: 500;
}

.stage-panelbody {
  flex: 1 1 auto;
  padding: 8px 10px;
  overflow: auto;
}

.stage-panelclose {
  border: none;
  background: transparent;
  color: #9ca3af;
  cursor: pointer;
  font-size: 14px;
}

.stage-panelclose:hover {
  color: #f9fafb;
}

.panel-fade-enter-active,
.panel-fade-leave-active {
  transition: opacity 0.14s ease-out, transform 0.14s ease-out;
}

.panel-fade-enter-from,
.panel-fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
