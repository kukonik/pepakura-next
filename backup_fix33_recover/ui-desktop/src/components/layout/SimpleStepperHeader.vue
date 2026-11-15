<template>
  <header class="stepper-root">
    <div
      v-for="step in steps"
      :key="step.id"
      class="step-item"
      :class="stepClass(step.id)"
      @click="handleStepClick(step.id)"
    >
      <div class="step-marker">
        <span v-if="isCompleted(step.id)" class="step-check">✓</span>
        <span v-else class="step-index">
          {{ step.index }}
        </span>
      </div>
      <div class="step-texts">
        <span class="step-title">{{ step.title }}</span>
        <span class="step-subtitle">{{ step.subtitle }}</span>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { StageId } from '@/stores/project'
import { useProjectStore } from '@/stores/project'

interface StepDef {
  id: StageId
  index: number
  title: string
  subtitle: string
}

const project = useProjectStore()

const steps = computed<StepDef[]>(() => [
  {
    id: 'text',
    index: 1,
    title: 'Идея',
    subtitle: 'Описание модели'
  },
  {
    id: 'twod',
    index: 2,
    title: 'Референсы',
    subtitle: 'Изображения и скетчи'
  },
  {
    id: 'threed',
    index: 3,
    title: 'Модель',
    subtitle: '3D‑файл и проверка'
  },
  {
    id: 'unfold',
    index: 4,
    title: 'Развёртка',
    subtitle: 'Подготовка к печати'
  }
])

const currentStage = computed<StageId>(() => project.stage)

function isCompleted(id: StageId) {
  const order: StageId[] = ['text', 'twod', 'threed', 'unfold']
  return order.indexOf(id) < order.indexOf(currentStage.value)
}

function isActive(id: StageId) {
  return currentStage.value === id
}

function stepClass(id: StageId) {
  return {
    active: isActive(id),
    completed: isCompleted(id),
    upcoming: !isActive(id) && !isCompleted(id)
  }
}

function handleStepClick(id: StageId) {
  // в простом режиме можно разрешить произвольную навигацию
  project.setStage(id)
}
</script>

<style scoped>
.stepper-root {
  height: 54px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.5);
  padding: 0 0.85rem;
  display: flex;
  align-items: stretch;
  gap: 0.4rem;
  background: rgba(15, 23, 42, 0.98);
}

.step-item {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.3rem 0.45rem;
  border-radius: 0.7rem;
  cursor: pointer;
  transition: background 0.15s ease, border-color 0.15s ease, transform 0.1s ease;
  border: 1px solid transparent;
}

.step-item:hover {
  transform: translateY(-1px);
}

.step-item.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  border-color: rgba(191, 219, 254, 0.95);
}

.step-item.completed {
  background: rgba(15, 23, 42, 0.96);
  border-color: rgba(148, 163, 184, 0.8);
}

.step-item.upcoming {
  background: rgba(15, 23, 42, 0.9);
  border-color: rgba(30, 64, 175, 0.6);
}

.step-marker {
  width: 26px;
  height: 26px;
  border-radius: 999px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(15, 23, 42, 0.98);
  border: 1px solid rgba(148, 163, 184, 0.8);
  font-size: 0.8rem;
  flex-shrink: 0;
  color: #e5e7eb;
}

.step-item.active .step-marker {
  background: rgba(15, 23, 42, 0.96);
  border-color: rgba(219, 234, 254, 0.95);
  color: #e0f2fe;
}

.step-item.completed .step-marker {
  background: rgba(22, 163, 74, 0.2);
  border-color: rgba(34, 197, 94, 0.9);
  color: #bbf7d0;
}

.step-texts {
  display: flex;
  flex-direction: column;
  gap: 0.05rem;
  min-width: 0;
}

.step-title {
  font-size: 0.86rem;
  font-weight: 500;
}

.step-subtitle {
  font-size: 0.72rem;
  color: #cbd5e1;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}

.step-check {
  font-size: 0.9rem;
}

.step-index {
  font-size: 0.8rem;
}
</style>
