<template>
  <div class="stage-root">
    <section class="left-panel">
      <header class="panel-header">
        <h2 class="panel-title">Бумажная развёртка</h2>
        <p class="panel-subtitle">
          Просмотр раскроя по листам. Полноценное редактирование пока в разработке.
        </p>
      </header>

      <div class="paper-settings">
        <p class="group-title">Параметры бумаги</p>

        <label class="field">
          <span class="field-label">Формат листа</span>
          <select v-model="paperFormat" class="select">
            <option value="A4">A4 (210×297 мм)</option>
            <option value="A3">A3 (297×420 мм)</option>
            <option value="Letter">Letter (8.5×11")</option>
          </select>
        </label>

        <label class="field">
          <span class="field-label">Поля</span>
          <div class="field-inline">
            <input
              v-model.number="marginMm"
              type="number"
              class="input"
              min="0"
              max="30"
            />
            <span class="field-suffix">мм</span>
          </div>
        </label>

        <label class="field">
          <span class="field-label">Масштаб</span>
          <div class="field-inline">
            <input
              v-model.number="scalePercent"
              type="number"
              class="input"
              min="10"
              max="400"
            />
            <span class="field-suffix">%</span>
          </div>
        </label>
      </div>

      <div class="actions-group">
        <p class="group-title">Действия раскроя (в разработке)</p>
        <div class="group-buttons">
          <button class="ghost-btn" :disabled="!hasUnfold" @click="stub('repack')">
            Переразложить острова
          </button>
          <button class="ghost-btn" :disabled="!hasUnfold" @click="stub('move')">
            Ручное перемещение
          </button>
          <button class="ghost-btn" :disabled="!hasUnfold" @click="stub('tabs')">
            Клапаны и линии сгиба
          </button>
        </div>
      </div>

      <div class="actions-group">
        <p class="group-title">Экспорт</p>
        <div class="group-buttons">
          <button class="primary-btn" :disabled="!hasUnfold" @click="stub('export-pdf')">
            Экспорт в PDF (заглушка)
          </button>
          <button class="ghost-btn" :disabled="!hasUnfold" @click="stub('export-png')">
            Экспорт PNG (заглушка)
          </button>
        </div>
      </div>
    </section>

    <section class="center-panel">
      <header class="sub-header">
        <div class="sub-titles">
          <h3 class="sub-title">Предпросмотр листов</h3>
          <p class="sub-subtitle">
            Здесь будет реальный раскрой по листам. Сейчас отображается только заглушка.
          </p>
        </div>

        <div class="pages-info">
          <span class="pages-count">
            Листы: {{ sheetsText }}
          </span>
          <span class="pages-count">
            Острова: {{ patchesText }}
          </span>
        </div>
      </header>

      <div class="viewer">
        <div class="viewer-inner">
          <div v-if="hasUnfold" class="viewer-grid">
            <div
              v-for="n in sheetCount"
              :key="n"
              class="sheet-placeholder"
            >
              <div class="sheet-header">
                Лист {{ n }}
              </div>
              <div class="sheet-body">
                <p class="sheet-text">
                  Здесь будет раскладка островов для листа {{ n }}.
                </p>
              </div>
            </div>
          </div>
          <div v-else class="viewer-empty">
            <p>Развёртка ещё не построена. Получите 3D‑модель и запустите расчёт раскроя на стороне backend.</p>
          </div>
        </div>
      </div>
    </section>

    <aside class="right-panel">
      <h3 class="side-title">Состояние раскроя</h3>

      <div class="status-block">
        <p class="status-line">
          Статус:
          <span
            class="status-tag"
            :class="{
              ok: unfoldStatus === 'ready',
              bad: unfoldStatus === 'error',
              idle: unfoldStatus !== 'ready' && unfoldStatus !== 'error'
            }"
          >
            {{ unfoldStatusLabel }}
          </span>
        </p>
        <p class="status-line">
          Острова:
          <span class="status-value">
            {{ patchesText }}
          </span>
        </p>
        <p class="status-line">
          Листы:
          <span class="status-value">
            {{ sheetsText }}
          </span>
        </p>
      </div>

      <div class="side-section">
        <p class="side-label">Связь с 3D</p>
        <p class="side-note">
          {{ hasThreeD ? '3D‑модель готова, развёртка может быть пересчитана при изменениях.' : '3D‑модели нет — развёртка будет недоступна.' }}
        </p>
      </div>

      <div class="side-section">
        <p class="side-label">Подсказки по печати</p>
        <ul class="tips-list">
          <li>Учитывайте поля принтера, чтобы линии не обрезались.</li>
          <li>Проверяйте масштаб по тестовому квадрату 10×10 мм.</li>
          <li>Для больших моделей лучше использовать A3 или печать в масштабе &lt; 100%.</li>
        </ul>
      </div>
    </aside>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useTauriModelLoader } from '@/composables/useTauriModelLoader'

