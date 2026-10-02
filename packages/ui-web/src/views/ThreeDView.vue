<template>
  <div class="app-wrapper">
    
    <!-- Боковая панель -->
    <transition name="slide">
      <div v-if="isSidebarOpen" class="sidebar left-panel glass-panel">
        <div class="sidebar-header">
          <h3>{{ store.currentProject?.name || 'Untitled' }}</h3>
          <button @click="isSidebarOpen = false" class="icon-btn neon-close">×</button>
        </div>
        <div class="sidebar-content">
          <div class="mode-grid">
            <button v-for="mode in modes" :key="mode.key" @click="store.setMode(mode.key)" :class="['mode-btn', { active: store.appMode === mode.key }]">
              {{ mode.label }}
            </button>
          </div>
        </div>
      </div>
    </transition>

    <div v-if="!isSidebarOpen" class="floating-trigger" @click="isSidebarOpen = true"><span>☰</span></div>

    <!-- 3D Вьювер -->
    <ThreeDViewer ref="viewerRef" class="full-screen-viewer">
      <template #top>
        <div class="top-toolbar">
           <button @click="showSettings = true" class="neon-btn" title="Настройки"><i class="icon">⚙</i></button>
           <button @click="toggleFullscreen" class="neon-btn" title="Полный экран"><i class="icon">⤢</i></button>
        </div>
      </template>
      <template #bottom>
        <div class="bottom-toolbar glass-panel">
          <div class="toolbar-group">
            <label class="neon-btn file-btn">
              <i class="icon">📂 Load OBJ+MTL+PNG</i>
              <!-- MULTIPLE! -->
              <input type="file" multiple style="display: none" @change="handleFilesUpload" />
            </label>
          </div>
          <div class="divider"></div>
          <div class="toolbar-group">
            <button @click="resetFile" class="neon-btn danger"><i class="icon">🗑 Сброс</i></button>
            <button @click="toggleUrlPanel" class="neon-btn"><i class="icon">🔗 URL</i></button>
            <button @click="resetCamera" class="neon-btn"><i class="icon">🎥 Камера</i></button>
          </div>
        </div>
      </template>
    </ThreeDViewer>
    
    <!-- Настройки (Свет + Масштаб) -->
    <div v-if="showSettings" class="settings-modal glass-panel">
      <h3>Настройки сцены</h3>
      
      <div class="control-group">
        <label>Освещение (Ambient)</label>
        <input type="range" min="0" max="2" step="0.1" v-model.number="lightAmbient" @input="updateSettings">
      </div>
      
      <div class="control-group">
        <label>Направленный свет (Dir)</label>
        <input type="range" min="0" max="2" step="0.1" v-model.number="lightDir" @input="updateSettings">
      </div>
      
      <div class="control-group">
        <label>Масштаб модели</label>
        <input type="range" min="0.1" max="3" step="0.1" v-model.number="modelScale" @input="updateSettings">
      </div>

      <div class="modal-actions">
        <button @click="showSettings = false" class="neon-btn primary">Закрыть</button>
      </div>
    </div>

    <!-- URL Panel -->
    <div v-if="showUrlInput" class="url-modal glass-panel">
      <h3>Загрузка по URL</h3>
      <input v-model="objUrl" placeholder="OBJ URL (http://...)" type="text" class="neon-input" />
      <input v-model="mtlUrl" placeholder="MTL URL (http://...)" type="text" class="neon-input" />
      <div class="modal-actions">
        <button @click="loadFromUrl" class="neon-btn primary">Загрузить</button>
        <button @click="showUrlInput = false" class="neon-btn">Отмена</button>
      </div>
    </div>

    <div v-if="store.isLoading" class="loader-overlay">
      <div class="spinner"></div>
      <p>Обработка...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import ThreeDViewer from '@shared/components/viewer/ThreeDViewer.vue';
import { useProjectStore } from '@shared/stores/useProjectStore';
import { AppMode } from '@shared/types/Project';

const store = useProjectStore();
const isSidebarOpen = ref(false);
const showUrlInput = ref(false);
const showSettings = ref(false); // Новое состояние
const objUrl = ref('');
const mtlUrl = ref('');

// Настройки
const lightAmbient = ref(0.6);
const lightDir = ref(0.8);
const modelScale = ref(1.0);

const viewerRef = ref<InstanceType<typeof ThreeDViewer>>();
const modes = [
  { key: AppMode.VIEWER_3D, label: '3D Вид' },
  { key: AppMode.EDITOR_2D, label: '2D Развёртка' },
  { key: AppMode.TXT_MODE, label: 'Текст' },
  { key: AppMode.PRINT_MODE, label: 'Печать' }
];

// --- Logic ---

// Обработка множества файлов
function handleFilesUpload(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    store.isLoading = true;
    // Передаем все файлы (OBJ, MTL, PNG)
    viewerRef.value?.loadObjFromFiles(target.files)
      .finally(() => {
        store.isLoading = false;
      });
  }
}

function updateSettings() {
  viewerRef.value?.updateLights(lightAmbient.value, lightDir.value);
  viewerRef.value?.updateModelScale(modelScale.value);
}

function resetFile() {
  viewerRef.value?.resetFiles();
  // Сброс инпутов
  const inp = document.querySelector('input[type="file"]') as HTMLInputElement;
  if (inp) inp.value = '';
  store.updateProjectModel(null, null);
}

function resetCamera() { viewerRef.value?.resetCamera(); }

