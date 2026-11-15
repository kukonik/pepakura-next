<script setup lang="ts">
import { computed } from 'vue'
import type { Warning } from '@/shared/types/pepa-scene'

const props = defineProps<{
  diagnostics: Warning[]
}>()

const hasIssues = computed(() => props.diagnostics.some(w => w.severity !== 'info'))
</script>

<template>
  <div v-if="diagnostics.length" class="diagnostics-panel" :class="{ 'has-issues': hasIssues }">
    <div class="diagnostics-header">
      <span class="icon">⚠️</span>
      <h3>Диагностика модели</h3>
    </div>
    <div
      v-for="(warn, i) in diagnostics"
      :key="i"
      class="diagnostic-item"
      :data-severity="warn.severity"
    >
      <span class="severity-badge" :class="warn.severity" />
      <span class="message">{{ warn.message }}</span>
      <span
        v-if="warn.partId"
        class="part-ref"
        @click="('highlight', warn.partId)"
      >
        [выделить]
      </span>
    </div>
  </div>
</template>

<style scoped>
.diagnostics-panel {
  padding: 16px;
  border-radius: 8px;
  background: var(--bg-secondary, #1e1e1e);
  margin-top: 16px;
  border: 1px solid var(--border-color, #333);
}

.diagnostics-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-color, #333);
}

.diagnostics-header h3 {
  margin: 0;
  font-size: 16px;
  color: var(--text-color, #fff);
}

.diagnostics-panel.has-issues {
  border-left: 4px solid var(--color-warning, #ff9800);
}

.diagnostic-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-color-light, #2a2a2a);
}

.diagnostic-item:last-child {
  border-bottom: none;
}

.severity-badge {
  display: inline-block;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.severity-badge.info { background: var(--color-info, #2196f3); }
severity-badge.warning { background: var(--color-warning, #ff9800); }
.severity-badge.error { background: var(--color-error, #f44336); }

.message {
  flex: 1;
  font-size: 14px;
  color: var(--text-color, #fff);
}

.part-ref {
  color: var(--color-accent, #4caf50);
  cursor: pointer;
  font-size: 12px;
  opacity: 0.8;
}

.part-ref:hover {
  opacity: 1;
  text-decoration: underline;
}
</style>