const project = useProjectStore()
const { showMessage } = useTauriModelLoader()

const unfoldStatus = computed(() => project.unfold.status)

const unfoldStatusLabel = computed(() => {
  switch (unfoldStatus.value) {
    case 'ready':
      return 'развёртка готова'
    case 'calculating':
      return 'идёт расчёт'
    case 'error':
      return 'ошибка расчёта'
    default:
      return 'нет данных'
  }
})

const hasUnfold = computed(() => unfoldStatus.value === 'ready')

const hasThreeD = computed(() => project.threeD.status === 'ready')

const patchesText = computed(() =>
  project.unfold.estimatedPatches != null
    ? `${project.unfold.estimatedPatches} островов`
    : '—'
)

const sheetsText = computed(() =>
  project.unfold.estimatedSheets != null
    ? `${project.unfold.estimatedSheets} листов`
    : '—'
)

const sheetCount = computed(() =>
  project.unfold.estimatedSheets != null ? project.unfold.estimatedSheets : 0
)

const paperFormat = ref<'A4' | 'A3' | 'Letter'>('A4')
const marginMm = ref(5)
const scalePercent = ref(100)

function stub(kind: 'repack' | 'move' | 'tabs' | 'export-pdf' | 'export-png') {
  switch (kind) {
    case 'repack':
      showMessage('Переразложение островов по листам пока в разработке.', 'info')
      break
    case 'move':
      showMessage('Интерактивное перемещение островов пока в разработке.', 'info')
      break
    case 'tabs':
      showMessage('Редактирование клапанов и линий сгиба пока в разработке.', 'info')
      break
    case 'export-pdf':
      showMessage('Экспорт развёртки в PDF пока в разработке.', 'info')
      break
    case 'export-png':
      showMessage('Экспорт листов в PNG пока в разработке.', 'info')
      break
  }
}
</script>

<style scoped>
.stage-root {
  flex: 1;
  display: flex;
  padding: 0.75rem;
  gap: 0.75rem;
  min-height: 0;
  border-radius: 0.9rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 40%, #000 100%);
}

/* левая панель – параметры и действия */
.left-panel {
  width: 260px;
  min-width: 220px;
  max-width: 320px;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.panel-header {
  padding-bottom: 0.3rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.5);
}

.panel-title {
  font-size: 0.95rem;
  font-weight: 500;
  margin: 0 0 0.15rem;
}

.panel-subtitle {
  font-size: 0.8rem;
  color: #9ca3af;
  margin: 0;
}

.paper-settings {
  border-radius: 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.6);
  padding: 0.4rem 0.5rem;
  background: rgba(15, 23, 42, 0.96);
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.group-title {
  font-size: 0.78rem;
  color: #cbd5e1;
  margin: 0 0 0.35rem;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.field-label {
  font-size: 0.78rem;
  color: #cbd5e1;
}

.field-inline {
  display: flex;
  align-items: center;
  gap: 0.3rem;
}

.field-suffix {
  font-size: 0.78rem;
  color: #9ca3af;
}

.select,
.input {
  border-radius: 0.5rem;
  border: 1px solid rgba(148, 163, 184, 0.8);
  background: rgba(15, 23, 42, 0.98);
  color: #e5e7eb;
  font-size: 0.8rem;
  padding: 0.25rem 0.5rem;
}

.actions-group {
  border-radius: 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.6);
  padding: 0.4rem 0.5rem;
  background: rgba(15, 23, 42, 0.96);
  margin-top: 0.3rem;
}

