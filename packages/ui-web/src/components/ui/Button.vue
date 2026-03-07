<!-- src/components/ui/Button.vue -->
<template>
  <button :class="computedClasses" :disabled="loading || disabled" @click="handleClick">
    <span v-if="loading" class="button-loader"></span>
    <span v-if="$slots.default" class="button-content">
      <slot />
    </span>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger'
  size?: 'xs' | 'sm' | 'md' | 'lg'
  loading?: boolean
  disabled?: boolean
  fullWidth?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  loading: false,
  disabled: false,
  fullWidth: false
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const computedClasses = computed(() => [
  'button',
  `button-${props.variant}`,
  `button-${props.size}`,
  {
    'button-loading': props.loading,
    'button-disabled': props.disabled,
    'button-full-width': props.fullWidth
  }
])

const handleClick = (event: MouseEvent) => {
  if (!props.loading && !props.disabled) {
    emit('click', event)
  }
}
</script>

<style scoped>
.button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
  outline: none;
}

.button:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

/* Variants */
.button-primary {
  background-color: var(--color-primary);
  color: white;
}

.button-primary:hover:not(:disabled) {
  background-color: var(--color-primary-dark);
}

.button-secondary {
  background-color: var(--color-secondary);
  color: white;
}

.button-outline {
  background-color: transparent;
  border: 1px solid var(--color-border);
  color: var(--color-text);
}

.button-outline:hover:not(:disabled) {
  border-color: var(--color-primary);
}

.button-ghost {
  background-color: transparent;
  color: var(--color-text);
}

.button-ghost:hover:not(:disabled) {
  background-color: var(--color-surface-variant);
}

.button-danger {
  background-color: var(--color-error);
  color: white;
}

/* Sizes */
.button-xs {
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
  height: 24px;
}

.button-sm {
  padding: 0.375rem 0.75rem;
  font-size: 0.875rem;
  height: 32px;
}

.button-md {
  padding: 0.5rem 1rem;
  font-size: 1rem;
  height: 40px;
}

.button-lg {
  padding: 0.75rem 1.5rem;
  font-size: 1.125rem;
  height: 48px;
}

.button-full-width {
  width: 100%;
}

.button-loader {
  width: 16px;
  height: 16px;
  border: 2px solid currentColor;
  border-bottom-color: transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-right: 0.5rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