function loadFromUrl() {
  if (objUrl.value && viewerRef.value) {
    store.isLoading = true;
    store.updateProjectModel(objUrl.value, mtlUrl.value || null);
    viewerRef.value.loadObjFromUrl(objUrl.value, mtlUrl.value || null)
      .finally(() => {
        store.isLoading = false;
        showUrlInput.value = false;
      });
  }
}

function toggleUrlPanel() { showUrlInput.value = !showUrlInput.value; }
function toggleFullscreen() {
  if (!document.fullscreenElement) document.documentElement.requestFullscreen();
  else if (document.exitFullscreen) document.exitFullscreen();
}
</script>

<style scoped>
/* ... (Стили оставляем прежними, добавляем только настройки) ... */
.app-wrapper { width: 100vw; height: 100vh; position: relative; overflow: hidden; background: #000; }
.full-screen-viewer { position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: 1; }

.glass-panel {
  background: rgba(20, 20, 30, 0.65); backdrop-filter: blur(16px);
  border: 1px solid rgba(108, 92, 231, 0.3); box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);
  color: #fff; border-radius: 12px;
}

.neon-btn {
  background: rgba(0, 0, 0, 0.4); border: 1px solid rgba(108, 92, 231, 0.5);
  color: #e0e0e0; padding: 8px 16px; border-radius: 6px; cursor: pointer;
  transition: all 0.3s; font-size: 13px; font-weight: 600; display: flex; align-items: center; gap: 8px;
}
.neon-btn:hover {
  border-color: #6c5ce7; box-shadow: 0 0 15px rgba(108, 92, 231, 0.6); color: #fff; transform: translateY(-1px);
}
.neon-btn.danger { border-color: rgba(255, 118, 117, 0.5); }
.neon-btn.danger:hover { border-color: #ff7675; box-shadow: 0 0 15px rgba(255, 118, 117, 0.6); }
.neon-btn.primary { background: rgba(108, 92, 231, 0.2); border-color: #6c5ce7; }
.neon-btn.primary:hover { background: rgba(108, 92, 231, 0.6); box-shadow: 0 0 20px rgba(108, 92, 231, 0.8); }

.top-toolbar { position: absolute; top: 20px; right: 20px; display: flex; gap: 10px; z-index: 10; }
.bottom-toolbar { position: absolute; bottom: 30px; left: 50%; transform: translateX(-50%); display: flex; align-items: center; gap: 20px; padding: 10px 20px; z-index: 10; pointer-events: auto; }
.toolbar-group { display: flex; gap: 10px; }
.divider { width: 1px; height: 24px; background: rgba(255,255,255,0.1); }

.left-panel { position: absolute; top: 0; left: 0; width: 280px; height: 100%; z-index: 20; padding: 20px; display: flex; flex-direction: column; }
.sidebar-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; padding-bottom: 15px; border-bottom: 1px solid rgba(255,255,255,0.1); }
.sidebar-header h3 { margin: 0; font-weight: 300; }
.neon-close { background: none; border: none; color: #ff7675; font-size: 1.5rem; cursor: pointer; }
.mode-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }
.mode-btn { background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); color: #aaa; padding: 12px; border-radius: 8px; cursor: pointer; text-align: center; transition: 0.2s; }
.mode-btn:hover { background: rgba(255,255,255,0.08); color: #fff; }
.mode-btn.active { background: rgba(108, 92, 231, 0.4); border-color: #6c5ce7; color: #fff; box-shadow: 0 0 10px rgba(108, 92, 231, 0.3); }

.floating-trigger { position: absolute; top: 20px; left: 20px; z-index: 15; width: 40px; height: 40px; background: rgba(20, 20, 30, 0.8); border: 1px solid rgba(108, 92, 231, 0.5); border-radius: 50%; color: white; font-size: 1.2rem; display: flex; justify-content: center; align-items: center; cursor: pointer; backdrop-filter: blur(4px); transition: 0.3s; }
.floating-trigger:hover { box-shadow: 0 0 15px rgba(108, 92, 231, 0.6); transform: scale(1.1); }

.settings-modal, .url-modal {
  position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%);
  width: 350px; padding: 25px; z-index: 100;
  box-shadow: 0 0 50px rgba(0,0,0,0.8);
}
.settings-modal h3, .url-modal h3 { margin-top: 0; margin-bottom: 20px; color: #fff; }

.control-group { margin-bottom: 20px; }
.control-group label { display: block; margin-bottom: 8px; color: #aaa; font-size: 0.9rem; }
.control-group input[type="range"] { width: 100%; cursor: pointer; }

.neon-input { width: 100%; background: rgba(0,0,0,0.5); border: 1px solid #444; color: white; padding: 10px; border-radius: 4px; margin-bottom: 8px; }
.neon-input:focus { outline: none; border-color: #6c5ce7; box-shadow: 0 0 10px rgba(108, 92, 231, 0.5); }

.modal-actions { display: flex; justify-content: flex-end; gap: 10px; margin-top: 20px; }

.loader-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); z-index: 50; display: flex; flex-direction: column; justify-content: center; align-items: center; color: #6c5ce7; font-weight: bold; letter-spacing: 2px; }
.spinner { width: 50px; height: 50px; border: 4px solid rgba(108, 92, 231, 0.3); border-top: 4px solid #6c5ce7; border-radius: 50%; animation: spin 1s linear infinite; margin-bottom: 20px; }
@keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }

.slide-enter-active, .slide-leave-active { transition: transform 0.4s cubic-bezier(0.16, 1, 0.3, 1); }
.slide-enter-from, .slide-leave-to { transform: translateX(-100%); }
.icon { font-style: normal; }
</style>
