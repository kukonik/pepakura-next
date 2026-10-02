<template>
  <div class="stage-root">
    <section class="left-panel">
      <header class="panel-header">
        <h2 class="panel-title">2D референсы и чертежи</h2>
        <p class="panel-subtitle">
          Здесь будут эскизы, ракурсы и схемы, которые пойдут в 3D‑генерацию.
        </p>
      </header>

      <div class="refs-list">
        <div class="refs-header">
          <span class="refs-title">Референсы проекта</span>
          <button class="small-btn" @click="addRefStub">
            Добавить изображение (заглушка)
          </button>
        </div>

        <div
          v-if="refs.length === 0"
          class="refs-empty"
        >
          Пока нет референсов. Здесь появятся загруженные и сгенерированные изображения.
        </div>

        <ul
          v-else
          class="refs-items"
        >
          <li
            v-for="ref in refs"
            :key="ref.id"
            class="refs-item"
            :class="{ active: ref.id === activeRefId }"
            @click="setActiveRef(ref.id)"
          >
            <div class="thumb" />
            <div class="ref-meta">
              <div class="ref-line">
                <span class="ref-name">{{ ref.name }}</span>
                <span class="ref-tag">{{ ref.kindLabel }}</span>
              </div>
              <div class="ref-line meta">
                <span class="ref-origin">{{ ref.originLabel }}</span>
                <span class="ref-size">{{ ref.sizeLabel }}</span>
              </div>
            </div>
          </li>
        </ul>
      </div>
    </section>

    <section class="center-panel">
      <header class="sub-header">
        <div class="sub-titles">
          <h3 class="sub-title">
            Предпросмотр кадра
          </h3>
          <p class="sub-subtitle">
            Пока только просмотр. В дальнейшем здесь появятся кадрирование, маски и разметка для AI.
          </p>
        </div>

        <div class="actions-row">
          <button
            class="ghost-btn"
            :disabled="!activeRef"
            @click="showStub('crop')"
          >
            Обрезать кадр (в разработке)
          </button>
          <button
            class="ghost-btn"
            :disabled="!activeRef"
            @click="showStub('mask')"
          >
            Маска / выделение (в разработке)
          </button>
          <button
            class="ghost-btn"
            :disabled="!activeRef"
            @click="showStub('ai-view')"
          >
            Новый ракурс (AI, в разработке)
          </button>
        </div>
      </header>

      <div class="viewer">
        <div class="viewer-inner">
          <div v-if="activeRef" class="viewer-placeholder">
            <p class="viewer-title">
              {{ activeRef.name }}
            </p>
            <p class="viewer-text">
              Здесь будет реальное 2D‑изображение/чертёж. Пока заглушка для предпросмотра.
            </p>
            <p class="viewer-meta">
              {{ activeRef.originLabel }} • {{ activeRef.kindLabel }} • {{ activeRef.sizeLabel }}
            </p>
          </div>
          <div v-else class="viewer-empty">
            <p>Выберите референс слева или добавьте новый (пока заглушка).</p>
          </div>
        </div>
      </div>
    </section>

    <aside class="right-panel">
      <h3 class="side-title">Состояние 2D‑этапа</h3>

      <div class="status-block">
        <p class="status-line">
          Референсы:
          <span class="status-value">
            {{ refs.length }} шт.
          </span>
        </p>
        <p class="status-line">
          Связь с TXT:
          <span class="status-tag">
            {{ hasTxtLink ? 'Есть текстовое ТЗ' : 'Без текста' }}
          </span>
        </p>
        <p class="status-line">
          Связь с 3D:
          <span class="status-tag">
            {{ hasThreeDLink ? 'Есть 3D‑модель' : 'Ещё нет 3D' }}
          </span>
        </p>
      </div>

      <div class="side-section">
        <p class="side-label">Действия этапа 2D → 3D</p>
        <button
          class="primary-btn"
          :disabled="!refs.length"
          @click="showStub('send-to-3d')"
        >
          Использовать референсы для 3D (заглушка)
        </button>
        <p class="side-note">
          В будущем здесь будет запуск AI‑пайплайна 2D → 3D (многоракурсная реконструкция, текст‑подсказки).
        </p>
      </div>

      <div class="side-section">
        <p class="side-label">Подсказки по 2D</p>
        <ul class="tips-list">
          <li>Загрузите хотя бы 2–3 ракурса объекта.</li>
          <li>Добавьте чертежи или схемы, если они есть.</li>
          <li>Старайтесь избегать слишком тёмных и размытых изображений.</li>
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

type RefKind = 'photo' | 'sketch' | 'blueprint'
type RefOrigin = 'imported' | 'generated' | 'mixed'

interface TwoDRef {
  id: string
  name: string
  kind: RefKind
  origin: RefOrigin
  width: number | null
  height: number | null
}

