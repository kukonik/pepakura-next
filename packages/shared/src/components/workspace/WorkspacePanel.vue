<template>
  <div class="workspace-panel" :class="{ active: isActive }">
    <div class="panel-header" @mousedown="startDrag">
      <span class="panel-title">{{ title }}</span>
      <div class="panel-controls">
        <button @click="$emit('close')" class="panel-btn">×</button>
      </div>
    </div>
    <div class="panel-content">
      <slot></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ title: string; isActive: boolean }>();
defineEmits(['close']);

function startDrag() {
  // Заготовка под будущий Drag & Drop ресайз
}
</script>

<style scoped>
.workspace-panel {
  display: flex; flex-direction: column; height: 100%; border: 1px solid #333;
  background: #111; min-width: 200px;
}
.workspace-panel.active { border-color: #6c5ce7; }

.panel-header {
  padding: 8px 12px; background: #1e1e24; border-bottom: 1px solid #333;
  display: flex; justify-content: space-between; align-items: center;
  user-select: none; cursor: default;
}
.panel-title { font-size: 0.8rem; color: #aaa; font-weight: 600; text-transform: uppercase; }
.panel-btn { background: none; border: none; color: #ff7675; font-size: 1.2rem; cursor: pointer; }
.panel-btn:hover { color: #fff; }

.panel-content { flex: 1; position: relative; overflow: hidden; }
</style>
