<template>
  <div class="editor-container">
    <!-- Простой заглушечный просмотрщик (пока без Three.js) -->
    <div class="viewer-placeholder">
      <div class="cube-3d">
        <div class="cube-face front">3D</div>
        <div class="cube-face back">VIEW</div>
        <div class="cube-face top">OBJ</div>
        <div class="cube-face bottom">FBX</div>
        <div class="cube-face left">STL</div>
        <div class="cube-face right">GLTF</div>
      </div>
      <div class="drop-hint">⬇️ Перетащите 3D-модель сюда</div>
    </div>
    
    <!-- Всплывающие панели -->
    <div class="floating-toolbar">
      <button class="tool-btn" @click="togglePanel('left')" title="Инструменты">🔧</button>
      <button class="tool-btn" @click="togglePanel('right')" title="Свойства">📊</button>
      <button class="tool-btn" @click="togglePanel('top')" title="Поиск">🔍</button>
      <button class="tool-btn" @click="openFile" title="Открыть файл">📂</button>
      <button class="tool-btn" @click="toggleAI" title="AI Ассистент">🤖</button>
    </div>
    
    <!-- Панели -->
    <transition name="slide-left">
      <div v-if="panels.left" class="panel left-panel">
        <div class="panel-header">Инструменты</div>
        <div class="panel-content">
          <button class="panel-btn">✂️ Разворачивание</button>
          <button class="panel-btn">🎨 Материалы</button>
          <button class="panel-btn">📐 Измерения</button>
          <button class="panel-btn">✏️ Редактировать швы</button>
        </div>
        <button class="panel-close" @click="closePanel('left')">✕</button>
      </div>
    </transition>
    
    <transition name="slide-right">
      <div v-if="panels.right" class="panel right-panel">
        <div class="panel-header">Свойства модели</div>
        <div class="panel-content">
          <div class="prop-row"><span>Имя:</span> <span>{{ modelName || '—' }}</span></div>
          <div class="prop-row"><span>Формат:</span> <span>{{ format || '—' }}</span></div>
          <div class="prop-row"><span>Треугольники:</span> <span>{{ triangles.toLocaleString() }}</span></div>
          <div class="prop-row"><span>Материалы:</span> <span>{{ materials }}</span></div>
        </div>
        <button class="panel-close" @click="closePanel('right')">✕</button>
      </div>
    </transition>
    
    <transition name="slide-top">
      <div v-if="panels.top" class="panel top-panel">
        <div class="panel-header">Умный поиск</div>
        <div class="panel-content">
          <input v-model="searchQuery" @keyup.enter="search" class="search-input" placeholder="Найти модель, шов, материал...">
          <div v-if="searchResults.length" class="search-results">
            <div v-for="(res, i) in searchResults" :key="i" class="search-result">{{ res }}</div>
          </div>
        </div>
        <button class="panel-close" @click="closePanel('top')">✕</button>
      </div>
    </transition>
    
    <!-- AI Ассистент -->
    <div v-if="showAI" class="ai-modal" @click.self="showAI = false">
      <div class="ai-content">
        <div class="ai-header">🤖 AI Ассистент</div>
        <div class="ai-chat">
          <div class="ai-msg ai-msg-ai">Привет! Опишите задачу для разворачивания модели.</div>
          <div v-for="(msg, i) in aiChat" :key="i" class="ai-msg" :class="msg.role === 'user' ? 'ai-msg-user' : 'ai-msg-ai'">
            {{ msg.text }}
          </div>
        </div>
        <div class="ai-input">
          <input v-model="aiQuery" @keyup.enter="sendAI" placeholder="Например: 'найди оптимальные швы для робота'">
          <button @click="sendAI">➤</button>
        </div>
      </div>
    </div>
    
    <!-- Статус-бар -->
    <div class="status-bar">
      <span class="status-ready">ГОТОВ</span>
      <span class="status-hint">Перетащите файл или нажмите 📂</span>
      <span class="status-version">Pepakura Next v0.1</span>
    </div>
  </div>
