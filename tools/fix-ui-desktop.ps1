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
  $logDir = Join-Path $root 'logs'
  if (-not (Test-Path $logDir)) { New-Item -ItemType Directory -Path $logDir | Out-Null }
  $logFile = Join-Path $logDir 'fix-ui-desktop.log'

  Start-Transcript -Path $logFile -Append | Out-Null
  Write-Log "Starting ui-desktop fix script" 'INFO'

  if (-not (Test-Path $root)) {
    Write-Log "Root path not found: $root" 'ERROR'
    throw "Root path not found: $root"
  }

  if (-not (Test-Path $desktopDir)) {
    Write-Log "Desktop package not found: $desktopDir" 'ERROR'
    throw "Desktop package not found: $desktopDir"
  }

  Set-Location $desktopDir
  Write-Log "Current directory: $desktopDir" 'INFO'

  # aiStore.ts
  $aiStorePath = Join-Path $desktopDir 'src\stores\aiStore.ts'
  $aiStoreContent = @'
import { defineStore } from "pinia";

export type AiBackendType = "ollama" | "openai" | "custom";

export interface AiBackendConfig {
  type: AiBackendType;
  endpoint: string;
  apiKey?: string;
  model: string;
  temperature?: number;
  maxTokens?: number;
}

export interface AiState {
  currentBackend: AiBackendType;
  backends: Record<AiBackendType, AiBackendConfig>;
  isBusy: boolean;
  lastError: string | null;
}

export const useAiStore = defineStore("ai", {
  state: (): AiState => ({
    currentBackend: "ollama",
    backends: {
      ollama: {
        type: "ollama",
        endpoint: "http://localhost:11434",
        model: "llama3",
        temperature: 0.2,
        maxTokens: 1024,
      },
      openai: {
        type: "openai",
        endpoint: "https://api.openai.com/v1",
        model: "gpt-4.1",
        temperature: 0.3,
        maxTokens: 2048,
      },
      custom: {
        type: "custom",
        endpoint: "",
        model: "",
        temperature: 0.3,
        maxTokens: 1024,
      },
    },
    isBusy: false,
    lastError: null,
  }),

  getters: {
    activeConfig(state): AiBackendConfig {
      return state.backends[state.currentBackend];
    },
  },

  actions: {
    setBackend(type: AiBackendType) {
      this.currentBackend = type;
    },

    updateBackendConfig(type: AiBackendType, patch: Partial<AiBackendConfig>) {
      this.backends[type] = { ...this.backends[type], ...patch };
    },

    setBusy(value: boolean) {
      this.isBusy = value;
    },

    setError(message: string | null) {
      this.lastError = message;
    },
  },
});
'@
  Set-Content -Path $aiStorePath -Value $aiStoreContent -Encoding UTF8
  Write-Log "aiStore.ts written" 'INFO'

  # UnfoldEditorView.vue
  $unfoldViewPath = Join-Path $desktopDir 'src\views\UnfoldEditorView.vue'
  $unfoldViewContent = @'
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useAiStore } from "../stores/aiStore";
import type { Mesh } from "three";

const seamsStore = useAiStore();
const canvasRef = ref<HTMLCanvasElement | null>(null);
const isReady = ref(false);
const lastLoadedFile = ref<string | null>(null);

let currentMesh: Mesh | null = null;

function handleFileChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  lastLoadedFile.value = file.name;
}

function resetScene() {
  currentMesh = null;
  seamsStore.setError(null);
  seamsStore.setBusy(false);
  seamsStore.currentBackend = "ollama";
}

onMounted(() => {
  isReady.value = true;
});

onUnmounted(() => {
  resetScene();
});
</script>

<template>
  <div class="unfold-editor-view">
    <div class="sidebar">
      <h2>Редактор развёртки (заглушка)</h2>

      <div class="section">
        <label class="label">3D модель</label>
        <input type="file" accept=".obj,.stl,.fbx,.gltf,.glb" @change="handleFileChange" />
        <p v-if="lastLoadedFile" class="hint">
          Загружен файл: <strong>{{ lastLoadedFile }}</strong>
        </p>
        <p v-else class="hint">Файл ещё не загружен.</p>
      </div>

      <div class="section">
        <label class="label">AI backend</label>
        <select v-model="seamsStore.currentBackend">
          <option value="ollama">Ollama (локально)</option>
          <option value="openai">OpenAI</option>
          <option value="custom">Custom</option>
        </select>
      </div>

      <div class="section">
        <button type="button" @click="resetScene">Сбросить сцену</button>
      </div>

      <div class="section status">
        <p>Статус: <strong>{{ seamsStore.isBusy ? "Обработка..." : "Готов" }}</strong></p>
        <p v-if="seamsStore.lastError" class="error">Ошибка: {{ seamsStore.lastError }}</p>
      </div>
    </div>

    <div class="viewport-container">
      <div v-if="!isReady" class="placeholder">
        Инициализация 3D сцены...
      </div>
      <canvas
        v-else
        ref="canvasRef"
        class="viewport"
      ></canvas>
    </div>
  </div>
</template>

<style scoped>
.unfold-editor-view {
  display: flex;
  width: 100%;
  height: 100vh;
  box-sizing: border-box;
}

.sidebar {
  width: 280px;
  padding: 16px;
  box-sizing: border-box;
  border-right: 1px solid #444;
  background-color: #1f1f1f;
  color: #f5f5f5;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.label {
  font-size: 14px;
  font-weight: 600;
}

.hint {
  font-size: 12px;
  color: #aaa;
}

.status {
  margin-top: auto;
  font-size: 13px;
}

.error {
  color: #ff6b6b;
}

.viewport-container {
  flex: 1;
  position: relative;
  background-color: #101010;
}

.viewport {
  width: 100%;
  height: 100%;
  display: block;
}

.placeholder {
  position: absolute;
  inset: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  color: #888;
  font-size: 14px;
}
</style>
'@
  Set-Content -Path $unfoldViewPath -Value $unfoldViewContent -Encoding UTF8
  Write-Log "UnfoldEditorView.vue written" 'INFO'

  Write-Log "Fix script completed successfully" 'INFO'
}
catch {
  Write-Log "ERROR: $($_.Exception.Message)" 'ERROR'
  throw
}
