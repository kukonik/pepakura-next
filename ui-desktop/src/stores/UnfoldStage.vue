<template>
  <div class="unfold-root">
    <div class="unfold-inner">
      <header class="unfold-header">
        <div class="header-left">
          <h2 class="header-title">Развёртка на листы (Paper)</h2>
          <p class="header-subtitle">
            Настройте авторазвёртку, масштаб и компоновку деталей на листах бумаги.
          </p>
        </div>
        <div class="header-right" v-if="hasModel">
          <div class="model-meta">
            <div class="meta-row">
              <span class="meta-label">Файл:</span>
              <span class="meta-value">{{ project.threeD.sourcePath || '—' }}</span>
            </div>
            <div class="meta-row">
              <span class="meta-label">Формат:</span>
              <span class="meta-value">{{ project.threeD.format || '—' }}</span>
            </div>
            <div class="meta-row">
              <span class="meta-label">Граней:</span>
              <span class="meta-value">{{ project.threeD.faces ?? '—' }}</span>
            </div>
            <div class="meta-row">
              <span class="meta-label">Частей:</span>
              <span class="meta-value">{{ project.threeD.parts ?? '—' }}</span>
            </div>
          </div>
        </div>
      </header>

      <section class="unfold-body">
        <aside class="settings-panel">
          <h3 class="panel-title">Параметры развёртки</h3>

          <div class="panel-section">
            <h4 class="section-title">Состояние</h4>
            <p class="status-line">
              Статус:
              <span
                class="status-chip"
                :class="`status-${unfoldStatus}`"
              >
                {{ statusLabel }}
              </span>
            </p>
            <p class="status-description">
              {{ statusDescription }}
            </p>
            <p v-if="unfoldError" class="status-error-text">
              {{ unfoldError }}
            </p>
          </div>

          <div class="panel-section">
            <h4 class="section-title">Листы</h4>
            <div class="field">
              <label class="field-label">Формат бумаги</label>
              <select
                v-model="paperFormat"
                class="field-input"
              >
                <option value="A4">A4 (210 × 297 мм)</option>
                <option value="A3">A3 (297 × 420 мм)</option>
                <option value="Letter">Letter (8.5 × 11")</option>
              </select>
            </div>

            <div class="field">
              <label class="field-label">Отступы по краям</label>
              <div class="field-inline">
                <input
                  v-model.number="marginMm"
                  type="number"
                  min="0"
                  max="50"
                  class="field-input"
                />
                <span class="field-suffix">мм</span>
              </div>
            </div>

            <div class="field">
              <label class="field-label">Максимум листов</label>
              <div class="field-inline">
                <input
                  v-model.number="maxSheets"
                  type="number"
                  min="1"
                  max="200"
                  class="field-input"
                />
                <span class="field-suffix">шт</span>
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h4 class="section-title">Масштаб</h4>
            <div class="field">
              <label class="field-label">Масштаб модели</label>
              <div class="field-inline">
                <input
                  v-model.number="scale"
                  type="number"
                  step="0.01"
                  min="0.01"
                  max="100"
                  class="field-input"
                />
                <span class="field-suffix">×</span>
              </div>
            </div>
          </div>

          <div class="panel-section buttons">
            <button
              type="button"
              class="primary-btn"
              :disabled="!hasModel || unfoldStatus === 'running'"
              @click="onRunAutoUnfold"
            >
              Авторазвёртка
            </button>
            <button
              type="button"
              class="ghost-btn"
              :disabled="unfoldStatus !== 'ready'"
              @click="onRefreshPreview"
            >
              Обновить превью
            </button>
          </div>

          <p v-if="!hasModel" class="hint-text">
            Загрузите 3D‑модель на предыдущем этапе, чтобы выполнить развёртку.
          </p>
        </aside>

        <main class="preview-panel">
          <header class="preview-header">
            <div class="preview-text">
              <h3 class="preview-title">
                Превью листов бумаги
              </h3>
              <p class="preview-subtitle">
                {{
                  hasModel
                    ? 'Просмотрите расположение деталей на листах и при необходимости измените параметры.'
                    : 'Нет загруженной модели. Развёртка недоступна.'
                }}
              </p>
            </div>
            <div class="preview-meta" v-if="hasModel && totalSheets > 0">
              <span class="pill">
                {{ totalSheets }} лист(ов)
              </span>
              <span class="pill">
                {{ totalParts }} деталей
              </span>
            </div>
          </header>

          <div class="preview-content">
            <div
              v-if="!hasModel"
              class="preview-empty"
            >
              <p class="empty-title">Нет развёртки</p>
              <p class="empty-text">
                Вернитесь на вкладку 3D, загрузите модель и запустите авторазвёртку.
              </p>
            </div>

            <div
              v-else-if="unfoldStatus === 'running'"
              class="preview-empty"
            >
              <p class="empty-title">Выполняется развёртка…</p>
              <p class="empty-text">
                Пожалуйста, подождите. Для сложных моделей это может занять немного времени.
              </p>
            </div>

            <div
              v-else-if="unfoldStatus === 'error'"
              class="preview-empty"
            >
              <p class="empty-title">Ошибка развёртки</p>
              <p class="empty-text">
                {{ unfoldError || 'При развёртке произошла ошибка.' }}
              </p>
            </div>

            <div
              v-else-if="unfoldStatus === 'ready' && sheets.length === 0"
              class="preview-empty"
            >
              <p class="empty-title">Нет листов</p>
              <p class="empty-text">
                Движок не вернул ни одного листа. Попробуйте изменить параметры или модель.
              </p>
            </div>

            <div
              v-else
              class="sheets-grid"
            >
              <div
                v-for="sheet in sheets"
                :key="sheet.id"
                class="sheet-card"
              >
                <div class="sheet-header">
                  <span class="sheet-title">Лист {{ sheet.index }}</span>
                  <span class="sheet-meta">
                    {{ sheet.parts.length }} деталей
                  </span>
                </div>
                <div class="sheet-preview">
                  <div
                    v-for="part in sheet.parts"
                    :key="part.id"
                    class="piece"
                    :style="pieceStyle(sheet, part)"
                  ></div>
                </div>
              </div>
            </div>
          </div>
        </main>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useUnfoldStore } from '@/stores/unfoldStore'