.group-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 0.3rem;
}

/* центр – viewer листов */
.center-panel {
  flex: 1.6;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.55rem;
}

.sub-header {
  display: flex;
  justify-content: space-between;
  gap: 0.8rem;
  align-items: flex-start;
}

.sub-titles {
  max-width: 70%;
}

.sub-title {
  font-size: 0.9rem;
  margin: 0 0 0.1rem;
}

.sub-subtitle {
  font-size: 0.78rem;
  color: #9ca3af;
  margin: 0;
}

.pages-info {
  display: flex;
  gap: 0.5rem;
  font-size: 0.78rem;
  color: #cbd5e1;
}

.viewer {
  flex: 1;
  min-height: 0;
}

.viewer-inner {
  height: 100%;
  border-radius: 0.8rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 45%, #000 100%);
  padding: 0.8rem;
  overflow: auto;
}

.viewer-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 0.6rem;
}

.sheet-placeholder {
  border-radius: 0.4rem;
  border: 1px dashed rgba(148, 163, 184, 0.8);
  background: rgba(15, 23, 42, 0.96);
  display: flex;
  flex-direction: column;
  min-height: 140px;
}

.sheet-header {
  padding: 0.3rem 0.4rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.5);
  font-size: 0.78rem;
  color: #cbd5e1;
}

.sheet-body {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.4rem;
}

.sheet-text {
  font-size: 0.78rem;
  color: #9ca3af;
  text-align: center;
  margin: 0;
}

.viewer-empty {
  text-align: center;
  max-width: 520px;
  font-size: 0.8rem;
  color: #cbd5e1;
}

/* правая панель – статус и подсказки */
.right-panel {
  width: 260px;
  min-width: 220px;
  max-width: 320px;
  border-left: 1px solid rgba(148, 163, 184, 0.5);
  padding-left: 0.7rem;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.side-title {
  font-size: 0.85rem;
  font-weight: 500;
  margin: 0;
}

.status-block {
  border-radius: 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.6);
  padding: 0.4rem 0.5rem;
  font-size: 0.78rem;
  background: rgba(15, 23, 42, 0.96);
}

.status-line {
  margin: 0.1rem 0;
  display: flex;
  gap: 0.25rem;
  align-items: baseline;
}

.status-tag {
  padding: 0.05rem 0.4rem;
  border-radius: 999px;
  font-size: 0.72rem;
  border: 1px solid rgba(148, 163, 184, 0.6);
}
.status-tag.ok {
  color: #bbf7d0;
  border-color: rgba(22, 163, 74, 0.9);
}
.status-tag.bad {
  color: #fecaca;
  border-color: rgba(239, 68, 68, 0.9);
}
.status-tag.idle {
  color: #9ca3af;
}

.status-value {
  color: #e5e7eb;
}

.side-section {
  font-size: 0.78rem;
}

.side-label {
  margin: 0 0 0.15rem;
  color: #cbd5e1;
}

.side-note {
  font-size: 0.76rem;
  color: #9ca3af;
  margin: 0.25rem 0 0;
}

.tips-list {
  margin: 0;
  padding-left: 1.1rem;
  color: #9ca3af;
}
.tips-list li {
  margin-bottom: 0.15rem;
}

/* кнопки */
.primary-btn {
  border-radius: 999px;
  border: 1px solid transparent;
  padding: 0.35rem 0.9rem;
  font-size: 0.8rem;
  cursor: pointer;
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #f9fafb;
}
.primary-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.ghost-btn {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.8);
  background: transparent;
  color: #e5e7eb;
  font-size: 0.75rem;
  padding: 0.18rem 0.7rem;
  cursor: pointer;
}
.ghost-btn:disabled {
  opacity: 0.4;
  cursor: default;
}
</style>
