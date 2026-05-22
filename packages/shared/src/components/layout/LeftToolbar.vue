<script setup lang="ts">
interface StageItem {
  id: string;
  label: string;
}

const props = defineProps<{
  activeStage: string;
  stages: StageItem[];
}>();

const emit = defineEmits<{
  (e: "changeStage", id: string): void;
}>();

function onClick(id: string) {
  emit("changeStage", id);
}
</script>

<template>
  <div class="left-toolbar">
    <button
      v-for="stage in stages"
      :key="stage.id"
      type="button"
      class="toolbar-button"
      :class="{ 'toolbar-button--active': stage.id === activeStage }"
      @click="onClick(stage.id)"
    >
      <span class="toolbar-button-label">{{ stage.label }}</span>
    </button>
  </div>
</template>

<style scoped>
.left-toolbar {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  padding: 4px 0;
  gap: 4px;
}

.toolbar-button {
  width: 100%;
  border: none;
  background-color: transparent;
  color: #ccc;
  cursor: pointer;
  padding: 6px 0;
  font-size: 11px;
}

.toolbar-button--active {
  background-color: #3b82f6;
  color: #fff;
}

.toolbar-button-label {
  writing-mode: vertical-rl;
  transform: rotate(180deg);
}
</style>
