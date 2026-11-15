<template>
  <button 
    class="app-button" 
    :class="[variant, sizeClass, { disabled }]"
    :disabled="disabled"
    @click="handleClick"
  >
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'primary' | 'secondary' | 'danger'
  size?: 'small' | 'medium' | 'large'
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'medium',
  disabled: false
})

const sizeClass = computed(() => `size-${props.size}`)

const emit = defineEmits<{
  (e: 'click', event: MouseEvent): void
}>()

const handleClick = (event: MouseEvent) => {
  if (!props.disabled) {
    emit('click', event)
  }
}
</script>

<style scoped>
.app-button {
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.app-button:focus {
  outline: 2px solid var(--focus-outline);
  outline-offset: 2px;
}

.app-button.disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Variants */
.app-button.primary {
  background: var(--primary-color);
  color: white;
}

.app-button.primary:hover:not(.disabled) {
  background: var(--primary-hover);
}

.app-button.secondary {
  background: var(--secondary-bg);
  color: var(--text-color);
  border: 1px solid var(--border-color);
}

.app-button.secondary:hover:not(.disabled) {
  background: var(--secondary-hover);
}

.app-button.danger {
  background: var(--danger-color);
  color: white;
}

.app-button.danger:hover:not(.disabled) {
  background: var(--danger-hover);
}

/* Sizes */
.app-button.size-small {
  padding: 6px 12px;
  font-size: 0.875rem;
}

.app-button.size-medium {
  padding: 10px 16px;
  font-size: 1rem;
}

.app-button.size-large {
  padding: 12px 24px;
  font-size: 1.125rem;
}
</style>