const project = useProjectStore()
const unfold = useUnfoldStore()

const hasModel = computed(() => !!project.threeD.workingPath)

const paperFormat = ref<'A4' | 'A3' | 'Letter'>('A4')
const marginMm = ref<number>(5)
const maxSheets = ref<number>(20)
const scale = ref<number>(1)

const unfoldStatus = computed(() => unfold.status)
const unfoldError = computed(() => unfold.errorMessage)
const sheets = computed(() => unfold.result?.sheets ?? [])
const totalSheets = computed(() => unfold.totalSheets)
const totalParts = computed(() => unfold.totalParts)

const statusLabel = computed(() => {
  switch (unfoldStatus.value) {
    case 'idle':
      return 'Ожидание'
    case 'running':
      return 'Выполняется...'
    case 'ready':
      return 'Готово'
    case 'error':
      return 'Ошибка'
  }
})

const statusDescription = computed(() => {
  switch (unfoldStatus.value) {
    case 'idle':
      return 'Запустите авторазвёртку, чтобы получить шаблоны листов.'
    case 'running':
      return 'Выполняется развёртка. Это может занять несколько секунд для сложных моделей.'
    case 'ready':
      return 'Развёртка выполнена. Проверьте превью листов и при необходимости измените параметры.'
    case 'error':
      return 'При развёртке произошла ошибка. Попробуйте изменить параметры или модель.'
  }
})

function onRunAutoUnfold() {
  if (!hasModel.value) return

  unfold.runAutoUnfold({
    paperFormat: paperFormat.value,
    marginMm: marginMm.value,
    maxSheets: maxSheets.value,
    scale: scale.value,
  })
}

function onRefreshPreview() {
  if (unfoldStatus.value === 'ready' && unfold.result) {
    const jitter = (value: number) => {
      const delta = (Math.random() - 0.5) * 6
      return Math.min(85, Math.max(5, value + delta))
    }

    unfold.result.sheets.forEach(sheet => {
      sheet.parts.forEach(part => {
        part.bounds.x = jitter(part.bounds.x)
        part.bounds.y = jitter(part.bounds.y)
      })
    }
  }
}

function pieceStyle(sheet: (typeof sheets.value)[number], part: any) {
  const usableWidth = sheet.width_mm - sheet.margin_mm * 2
  const usableHeight = sheet.height_mm - sheet.margin_mm * 2

  const xRel = (part.bounds.x - sheet.margin_mm) / usableWidth
  const yRel = (part.bounds.y - sheet.margin_mm) / usableHeight
  const wRel = part.bounds.width / usableWidth
  const hRel = part.bounds.height / usableHeight

  const xPercent = Math.max(0, Math.min(1, xRel)) * 100
  const yPercent = Math.max(0, Math.min(1, yRel)) * 100
  const wPercent = Math.max(2, Math.min(1, wRel)) * 100
  const hPercent = Math.max(2, Math.min(1, hRel)) * 100

  return {
    width: `${wPercent}%`,
    height: `${hPercent}%`,
    left: `${xPercent}%`,
    top: `${yPercent}%`,
  }
}
</script>

<style scoped>
.unfold-root {
  flex: 1;
  display: flex;
  min-height: 0;
}

.unfold-inner {
  flex: 1;
  border-radius: 0.9rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 40%, #000 100%);
  padding: 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.unfold-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 0.8rem;
  padding-bottom: 0.45rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.5);
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.header-title {
  margin: 0;
  font-size: 0.95rem;
  font-weight: 500;
}

.header-subtitle {
  margin: 0;
  font-size: 0.8rem;
  color: #9ca3af;
}

.header-right {
  max-width: 320px;
}

.model-meta {
  font-size: 0.78rem;
  background: rgba(15, 23, 42, 0.96);
  border-radius: 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.6);
  padding: 0.45rem 0.6rem;
}

.meta-row {
  display: flex;
  gap: 0.3rem;
}

