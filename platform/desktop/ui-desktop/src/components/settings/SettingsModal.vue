<template>
  <div class="settings-modal">
    <div class="modal-backdrop" @click="close"></div>
    <div class="modal-content">
      <div class="modal-header">
        <h2>{{ $t('settings.title') }}</h2>
        <button class="close-btn" @click="close" aria-label="Close">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
      <div class="modal-body">
        <div class="tabs">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            :class="['tab-btn', { active: activeTab === tab.id }]"
            @click="activeTab = tab.id"
          >
            {{ $t(tab.label) }}
          </button>
        </div>
        <div class="tab-content">
          <!-- General Tab -->
          <div v-if="activeTab === 'general'" class="tab-pane">
            <div class="form-group">
              <label for="language">{{ $t('settings.language') }}</label>
              <select id="language" v-model="settings.language" @change="onLanguageChange">
                <option value="ru">Русский</option>
                <option value="en">English</option>
              </select>
            </div>
            <div class="form-group">
              <label for="theme">{{ $t('settings.theme') }}</label>
              <select id="theme" v-model="settings.theme" @change="onThemeChange">
                <option value="light">{{ $t('settings.themeLight') }}</option>
                <option value="dark">{{ $t('settings.themeDark') }}</option>
                <option value="system">{{ $t('settings.themeSystem') }}</option>
              </select>
            </div>
            <div class="form-group">
              <label for="defaultExportPath">{{ $t('settings.defaultExportPath') }}</label>
              <div class="input-with-button">
                <input
                  id="defaultExportPath"
                  type="text"
                  v-model="settings.defaultExportPath"
                  :placeholder="$t('settings.defaultExportPathPlaceholder')"
                />
                <button class="secondary-btn" @click="browseExportPath">
                  {{ $t('settings.browse') }}
                </button>
              </div>
            </div>
          </div>

          <!-- Unfold Tab -->
          <div v-if="activeTab === 'unfold'" class="tab-pane">
            <div class="form-group">
              <label for="preserveDetail">
                <input
                  id="preserveDetail"
                  type="checkbox"
                  v-model="settings.unfoldConfig.preserveDetail"
                />
                {{ $t('settings.preserveDetail') }}
              </label>
            </div>
            <div class="form-group">
              <label for="maxIterations">{{ $t('settings.maxIterations') }}</label>
              <input
                id="maxIterations"
                type="number"
                v-model.number="settings.unfoldConfig.maxIterations"
                min="1"
                max="1000"
              />
            </div>
            <div class="form-group">
              <label for="tolerance">{{ $t('settings.tolerance') }}</label>
              <input
                id="tolerance"
                type="number"
                v-model.number="settings.unfoldConfig.tolerance"
                step="0.000001"
                min="0.000001"
                max="0.1"
              />
              <span class="hint">{{ $t('settings.toleranceHint') }}</span>
            </div>
          </div>

          <!-- AI Tab -->
          <div v-if="activeTab === 'ai'" class="tab-pane">
            <div class="form-group">
              <label for="aiProvider">{{ $t('settings.aiProvider') }}</label>
              <select id="aiProvider" v-model="settings.aiConfig.provider">
                <option value="ollama">Ollama</option>
                <option value="openai">OpenAI</option>
                <option value="huggingface">Hugging Face</option>
              </select>
            </div>
            <div class="form-group">
              <label for="aiModel">{{ $t('settings.aiModel') }}</label>
              <input
                id="aiModel"
                type="text"
                v-model="settings.aiConfig.model"
                :placeholder="$t('settings.aiModelPlaceholder')"
              />
            </div>
            <div class="form-group">
              <label for="apiKey">{{ $t('settings.apiKey') }}</label>
              <input
                id="apiKey"
                type="password"
                v-model="settings.aiConfig.apiKey"
                :placeholder="$t('settings.apiKeyPlaceholder')"
              />
              <span class="hint">{{ $t('settings.apiKeyHint') }}</span>
            </div>
          </div>

          <!-- About Tab -->
          <div v-if="activeTab === 'about'" class="tab-pane">
            <div class="about-section">
              <h3>Pepakura Next</h3>
              <p>{{ $t('settings.version') }}: {{ appVersion }}</p>
              <p>{{ $t('settings.license') }}: MIT</p>
              <p>{{ $t('settings.repository') }}: <a href="https://github.com/kuzkonik/pepakura-next" target="_blank">GitHub</a></p>
              <p>{{ $t('settings.credits') }}: <a href="https://kuzkonik.com" target="_blank">kuzkonik</a></p>
            </div>
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="secondary-btn" @click="resetSettings">
          {{ $t('settings.resetToDefaults') }}
        </button>
        <div class="spacer"></div>
        <button class="secondary-btn" @click="close">
          {{ $t('common.cancel') }}
        </button>
        <button class="primary-btn" @click="saveSettings">
          {{ $t('common.save') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSettingsStore } from '@/stores/settings.store'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'

const emit = defineEmits<{
  (e: 'close'): void
}>()

const settingsStore = useSettingsStore()
const { locale } = useI18n()

const activeTab = ref('general')
const settings = ref<AppSettings>({
  language: 'ru',
  theme: 'system',
  defaultExportPath: '',
  unfoldConfig: {
    preserveDetail: true,
    maxIterations: 100,
    tolerance: 1e-6,
  },
  aiConfig: {
    provider: 'ollama',
    model: 'llama3.2',
    apiKey: null,
  },
})
const appVersion = ref('0.1.0')

const tabs = [
  { id: 'general', label: 'settings.tabGeneral' },
  { id: 'unfold', label: 'settings.tabUnfold' },
  { id: 'ai', label: 'settings.tabAI' },
  { id: 'about', label: 'settings.tabAbout' },
]

onMounted(async () => {
  await loadSettings()
  await loadAppVersion()
})

async function loadSettings() {
  try {
    const loaded = await invoke<AppSettings>('get_settings')
    settings.value = loaded
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
}

async function loadAppVersion() {
  try {
    appVersion.value = await invoke<string>('get_app_version')
  } catch (error) {
    console.error('Failed to load app version:', error)
  }
}

function onLanguageChange() {
  locale.value = settings.value.language
}

function onThemeChange() {
  // TODO: apply theme change
  console.log('Theme changed to:', settings.value.theme)
}

async function browseExportPath() {
  try {
    const selected = await invoke<string>('open_dialog', {
      title: 'Select default export folder',
      directory: true,
    })
    if (selected) {
      settings.value.defaultExportPath = selected
    }
  } catch (error) {
    console.error('Failed to browse path:', error)
  }
}

async function saveSettings() {
  try {
    await invoke('save_settings', { settings: settings.value })
    settingsStore.settings = settings.value
    emit('close')
  } catch (error) {
    console.error('Failed to save settings:', error)
    alert('Failed to save settings: ' + error)
  }
}

async function resetSettings() {
  if (confirm('Are you sure you want to reset all settings to defaults?')) {
    try {
      const defaultSettings = await invoke<AppSettings>('reset_settings')
      settings.value = defaultSettings
      settingsStore.settings = defaultSettings
      locale.value = defaultSettings.language
    } catch (error) {
      console.error('Failed to reset settings:', error)
    }
  }
}

function close() {
  emit('close')
}
</script>

<style scoped>
.settings-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-backdrop {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
}

.modal-content {
  position: relative;
  background-color: var(--color-background);
  border-radius: 12px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  width: 90%;
  max-width: 700px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.5rem;
  border-bottom: 1px solid var(--color-border);
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text);
  padding: 0.5rem;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  background-color: var(--color-hover);
}

.modal-body {
  flex: 1;
  overflow: auto;
  padding: 1.5rem;
}

.tabs {
  display: flex;
  border-bottom: 1px solid var(--color-border);
  margin-bottom: 1.5rem;
}

.tab-btn {
  padding: 0.75rem 1.5rem;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.tab-btn:hover {
  color: var(--color-text);
}

.tab-btn.active {
  color: var(--color-primary);
  border-bottom-color: var(--color-primary);
}

.tab-pane {
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.form-group input[type="text"],
.form-group input[type="number"],
.form-group input[type="password"],
.form-group select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background-color: var(--color-input-background);
  color: var(--color-text);
  font-size: 1rem;
}

.form-group input[type="checkbox"] {
  margin-right: 0.5rem;
}

.input-with-button {
  display: flex;
  gap: 0.5rem;
}

.input-with-button input {
  flex: 1;
}

.hint {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.about-section {
  line-height: 1.6;
}

.about-section a {
  color: var(--color-primary);
  text-decoration: none;
}

.about-section a:hover {
  text-decoration: underline;
}

.modal-footer {
  display: flex;
  align-items: center;
  padding: 1.5rem;
  border-top: 1px solid var(--color-border);
}

.spacer {
  flex: 1;
}

.primary-btn,
.secondary-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: background-color 0.2s;
}

.primary-btn {
  background-color: var(--color-primary);
  color: white;
}

.primary-btn:hover {
  background-color: var(--color-primary-dark);
}

.secondary-btn {
  background-color: var(--color-secondary);
  color: var(--color-text);
  margin-right: 0.5rem;
}

.secondary-btn:hover {
  background-color: var(--color-hover);
}
</style>