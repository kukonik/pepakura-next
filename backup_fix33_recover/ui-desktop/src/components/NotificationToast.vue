<template>
  <TransitionGroup name="toast-list" tag="div" class="toast-container">
    <div
      v-for="toast in toasts"
      :key="toast.id"
      class="toast-item"
      :class="toast.type"
    >
      <div class="toast-content">
        <span class="toast-message">{{ toast.message }}</span>
        <button @click="removeToast(toast.id)" class="toast-close">&times;</button>
      </div>
    </div>
  </TransitionGroup>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'

interface Toast {
  id: string
  message: string
  type: 'success' | 'error' | 'info'
}

const toasts: Ref<Toast[]> = ref([])

const addToast = (message: string, type: Toast['type'] = 'info') => {
  const id = crypto.randomUUID()
  toasts.value.push({ id, message, type })
  setTimeout(() => removeToast(id), 5000)
}

const removeToast = (id: string) => {
  toasts.value = toasts.value.filter(t => t.id !== id)
}

defineExpose({ addToast })
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-width: 400px;
}

.toast-item {
  padding: 12px 16px;
  border-radius: 8px;
  color: white;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  animation: slideIn 0.3s ease-out;
  max-width: 100%;
}

.toast-item.success { background: #10b981; }
.toast-item.error { background: #ef4444; }
.toast-item.info { background: #3b82f6; }

.toast-close {
  background: none;
  border: none;
  color: inherit;
  font-size: 18px;
  cursor: pointer;
  margin-left: 12px;
  opacity: 0.8;
}

.toast-close:hover { opacity: 1; }

@keyframes slideIn {
  from { transform: translateX(100%); opacity: 0; }
  to { transform: translateX(0); opacity: 1; }
}
</style>
