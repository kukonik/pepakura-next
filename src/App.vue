<template>
  <div id="app">
    <header>
      <h1>Pepakura Next - AI 3D Model Generator</h1>
    </header>
    <main>
      <AIGenerator />
      <div class="model-viewer-section">
        <ModelViewer3D ref="modelViewer" />
        <UnfoldButton />
      </div>
      <div class="sheet-preview-section">
        <SheetPreview />
      </div>
    </main>
  </div>
</template>

<script setup>
import AIGenerator from '@/components/AIGenerator.vue'
import ModelViewer3D from '@/components/ModelViewer3D.vue'
import SheetPreview from '@/components/SheetPreview.vue'
import UnfoldButton from '@/components/UnfoldButton.vue'
import { ref } from 'vue'

const modelViewer = ref(null)
</script>

<style>
#app {
  font-family: Arial, sans-serif;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

header {
  background-color: #2c3e50;
  color: white;
  padding: 20px;
  margin-bottom: 30px;
}

.model-viewer-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 20px 0;
}

.sheet-preview-section {
  margin: 20px 0;
  height: 500px;
}
</style>