const refs = ref<TwoDRef[]>([])

const activeRefId = ref<string | null>(null)

const activeRef = computed(() => refs.value.find(r => r.id === activeRefId.value) ?? null)

const hasTxtLink = computed(() => {
  const txt = project.text?.rawText
  return typeof txt === 'string' && txt.trim().length > 0
})

const hasThreeDLink = computed(() => project.threeD.status === 'ready')

function setActiveRef(id: string) {
  activeRefId.value = id
}

function addRefStub() {
  const id = `ref-${Date.now()}`
  const item: TwoDRef = {
    id,
    name: `Референс ${refs.value.length + 1}`,
    kind: 'photo',
    origin: 'imported',
    width: null,
    height: null
  }
  refs.value = [...refs.value, item]
  activeRefId.value = id

  showMessage(
    'Пока добавляется только заглушка референса. Загрузка файлов и реальных изображений будет подключена позже.',
    'info'
  )
}

function showStub(kind: 'crop' | 'mask' | 'ai-view' | 'send-to-3d') {
  if (!activeRef.value && kind !== 'send-to-3d') {
    showMessage('Сначала выберите референс слева.', 'warning')
    return
  }

  switch (kind) {
    case 'crop':
      showMessage('Обрезка/кадрирование 2D‑изображения пока в разработке.', 'info')
      break
    case 'mask':
      showMessage('Редактор масок и выделений пока в разработке.', 'info')
      break
    case 'ai-view':
      showMessage('Генерация новых ракурсов по 2D‑референсам пока в разработке.', 'info')
      break
    case 'send-to-3d':
      showMessage(
        'Передача 2D‑референсов в 3D‑этап будет добавлена позже. Сейчас это только заглушка действия.',
        'info'
      )
      break
  }
}

const refsWithLabels = computed(() =>
  refs.value.map(r => ({
    ...r,
    kindLabel:
      r.kind === 'photo'
        ? 'Фото/рендер'
        : r.kind === 'sketch'
          ? 'Эскиз'
          : 'Чертёж',
    originLabel:
      r.origin === 'imported'
        ? 'Импорт'
        : r.origin === 'generated'
          ? 'AI‑генерация'
          : 'Смешанный',
    sizeLabel:
      r.width && r.height
        ? `${r.width}×${r.height}px`
        : 'Размер неизвестен'
  }))
)

const refsComputed = computed(() => refsWithLabels.value)

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

/* левая колонка – список референсов */
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

.refs-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.refs-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.refs-title {
  font-size: 0.8rem;
  color: #cbd5e1;
}

.refs-empty {
  font-size: 0.78rem;
  color: #9ca3af;
  padding: 0.4rem 0;
}

.refs-items {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  overflow-y: auto;
}

.refs-item {
  display: flex;
  gap: 0.4rem;
  padding: 0.3rem 0.35rem;
  border-radius: 0.6rem;
  cursor: pointer;
  border: 1px solid transparent;
  background: rgba(15, 23, 42, 0.9);
}

.refs-item.active {
  border-color: rgba(59, 130, 246, 0.9);
  background: radial-gradient(circle at top left, #1e293b, #020617);
}

.thumb {
  width: 40px;
  height: 40px;
  border-radius: 0.4rem;
  background: #020617;
  border: 1px solid rgba(148, 163, 184, 0.6);
}

.ref-meta {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.ref-line {
  display: flex;
  justify-content: space-between;
  gap: 0.3rem;
}

.ref-name {
  font-size: 0.8rem;
  color: #e5e7eb;
}

.ref-tag {
  font-size: 0.72rem;
  color: #cbd5e1;
}

.ref-line.meta {
  font-size: 0.72rem;
  color: #9ca3af;
}

.ref-origin {
}

.ref-size {
}

/* центр – viewer и панель действий */
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
  max-width: 60%;
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

.actions-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.3rem;
  justify-content: flex-end;
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
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.8rem;
}

.viewer-placeholder,
.viewer-empty {
  text-align: center;
  max-width: 480px;
}

.viewer-title {
  font-size: 0.9rem;
  margin: 0 0 0.25rem;
}

.viewer-text {
  font-size: 0.8rem;
  color: #cbd5e1;
  margin: 0 0 0.2rem;
}

.viewer-meta {
  font-size: 0.75rem;
  color: #9ca3af;
}

/* правая колонка – статус и действия */
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

.status-value {
  color: #e5e7eb;
}

.status-tag {
  color: #cbd5e1;
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

/* общие кнопки */
.small-btn {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.8);
  background: rgba(15, 23, 42, 0.96);
  color: #e5e7eb;
  font-size: 0.75rem;
  padding: 0.15rem 0.6rem;
  cursor: pointer;
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
</style>
