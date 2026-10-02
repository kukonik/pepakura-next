<template>
  <div class="material-editor">
    <!-- Панель инструментов материалов -->
    <div class="material-toolbar">
      <div class="toolbar-group">
        <button 
          class="tool-btn" 
          @click="createNewMaterial"
          title="Создать новый материал"
        >
          <i class="fas fa-plus"></i> Новый
        </button>
        <button 
          class="tool-btn" 
          @click="openMaterialLibrary"
          title="Библиотека материалов"
        >
          <i class="fas fa-book"></i> Библиотека
        </button>
        <button 
          class="tool-btn" 
          @click="duplicateMaterial"
          title="Дублировать материал"
          :disabled="!selectedMaterial"
        >
          <i class="fas fa-copy"></i> Дублировать
        </button>
      </div>
      
      <div class="toolbar-group">
        <button 
          class="tool-btn" 
          @click="applyMaterialToSelection"
          title="Применить к выделению"
          :disabled="!selectedMaterial"
        >
          <i class="fas fa-fill-drip"></i> Применить
        </button>
        <button 
          class="tool-btn" 
          @click="removeMaterial"
          title="Удалить материал"
          :disabled="!selectedMaterial"
        >
          <i class="fas fa-trash"></i> Удалить
        </button>
      </div>
    </div>
    
    <!-- Основная область редактора -->
    <div class="editor-main">
      <!-- Панель списка материалов -->
      <div class="materials-panel">
        <h3>Материалы</h3>
        <div class="materials-list">
          <div 
            v-for="material in materials" 
            :key="material.id"
            class="material-item"
            :class="{ selected: selectedMaterial === material.id }"
            @click="selectMaterial(material.id)"
          >
            <div class="material-preview" :style="{ backgroundColor: material.albedo }"></div>
            <div class="material-info">
              <div class="material-name">{{ material.name }}</div>
              <div class="material-type">{{ material.type }}</div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Панель свойств материала -->
      <div class="properties-panel" v-if="selectedMaterial">
        <h3>Свойства материала: {{ currentMaterial.name }}</h3>
        
        <!-- Основные параметры PBR -->
        <div class="property-section">
          <h4><i class="fas fa-palette"></i> Основные параметры</h4>
          
          <div class="property-group">
            <label>Название:</label>
            <input 
              type="text" 
              v-model="currentMaterial.name" 
              @change="updateMaterialProperty('name')"
            />
          </div>
          
          <div class="property-group">
            <label>Тип материала:</label>
            <select v-model="currentMaterial.type" @change="updateMaterialProperty('type')">
              <option value="standard">Стандартный (PBR)</option>
              <option value="basic">Базовый</option>
              <option value="lambert">Ламберт</option>
              <option value="phong">Фонг</option>
              <option value="custom">Пользовательский</option>
            </select>
          </div>
          
          <div class="property-group color-picker">
            <label>Основной цвет (Albedo):</label>
            <div class="color-input">
              <input 
                type="color" 
                v-model="currentMaterial.albedo" 
                @change="updateMaterialProperty('albedo')"
              />
              <span class="color-value">{{ currentMaterial.albedo }}</span>
            </div>
          </div>
        </div>
        
        <!-- Параметры PBR -->
        <div class="property-section" v-if="currentMaterial.type === 'standard'">
          <h4><i class="fas fa-sliders-h"></i> Параметры PBR</h4>
          
          <div class="property-group">
            <label>Шероховатость (Roughness): {{ currentMaterial.roughness.toFixed(2) }}</label>
            <input 
              type="range" 
              min="0" 
              max="1" 
              step="0.01" 
              v-model="currentMaterial.roughness" 
              @change="updateMaterialProperty('roughness')"
            />
          </div>
          
          <div class="property-group">
            <label>Металличность (Metalness): {{ currentMaterial.metalness.toFixed(2) }}</label>
            <input 
              type="range" 
              min="0" 
              max="1" 
              step="0.01" 
              v-model="currentMaterial.metalness" 
              @change="updateMaterialProperty('metalness')"
            />
          </div>
          
          <div class="property-group">
            <label>Самосвечение (Emissive): {{ currentMaterial.emissive.toFixed(2) }}</label>
            <input 
              type="range" 
              min="0" 
              max="1" 
              step="0.01" 
              v-model="currentMaterial.emissive" 
              @change="updateMaterialProperty('emissive')"
            />
          </div>
          
          <div class="property-group">
            <label>Прозрачность (Opacity): {{ currentMaterial.opacity.toFixed(2) }}</label>
            <input 
              type="range" 
              min="0" 
              max="1" 
              step="0.01" 
              v-model="currentMaterial.opacity" 
              @change="updateMaterialProperty('opacity')"
            />
          </div>
        </div>
        
        <!-- Текстуры -->
        <div class="property-section">
          <h4><i class="fas fa-images"></i> Текстуры</h4>
          
          <div class="texture-slot">
            <div class="texture-label">Albedo Map:</div>
            <div class="texture-preview" @click="openTexturePicker('albedoMap')">
              <img v-if="currentMaterial.albedoMap" :src="currentMaterial.albedoMap" alt="Albedo Map" />
              <div v-else class="texture-placeholder">
                <i class="fas fa-image"></i>
              </div>
            </div>
            <button 
              class="texture-btn" 
              @click="openTexturePicker('albedoMap')"
              :class="{ assigned: currentMaterial.albedoMap }"
            >
              {{ currentMaterial.albedoMap ? 'Изменить' : 'Назначить' }}
            </button>
          </div>
          
          <div class="texture-slot" v-if="currentMaterial.type === 'standard'">
            <div class="texture-label">Normal Map:</div>
            <div class="texture-preview" @click="openTexturePicker('normalMap')">
              <img v-if="currentMaterial.normalMap" :src="currentMaterial.normalMap" alt="Normal Map" />
              <div v-else class="texture-placeholder">
                <i class="fas fa-mountain"></i>
              </div>
            </div>
            <button 
              class="texture-btn" 
              @click="openTexturePicker('normalMap')"
              :class="{ assigned: currentMaterial.normalMap }"
            >
              {{ currentMaterial.normalMap ? 'Изменить' : 'Назначить' }}
            </button>
          </div>
          
          <div class="texture-slot" v-if="currentMaterial.type === 'standard'">
            <div class="texture-label">Roughness Map:</div>
            <div class="texture-preview" @click="openTexturePicker('roughnessMap')">
              <img v-if="currentMaterial.roughnessMap" :src="currentMaterial.roughnessMap" alt="Roughness Map" />
              <div v-else class="texture-placeholder">
                <i class="fas fa-border-all"></i>
              </div>
            </div>
            <button 
              class="texture-btn" 
              @click="openTexturePicker('roughnessMap')"
              :class="{ assigned: currentMaterial.roughnessMap }"
            >
              {{ currentMaterial.roughnessMap ? 'Изменить' : 'Назначить' }}
            </button>
          </div>
          
          <div class="texture-slot" v-if="currentMaterial.type === 'standard'">
            <div class="texture-label">Metalness Map:</div>
            <div class="texture-preview" @click="openTexturePicker('metalnessMap')">
              <img v-if="currentMaterial.metalnessMap" :src="currentMaterial.metalnessMap" alt="Metalness Map" />
              <div v-else class="texture-placeholder">
                <i class="fas fa-gem"></i>
              </div>
            </div>
            <button 
              class="texture-btn" 
              @click="openTexturePicker('metalnessMap')"
              :class="{ assigned: currentMaterial.metalnessMap }"
            >
              {{ currentMaterial.metalnessMap ? 'Изменить' : 'Назначить' }}
            </button>
          </div>
        </div>
        
        <!-- Превью материала -->
        <div class="property-section">
          <h4><i class="fas fa-eye"></i> Предпросмотр</h4>
          <div class="material-preview-large">
            <div 
              class="preview-sphere" 
              :style="{
                backgroundColor: currentMaterial.albedo,
                backgroundImage: currentMaterial.albedoMap ? `url(${currentMaterial.albedoMap})` : 'none',
                opacity: currentMaterial.opacity
              }"
            ></div>
          </div>
        </div>
      </div>
      
      <!-- Панель библиотеки материалов -->
      <div class="library-panel" v-if="showLibrary">
        <h3>Библиотека материалов</h3>
        <div class="library-categories">
          <button 
            v-for="category in materialCategories" 
            :key="category.id"
            class="category-btn"
            :class="{ active: selectedCategory === category.id }"
            @click="selectCategory(category.id)"
          >
            {{ category.name }}
          </button>
        </div>
        
        <div class="library-materials">
          <div 
            v-for="material in filteredLibraryMaterials" 
            :key="material.id"
            class="library-material-item"
            @click="applyLibraryMaterial(material)"
          >
            <div class="library-material-preview" :style="{ backgroundColor: material.albedo }"></div>
            <div class="library-material-name">{{ material.name }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

// Состояние редактора
const selectedMaterial = ref<string | null>(null)
const showLibrary = ref(false)
const selectedCategory = ref('metals')

// Данные материалов
const materials = ref([
  { 
    id: 'mat1', 
    name: 'Сталь', 
    type: 'standard',
    albedo: '#c0c0c0',
    roughness: 0.2,
    metalness: 0.9,
    emissive: 0.0,
    opacity: 1.0,
    albedoMap: null,
    normalMap: null,
    roughnessMap: null,
    metalnessMap: null
  },
  { 
    id: 'mat2', 
    name: 'Дерево', 
    type: 'standard',
    albedo: '#8B4513',
    roughness: 0.8,
    metalness: 0.0,
    emissive: 0.0,
    opacity: 1.0,
    albedoMap: null,
    normalMap: null,
    roughnessMap: null,
    metalnessMap: null
  },
  { 
    id: 'mat3', 
    name: 'Пластик', 
    type: 'standard',
    albedo: '#1E90FF',
    roughness: 0.3,
    metalness: 0.0,
    emissive: 0.0,
    opacity: 1.0,
    albedoMap: null,
    normalMap: null,
    roughnessMap: null,
    metalnessMap: null
  }
])

// Вычисляемый активный материал
const currentMaterial = computed(() => {
  if (!selectedMaterial.value) return null
  return materials.value.find(m => m.id === selectedMaterial.value) || null
})

// Категории материалов в библиотеке
const materialCategories = ref([
  { id: 'metals', name: 'Металлы' },
  { id: 'plastics', name: 'Пластик' },
  { id: 'woods', name: 'Дерево' },
  { id: 'stones', name: 'Камень' },
  { id: 'fabrics', name: 'Ткани' }
])

// Материалы в библиотеке
const libraryMaterials = ref([
  { id: 'lib1', name: 'Золото', albedo: '#FFD700', category: 'metals' },
  { id: 'lib2', name: 'Серебро', albedo: '#C0C0C0', category: 'metals' },
  { id: 'lib3', name: 'Медь', albedo: '#B87333', category: 'metals' },
  { id: 'lib4', name: 'Алюминий', albedo: '#C0C0C0', category: 'metals' },
  { id: 'lib5', name: 'Глянцевый пластик', albedo: '#FF69B4', category: 'plastics' },
  { id: 'lib6', name: 'Матовый пластик', albedo: '#708090', category: 'plastics' },
  { id: 'lib7', name: 'Дуб', albedo: '#8B4513', category: 'woods' },
  { id: 'lib8', name: 'Сосна', albedo: '#D2B48C', category: 'woods' },
  { id: 'lib9', name: 'Гранит', albedo: '#696969', category: 'stones' },
  { id: 'lib10', name: 'Мрамор', albedo: '#F5F5F5', category: 'stones' },
  { id: 'lib11', name: 'Хлопок', albedo: '#FFFFFF', category: 'fabrics' },
  { id: 'lib12', name: 'Шелк', albedo: '#FFFFF0', category: 'fabrics' }
])

// Фильтрованные материалы библиотеки
const filteredLibraryMaterials = computed(() => {
  return libraryMaterials.value.filter(m => m.category === selectedCategory.value)
})

// Методы управления материалами
function createNewMaterial() {
  const newMaterial = {
    id: `mat${Date.now()}`,
    name: 'Новый материал',
    type: 'standard',
    albedo: '#FFFFFF',
    roughness: 0.5,
    metalness: 0.0,
    emissive: 0.0,
    opacity: 1.0,
    albedoMap: null,
    normalMap: null,
    roughnessMap: null,
    metalnessMap: null
  }
  
  materials.value.push(newMaterial)
  selectedMaterial.value = newMaterial.id
  console.log('Создан новый материал:', newMaterial.name)
}

function selectMaterial(materialId: string) {
  selectedMaterial.value = materialId
  console.log('Выбран материал:', materialId)
}

function updateMaterialProperty(property: string) {
  if (!currentMaterial.value) return
  console.log(`Обновлено свойство ${property}:`, currentMaterial.value[property])
}

function duplicateMaterial() {
  if (!selectedMaterial.value || !currentMaterial.value) return
  
  const original = currentMaterial.value
  const duplicated = {
    ...original,
    id: `mat${Date.now()}`,
    name: `${original.name} (копия)`
  }
  
  materials.value.push(duplicated)
  selectedMaterial.value = duplicated.id
  console.log('Материал дублирован:', duplicated.name)
}

function removeMaterial() {
  if (!selectedMaterial.value) return
  
  const index = materials.value.findIndex(m => m.id === selectedMaterial.value)
  if (index !== -1) {
    materials.value.splice(index, 1)
    selectedMaterial.value = materials.value.length > 0 ? materials.value[0].id : null
    console.log('Материал удален')
  }
}

function applyMaterialToSelection() {
  if (!selectedMaterial.value) return
  console.log('Материал применен к выделению:', selectedMaterial.value)
}

// Методы управления библиотекой
function openMaterialLibrary() {
  showLibrary.value = !showLibrary.value
  console.log('Открыта библиотека материалов')
}

function selectCategory(categoryId: string) {
  selectedCategory.value = categoryId
  console.log('Выбрана категория:', categoryId)
}

function applyLibraryMaterial(material: any) {
  if (!selectedMaterial.value || !currentMaterial.value) return
  
  // Применяем свойства из библиотеки к текущему материалу
  currentMaterial.value.albedo = material.albedo
  currentMaterial.value.name = material.name
  console.log('Применен материал из библиотеки:', material.name)
}

// Методы управления текстурами
function openTexturePicker(textureType: string) {
  console.log('Открытие выбора текстуры:', textureType)
  // Здесь будет логика открытия диалога выбора текстуры
}

// Инициализация - выбираем первый материал
if (materials.value.length > 0) {
  selectedMaterial.value = materials.value[0].id
}
</script>

<style scoped>
.material-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #0f172a;
  color: #e2e8f0;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.material-toolbar {
  display: flex;
  align-items: center;
  padding: 0.8rem 1rem;
  background: rgba(15, 23, 42, 0.95);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  gap: 1.5rem;
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.tool-btn {
  padding: 0.6rem 1rem;
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(30, 41, 59, 0.8);
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
  font-size: 0.9rem;
}

.tool-btn:hover:not(:disabled) {
  background: rgba(56, 70, 95, 0.9);
  color: #fff;
}

.tool-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.editor-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.materials-panel {
  width: 250px;
  background: rgba(15, 23, 42, 0.8);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
  padding: 1rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.materials-panel h3 {
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
  color: #e2e8f0;
}

.materials-list {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
}

.material-item {
  padding: 0.8rem;
  border-radius: 8px;
  background: rgba(30, 41, 59, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.08);
  display: flex;
  align-items: center;
  gap: 0.8rem;
  cursor: pointer;
  transition: all 0.2s;
}

.material-item:hover {
  background: rgba(56, 70, 95, 0.8);
}

.material-item.selected {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  border: none;
}

.material-preview {
  width: 40px;
  height: 40px;
  border-radius: 6px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.material-info {
  flex: 1;
}

.material-name {
  font-weight: 500;
  margin-bottom: 0.2rem;
}

.material-type {
  font-size: 0.8rem;
  color: #94a3b8;
}

.properties-panel {
  flex: 1;
  background: rgba(15, 23, 42, 0.9);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
  padding: 1.5rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.properties-panel h3 {
  margin: 0 0 1rem 0;
  font-size: 1.2rem;
  color: #e2e8f0;
}

.property-section {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 10px;
  padding: 1.2rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.property-section h4 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: #e2e8f0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.property-group {
  margin-bottom: 1.2rem;
}

.property-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  color: #94a3b8;
}

.property-group input[type="text"],
.property-group select {
  width: 100%;
  padding: 0.7rem;
  background: rgba(15, 23, 42, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: #e2e8f0;
  font-size: 0.95rem;
}

.property-group input[type="range"] {
  width: 100%;
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  outline: none;
  -webkit-appearance: none;
}

.property-group input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 16px;
  height: 16px;
  background: #3b82f6;
  border-radius: 50%;
  cursor: pointer;
}

.color-picker {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.color-input {
  display: flex;
  align-items: center;
  gap: 0.8rem;
}

.color-input input[type="color"] {
  width: 50px;
  height: 35px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  background: rgba(15, 23, 42, 0.8);
}

.color-value {
  font-family: monospace;
  font-size: 0.9rem;
  color: #94a3b8;
}

.texture-slot {
  margin-bottom: 1.5rem;
}

.texture-label {
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  color: #94a3b8;
}

.texture-preview {
  width: 100%;
  height: 120px;
  background: rgba(15, 23, 42, 0.8);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  margin-bottom: 0.8rem;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  overflow: hidden;
}

.texture-preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.texture-placeholder {
  color: #94a3b8;
  font-size: 2rem;
}

.texture-btn {
  width: 100%;
  padding: 0.7rem;
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(30, 41, 59, 0.8);
  color: #cbd5e1;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 0.9rem;
}

.texture-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  color: #fff;
}

.texture-btn.assigned {
  background: linear-gradient(135deg, #10b981, #34d399);
  color: white;
  border: none;
}

.material-preview-large {
  display: flex;
  justify-content: center;
  margin-top: 1rem;
}

.preview-sphere {
  width: 150px;
  height: 150px;
  border-radius: 50%;
  border: 2px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.3);
  background-size: cover;
  background-position: center;
}

.library-panel {
  width: 300px;
  background: rgba(15, 23, 42, 0.8);
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  overflow-y: auto;
}

.library-panel h3 {
  margin: 0 0 1rem 0;
  font-size: 1.2rem;
  color: #e2e8f0;
}

.library-categories {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.category-btn {
  padding: 0.5rem 1rem;
  border-radius: 20px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(30, 41, 59, 0.8);
  color: #cbd5e1;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.category-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  color: #fff;
}

.category-btn.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  border: none;
}

.library-materials {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

.library-material-item {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  padding: 1rem;
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
}

.library-material-item:hover {
  background: rgba(56, 70, 95, 0.8);
  transform: translateY(-3px);
}

.library-material-preview {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  margin: 0 auto 0.8rem;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.library-material-name {
  font-size: 0.9rem;
  color: #e2e8f0;
}
</style>