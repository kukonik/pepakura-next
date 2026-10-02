Write-Host "`n🚀 Интеграция системного диалога файлов + превьюера...`n" -ForegroundColor Magenta

# ============================================================================
# 1. Создать компонент превьюера
# ============================================================================
$previewPath = "D:\Dev\pepakura-next\packages\ui-desktop\src\components\SimpleModelPreview.vue"
$previewContent = @'
<template>
  <div class="model-preview">
    <canvas ref="canvas" class="preview-canvas"></canvas>
    <div v-if="loading" class="preview-loading">📂 Загрузка модели...</div>
    <div v-if="error" class="preview-error">❌ {{ error }}</div>
    <div v-if="filePath" class="preview-info">
      <div class="preview-name">📄 {{ fileName }}</div>
      <div class="preview-path">{{ filePath }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const canvas = ref<HTMLCanvasElement | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const filePath = ref<string | null>(null)
const fileName = ref<string | null>(null)

const drawPlaceholder = () => {
  if (!canvas.value) return
  const ctx = canvas.value.getContext('2d')
  if (!ctx) return
  
  // Очистить канвас
  ctx.clearRect(0, 0, canvas.value.width, canvas.value.height)
  
  // Фон
  ctx.fillStyle = 'var(--bg-tertiary)'
  ctx.fillRect(0, 0, canvas.value.width, canvas.value.height)
  
  // Нарисовать 3D-куб как заглушку
  const centerX = canvas.value.width / 2
  const centerY = canvas.value.height / 2
  const size = 60
  
  ctx.fillStyle = '#6366f1'
  ctx.fillRect(centerX - size/2, centerY - size/2, size, size)
  
  // Добавить текст "3D"
  ctx.fillStyle = 'white'
  ctx.font = 'bold 24px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto'
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'
  ctx.fillText('3D', centerX, centerY)
  
  // Обводка
  ctx.strokeStyle = '#8b5cf6'
  ctx.lineWidth = 4
  ctx.strokeRect(centerX - size/2, centerY - size/2, size, size)
}

onMounted(() => {
  if (canvas.value) {
    canvas.value.width = 200
    canvas.value.height = 200
    drawPlaceholder()
  }
}

const loadModel = (path: string) => {
  filePath.value = path
  fileName.value = path.split(/[\\/]/).pop() || 'model.obj'
  drawPlaceholder()
}

defineExpose({ loadModel })
</script>

<style scoped>
.model-preview {
  position: relative;
  width: 200px;
  height: 200px;
  margin: 20px auto;
}

.preview-canvas {
  width: 100%;
  height: 100%;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  transition: var(--transition-normal);
}

.preview-canvas:hover {
  box-shadow: var(--neon-glow-blue);
  transform: scale(1.02);
}

.preview-loading, .preview-error, .preview-info {
  position: absolute;
  bottom: 10px;
  left: 0;
  right: 0;
  text-align: center;
  padding: 8px;
  border-radius: var(--radius-sm);
  font-size: 14px;
  font-weight: 500;
}

.preview-loading {
  background: rgba(99, 102, 241, 0.15);
  color: var(--accent-primary);
  animation: pulse 2s ease-in-out infinite;
}

.preview-error {
  background: rgba(239, 68, 68, 0.15);
  color: var(--error);
}

.preview-info {
  background: rgba(16, 185, 129, 0.15);
  color: var(--success);
}

.preview-name {
  font-weight: 700;
  font-size: 16px;
  margin-bottom: 4px;
}

.preview-path {
  font-size: 12px;
  color: var(--text-tertiary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 180px;
  margin: 0 auto;
}

@keyframes pulse {
  0%, 100% { opacity: 0.7; }
  50% { opacity: 1; }
}
</style>
