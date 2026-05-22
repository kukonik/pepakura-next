<template>
  <div class="stats-panel">
    <h2>Статистика</h2>
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon">&#128193;</div>
        <div class="stat-content">
          <div class="stat-value">{{ stats.total }}</div>
          <div class="stat-label">Проектов</div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-icon">&#128190;</div>
        <div class="stat-content">
          <div class="stat-value">{{ formatFileSize(stats.totalSize) }}</div>
          <div class="stat-label">Общий размер</div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-icon">&#8987;</div>
        <div class="stat-content">
          <div class="stat-value">{{ stats.recentCount }}</div>
          <div class="stat-label">За неделю</div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-icon">&#128336;</div>
        <div class="stat-content">
          <div class="stat-value">{{ lastActiveText }}</div>
          <div class="stat-label">Последняя активность</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useProjectStore } from '../../stores/project.store'

const projectStore = useProjectStore()
const stats = computed(() => projectStore.stats)

const lastActiveText = computed(() => {
  if (!stats.value.lastActive) return 'Никогда'
  const now = new Date()
  const last = new Date(stats.value.lastActive)
  const diff = now.getTime() - last.getTime()
  const hours = Math.floor(diff / (1000 * 60 * 60))
  
  if (hours < 1) return 'Сейчас'
  if (hours < 24) return hours + ' часов назад'
  const days = Math.floor(hours / 24)
  return days + ' дней назад'
})

const formatFileSize = (size) => {
  if (size < 1) return Math.round(size * 1024) + ' KB'
  return size.toFixed(1) + ' MB'
}
</script>

<style scoped>
.stats-panel {
  padding: 20px;
  background: var(--bg-secondary);
  border-radius: 8px;
  box-shadow: var(--shadow-sm);
}

.stats-panel h2 {
  margin: 0 0 20px 0;
  font-size: 1.5rem;
  color: var(--text-primary);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 16px;
}

.stat-card {
  display: flex;
  align-items: center;
  padding: 16px;
  background: var(--bg-primary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.stat-icon {
  font-size: 1.5rem;
  margin-right: 12px;
  opacity: 0.7;
}

.stat-content {
  flex: 1;
}

.stat-value {
  font-size: 1.2rem;
  font-weight: 600;
  color: var(--primary-color);
  margin-bottom: 4px;
}

.stat-label {
  font-size: 0.8rem;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
