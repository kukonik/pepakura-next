<template>
  <div class="settings-root">
    <section class="panel">
      <h2 class="panel-title">Настройки приложения</h2>
      <p class="panel-subtitle">
        Базовые параметры интерфейса и работы AI. Сделано просто, без перегрузки.
      </p>

      <div class="form">
        <div class="group">
          <h3 class="group-title">Интерфейс</h3>

          <label class="field">
            <span class="label">Режим интерфейса</span>
            <div class="segmented">
              <button
                class="segmented-btn"
                :class="{ active: interfaceMode === 'simple' }"
                type="button"
                @click="setInterfaceMode('simple')"
              >
                Простой
                <span class="segmented-caption">
                  Пошаговый мастер для новичков
                </span>
              </button>
              <button
                class="segmented-btn"
                :class="{ active: interfaceMode === 'advanced' }"
                type="button"
                @click="setInterfaceMode('advanced')"
              >
                Расширенный
                <span class="segmented-caption">
                  Сайдбар TXT/2D/3D/Paper и панели
                </span>
              </button>
            </div>
          </label>

          <label class="field">
            <span class="label">Тема</span>
            <select v-model="theme" class="select">
              <option value="dark">Тёмная</option>
              <option value="light">Светлая</option>
              <option value="system">Как в системе</option>
            </select>
          </label>
        </div>

        <div class="group">
          <h3 class="group-title">AI и генерация</h3>

          <label class="field">
            <span class="label">Режим AI</span>
            <select v-model="aiMode" class="select">
              <option value="auto">Авто</option>
              <option value="local">Только локальный</option>
              <option value="cloud">Только облачный</option>
            </select>
          </label>

          <label class="field checkbox">
            <input
              type="checkbox"
              v-model="sendTelemetry"
            />
            <span>Отправлять анонимную статистику использования</span>
          </label>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useUiStore } from '@/stores/ui'

const ui = useUiStore()

// interface mode из Pinia
const interfaceMode = computed(() => ui.interfaceMode)
function setInterfaceMode(mode: 'simple' | 'advanced') {
  ui.setInterfaceMode(mode)
}

// Остальные настройки пока локальные, при необходимости
// их можно будет вынести в отдельный settingsStore.
const theme = ref<'dark' | 'light' | 'system'>('dark')
const aiMode = ref<'auto' | 'local' | 'cloud'>('auto')
const sendTelemetry = ref(false)

// Пример простой persistance в localStorage (без требований к схеме)
watch(
  () => ({
    interfaceMode: ui.interfaceMode,
    theme: theme.value,
    aiMode: aiMode.value,
    sendTelemetry: sendTelemetry.value
  }),
  value => {
    try {
      window.localStorage.setItem('pepakura-next.settings', JSON.stringify(value))
    } catch {
      // ignore
    }
  },
  { deep: true }
)

// Инициализация из localStorage
const raw = window.localStorage.getItem('pepakura-next.settings')
if (raw) {
  try {
    const parsed = JSON.parse(raw) as {
      interfaceMode?: 'simple' | 'advanced'
      theme?: 'dark' | 'light' | 'system'
      aiMode?: 'auto' | 'local' | 'cloud'
      sendTelemetry?: boolean
    }

    if (parsed.interfaceMode === 'simple' || parsed.interfaceMode === 'advanced') {
      ui.setInterfaceMode(parsed.interfaceMode)
    }
    if (parsed.theme === 'dark' || parsed.theme === 'light' || parsed.theme === 'system') {
      theme.value = parsed.theme
    }
    if (parsed.aiMode === 'auto' || parsed.aiMode === 'local' || parsed.aiMode === 'cloud') {
      aiMode.value = parsed.aiMode
    }
    if (typeof parsed.sendTelemetry === 'boolean') {
      sendTelemetry.value = parsed.sendTelemetry
    }
  } catch {
    // ignore
  }
}
</script>

<style scoped>
.settings-root {
  height: 100%;
  padding: 1.2rem 1.6rem;
  color: #e5e7eb;
}

.panel {
  max-width: 720px;
}

.panel-title {
  font-size: 1.2rem;
  margin: 0 0 0.2rem;
  font-weight: 600;
}

.panel-subtitle {
  font-size: 0.9rem;
  color: #cbd5e1;
  margin: 0 0 0.8rem;
}

.form {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
}

.group {
  border-radius: 0.8rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: rgba(15, 23, 42, 0.96);
  padding: 0.7rem 0.8rem;
}

.group-title {
  font-size: 0.9rem;
  margin: 0 0 0.5rem;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  font-size: 0.8rem;
  margin-bottom: 0.6rem;
}

.label {
  color: #cbd5e1;
}

.select {
  border-radius: 0.5rem;
  padding: 0.3rem 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.8);
  background: rgba(15, 23, 42, 0.98);
  color: #e5e7eb;
}

.checkbox {
  flex-direction: row;
  align-items: center;
  gap: 0.4rem;
  margin-top: 0.3rem;
}

.segmented {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.4rem;
}

.segmented-btn {
  border-radius: 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: rgba(15, 23, 42, 0.98);
  color: #e5e7eb;
  font-size: 0.82rem;
  padding: 0.35rem 0.5rem;
  text-align: left;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}

.segmented-btn.active {
  background: radial-gradient(circle at top left, #3b82f6, #4f46e5);
  border-color: rgba(191, 219, 254, 0.95);
  color: #f9fafb;
}

.segmented-caption {
  font-size: 0.7rem;
  color: #cbd5e1;
}
</style>