</template>

<script setup lang="ts">
// ИМПОРТЫ НА САМОМ ВЕРХУ
import { ref } from 'vue'

// ГЛОБАЛЬНЫЕ ФУНКЦИИ УВЕДОМЛЕНИЙ (без composable!)
const notifications = ref<{ id: number; message: string; type: string }[]>([])

const addNotification = (message: string, type: string = 'info') => {
  const id = Date.now()
  notifications.value.push({ id, message, type })
  
  // Автоудаление через 3 секунды
  setTimeout(() => {
    notifications.value = notifications.value.filter(n => n.id !== id)
  }, 3000)
}

const success = (message: string) => addNotification(message, 'success')
const error = (message: string) => addNotification(message, 'error')

// ОСНОВНЫЕ ПЕРЕМЕННЫЕ
const panels = ref({ left: false, right: false, top: false })
const showAI = ref(false)
const searchQuery = ref('')
const searchResults = ref<string[]>([])
const aiQuery = ref('')
const aiChat = ref<{ role: 'user' | 'ai', text: string }[]>([])
const modelName = ref<string | null>(null)
const format = ref<string | null>(null)
const triangles = ref(0)
const materials = ref(0)

// МЕТОДЫ
const togglePanel = (panel: 'left' | 'right' | 'top') => {
  panels.value[panel] = !panels.value[panel]
}

const closePanel = (panel: 'left' | 'right' | 'top') => {
  panels.value[panel] = false
}

const openFile = () => {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.obj,.mtl,.fbx,.stl,.gltf,.glb,.3ds,.dae,.ply'
  input.multiple = false
  
  input.onchange = (e: Event) => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (file) {
      const name = file.name
      const ext = name.split('.').pop()?.toLowerCase() || ''
      
      modelName.value = name
      format.value = ext.toUpperCase()
      triangles.value = Math.floor(Math.random() * 50000) + 1000
      materials.value = Math.floor(Math.random() * 5) + 1
      
      success(`✅ Загружено: ${name}`)
    }
  }
  
  input.click()
}

// DRAG & DROP
document.addEventListener('dragover', (e) => {
  e.preventDefault()
}, true)

document.addEventListener('drop', (e) => {
  e.preventDefault()
  e.stopPropagation()
  
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    const file = files[0] as File
    const name = file.name || 'безымянный'
    const ext = name.split('.').pop()?.toLowerCase() || ''
    
    modelName.value = name
    format.value = ext.toUpperCase()
    triangles.value = Math.floor(Math.random() * 50000) + 1000
    materials.value = Math.floor(Math.random() * 5) + 1
    
    success(`✅ Перетащено: ${name}`)
  }
}, true)

const search = () => {
  if (searchQuery.value.length > 2) {
    searchResults.value = [
      `Результаты для "${searchQuery.value}"`,
      '• Модель: Robot_Arm_v3.obj',
      '• Шаблон развёртки: robot_arm_template.pnx',
      '• Материал: metallic_blue.mat'
    ]
    success(`🔍 Найдено 3 результата`)
  }
}

const toggleAI = () => {
  showAI.value = !showAI.value
}

const sendAI = () => {
  if (!aiQuery.value.trim()) return
  
  const query = aiQuery.value.trim()
  aiChat.value.push({ role: 'user', text: query })
  aiQuery.value = ''
  
  setTimeout(() => {
    aiChat.value.push({ 
      role: 'ai', 
      text: `🤖 Найдено оптимальное расположение швов для "${modelName.value || 'модели'}". Рекомендую разрез по минимальной кривизне.` 
    })
  }, 800)
}
</script>

