<template>
  <div id="app">
    <div v-if="!isPlatformReady" class="platform-loading">
      <div class="loading-spinner">Initializing platform...</div>
    </div>
    <SplitView v-else />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { usePlatform } from '@pepakura/shared/composables/usePlatform'
import SplitView from './components/SplitView.vue'

const { init } = usePlatform()
const isPlatformReady = ref(false)

onMounted(async () => {
  try {
    await init()
    isPlatformReady.value = true
  } catch (error) {
    console.error('Failed to initialize platform bridge:', error)
    isPlatformReady.value = true
  }
})
</script>

<style>
.platform-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  background: linear-gradient(155deg, #0b1120 0%, #1a202c 100%);
  color: #e2e8f0;
  font-size: 1.2rem;
}
#app {
  height: 100vh;
  overflow: hidden;
}
</style>