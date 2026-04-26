<!-- Компонент выбора пресета настроек развёртки -->
<template>
  <div class="preset-selector">
    <label v-if="label" class="preset-selector-label">{{ label }}</label>
    <div class="preset-selector-dropdown">
      <select
        :value="modelValue"
        @change="onSelectChange"
        class="preset-selector-select"
        :disabled="disabled"
      >
        <option value="" disabled v-if="showEmptyOption">
          {{ emptyOptionText }}
        </option>
        
        <!-- Группа стандартных пресетов -->
        <optgroup :label="standardGroupLabel">
          <option
            v-for="preset in groupedPresets.standard"
            :key="preset.id"
            :value="preset.id"
            class="preset-option"
            :data-category="preset.category"
          >
            {{ preset.icon }} {{ preset.name }}
          </option>
        </optgroup>
        
        <!-- Группа пользовательских пресетов, если они есть -->
        <optgroup
          v-if="groupedPresets.custom.length > 0"
          :label="customGroupLabel"
        >
          <option
            v-for="preset in groupedPresets.custom"
            :key="preset.id"
            :value="preset.id"
            class="preset-option"
            :data-category="preset.category"
          >
            {{ preset.icon }} {{ preset.name }}
          </option>
        </optgroup>
      </select>
      
      <div class="preset-selector-actions" v-if="showActions">
        <button
          v-if="showApplyButton"
          @click="applySelected"
          class="preset-apply-button"
          :disabled="!modelValue"
        >
          Применить
        </button>
        <button
          v-if="showSaveButton"
          @click="emit('save')"
          class="preset-save-button"
        >
          Сохранить как...
        </button>
      </div>
    </div>
    
    <!-- Информация о выбранном пресете -->
    <div v-if="showDetails && activePreset" class="preset-details">
      <div class="preset-details-header">
        <span class="preset-icon">{{ activePreset.icon }}</span>
        <span class="preset-name">{{ activePreset.name }}</span>
      </div>
      <div v-if="activePresetConfig" class="preset-config-summary">
        <div class="config-item">
          <span class="config-label">Бумага:</span>
          <span class="config-value">{{ activePresetConfig.paperSize || 'A4' }}</span>
        </div>
        <div class="config-item">
          <span class="config-label">Масштаб:</span>
          <span class="config-value">{{ activePresetConfig.scale || 1 }}x</span>
        </div>
        <div class="config-item">
          <span class="config-label">Клапаны:</span>
          <span class="config-value">{{ activePresetConfig.tabSize || 10 }} мм</span>
        </div>
        <div class="config-item">
          <span class="config-label">Алгоритм:</span>
          <span class="config-value">{{ activePresetConfig.algorithm || 'lscm' }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { usePresetsStore } from '../stores/presets.store';

interface Props {
  // Значение выбранного пресета (id)
  modelValue?: string;
  
  // Настройки отображения
  label?: string;
  showEmptyOption?: boolean;
  emptyOptionText?: string;
  standardGroupLabel?: string;
  customGroupLabel?: string;
  showDetails?: boolean;
  showActions?: boolean;
  showApplyButton?: boolean;
  showSaveButton?: boolean;
  disabled?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  label: 'Пресет настроек',
  showEmptyOption: true,
  emptyOptionText: 'Выберите пресет...',
  standardGroupLabel: 'Стандартные',
  customGroupLabel: 'Мои пресеты',
  showDetails: true,
  showActions: true,
  showApplyButton: true,
  showSaveButton: false,
  disabled: false,
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'select': [presetId: string];
  'apply': [presetId: string];
  'save': [];
}>();

const presetsStore = usePresetsStore();

const groupedPresets = computed(() => presetsStore.groupedPresets);
const activePreset = computed(() => 
  props.modelValue 
    ? presetsStore.allPresets.find(p => p.id === props.modelValue)
    : null
);
const activePresetConfig = computed(() => activePreset.value?.config);

const onSelectChange = (event: Event) => {
  const target = event.target as HTMLSelectElement;
  const value = target.value;
  emit('update:modelValue', value);
  emit('select', value);
};

const applySelected = () => {
  if (props.modelValue) {
    const config = presetsStore.applyPreset(props.modelValue);
    emit('apply', props.modelValue);
    // Конфиг может быть использован внешне
  }
};
</script>

<style scoped>
.preset-selector {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
}

.preset-selector-label {
  font-weight: 500;
  font-size: 14px;
  color: var(--color-text);
}

.preset-selector-dropdown {
  display: flex;
  gap: 8px;
  align-items: center;
}

.preset-selector-select {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background-color: var(--color-background);
  color: var(--color-text);
  font-size: 14px;
  cursor: pointer;
  transition: border-color 0.2s;
}

.preset-selector-select:focus {
  outline: none;
  border-color: var(--color-primary);
}

.preset-selector-select:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.preset-selector-actions {
  display: flex;
  gap: 4px;
}

.preset-apply-button,
.preset-save-button {
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background-color: var(--color-background);
  color: var(--color-text);
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.preset-apply-button:hover:not(:disabled),
.preset-save-button:hover:not(:disabled) {
  background-color: var(--color-background-hover);
}

.preset-apply-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.preset-details {
  margin-top: 12px;
  padding: 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background-color: var(--color-background-secondary);
}

.preset-details-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  font-weight: 500;
}

.preset-icon {
  font-size: 18px;
}

.preset-name {
  font-size: 14px;
}

.preset-config-summary {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 8px;
  font-size: 13px;
}

.config-item {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
  border-bottom: 1px dashed var(--color-border-light);
}

.config-label {
  color: var(--color-text-secondary);
}

.config-value {
  font-weight: 500;
  color: var(--color-text);
}
</style>