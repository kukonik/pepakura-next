<template>
  <div class="ai-assistant-page">
    <div class="ai-content">
      <AiAssistantPanel ref="aiPanelRef" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import AiAssistantPanel from '../components/ai/AiAssistantPanel.vue'

const aiPanelRef = ref<InstanceType<typeof AiAssistantPanel> | null>(null)
</script>

<style scoped>
.ai-assistant-page {
  height: 100%;
  background: var(--bg-secondary);
}

.ai-content {
  height: 100%;
  padding: 20px;
}
</style>
