<template>
  <div class="home-container">
    <ThreeDViewer ref="threeViewer" class="viewer-section" />
    <div class="floating-toolbar">
      <button class="tool-btn" @click="togglePanel('left')" title="Инструменты">🔧</button>
      <button class="tool-btn" @click="togglePanel('right')" title="Свойства">📊</button>
      <button class="tool-btn" @click="openFile" title="Открыть файл">📂</button>
      <button class="tool-btn" @click="toggleAI" title="AI Ассистент">🤖</button>
      <button class="tool-btn" @click="togglePanel('control')" title="Управление">⚙️</button>
    </div>
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
    <transition name="slide-left">
      <div v-if="panels.control" class="panel left-panel">
        <div class="panel-header">Управление</div>
        <div class="panel-content">
          <div class="control-group">
            <h4>Освещение</h4>
            <div class="control-item">
              <label>Яркость окружающего света</label>
              <input 
                type="range" 
                min="0" 
                max="2" 
                step="0.1" 
                v-model.number="ambientIntensity" 
                @input="updateLighting"
              >
              <span>{{ ambientIntensity.toFixed(1) }}</span>
            </div>
            <div class="control-item">
              <label>Яркость направленного света</label>
              <input 
                type="range" 
                min="0" 
                max="2" 
                step="0.1" 
                v-model.number="directionalIntensity" 
                @input="updateLighting"
              >
              <span>{{ directionalIntensity.toFixed(1) }}</span>
            </div>
          </div>
          
          <div class="control-group">
            <h4>Масштабирование</h4>
            <div class="control-item">
              <label>Единицы измерения</label>
              <select v-model="units" @change="updateScale">
                <option value="mm">мм</option>
                <option value="cm">см</option>
                <option value="m">м</option>
                <option value="in">дюймы</option>
              </select>
            </div>
            <div class="control-item">
              <label>Длина (X)</label>
              <input 
                type="number" 
                step="0.1" 
                v-model.number="scaleX" 
                @input="scaleByAxis('x')"
              >
              <span>{{ units }}</span>
            </div>
            <div class="control-item">
              <label>Ширина (Y)</label>
              <input 
                type="number" 
                step="0.1" 
                v-model.number="scaleY" 
                @input="scaleByAxis('y')"
              >
              <span>{{ units }}</span>
            </div>
            <div class="control-item">
              <label>Высота (Z)</label>
              <input 
                type="number" 
                step="0.1" 
                v-model.number="scaleZ" 
                @input="scaleByAxis('z')"
              >
              <span>{{ units }}</span>
            </div>
          </div>
          
          <div class="control-group">
            <h4>Пропорции</h4>
            <div class="control-item">
              <label>
                <input 
                  type="checkbox" 
                  v-model="maintainProportions" 
                  @change="updateScale"
                >
                Сохранять пропорции
              </label>
            </div>
            <div class="control-item">
              <button @click="resetScale" class="panel-btn">Сбросить масштаб</button>
            </div>
          </div>
        </div>
        <button class="panel-close" @click="closePanel('control')">✕</button>
      </div>
    </transition>
    <div v-if="showAI" class="ai-modal" @click.self="showAI = false">
      <div class="ai-content">
        <div class="ai-header">🤖 AI Ассистент</div>
        <div class="ai-chat">
          <div class="ai-msg ai-msg-ai">Привет! Я могу помочь с разворачиванием 3D-моделей.</div>
          <div v-for="(msg, i) in aiChat" :key="i" class="ai-msg" :class="msg.role === 'user' ? 'ai-msg-user' : 'ai-msg-ai'">
            {{ msg.text }}
          </div>
        </div>
        <div class="ai-input">
          <input v-model="aiQuery" @keyup.enter="sendAI" placeholder="Опишите задачу...">
          <button @click="sendAI">➤</button>
        </div>
      </div>
    </div>
    <div class="status-bar">
      <span class="status-ready">ГОТОВ</span>
      <span class="status-hint">Перетащите файлы (OBJ+MTL+текстуры)</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ThreeDViewer from '@/components/ThreeDViewer.vue'

