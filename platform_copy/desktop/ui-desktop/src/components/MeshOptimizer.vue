<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useProjectStore } from '../stores/projectStore'

const store = useProjectStore()

// Параметры UV-развёртки
const chartAngle = ref(75) // градусы
const packResolution = ref(1024) // пиксели
const padding = ref(4) // пиксели
const bilinear = ref(true)
const blockAlign = ref(false)
const bruteForce = ref(false)
const maxChartSize = ref(0) // 0 = без ограничений
const maxIterations = ref(1)

// Состояние выполнения
const isUnwrapping = ref(false)
const uvPreviewUrl = ref<string | null>(null)
const errorMessage = ref<string | null>(null)
const lastResult = ref<any>(null)

// Вычисляемые параметры для отображения
const chartAngleLabel = computed(() => `${chartAngle.value}°`)
const packResolutionLabel = computed(() => `${packResolution.value} px`)
const paddingLabel = computed(() => `${padding.value} px`)
const maxChartSizeLabel = computed(() => maxChartSize.value === 0 ? 'Без ограничений' : `${maxChartSize.value} вершин`)

// Функция для запуска UV-развёртки
const unwrapMesh = async () => {
  if (!store.modelPath) {
    errorMessage.value = 'Сначала загрузите 3D модель'
    return
  }

  isUnwrapping.value = true
  errorMessage.value = null
  uvPreviewUrl.value = null

  try {
    const options = {
      chart_angle: chartAngle.value,
      pack_resolution: packResolution.value,
      padding: padding.value,
      bilinear: bilinear.value,
      block_align: blockAlign.value,
      brute_force: bruteForce.value,
      max_chart_size: maxChartSize.value,
      max_iterations: maxIterations.value,
    }

    const result = await invoke<any>('mesh_unwrap', {
      filePath: store.modelPath,
      options,
    })

    lastResult.value = result

    if (result.success) {
      // Предполагаем, что результат содержит путь к текстуре или данные UV
      if (result.uv_texture_path) {
        // Для превью можно загрузить изображение
        uvPreviewUrl.value = result.uv_texture_path
      }
      // Обновляем модель в хранилище (если нужно)
      store.modelPath = result.updated_mesh_path || store.modelPath
      store.modelInfo = result.info || store.modelInfo
    } else {
      errorMessage.value = result.error || 'Неизвестная ошибка'
    }
  } catch (err) {
    errorMessage.value = err instanceof Error ? err.message : String(err)
  } finally {
    isUnwrapping.value = false
  }
}

// Сброс параметров к значениям по умолчанию
const resetOptions = () => {
  chartAngle.value = 75
  packResolution.value = 1024
  padding.value = 4
  bilinear.value = true
  blockAlign.value = false
  bruteForce.value = false
  maxChartSize.value = 0
  maxIterations.value = 1
}

// Автоматический запуск при изменении параметров (опционально)
watch([chartAngle, packResolution, padding], () => {
  // Можно добавить debounce и автоматический пересчёт, но пока оставим ручной запуск
})
</script>

