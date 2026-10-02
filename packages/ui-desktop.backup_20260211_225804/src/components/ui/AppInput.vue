<template>
  <div class="app-input-wrapper">
    <label v-if="label" :for="id" class="input-label">{{ label }}</label>
    <input
      :id="id"
      :type="type"
      :value="modelValue"
      @input="handleInput"
      :placeholder="placeholder"
      :disabled="disabled"
      class="app-input"
      :class="{ error: hasError }"
    />
    <div v-if="hasError" class="error-message">{{ errorMessage }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  modelValue?: string
  label?: string
  placeholder?: string
  type?: string
  disabled?: boolean
  hasError?: boolean
  errorMessage?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  label: '',
  placeholder: '',
  type: 'text',
  disabled: false,
  hasError: false,
  errorMessage: ''
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

// Generate unique ID for input
const id = computed(() => `input-${Math.random().toString(36).substr(2, 9)}`)

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
}
</script>

<style scoped>
.app-input-wrapper {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 16px;
}

.input-label {
  font-weight: 500;
  color: var(--text-color);
}

.app-input {
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 1rem;
  transition: border-color 0.2s;
  background: var(--input-bg);
  color: var(--text-color);
}

.app-input:focus {
  outline: none;
  border-color: var(--focus-outline);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.app-input.error {
  border-color: var(--danger-color);
}

.app-input:disabled {
  background: var(--input-disabled-bg);
  cursor: not-allowed;
}

.error-message {
  color: var(--danger-color);
  font-size: 0.875rem;
}
</style>