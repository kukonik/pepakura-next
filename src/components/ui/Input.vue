<!-- src/components/ui/Input.vue -->
<template>
  <div class="input-wrapper" :class="{ 'has-error': error, 'has-icon': icon }">
    <input
      ref="inputRef"
      :type="type"
      :value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      @input="handleInput"
      @focus="handleFocus"
      @blur="handleBlur"
    />
    <span v-if="icon" class="input-icon" @click="handleIconClick">
      {{ icon }}
    </span>
    <div v-if="error" class="input-error">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  modelValue?: string
  type?: string
  placeholder?: string
  disabled?: boolean
  error?: string
  icon?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  modelValue: ''
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
  'icon-click': []
  focus: []
  blur: []
}>()

const inputRef = ref<HTMLInputElement>()

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
}

const handleFocus = () => {
  emit('focus')
}

const handleBlur = () => {
  emit('blur')
}

const handleIconClick = () => {
  emit('icon-click')
}

// Публичный метод для фокуса
defineExpose({
  focus: () => inputRef.value?.focus()
})
</script>

<style scoped>
.input-wrapper {
  position: relative;
  width: 100%;
}

.input-wrapper input {
  width: 100%;
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  font-size: 1rem;
  background-color: var(--color-background);
  color: var(--color-text);
  transition: border-color 0.2s ease;
}

.input-wrapper input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.input-wrapper input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.input-wrapper.has-error input {
  border-color: var(--color-error);
}

.input-wrapper.has-icon input {
  padding-right: 2.5rem;
}

.input-icon {
  position: absolute;
  right: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  cursor: pointer;
  color: var(--color-text-secondary);
}

.input-error {
  color: var(--color-error);
  font-size: 0.875rem;
  margin-top: 0.25rem;
}
</style>