<template>
  <div class="mesh-optimizer">
    <div class="section-header">
      <h3 class="section-title">
        <i class="fas fa-map"></i> UV Unwrap
      </h3>
      <div class="badge">xatlas</div>
    </div>

    <div class="form-group">
      <label>
        <i class="fas fa-angle-double-right"></i>
        Угол чарта: {{ chartAngleLabel }}
      </label>
      <input
        type="range"
        v-model.number="chartAngle"
        min="1"
        max="180"
        step="1"
        class="form-control"
      />
      <div class="hint">Определяет максимальный угол между гранями для объединения в чарт.</div>
    </div>

    <div class="form-group">
      <label>
        <i class="fas fa-expand-alt"></i>
        Разрешение упаковки: {{ packResolutionLabel }}
      </label>
      <input
        type="range"
        v-model.number="packResolution"
        min="256"
        max="4096"
        step="256"
        class="form-control"
      />
      <div class="hint">Размер текстуры атласа (ширина и высота).</div>
    </div>

    <div class="form-group">
      <label>
        <i class="fas fa-border-style"></i>
        Отступ: {{ paddingLabel }}
      </label>
      <input
        type="range"
        v-model.number="padding"
        min="0"
        max="32"
        step="1"
        class="form-control"
      />
      <div class="hint">Отступ между чартами в пикселях.</div>
    </div>

    <div class="form-group">
      <label>
        <i class="fas fa-vector-square"></i>
        Максимальный размер чарта: {{ maxChartSizeLabel }}
      </label>
      <input
        type="range"
        v-model.number="maxChartSize"
        min="0"
        max="10000"
        step="100"
        class="form-control"
      />
      <div class="hint">0 = без ограничений. Ограничивает количество вершин в одном чарте.</div>
    </div>

    <div class="form-group">
      <label>
        <i class="fas fa-sync-alt"></i>
        Максимум итераций: {{ maxIterations }}
      </label>
      <input
        type="range"
        v-model.number="maxIterations"
        min="1"
        max="10"
        step="1"
        class="form-control"
      />
      <div class="hint">Количество итераций алгоритма упаковки.</div>
    </div>

    <div class="checkbox-group">
      <label>
        <input type="checkbox" v-model="bilinear" />
        <span>Билинейная фильтрация</span>
      </label>
      <label>
        <input type="checkbox" v-model="blockAlign" />
        <span>Выравнивание по блокам</span>
      </label>
      <label>
        <input type="checkbox" v-model="bruteForce" />
        <span>Полный перебор (медленно)</span>
      </label>
    </div>

    <div class="action-buttons">
      <button
        class="btn primary"
        @click="unwrapMesh"
        :disabled="isUnwrapping || !store.modelPath"
      >
        <i class="fas fa-cut" v-if="!isUnwrapping"></i>
        <i class="fas fa-spinner fa-spin" v-else></i>
        {{ isUnwrapping ? 'Выполняется...' : 'Запустить UV‑развёртку' }}
      </button>
      <button class="btn secondary" @click="resetOptions">
        <i class="fas fa-undo"></i> Сбросить
      </button>
    </div>

    <div v-if="errorMessage" class="error-message">
      <i class="fas fa-exclamation-triangle"></i> {{ errorMessage }}
    </div>

    <div v-if="uvPreviewUrl" class="preview-section">
      <h4><i class="fas fa-image"></i> Превью UV-атласа</h4>
      <div class="preview-container">
        <img :src="uvPreviewUrl" alt="UV Atlas" class="preview-image" />
      </div>
      <div class="preview-info" v-if="lastResult">
        <p><strong>Чартов:</strong> {{ lastResult.chart_count || 'N/A' }}</p>
        <p><strong>Площадь использования:</strong> {{ lastResult.utilization ? (lastResult.utilization * 100).toFixed(1) + '%' : 'N/A' }}</p>
        <p><strong>Время:</strong> {{ lastResult.elapsed_time ? lastResult.elapsed_time.toFixed(2) + ' с' : 'N/A' }}</p>
      </div>
    </div>

    <div v-else class="preview-placeholder">
      <i class="fas fa-map-marked-alt"></i>
      <p>После развёртки здесь появится UV-атлас.</p>
    </div>
  </div>
</template>

<style scoped>
.mesh-optimizer {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 14px;
  padding: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
  display: flex;
  flex-direction: column;
  gap: 1.2rem;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.section-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #e2e8f0;
  display: flex;
  align-items: center;
  gap: 0.7rem;
}

.section-title i {
  color: #60a5fa;
}

.badge {
  background: rgba(96, 165, 250, 0.2);
  color: #60a5fa;
  padding: 0.2rem 0.6rem;
  border-radius: 10px;
  font-size: 0.75rem;
}

.form-group {
  margin-bottom: 1rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  color: #cbd5e1;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.hint {
  font-size: 0.8rem;
  color: #94a3b8;
  margin-top: 0.3rem;
}

.form-control {
  width: 100%;
  padding: 0.8rem;
  background: rgba(15, 23, 42, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: #e2e8f0;
}

.form-control:focus {
  outline: none;
  border-color: #60a5fa;
  box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
  margin: 1rem 0;
}

.checkbox-group label {
  display: flex;
  align-items: center;
  gap: 0.7rem;
  cursor: pointer;
}

.checkbox-group input[type="checkbox"] {
  width: 1.2rem;
  height: 1.2rem;
  accent-color: #60a5fa;
}

.action-buttons {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

.btn {
  padding: 0.8rem 1.5rem;
  border-radius: 10px;
  border: none;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.7rem;
  transition: all 0.2s;
  flex: 1;
}

.btn.primary {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.btn.primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 0 15px rgba(96, 165, 250, 0.5);
}

.btn.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.secondary {
  background: rgba(71, 85, 105, 0.6);
  color: #e2e8f0;
}

.btn.secondary:hover {
  background: rgba(96, 165, 250, 0.2);
}

.error-message {
  background: rgba(239, 68, 68, 0.2);
  border: 1px solid rgba(239, 68, 68, 0.4);
  color: #fca5a5;
  padding: 0.8rem;
  border-radius: 10px;
  font-size: 0.9rem;
  display: flex;
  align-items: center;
  gap: 0.7rem;
}

.preview-section {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.preview-section h4 {
  font-size: 1rem;
  color: #cbd5e1;
  margin-bottom: 1rem;
  display: flex;
  align-items: center;
  gap: 0.7rem;
}

.preview-container {
  background: rgba(15, 23, 42, 0.8);
  border-radius: 10px;
  padding: 1rem;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 200px;
}

.preview-image {
  max-width: 100%;
  max-height: 300px;
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.preview-info {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
  margin-top: 1rem;
  font-size: 0.9rem;
  color: #94a3b8;
}

.preview-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  color: #64748b;
  text-align: center;
  border: 2px dashed rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  margin-top: 1rem;
}

.preview-placeholder i {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}
</style>