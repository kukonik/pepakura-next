#requires -Version 7.0
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Write-Log {
  param(
    [Parameter(Mandatory)][string]$Message,
    [ValidateSet('INFO','WARN','ERROR')][string]$Level = 'INFO'
  )
  $ts = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
  Write-Host "[$ts][$Level] $Message"
}

try {
  $root = 'D:\Dev\pepakura-next'
  $desktopDir = Join-Path $root 'packages\ui-desktop'
  if (-not (Test-Path $desktopDir)) {
    Write-Log "Desktop package not found: $desktopDir" 'ERROR'
    throw "Desktop package not found: $desktopDir"
  }

  Set-Location $desktopDir
  Write-Log "Current directory: $desktopDir" 'INFO'

  # SettingsView.vue: безопасный доступ к activeConfig + русские подписи
  $settingsPath = Join-Path $desktopDir 'src\views\SettingsView.vue'
  if (-not (Test-Path $settingsPath)) {
    Write-Log "SettingsView.vue not found at $settingsPath" 'ERROR'
    throw "SettingsView.vue not found"
  }

  $settingsContent = @'
<script setup lang="ts">
import { computed } from "vue";
import { useAiStore } from "../stores/aiStore";

const aiStore = useAiStore();

const activeConfig = computed(() => aiStore.activeConfig ?? {
  type: aiStore.currentBackend,
  endpoint: "",
  model: "",
  temperature: 0.3,
  maxTokens: 1024,
});

function handleBackendChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value as "ollama" | "openai" | "custom";
  aiStore.setBackend(value);
}

function handleEndpointChange(event: Event) {
  aiStore.updateBackendConfig(aiStore.currentBackend, { endpoint: (event.target as HTMLInputElement).value });
}

function handleModelChange(event: Event) {
  aiStore.updateBackendConfig(aiStore.currentBackend, { model: (event.target as HTMLInputElement).value });
}

function handleTemperatureChange(event: Event) {
  const v = Number((event.target as HTMLInputElement).value);
  aiStore.updateBackendConfig(aiStore.currentBackend, { temperature: isNaN(v) ? 0.3 : v });
}

function handleMaxTokensChange(event: Event) {
  const v = Number((event.target as HTMLInputElement).value);
  aiStore.updateBackendConfig(aiStore.currentBackend, { maxTokens: isNaN(v) ? 1024 : v });
}
</script>

<template>
  <div class="settings-view">
    <h2>Настройки AI</h2>

    <div class="section">
      <label class="label">Провайдер</label>
      <select :value="aiStore.currentBackend" @change="handleBackendChange">
        <option value="ollama">Ollama (локально)</option>
        <option value="openai">OpenAI</option>
        <option value="custom">Произвольный</option>
      </select>
    </div>

    <div class="section">
      <label class="label">Endpoint</label>
      <input
        type="text"
        :value="activeConfig.endpoint"
        @input="handleEndpointChange"
      />
    </div>

    <div class="section">
      <label class="label">Модель</label>
      <input
        type="text"
        :value="activeConfig.model"
        @input="handleModelChange"
      />
    </div>

    <div class="section">
      <label class="label">Температура</label>
      <input
        type="number"
        step="0.1"
        min="0"
        max="2"
        :value="activeConfig.temperature"
        @input="handleTemperatureChange"
      />
    </div>

    <div class="section">
      <label class="label">Максимум токенов</label>
      <input
        type="number"
        min="1"
        max="32768"
        :value="activeConfig.maxTokens"
        @input="handleMaxTokensChange"
      />
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 16px;
  color: #f5f5f5;
}

.section {
  margin-bottom: 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.label {
  font-size: 14px;
  font-weight: 600;
}

input,
select {
  padding: 4px 8px;
  background-color: #222;
  border: 1px solid #444;
  color: #f5f5f5;
  border-radius: 4px;
}
</style>
'@

  Set-Content -Path $settingsPath -Value $settingsContent -Encoding UTF8
  Write-Log "SettingsView.vue written" 'INFO'

  Write-Log "fix-ui-desktop-settings completed successfully" 'INFO'
}
catch {
  Write-Log "ERROR: $($_.Exception.Message)" 'ERROR'
  throw
}