<style scoped>
/* СТИЛИ БЕЗ ИЗМЕНЕНИЙ */
.editor-container { width: 100vw; height: 100vh; background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%); overflow: hidden; position: relative; color: white; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; }
.viewer-placeholder { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; }
.cube-3d { width: 240px; height: 240px; position: relative; transform-style: preserve-3d; animation: rotate 12s infinite linear; margin-bottom: 30px; }
@keyframes rotate { 0% { transform: rotateX(0deg) rotateY(0deg); } 100% { transform: rotateX(360deg) rotateY(360deg); } }
.cube-face { position: absolute; width: 240px; height: 240px; background: linear-gradient(135deg, #6366f1, #8b5cf6); border-radius: 20px; display: flex; align-items: center; justify-content: center; font-size: 42px; font-weight: 800; color: white; box-shadow: 0 15px 40px rgba(99, 102, 241, 0.6); backface-visibility: hidden; }
.cube-face.front { transform: translateZ(120px); }
.cube-face.back { transform: rotateY(180deg) translateZ(120px); }
.cube-face.top { transform: rotateX(90deg) translateZ(120px); }
.cube-face.bottom { transform: rotateX(-90deg) translateZ(120px); }
.cube-face.left { transform: rotateY(-90deg) translateZ(120px); }
.cube-face.right { transform: rotateY(90deg) translateZ(120px); }
.drop-hint { font-size: 24px; color: rgba(255, 255, 255, 0.85); margin-top: 25px; background: rgba(0, 0, 0, 0.4); padding: 18px 40px; border-radius: 20px; display: inline-block; border: 2px solid rgba(99, 102, 241, 0.5); animation: pulse 2s infinite; }
@keyframes pulse { 0%, 100% { opacity: 0.8; } 50% { opacity: 1; } }
.floating-toolbar { position: fixed; bottom: 30px; left: 50%; transform: translateX(-50%); display: flex; gap: 12px; z-index: 1000; }
.tool-btn { width: 56px; height: 56px; border-radius: 28px; background: white; border: none; font-size: 24px; cursor: pointer; box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3); transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275); }
.tool-btn:hover { transform: scale(1.2) translateY(-5px); box-shadow: 0 10px 30px rgba(99, 102, 241, 0.7); }
.panel { position: fixed; background: rgba(30, 41, 59, 0.95); border-radius: 20px; box-shadow: 0 15px 50px rgba(0, 0, 0, 0.5); z-index: 900; backdrop-filter: blur(10px); border: 1px solid rgba(99, 102, 241, 0.4); }
.left-panel { top: 20px; left: 20px; width: 300px; height: calc(100% - 40px); padding: 25px; display: flex; flex-direction: column; }
.right-panel { top: 20px; right: 20px; width: 320px; height: calc(100% - 40px); padding: 25px; display: flex; flex-direction: column; }
.top-panel { top: 20px; left: 50%; transform: translateX(-50%); width: 600px; max-width: 90%; padding: 25px; border-radius: 24px; }
.panel-header { font-size: 22px; font-weight: 700; margin-bottom: 25px; color: #6366f1; text-align: center; }
.panel-content { flex: 1; overflow-y: auto; }
.panel-btn { display: block; width: 100%; padding: 16px; margin-bottom: 15px; background: rgba(99, 102, 241, 0.15); border: 2px solid rgba(99, 102, 241, 0.3); border-radius: 16px; color: white; font-size: 18px; font-weight: 600; cursor: pointer; transition: all 0.3s; }
.panel-btn:hover { background: rgba(99, 102, 241, 0.3); transform: translateX(5px); }
.panel-close { position: absolute; top: 15px; right: 15px; width: 40px; height: 40px; background: rgba(255, 255, 255, 0.15); border: none; border-radius: 50%; color: white; font-size: 24px; cursor: pointer; transition: all 0.3s; }
.panel-close:hover { background: rgba(255, 0, 0, 0.3); transform: rotate(90deg); }
.prop-row { display: flex; justify-content: space-between; padding: 14px 0; border-bottom: 1px solid rgba(255, 255, 255, 0.1); font-size: 17px; }
.prop-row span:first-child { color: rgba(255, 255, 255, 0.7); }
.search-input { width: 100%; padding: 16px 20px; border-radius: 16px; border: 2px solid rgba(99, 102, 241, 0.4); background: rgba(15, 23, 42, 0.7); color: white; font-size: 18px; margin-bottom: 20px; }
.status-bar { position: fixed; bottom: 20px; left: 20px; right: 20px; height: 50px; background: rgba(15, 23, 42, 0.9); border-radius: 25px; display: flex; align-items: center; justify-content: space-between; padding: 0 30px; z-index: 800; border: 1px solid rgba(99, 102, 241, 0.3); }
.status-ready { color: #10b981; font-weight: 700; font-size: 18px; }
.status-ready::before { content: '●'; margin-right: 10px; color: #10b981; font-size: 22px; }
.status-hint { color: #38bdf8; font-style: italic; font-size: 16px; }
.status-version { color: rgba(255, 255, 255, 0.6); font-size: 15px; }
.slide-left-enter-active, .slide-left-leave-active, .slide-right-enter-active, .slide-right-leave-active, .slide-top-enter-active, .slide-top-leave-active { transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275); }
.slide-left-enter-from, .slide-left-leave-to { transform: translateX(-100%); opacity: 0; }
.slide-right-enter-from, .slide-right-leave-to { transform: translateX(100%); opacity: 0; }
.slide-top-enter-from, .slide-top-leave-to { transform: translateY(-50px) scale(0.95); opacity: 0; }
.ai-modal { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.85); display: flex; align-items: center; justify-content: center; z-index: 1000; }
.ai-content { width: 700px; max-width: 95%; background: linear-gradient(145deg, #1e293b, #0f172a); border-radius: 28px; overflow: hidden; border: 1px solid rgba(99, 102, 241, 0.5); }
.ai-header { padding: 25px; background: linear-gradient(90deg, #6366f1, #8b5cf6); text-align: center; font-size: 28px; font-weight: 700; color: white; }
.ai-chat { height: 400px; overflow-y: auto; padding: 25px; background: rgba(0, 0, 0, 0.3); display: flex; flex-direction: column; gap: 20px; }
.ai-msg { max-width: 85%; padding: 18px 25px; border-radius: 20px; line-height: 1.6; font-size: 17px; }
.ai-msg-ai { background: rgba(30, 41, 59, 0.8); align-self: flex-start; border-bottom-left-radius: 8px; }
.ai-msg-user { background: rgba(99, 102, 241, 0.3); align-self: flex-end; border-bottom-right-radius: 8px; }
.ai-input { display: flex; padding: 20px; border-top: 1px solid rgba(255, 255, 255, 0.1); }
.ai-input input { flex: 1; padding: 18px 25px; border-radius: 20px; border: 2px solid rgba(99, 102, 241, 0.4); background: rgba(30, 41, 59, 0.7); color: white; font-size: 18px; }
.ai-input button { width: 70px; height: 60px; margin-left: 15px; border-radius: 20px; background: linear-gradient(135deg, #6366f1, #8b5cf6); border: none; color: white; font-size: 26px; cursor: pointer; transition: all 0.3s; }
.ai-input button:hover { transform: scale(1.1); box-shadow: 0 10px 30px rgba(99, 102, 241, 0.6); }
@media (max-width: 900px) { .left-panel, .right-panel { width: 280px; } .top-panel { width: 90%; } .floating-toolbar { bottom: 15px; } .cube-3d { width: 180px; height: 180px; } .cube-face { width: 180px; height: 180px; font-size: 32px; } }
@media (max-width: 600px) { .floating-toolbar { flex-direction: column; bottom: 80px; left: 20px; transform: none; } .tool-btn { width: 60px; height: 60px; font-size: 28px; } .status-bar { flex-direction: column; height: auto; padding: 15px; gap: 10px; } }
</style>
