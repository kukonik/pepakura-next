<template>
  <div class="three-d-viewer-page-root">
    <SharedThreeDViewerCanvas />
  </div>
</template>

<script setup lang="ts">
import SharedThreeDViewerCanvas from '@pepakura-next/shared/src/components/viewer/SharedThreeDViewerCanvas.vue'
</script>

<style scoped>
.three-d-viewer-page-root {
  position: fixed;
  inset: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: #000000;
}
</style>