.meta-label {
  color: #9ca3af;
}

.meta-value {
  color: #e5e7eb;
}

.unfold-body {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-columns: 260px minmax(0, 1fr);
  gap: 0.75rem;
  margin-top: 0.3rem;
}

.settings-panel {
  border-radius: 0.8rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 40%, #020617 100%);
  padding: 0.6rem 0.7rem;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.panel-title {
  margin: 0;
  font-size: 0.9rem;
  font-weight: 500;
}

.panel-section {
  border-top: 1px solid rgba(148, 163, 184, 0.5);
  padding-top: 0.45rem;
  margin-top: 0.3rem;
}

.panel-section.buttons {
  border-top: none;
  padding-top: 0.2rem;
  margin-top: 0.2rem;
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.section-title {
  margin: 0 0 0.3rem;
  font-size: 0.82rem;
  color: #e5e7eb;
}

.status-line {
  margin: 0;
  font-size: 0.8rem;
  color: #e5e7eb;
}

.status-chip {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin-left: 0.3rem;
  padding: 0.05rem 0.45rem;
  border-radius: 999px;
  font-size: 0.74rem;
  border: 1px solid transparent;
}

.status-idle {
  border-color: rgba(148, 163, 184, 0.8);
  background: rgba(15, 23, 42, 0.96);
  color: #e5e7eb;
}

.status-running {
  border-color: rgba(234, 179, 8, 0.9);
  background: rgba(161, 98, 7, 0.2);
  color: #facc15;
}

.status-ready {
  border-color: rgba(34, 197, 94, 0.9);
  background: rgba(22, 163, 74, 0.2);
  color: #bbf7d0;
}

.status-error {
  border-color: rgba(248, 113, 113, 0.9);
  background: rgba(185, 28, 28, 0.2);
  color: #fecaca;
}

.status-description {
  margin: 0.25rem 0 0;
  font-size: 0.78rem;
  color: #9ca3af;
}

.status-error-text {
  margin: 0.25rem 0 0;
  font-size: 0.76rem;
  color: #fecaca;
}

.field {
  margin-bottom: 0.35rem;
}

.field-label {
  display: block;
  font-size: 0.78rem;
  color: #cbd5e1;
  margin-bottom: 0.15rem;
}

.field-input {
  width: 100%;
  background: rgba(15, 23, 42, 0.96);
  border-radius: 0.4rem;
  border: 1px solid rgba(148, 163, 184, 0.8);
  padding: 0.18rem 0.35rem;
  font-size: 0.8rem;
  color: #e5e7eb;
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

.primary-btn,
.ghost-btn {
  width: 100%;
  border-radius: 0.7rem;
  padding: 0.3rem 0.6rem;
  font-size: 0.8rem;
  cursor: pointer;
}

.primary-btn {
  border: 1px solid rgba(59, 130, 246, 0.9);
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #f9fafb;
}

.primary-btn:disabled {
  opacity: 0.45;
  cursor: default;
}

.ghost-btn {
  border: 1px solid rgba(148, 163, 184, 0.8);
  background: rgba(15, 23, 42, 0.98);
  color: #e5e7eb;
}

.hint-text {
  margin: 0.2rem 0 0;
  font-size: 0.76rem;
  color: #9ca3af;
}

.preview-panel {
  border-radius: 0.8rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 40%, #000 100%);
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.preview-header {
  padding: 0.45rem 0.6rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.6);
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 0.7rem;
}

.preview-text {
  display: flex;
  flex-direction: column;
  gap: 0.08rem;
}

.preview-title {
  margin: 0;
  font-size: 0.9rem;
  font-weight: 500;
}

.preview-subtitle {
  margin: 0;
  font-size: 0.8rem;
  color: #9ca3af;
}

.preview-meta {
  display: flex;
  gap: 0.3rem;
}

.pill {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.8);
  padding: 0.12rem 0.5rem;
  font-size: 0.78rem;
  color: #e5e7eb;
}

.preview-content {
  flex: 1;
  min-height: 0;
  padding: 0.6rem;
}

.preview-empty {
  margin: auto;
  text-align: center;
  max-width: 360px;
  color: #e5e7eb;
}

.empty-title {
  font-size: 0.9rem;
  margin-bottom: 0.2rem;
}

.empty-text {
  font-size: 0.8rem;
  color: #9ca3af;
}

.sheets-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 0.6rem;
}

.sheet-card {
  border-radius: 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 70%, #020617 100%);
  padding: 0.4rem;
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.sheet-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.78rem;
  color: #cbd5e1;
}

.sheet-preview {
  position: relative;
  margin-top: 0.15rem;
  width: 100%;
  padding-top: 141.4%;
  border-radius: 0.4rem;
  background: radial-gradient(circle at top left, #020617 0, #020617 35%, #000 100%);
  overflow: hidden;
}

.piece {
  position: absolute;
  border-radius: 0.15rem;
  border: 1px solid rgba(148, 163, 184, 0.9);
  background: linear-gradient(
    135deg,
    rgba(59, 130, 246, 0.18),
    rgba(129, 140, 248, 0.28)
  );
}
</style>
