<template>
  <section class="viewer-root">
    <header class="viewer-header">
      <h2 class="viewer-title">{{ title }}</h2>
    </header>

    <div class="viewer-body">
      <TxtStage v-if="stage === 'txt'" />
      <TwoDStage v-else-if="stage === '2d'" />
      <ThreeDStage v-else-if="stage === '3d'" />
      <UnfoldStage v-else-if="stage === 'unfold'" />
    </div>
  </section>
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

const title = computed(() => {
  switch (stage.value) {
    case 'txt':
      return 'AI генерация модели из текста'
    case '2d':
      return '2D редактор и подготовка к 3D'
    case '3d':
      return '3D редактор и просмотрщик'
    case 'unfold':
      return 'Бумажная развёртка и листы'
  }
})
</script>

<style scoped>
.viewer-root {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.viewer-header {
  padding: 0.6rem 0.9rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.35);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.viewer-title {
  font-size: 0.95rem;
  font-weight: 500;
  color: #e5e7eb;
}

.viewer-body {
  flex: 1;
  min-height: 0;
  display: flex;
}
</style>