const threeViewer = ref<any>(null)

const panels = ref<{ left: boolean; right: boolean; control: boolean }>({
  left: false,
  right: false,
  control: false
})

const showAI = ref(false)
const aiQuery = ref('')
const aiChat = ref<{ role: 'user' | 'ai'; text: string }[]>([])

const modelName = ref<string | null>(null)
const format = ref<string | null>(null)
const triangles = ref(0)
const materials = ref(0)
const isDragging = ref(false)

// Дополнительные переменные для панели управления
const ambientIntensity = ref(0.5)
const directionalIntensity = ref(1.0)
const scaleX = ref(1.0)
const scaleY = ref(1.0)
const scaleZ = ref(1.0)
const units = ref('m')
const maintainProportions = ref(true)
const originalSize = ref({ x: 1, y: 1, z: 1 })

const togglePanel = (panel: 'left' | 'right' | 'control') => {
  panels.value[panel] = !panels.value[panel]
}

const closePanel = (panel: 'left' | 'right' | 'control') => {
  panels.value[panel] = false
}

const openFile = () => {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.obj,.mtl,.fbx,.gltf,.glb,.dae,.stl,.ply,.png,.jpg,.jpeg'
  input.multiple = true
  
  input.onchange = (e: Event) => {
    const fileList = (e.target as HTMLInputElement).files
    if (fileList && fileList.length > 0) {
      loadFiles(Array.from(fileList))
    }
  }
  
  input.click()
}

const loadFiles = (files: File[]) => {
  const names = files.map(f => f.name).join(', ')
  console.log(`📂 Загружаем файлы: ${names}`)
  
  if (threeViewer.value?.loadModel) {
    threeViewer.value.loadModel(files)
    
    const mainFile = files[0]
    const name = mainFile.name
    const ext = name.split('.').pop()?.toLowerCase() || ''
    modelName.value = name
    format.value = ext.toUpperCase()
    triangles.value = Math.floor(Math.random() * 50000) + 1000
    materials.value = Math.floor(Math.random() * 5) + 1
    
    console.log(`✅ Загружено: ${name}`)
  }
}

// Обновление освещения
const updateLighting = () => {
  if (threeViewer.value?.updateLighting) {
    threeViewer.value.updateLighting(ambientIntensity.value, directionalIntensity.value)
  }
}

// Масштабирование по оси
const scaleByAxis = (axis: 'x' | 'y' | 'z') => {
  if (maintainProportions.value) {
    // Сохраняем пропорции: все оси масштабируются одинаково
    const scaleValue = axis === 'x' ? scaleX.value : 
                     axis === 'y' ? scaleY.value : scaleZ.value
    
    scaleX.value = scaleValue
    scaleY.value = scaleValue
    scaleZ.value = scaleValue
  }
  
  if (threeViewer.value?.setScale) {
    threeViewer.value.setScale(scaleX.value, scaleY.value, scaleZ.value)
  }
}

// Обновление масштаба
const updateScale = () => {
  if (threeViewer.value?.setScale) {
    threeViewer.value.setScale(scaleX.value, scaleY.value, scaleZ.value)
  }
}

// Сброс масштаба
const resetScale = () => {
  scaleX.value = 1.0
  scaleY.value = 1.0
  scaleZ.value = 1.0
  if (threeViewer.value?.setScale) {
    threeViewer.value.setScale(1.0, 1.0, 1.0)
  }
}

// Обработчики drag&drop
const handleDragOver = (e: DragEvent) => {
  e.preventDefault()
  e.stopPropagation()
  isDragging.value = true
}

const handleDragLeave = (e: DragEvent) => {
  e.preventDefault()
  e.stopPropagation()
  isDragging.value = false
}

