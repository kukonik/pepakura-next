<template>
  <div class="stage-root">
    <section class="text-main">
      <header class="panel-header">
        <h2 class="panel-title">AI генерация модели из текста</h2>
        <p class="panel-subtitle">
          Опишите объект словами, ассистент подготовит 2D/3D‑заготовку и настройки развёртки.
        </p>
      </header>

      <div class="content">
        <div class="editor-block">
          <label class="field-label">
            Текстовое описание
          </label>
          <textarea
            v-model="rawText"
            class="text-input"
            rows="8"
            placeholder="Например: «Низкополигональная голова дракона высотой около 20 см, с крупными гранями, без мелких деталей. Нужно собрать из 10–15 листов A4»."
          />
          <p class="hint">
            Чем конкретнее задание (размер, стилистика, количество деталей, формат бумаги), тем проще будет раскрой.
          </p>
        </div>

        <div class="ai-actions">
          <button
            class="primary-btn"
            :disabled="!canRunAi"
            @click="runAi"
          >
            <span>Сгенерировать проект из текста (заглушка)</span>
          </button>
          <p class="ai-note">
            Режим: Авто (AI доступен). Далее появится связка:
            текст → 2D референсы → 3D модель → развёртка.
          </p>
        </div>
      </div>
    </section>

    <aside class="side-panel">
      <h3 class="side-title">Состояние проекта</h3>

      <div class="status-block">
        <p class="status-line">
          3D модель:
          <span v-if="threeStatus === 'ready'" class="status-ok">
            готова ({{ facesText }}, {{ partsText }})
          </span>
          <span v-else-if="threeStatus === 'error'" class="status-bad">
            ошибка загрузки
          </span>
          <span v-else class="status-idle">
            не загружена
          </span>
        </p>
        <p class="status-line">
          Бумажная развёртка:
          <span v-if="hasUnfoldEstimates" class="status-ok">
            ~{{ patchesText }}, ~{{ sheetsText }}
          </span>
          <span v-else class="status-idle">
            оценки ещё не посчитаны
          </span>
        </p>
      </div>

      <div class="side-section">
        <p class="side-label">Подсказки для текста</p>
        <ul class="tips-list">
          <li>Укажите примерный размер готовой модели (высота/ширина в см).</li>
          <li>Выберите стиль: низкополигональный, реалистичный, супер‑упрощённый.</li>
          <li>Опишите желаемое количество листов и сложность сборки.</li>
        </ul>
      </div>
    </aside>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useTauriModelLoader } from '@/composables/useTauriModelLoader'

const project = useProjectStore()
const { showMessage } = useTauriModelLoader()

const rawText = computed({
  get: () => project.text.rawText,
  set: v => project.setText({ rawText: v })
})

const threeStatus = computed(() => project.threeD.status)

const facesText = computed(() =>
  project.threeD.faces != null ? `${project.threeD.faces} граней` : 'граней: —'
)

const partsText = computed(() =>
  project.threeD.parts != null ? `${project.threeD.parts} частей` : 'частей: —'
)

const hasUnfoldEstimates = computed(
  () =>
    project.unfold.estimatedPatches != null &&
    project.unfold.estimatedSheets != null
)

const patchesText = computed(() =>
  project.unfold.estimatedPatches != null
    ? `${project.unfold.estimatedPatches} 2D‑островов`
    : 'островов: —'
)

const sheetsText = computed(() =>
  project.unfold.estimatedSheets != null
    ? `${project.unfold.estimatedSheets} листов`
    : 'листов: —'
)

const canRunAi = computed(() => rawText.value.trim().length > 0)

function runAi() {
  if (!canRunAi.value) return
  showMessage(
    'AI‑генерация проекта по тексту пока не подключена. Будет добавлена связка: текст → референсы/3D/раскрой.',
    'info'
  )
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

.text-main {
  flex: 1.4;
  min-width: 0;
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

.content {
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
  margin-top: 0.2rem;
}

.editor-block {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.field-label {
  font-size: 0.8rem;
  color: #cbd5e1;
}

.text-input {
  width: 100%;
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.9);
  background: rgba(15, 23, 42, 0.97);
  color: #e5e7eb;
  padding: 0.5rem 0.7rem;
  font-family: inherit;
  font-size: 0.82rem;
  resize: vertical;
}

.text-input::placeholder {
  color: #6b7280;
}

.hint {
  font-size: 0.78rem;
  color: #9ca3af;
}

.ai-actions {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.primary-btn {
  align-self: flex-start;
  border-radius: 999px;
  border: 1px solid transparent;
  padding: 0.35rem 0.9rem;
  font-size: 0.8rem;
  cursor: pointer;
  background: linear-gradient(135deg, #22c55e, #16a34a);
  color: #f9fafb;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.4rem;
}

.primary-btn:disabled {
  opacity: 0.4;
  cursor: default;
}

.ai-note {
  font-size: 0.78rem;
  color: #9ca3af;
}

.side-panel {
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

.status-ok {
  color: #bbf7d0;
}
.status-bad {
  color: #fecaca;
}
.status-idle {
  color: #9ca3af;
}

.side-section {
  font-size: 0.78rem;
}

.side-label {
  margin: 0 0 0.15rem;
  color: #cbd5e1;
}

.tips-list {
  margin: 0;
  padding-left: 1.1rem;
  color: #9ca3af;
}
.tips-list li {
  margin-bottom: 0.15rem;
}
</style>
