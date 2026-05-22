<template>
  <teleport to="body">
    <div class="messages-root">
      <transition-group name="toast" tag="div">
        <div
          v-for="msg in messages"
          :key="msg.id"
          class="toast"
          :class="`toast-${msg.level}`"
        >
          {{ msg.text }}
        </div>
      </transition-group>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { useTauriModelLoader } from '@/composables/useTauriModelLoader'

const { messages } = useTauriModelLoader()
</script>

<style scoped>
.messages-root {
  position: fixed;
  right: 1rem;
  top: 1rem;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
  pointer-events: none;
}

.toast {
  min-width: 220px;
  max-width: 320px;
  padding: 0.4rem 0.7rem;
  border-radius: 0.6rem;
  font-size: 0.78rem;
  color: #e5e7eb;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: rgba(15, 23, 42, 0.96);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4);
  pointer-events: auto;
}

.toast-info {
  border-color: rgba(59, 130, 246, 0.8);
}

.toast-success {
  border-color: rgba(34, 197, 94, 0.8);
}

.toast-error {
  border-color: rgba(248, 113, 113, 0.85);
}

.toast-enter-active,
.toast-leave-active {
  transition: all 0.2s ease-out;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