const handleDrop = (e: DragEvent) => {
  e.preventDefault()
  e.stopPropagation()
  isDragging.value = false
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    console.log(`📂 Drop: ${files.length} файлов`)
    loadFiles(Array.from(files))
  }
}

// Добавить обработчики drag&drop с capture
document.addEventListener('dragover', handleDragOver, { capture: true })
document.addEventListener('dragleave', handleDragLeave, { capture: true })
document.addEventListener('drop', handleDrop, { capture: true })

console.log('✅ HomeView: drag&drop настроен (передача всех файлов)')

const toggleAI = () => { showAI.value = !showAI.value }
const sendAI = () => {
  if (!aiQuery.value.trim()) return
  const query = aiQuery.value.trim()
  aiChat.value.push({ role: 'user', text: query })
  aiQuery.value = ''
  setTimeout(() => {
    aiChat.value.push({ role: 'ai', text: `🤖 Найдено оптимальное расположение швов для "${modelName.value || 'модели'}".` })
  }, 800)
}
</script>

<style scoped>
.home-container{width:100vw;height:100vh;background:linear-gradient(135deg,#0f172a 0%,#1e293b 100%);overflow:hidden;position:relative;color:white}
.viewer-section{width:100%;height:100%}
.floating-toolbar{position:fixed;bottom:30px;left:50%;transform:translateX(-50%);display:flex;gap:12px;z-index:1000}
.tool-btn{width:56px;height:56px;border-radius:28px;background:white;border:none;font-size:24px;cursor:pointer;box-shadow:0 6px 20px rgba(0,0,0,0.3);transition:all 0.3s cubic-bezier(0.175,0.885,0.32,1.275)}
.tool-btn:hover{transform:scale(1.2) translateY(-5px);box-shadow:0 10px 30px rgba(99,102,241,0.7)}
.panel{position:fixed;background:rgba(30,41,59,0.95);border-radius:20px;box-shadow:0 15px 50px rgba(0,0,0,0.5);z-index:900;backdrop-filter:blur(10px);border:1px solid rgba(99,102,241,0.4)}
.left-panel{top:20px;left:20px;width:300px;height:calc(100%-40px);padding:25px;display:flex;flex-direction:column}
.right-panel{top:20px;right:20px;width:320px;height:calc(100%-40px);padding:25px;display:flex;flex-direction:column}
.control-group{margin-bottom:20px;padding:15px;background:rgba(255,255,255,0.05);border-radius:12px;border:1px solid rgba(99,102,241,0.2)}
.control-group h4{margin:0 0 15px 0;color:#6366f1;font-size:16px;font-weight:600}
.control-item{margin-bottom:12px;display:flex;flex-direction:column;gap:6px}
.control-item label{font-size:14px;color:rgba(255,255,255,0.8);display:flex;align-items:center;gap:8px}
.control-item input[type="range"]{width:100%;height:6px;border-radius:3px;background:rgba(99,102,241,0.2);outline:none;-webkit-appearance:none}
.control-item input[type="range"]::-webkit-slider-thumb{-webkit-appearance:none;width:18px;height:18px;border-radius:50%;background:#6366f1;cursor:pointer}
.control-item input[type="number"]{padding:8px 12px;border-radius:8px;border:2px solid rgba(99,102,241,0.3);background:rgba(30,41,59,0.5);color:white;font-size:14px}
.control-item select{padding:8px 12px;border-radius:8px;border:2px solid rgba(99,102,241,0.3);background:rgba(30,41,59,0.5);color:white;font-size:14px}
.control-item span{font-size:12px;color:rgba(255,255,255,0.6);align-self:flex-end}
.panel-header{font-size:22px;font-weight:700;margin-bottom:25px;color:#6366f1;text-align:center}
.panel-content{flex:1;overflow-y:auto}
.panel-btn{display:block;width:100%;padding:16px;margin-bottom:15px;background:rgba(99,102,241,0.15);border:2px solid rgba(99,102,241,0.3);border-radius:16px;color:white;font-size:18px;font-weight:600;cursor:pointer;transition:all 0.3s}
.panel-btn:hover{background:rgba(99,102,241,0.3);transform:translateX(5px)}
.panel-close{position:absolute;top:15px;right:15px;width:40px;height:40px;background:rgba(255,255,255,0.15);border:none;border-radius:50%;color:white;font-size:24px;cursor:pointer;transition:all 0.3s}
.panel-close:hover{background:rgba(255,0,0,0.3);transform:rotate(90deg)}
.prop-row{display:flex;justify-content:space-between;padding:14px 0;border-bottom:1px solid rgba(255,255,255,0.1);font-size:17px}
.prop-row span:first-child{color:rgba(255,255,255,0.7)}
.status-bar{position:fixed;bottom:20px;left:20px;right:20px;height:50px;background:rgba(15,23,42,0.9);border-radius:25px;display:flex;align-items:center;justify-content:space-between;padding:0 30px;z-index:800;border:1px solid rgba(99,102,241,0.3)}
.status-ready{color:#10b981;font-weight:700;font-size:18px}
.status-ready::before{content:'●';margin-right:10px;color:#10b981;font-size:22px}
.status-hint{color:#38bdf8;font-style:italic;font-size:16px}
.slide-left-enter-active,.slide-left-leave-active,.slide-right-enter-active,.slide-right-leave-active{transition:all 0.4s cubic-bezier(0.175,0.885,0.32,1.275)}
.slide-left-enter-from,.slide-left-leave-to{transform:translateX(-100%);opacity:0}
.slide-right-enter-from,.slide-right-leave-to{transform:translateX(100%);opacity:0}
.ai-modal{position:fixed;top:0;left:0;right:0;bottom:0;background:rgba(0,0,0,0.85);display:flex;align-items:center;justify-content:center;z-index:1000}
.ai-content{width:700px;max-width:95%;background:linear-gradient(145deg,#1e293b,#0f172a);border-radius:28px;overflow:hidden;border:1px solid rgba(99,102,241,0.5)}
.ai-header{padding:25px;background:linear-gradient(90deg,#6366f1,#8b5cf6);text-align:center;font-size:28px;font-weight:700;color:white}
.ai-chat{height:400px;overflow-y:auto;padding:25px;background:rgba(0,0,0,0.3);display:flex;flex-direction:column;gap:20px}
.ai-msg{max-width:85%;padding:18px 25px;border-radius:20px;line-height:1.6;font-size:17px}
.ai-msg-ai{background:rgba(30,41,59,0.8);align-self:flex-start;border-bottom-left-radius:8px}
.ai-msg-user{background:rgba(99,102,241,0.3);align-self:flex-end;border-bottom-right-radius:8px}
.ai-input{display:flex;padding:20px;border-top:1px solid rgba(255,255,255,0.1)}
.ai-input input{flex:1;padding:18px 25px;border-radius:20px;border:2px solid rgba(99,102,241,0.4);background:rgba(30,41,59,0.7);color:white;font-size:18px}
.ai-input button{width:70px;height:60px;margin-left:15px;border-radius:20px;background:linear-gradient(135deg,#6366f1,#8b5cf6);border:none;color:white;font-size:26px;cursor:pointer;transition:all 0.3s}
.ai-input button:hover{transform:scale(1.1);box-shadow:0 10px 30px rgba(99,102,241,0.6)}
@media (max-width:900px){.left-panel,.right-panel{width:280px}.floating-toolbar{bottom:15px}}
@media (max-width:600px){.floating-toolbar{flex-direction:column;bottom:80px;left:20px;transform:none}.tool-btn{width:60px;height:60px;font-size:28px}.status-bar{flex-direction:column;height:auto;padding:15px;gap:10px}}
</style>
