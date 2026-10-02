<template>
  <footer class="status-bar">
    <div class="status-left">
      <span class="status-dot" :class="statusTypeClass"></span>
      <span class="status-text">{{ safeStatusMessage }}</span>
    </div>
    <div class="status-right">
      <span class="performance-text">{{ performanceStatus }}</span>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  statusMessage?: string
  statusType?: 'info' | 'error' | 'success' | 'warning'
  performanceStatus?: string
}>()

const safeStatusMessage = computed(
  () => props.statusMessage ?? 'Готов к работе'
)

const statusTypeClass = computed(() => {
  switch (props.statusType) {
    case 'error':
      return 'status-dot-error'
    case 'success':
      return 'status-dot-success'
    case 'warning':
      return 'status-dot-warning'
    default:
      return 'status-dot-info'
  }
})
</script>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.35rem 0.75rem;
  border-top: 1px solid rgba(148, 163, 184, 0.4);
  background: rgba(15, 23, 42, 0.95);
  font-size: 0.75rem;
  color: #e5e7eb;
}

.status-left {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.status-right {
  color: #9ca3af;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 999px;
}

.status-dot-info {
  background-color: #22c55e;
}

.status-dot-error {
  background-color: #ef4444;
}

.status-dot-success {
  background-color: #22c55e;
}

.status-dot-warning {
  background-color: #facc15;
}

.status-text {
  max-width: 460px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.performance-text {
  white-space: nowrap;
}
</style>
