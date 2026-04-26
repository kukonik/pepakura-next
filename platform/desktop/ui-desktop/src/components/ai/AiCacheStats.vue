<template>
  <div class="ai-cache-stats">
    <h3>🧠 Кэш AI</h3>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon hits">📈</div>
        <div class="stat-info">
          <div class="stat-value">{{ stats.hits }}</div>
          <div class="stat-label">Попаданий</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon misses">📉</div>
        <div class="stat-info">
          <div class="stat-value">{{ stats.misses }}</div>
          <div class="stat-label">Промахов</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon saves">💾</div>
        <div class="stat-info">
          <div class="stat-value">{{ stats.saves }}</div>
          <div class="stat-label">Сохранений</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon rate">🎯</div>
        <div class="stat-info">
          <div class="stat-value">{{ stats.hit_rate.toFixed(1) }}%</div>
          <div class="stat-label">Hit Rate</div>
        </div>
      </div>
    </div>

    <div class="cache-controls">
      <div class="toggle-setting">
        <label>
          <input
            type="checkbox"
            v-model="cacheEnabled"
            @change="toggleCacheEnabled"
          />
          <span>Включить кэширование</span>
        </label>
      </div>

      <div class="action-buttons">
        <button
          class="btn-refresh"
          @click="loadStats"
          :disabled="isLoading"
          title="Обновить статистику"
        >
          <i class="fas fa-sync" :class="{ 'fa-spin': isLoading }"></i>
        </button>

        <button
          class="btn-clear"
          @click="clearCache"
          :disabled="isClearing"
          title="Очистить кэш"
        >
          <i class="fas fa-trash"></i>
          {{ isClearing ? 'Очистка...' : 'Очистить кэш' }}
        </button>
      </div>
    </div>

    <div class="cache-info">
      <p>
        <i class="fas fa-info-circle"></i>
        Кэш ускоряет повторные запросы к AI, сохраняя ответы в памяти.
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface AiCacheStats {
  hits: number
  misses: number
  saves: number
  hit_rate: number
  size: number
}

const isLoading = ref(false)
const isClearing = ref(false)
const cacheEnabled = ref(true)

const stats = reactive<AiCacheStats>({
  hits: 0,
  misses: 0,
  saves: 0,
  hit_rate: 0,
  size: 0,
})

const loadStats = async () => {
  isLoading.value = true
  try {
    const loaded = await invoke<AiCacheStats>('ai_get_cache_stats')
    stats.hits = loaded.hits
    stats.misses = loaded.misses
    stats.saves = loaded.saves
    stats.hit_rate = loaded.hit_rate
    stats.size = loaded.size
  } catch (e) {
    console.error('Failed to load cache stats:', e)
  } finally {
    isLoading.value = false
  }
}

const clearCache = async () => {
  if (!confirm('Вы уверены, что хотите очистить кэш AI?')) return

  isClearing.value = true
  try {
    await invoke('ai_clear_cache')
    await loadStats()
  } catch (e) {
    console.error('Failed to clear cache:', e)
  } finally {
    isClearing.value = false
  }
}

const toggleCacheEnabled = async () => {
  try {
    await invoke('ai_set_cache_enabled', { enabled: cacheEnabled.value })
  } catch (e) {
    console.error('Failed to toggle cache:', e)
    cacheEnabled.value = !cacheEnabled.value
  }
}

const checkCacheContains = async (prompt: string): Promise<boolean> => {
  try {
    return await invoke<boolean>('ai_cache_contains', { prompt })
  } catch {
    return false
  }
}

onMounted(() => {
  loadStats()
})

defineExpose({
  checkCacheContains,
  loadStats,
})
</script>

<style scoped>
.ai-cache-stats {
  padding: 16px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 8px;
}

h3 {
  margin: 0 0 16px;
  font-size: 16px;
  color: var(--text-primary, #333);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-bottom: 16px;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-primary, #fff);
  border-radius: 6px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.stat-icon {
  font-size: 24px;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
}

.stat-icon.hits {
  background: #e8f5e9;
}

.stat-icon.misses {
  background: #ffebee;
}

.stat-icon.saves {
  background: #e3f2fd;
}

.stat-icon.rate {
  background: #fff3e0;
}

.stat-info {
  flex: 1;
}

.stat-value {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #333);
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary, #999);
}

.cache-controls {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.toggle-setting {
  display: flex;
  align-items: center;
}

.toggle-setting label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
}

.toggle-setting input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.action-buttons {
  display: flex;
  gap: 8px;
}

.btn-refresh,
.btn-clear {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-refresh {
  background: var(--bg-primary, #fff);
  border: 1px solid var(--border-color, #ddd);
  color: var(--text-primary, #333);
}

.btn-refresh:hover:not(:disabled) {
  background: var(--bg-tertiary, #e5e5e5);
}

.btn-clear {
  flex: 1;
  background: #ffebee;
  border: 1px solid #ffcdd2;
  color: #c62828;
}

.btn-clear:hover:not(:disabled) {
  background: #ffcdd2;
}

.btn-refresh:disabled,
.btn-clear:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.cache-info {
  padding: 12px;
  background: var(--bg-primary, #fff);
  border-radius: 6px;
  font-size: 13px;
  color: var(--text-secondary, #666);
}

.cache-info p {
  margin: 0;
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.cache-info i {
  color: var(--accent-color, #4a9eff);
  flex-shrink: 0;
}

.fa-spin {
  animation: fa-spin 1s infinite linear;
}

@keyframes fa-spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
