<!-- packages/shared/src/components/ModelViewer.vue -->
<template>
  <div class="model-viewer">
    <div class="model-viewer__placeholder">
      3D Model Viewer (stub)
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.model-viewer {
  width: 100%;
  height: 100%;
  min-height: 200px;
  background-color: #111;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 0.5rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.model-viewer__placeholder {
  opacity: 0.7;
  font-size: 0.9rem;
}
</style>
