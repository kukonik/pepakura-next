# 3D Viewer и 2D Editor в Pepakura Next

## Обзор

Интерактивные компоненты для просмотра и редактирования 3D моделей и их развёрток.

## Компоненты

### Viewer3D

Интерактивный 3D вьювер на базе Three.js.

**Возможности:**
- 🖱️ Orbit controls (вращение, панорамирование, масштаб)
- 🎯 Выделение граней
- 🔲 Режим каркаса
- 🔄 Автоматическое вращение
- ⌨️ Горячие клавиши

**Использование:**
```vue
<template>
  <Viewer3D 
    :mesh="mesh" 
    @faceSelect="onFaceSelect"
  />
</template>

<script setup>
import Viewer3D from '@/components/viewer/Viewer3D.vue'

function onFaceSelect(faceIndex) {
  console.log('Selected face:', faceIndex)
}
</script>
```

### UnfoldEditor

2D редактор развёрток с SVG.

**Возможности:**
- 🖱️ Перемещение по холсту
- 🔍 Масштабирование
- 🎯 Выделение деталей
- # Сетка
- 🔢 Номера деталей
- 📐 Свойства деталей

**Использование:**
```vue
<template>
  <UnfoldEditor 
    :unfolded-mesh="unfolded"
    @partSelect="onPartSelect"
    @export="onExport"
  />
</template>
```

### Workspace

Объединяет 3D и 2D виды с синхронизацией.

**Возможности:**
- ↔️ Разделение экрана
- 🔄 Синхронизация выделения
- 📊 Status bar

**Использование:**
```vue
<template>
  <Workspace 
    :mesh="mesh"
    :unfolded-mesh="unfolded"
  />
</template>
```

## Управление

### 3D Viewer

| Действие | Управление |
|----------|------------|
| Вращение | ЛКМ + drag |
| Панорамирование | ПКМ + drag |
| Масштаб | Колесо мыши |
| Выделение | Клик |
| Сброс вида | R |
| Каркас | W |
| Помощь | H |

### 2D Editor

| Действие | Управление |
|----------|------------|
| Панорамирование | ЛКМ + drag |
| Масштаб | Колесо мыши |
| Выделение | Клик |
| Сброс вида | R |
| Сетка | G |
| Номера | N |

## Синхронизация

Компонент `useViewLinking` синхронизирует выделение между 2D и 3D:

```typescript
import { useViewLinking } from '@/composables/useViewLinking'

const { 
  selectFace2D, 
  selectFace3D,
  selectedFace2D,
  selectedFace3D 
} = useViewLinking()

// Выделение в 2D автоматически выделяет в 3D
selectFace2D(5)
console.log(selectedFace3D.value) // 5
```

## API

### Viewer3D Props

| Prop | Тип | Описание |
|------|-----|----------|
| mesh | MeshData | 3D модель |
| unfoldedMesh | any | Развёртка |
| width | number | Ширина |
| height | number | Высота |

### Viewer3D Events

| Event | Payload | Описание |
|-------|---------|----------|
| faceSelect | number | Выделение грани |
| faceHover | number | Наведение на грань |

### UnfoldEditor Props

| Prop | Тип | Описание |
|------|-----|----------|
| unfoldedMesh | any | Развёртка |
| width | number | Ширина |
| height | number | Высота |

### UnfoldEditor Events

| Event | Payload | Описание |
|-------|---------|----------|
| partSelect | number | Выделение детали |
| export | 'svg' | 'pdf' | Экспорт |

## Примеры

### Базовое использование

```vue
<template>
  <div class="viewer-container">
    <Viewer3D :mesh="model" />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import Viewer3D from '@/components/viewer/Viewer3D.vue'

const model = ref({
  vertices: [
    [0, 0, 0],
    [1, 0, 0],
    [1, 1, 0],
    [0, 1, 0],
  ],
  faces: [
    [0, 1, 2],
    [0, 2, 3],
  ],
  name: 'Square'
})
</script>
```

### Синхронизированный вид

```vue
<template>
  <Workspace 
    :mesh="mesh"
    :unfolded-mesh="unfolded"
  />
</template>

<script setup>
import { computed } from 'vue'
import Workspace from '@/components/workspace/Workspace.vue'

const props = defineProps(['mesh', 'unfolded'])
</script>
```

### Программное выделение

```vue
<script setup>
import { ref } from 'vue'
import Viewer3D from '@/components/viewer/Viewer3D.vue'

const viewer = ref(null)

function highlightFace(index) {
  viewer.value?.selectFace(index)
}
</script>
```

## Стилизация

### Кастомные цвета

```css
.viewer-3d {
  --viewer-background: #1a1a1a;
  --viewer-grid: #333;
  --viewer-highlight: #ff0000;
}
```

### Тёмная тема

```css
.dark .viewer-3d {
  background: #1a1a1a;
}

.dark .editor-canvas {
  background: #2a2a2a;
}
```

## Производительность

### Оптимизация

- ✅ Instanced rendering для больших мешей
- ✅ Level of detail (LOD)
- ✅ Frustum culling
- ✅ Virtual scrolling для 2D

### Рекомендации

| Размер меша | Рекомендации |
|-------------|--------------|
| < 1000 вершин | Без ограничений |
| 1000-10000 вершин | Упростить геометрию |
| > 10000 вершин | Использовать LOD |

## Будущие улучшения

- [ ] Измерение расстояний
- [ ] Аннотации
- [ ] Экспорт скриншотов
- [ ] AR просмотр
- [ ] Поддержка текстур

## Лицензия

MIT
