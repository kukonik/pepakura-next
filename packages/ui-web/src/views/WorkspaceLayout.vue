<template>
  <div class="app-layout">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="logo">📐</div>
      
      <nav class="nav-menu">
        <button 
          class="nav-btn" 
          :class="{ active: currentView === '3d' }" 
          @click="currentView = '3d'"
          title="3D Viewer"
        >
          🧊
        </button>
        <button 
          class="nav-btn" 
          :class="{ active: currentView === '2d' }" 
          @click="currentView = '2d'"
          title="2D Unfold"
        >
          📄
        </button>
      </nav>

      <div class="tools-separator"></div>

      <div class="tools-menu">
        <button class="tool-btn" @click="handleSimplify" title="Упростить">
          ⚡
        </button>
        <button class="tool-btn" @click="openSettings" title="Настройки">
          ⚙
        </button>
        <button class="tool-btn" @click="handleExport" title="Экспорт">
          💾
        </button>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="main-content">
      <!-- Tabs Header -->
      <div class="tabs-header">
        <div 
          class="tab-item" 
          :class="{ active: currentView === '3d' }"
          @click="currentView = '3d'"
        >
          3D View
        </div>
        <div 
          class="tab-item" 
          :class="{ active: currentView === '2d' }"
          @click="currentView = '2d'"
        >
          2D Unfold
        </div>
      </div>

      <!-- Panels -->
      <div class="panels-container">
        <!-- 3D Panel -->
        <div v-show="currentView === '3d'" class="panel-3d">
          <ThreeDViewer ref="viewerRef">
            <template #top></template>
            
            <template #bottom>
              <button class="fab-btn" @click="triggerFileSelect" title="Загрузить файлы">
                📂 Open
              </button>
              <input 
                ref="fileInputRef" 
                type="file" 
                multiple 
                accept=".obj,.mtl,.png,.jpg,.jpeg" 
                style="display: none" 
                @change="handleFileChange"
              />
            </template>
          </ThreeDViewer>
        </div>

        <!-- 2D Panel -->
        <div v-show="currentView === '2d'" class="panel-2d">
          <h2>Развертка</h2>
          <p>Загрузите модель через 3D вкладку</p>
          <div class="placeholder-2d">2D Canvas Area</div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
// Используем относительный путь (../../../), чтобы обойти проблемы с алиасами Vite
// Путь: ui-web/src/views -> ../../../ -> packages/shared/src/...
import ThreeDViewer from '../../../shared/src/components/viewer/ThreeDViewer.vue';

const currentView = ref<'3d' | '2d'>('3d');
const viewerRef = ref<InstanceType<typeof ThreeDViewer> | null>(null);
const fileInputRef = ref<HTMLInputElement | null>(null);

const triggerFileSelect = () => fileInputRef.value?.click();

const handleFileChange = (e: Event) => {
  const target = e.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    viewerRef.value?.loadObjFromFiles(target.files);
  }
  target.value = '';
};

const handleSimplify = () => {
  if (viewerRef.value) {
    viewerRef.value.simplifyModel(0.5);
  }
};

const handleExport = () => alert('Экспорт в разработке');
const openSettings = () => alert('Настройки в разработке');
</script>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
  background-color: #1e1e1e;
  color: #e0e0e0;
  font-family: 'Segoe UI', sans-serif;
  overflow: hidden;
}

.sidebar { 
  width: 40px; 
  background: #252526; 
  border-right: 1px solid #333; 
  display: flex; 
  flex-direction: column; 
  align-items: center; 
  padding-top: 8px; 
  flex-shrink: 0; 
}

.logo { 
  font-size: 18px; 
  margin-bottom: 12px; 
}

.nav-menu, .tools-menu { 
  display: flex; 
  flex-direction: column; 
  gap: 6px; 
  width: 100%; 
  align-items: center; 
}

.tools-separator { 
  height: 1px; 
  width: 25px; 
  background: #444; 
  margin: 8px 0; 
}

.nav-btn, .tool-btn { 
  background: none; 
  border: none; 
  color: #aaa; 
  font-size: 16px; 
  cursor: pointer; 
  padding: 6px; 
  border-radius: 4px; 
  transition: 0.2s; 
  width: 32px;
  height: 32px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.nav-btn:hover, .tool-btn:hover { 
  background: #3e3e42; 
  color: #fff; 
}

.nav-btn.active { 
  color: #007acc; 
  border-left: 2px solid #007acc; 
}

.main-content { 
  flex-grow: 1; 
  display: flex; 
  flex-direction: column; 
  overflow: hidden; 
}

.tabs-header { 
  height: 32px; 
  background: #2d2d2d; 
  display: flex; 
  align-items: center; 
  padding-left: 10px; 
  border-bottom: 1px solid #333; 
}

.tab-item { 
  padding: 0 12px; 
  height: 100%; 
  display: flex; 
  align-items: center; 
  cursor: pointer; 
  color: #888; 
  font-size: 11px; 
  text-transform: uppercase; 
  letter-spacing: 0.5px;
  border-bottom: 2px solid transparent; 
}

.tab-item:hover { 
  color: #ddd; 
}

.tab-item.active { 
  color: #fff; 
  border-bottom: 2px solid #007acc; 
  background: #1e1e1e; 
}

.panels-container { 
  flex-grow: 1; 
  position: relative; 
  overflow: hidden; 
}

.panel-3d, .panel-2d { 
  width: 100%; 
  height: 100%; 
}

.panel-2d { 
  padding: 20px; 
  box-sizing: border-box; 
}

.placeholder-2d { 
  width: 100%; 
  height: 300px; 
  background: #2a2a2a; 
  border: 2px dashed #444; 
  display: flex; 
  justify-content: center; 
  align-items: center; 
  color: #666; 
  margin-top: 20px; 
}

.fab-btn {
  background: #007acc;
  color: white;
  border: none;
  padding: 8px 16px; 
  border-radius: 16px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  transition: all 0.2s;
  opacity: 0.9;
}
.fab-btn:hover { 
  opacity: 1; 
  transform: translateY(-1px); 
}
.fab-btn:active { 
  transform: translateY(1px); 
}
</style>
