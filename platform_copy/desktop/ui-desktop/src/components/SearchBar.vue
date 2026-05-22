<template>
  <div class="search-container">
    <i class="fas fa-search search-icon"></i>
    <input
      type="text"
      class="search-box"
      v-model="query"
      :placeholder="placeholder"
      @keyup.enter="onSearch"
      @input="onInput"
    />
    <div class="search-actions">
      <button class="search-btn secondary" @click="onWebSearch" :title="$t('actions.webSearch')">
        <i class="fas fa-globe"></i> {{ $t('actions.web') }}
      </button>
      <button class="search-btn primary" @click="onAiSearch" :title="$t('actions.aiSearch')">
        <i class="fas fa-robot"></i> {{ $t('actions.ai') }}
      </button>
    </div>
    <div v-if="suggestions.length > 0" class="suggestions">
      <div
        v-for="suggestion in suggestions"
        :key="suggestion.id"
        class="suggestion-item"
        @click="selectSuggestion(suggestion)"
      >
        <i :class="suggestion.icon"></i>
        <span>{{ suggestion.text }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface Suggestion {
  id: string
  text: string
  icon: string
  type: 'project' | 'file' | 'ai' | 'web'
}

const props = defineProps<{
  placeholder?: string
  modelValue?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  'search': [value: string]
  'web-search': [value: string]
  'ai-search': [value: string]
  'select-suggestion': [suggestion: Suggestion]
}>()

const query = ref(props.modelValue || '')
const suggestions = ref<Suggestion[]>([])

const placeholder = computed(() => props.placeholder || t('search.placeholder'))

// Mock suggestions based on query
watch(query, (newQuery) => {
  emit('update:modelValue', newQuery)
  if (newQuery.length > 2) {
    suggestions.value = [
      { id: '1', text: `Проект "${newQuery}"`, icon: 'fas fa-folder', type: 'project' },
      { id: '2', text: `Файл "${newQuery}.obj"`, icon: 'fas fa-file', type: 'file' },
      { id: '3', text: `AI: сгенерировать "${newQuery}"`, icon: 'fas fa-robot', type: 'ai' },
      { id: '4', text: `Искать в интернете "${newQuery}"`, icon: 'fas fa-globe', type: 'web' }
    ]
  } else {
    suggestions.value = []
  }
})

const onSearch = () => {
  emit('search', query.value)
}

const onInput = () => {
  // Already handled by watch
}

const onWebSearch = () => {
  emit('web-search', query.value)
}

const onAiSearch = () => {
  emit('ai-search', query.value)
}

const selectSuggestion = (suggestion: Suggestion) => {
  emit('select-suggestion', suggestion)
  query.value = suggestion.text
  suggestions.value = []
}
</script>

<style scoped>
.search-container {
  position: relative;
  flex: 1;
  max-width: 800px;
  margin: 0 auto;
}

.search-box {
  width: 100%;
  padding: 1rem 1rem 1rem 3rem;
  font-size: 1.1rem;
  border: 2px solid rgba(255, 255, 255, 0.1);
  border-radius: 2rem;
  background: rgba(30, 41, 59, 0.8);
  color: white;
  transition: all 0.3s ease;
}

.search-box:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.3);
}

.search-icon {
  position: absolute;
  left: 1.2rem;
  top: 50%;
  transform: translateY(-50%);
  color: #94a3b8;
  font-size: 1.2rem;
}

.search-actions {
  position: absolute;
  right: 0.5rem;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  gap: 0.5rem;
}

.search-btn {
  padding: 0.7rem 1.2rem;
  border: none;
  border-radius: 1.5rem;
  font-weight: 600;
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: white;
}

.search-btn.primary {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
}

.search-btn.secondary {
  background: rgba(71, 85, 105, 0.6);
}

.search-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.suggestions {
  position: absolute;
  top: calc(100% + 0.5rem);
  left: 0;
  right: 0;
  background: rgba(30, 41, 59, 0.95);
  border-radius: 1rem;
  padding: 0.5rem;
  z-index: 1000;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.suggestion-item {
  padding: 0.8rem 1rem;
  border-radius: 0.5rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.8rem;
  transition: background 0.2s;
}

.suggestion-item:hover {
  background: rgba(59, 130, 246, 0.2);
}

.suggestion-item i {
  color: #94a3b8;
  width: 1.2em;
}
</style>