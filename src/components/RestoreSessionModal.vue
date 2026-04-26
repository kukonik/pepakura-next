<template>
  <div v-if="show" class="modal-overlay">
    <div class="modal-content">
      <h2>Восстановление сессии</h2>
      <p>
        Обнаружена несохранённая сессия от
        <strong>{{ formattedDate }}</strong
        >.
      </p>
      <p>Хотите восстановить работу?</p>

      <div class="session-details" v-if="autosave">
        <p><strong>Дата:</strong> {{ formattedDate }}</p>
        <p><strong>Модель:</strong> {{ modelName }}</p>
        <p><strong>Развёртка:</strong> {{ hasUnfolded ? 'Да' : 'Нет' }}</p>
      </div>

      <div class="modal-actions">
        <button @click="restore" class="btn-primary">Восстановить</button>
        <button @click="discard" class="btn-secondary">Отклонить</button>
        <button @click="viewDetails" class="btn-link">Подробнее</button>
      </div>

      <div v-if="showDetails" class="details-panel">
        <h3>Детали автосохранения</h3>
        <pre>{{ autosaveDetails }}</pre>
        <button @click="showDetails = false" class="btn-link">Скрыть</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { autosaveService, type AutosaveRecord } from '../services/autosave.service'
import { useProjectStore } from '../stores/projectStore'

const props = defineProps<{
  show: boolean
  autosave: AutosaveRecord | null
}>()

const emit = defineEmits<{
  (e: 'restore'): void
  (e: 'discard'): void
  (e: 'close'): void
}>()

const showDetails = ref(false)
const projectStore = useProjectStore()

const formattedDate = computed(() => {
  if (!props.autosave) return ''
  return new Date(props.autosave.timestamp).toLocaleString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
})

const modelName = computed(() => {
  if (!props.autosave) return 'Неизвестно'
  try {
    // Попытка извлечь имя модели из состояния
    const state = JSON.parse(props.autosave.state)
    return state.modelPath || state.currentMesh?.name || 'Без названия'
  } catch {
    return 'Неизвестно'
  }
})

const hasUnfolded = computed(() => {
  if (!props.autosave) return false
  try {
    const state = JSON.parse(props.autosave.state)
    return !!state.unfoldedSvg || !!state.unfoldedResult
  } catch {
    return false
  }
})

const autosaveDetails = computed(() => {
  if (!props.autosave) return ''
  return JSON.stringify(props.autosave, null, 2)
})

const restore = async () => {
  if (!props.autosave?.id) return
  const success = await autosaveService.restore(props.autosave.id)
  if (success) {
    emit('restore')
  } else {
    alert('Не удалось восстановить сессию')
  }
}

const discard = async () => {
  if (!props.autosave?.id) return
  // Помечаем как повреждённое, чтобы больше не предлагать
  await autosaveService.markAsCorrupted(props.autosave.id)
  emit('discard')
}

const viewDetails = () => {
  showDetails.value = !showDetails.value
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  padding: 24px;
  max-width: 500px;
  width: 90%;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

h2 {
  margin-top: 0;
  color: #333;
}

.session-details {
  background: #f5f5f5;
  padding: 12px;
  border-radius: 6px;
  margin: 16px 0;
}

.session-details p {
  margin: 4px 0;
}

.modal-actions {
  display: flex;
  gap: 12px;
  margin-top: 20px;
}

.btn-primary {
  background-color: #007bff;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
  flex: 1;
}

.btn-primary:hover {
  background-color: #0056b3;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
  flex: 1;
}

.btn-secondary:hover {
  background-color: #545b62;
}

.btn-link {
  background: none;
  border: none;
  color: #007bff;
  cursor: pointer;
  text-decoration: underline;
  padding: 10px;
}

.details-panel {
  margin-top: 20px;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 6px;
  max-height: 200px;
  overflow-y: auto;
}

pre {
  font-size: 12px;
  white-space: pre-wrap;
}
</style>