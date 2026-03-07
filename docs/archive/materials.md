# Техническая документация: Система материалов

## Обзор

Система материалов в Pepakura Next обеспечивает обработку и применение материалов к 3D моделям. Поддерживаются различные типы материалов, включая PBR (Physically Based Rendering), стандартные и базовые материалы.

## Архитектура

### Основные компоненты

#### MaterialProcessor
Класс `MaterialProcessor` отвечает за обработку данных материалов и создание соответствующих объектов Three.js.

**Расположение:** `src/modules/materials/MaterialProcessor.ts`

**Основные методы:**
- `process(materials: MaterialData[]): Promise<THREE.Material[]>` - Обработка массива материалов
- `createPBRMaterial(materialData: MaterialData): Promise<THREE.MeshStandardMaterial>` - Создание PBR материала
- `createStandardMaterial(materialData: MaterialData): THREE.MeshStandardMaterial` - Создание стандартного материала
- `createBasicMaterial(materialData: MaterialData): THREE.MeshBasicMaterial` - Создание базового материала
- `applyTextures(material, textures): Promise<void>` - Применение текстур к материалу

### Интерфейсы данных

#### MaterialData
```typescript
interface MaterialData {
  name: string;                    // Имя материала
  type: string;                    // Тип материала: 'pbr', 'standard', 'basic'
  properties: MaterialProperties;  // Свойства материала
  textures: TextureData[];         // Массив текстур
}
```

#### MaterialProperties
```typescript
interface MaterialProperties {
  baseColor?: [number, number, number];  // RGB базовый цвет [0-1]
  roughness?: number;                     // Шероховатость [0-1]
  metalness?: number;                     // Металличность [0-1]
  emissive?: [number, number, number];    // RGB эмиссивный цвет [0-1]
  opacity?: number;                       // Прозрачность [0-1]
}
```

#### TextureData
```typescript
interface TextureData {
  type: 'baseColor' | 'normal' | 'roughness' | 'metalness' | 'emissive';
  url: string;        // URL текстуры
  data?: ArrayBuffer; // Данные текстуры (опционально)
}
```

## Типы материалов

### 1. PBR материалы (Physically Based Rendering)

PBR материалы обеспечивают реалистичное отображение с учетом физических свойств поверхности.

**Свойства:**
- `baseColor` - Базовый цвет материала
- `roughness` - Шероховатость поверхности (0 = зеркальная, 1 = матовая)
- `metalness` - Металличность (0 = диэлектрик, 1 = металл)
- `emissive` - Эмиссивный цвет (свечение)
- `opacity` - Прозрачность

**Поддерживаемые текстуры:**
- `baseColor` - Карта базового цвета (albedo/diffuse)
- `normal` - Карта нормалей
- `roughness` - Карта шероховатости
- `metalness` - Карта металличности
- `emissive` - Карта эмиссии

**Пример использования:**
```typescript
import { MaterialProcessor, MaterialData } from '@/modules/materials/MaterialProcessor';

const processor = new MaterialProcessor();

const materialData: MaterialData = {
  name: 'MetalSurface',
  type: 'pbr',
  properties: {
    baseColor: [0.8, 0.8, 0.9],
    roughness: 0.2,
    metalness: 0.9,
    emissive: [0, 0, 0]
  },
  textures: [
    {
      type: 'baseColor',
      url: '/textures/metal_albedo.jpg'
    },
    {
      type: 'normal',
      url: '/textures/metal_normal.jpg'
    },
    {
      type: 'roughness',
      url: '/textures/metal_roughness.jpg'
    }
  ]
};

const materials = await processor.process([materialData]);
```

### 2. Стандартные материалы

Стандартные материалы используют упрощенную модель освещения.

**Свойства:**
- `baseColor` - Базовый цвет
- `roughness` - Шероховатость
- `metalness` - Металличность

**Пример использования:**
```typescript
const materialData: MaterialData = {
  name: 'StandardMaterial',
  type: 'standard',
  properties: {
    baseColor: [0.5, 0.5, 0.5],
    roughness: 0.5,
    metalness: 0.0
  },
  textures: []
};
```

### 3. Базовые материалы

Базовые материалы не учитывают освещение, используются для простых объектов.

**Свойства:**
- `baseColor` - Базовый цвет
- `opacity` - Прозрачность

**Пример использования:**
```typescript
const materialData: MaterialData = {
  name: 'BasicMaterial',
  type: 'basic',
  properties: {
    baseColor: [1.0, 0.0, 0.0],
    opacity: 0.8
  },
  textures: []
};
```

## Загрузка текстур

### Загрузка по URL
```typescript
const textureData: TextureData = {
  type: 'baseColor',
  url: 'https://example.com/texture.jpg'
};
```

### Загрузка из ArrayBuffer
```typescript
const textureData: TextureData = {
  type: 'normal',
  url: '', // Может быть пустым
  data: arrayBuffer // Данные текстуры
};
```

## Интеграция с загрузчиками моделей

Материалы автоматически обрабатываются при загрузке моделей через универсальный загрузчик:

```typescript
import { NewUniversalModelLoader } from '@/loaders/NewUniversalModelLoader';

const loader = new NewUniversalModelLoader();
const result = await loader.loadOBJ('/models/model.obj', '/models/model.mtl');

// result.materials содержит массив MaterialData
if (result.materials) {
  const processor = new MaterialProcessor();
  const threeMaterials = await processor.process(result.materials);
  // Применение материалов к мешам
}
```

## Применение материалов к мешам

```typescript
import * as THREE from 'three';

// Создание материала
const processor = new MaterialProcessor();
const materials = await processor.process([materialData]);

// Применение к мешу
const geometry = new THREE.BoxGeometry();
const mesh = new THREE.Mesh(geometry, materials[0]);

// Или для нескольких материалов
const meshWithMultipleMaterials = new THREE.Mesh(geometry, materials);
```

## Оптимизация производительности

### Кэширование материалов
Материалы можно кэшировать для повторного использования:

```typescript
const materialCache = new Map<string, THREE.Material>();

async function getMaterial(materialData: MaterialData): Promise<THREE.Material> {
  const key = materialData.name;
  
  if (materialCache.has(key)) {
    return materialCache.get(key)!;
  }
  
  const processor = new MaterialProcessor();
  const materials = await processor.process([materialData]);
  const material = materials[0];
  
  materialCache.set(key, material);
  return material;
}
```

### Управление памятью
Важно освобождать ресурсы при удалении материалов:

```typescript
function disposeMaterial(material: THREE.Material) {
  // Освобождение текстур
  if (material instanceof THREE.MeshStandardMaterial) {
    material.map?.dispose();
    material.normalMap?.dispose();
    material.roughnessMap?.dispose();
    material.metalnessMap?.dispose();
    material.emissiveMap?.dispose();
  }
  
  // Освобождение материала
  material.dispose();
}
```

## Поддерживаемые форматы текстур

- JPEG (.jpg, .jpeg)
- PNG (.png)
- WebP (.webp)
- GIF (.gif)
- BMP (.bmp)

## Рекомендации по использованию

1. **PBR материалы** - Используйте для реалистичных объектов (металл, дерево, кожа)
2. **Стандартные материалы** - Используйте для простых объектов с освещением
3. **Базовые материалы** - Используйте для UI элементов и простых декораций

4. **Размеры текстур:**
   - Рекомендуется использовать степени двойки (256x256, 512x512, 1024x1024)
   - Максимальный размер: 2048x2048 для большинства случаев
   - Используйте сжатие текстур для уменьшения размера файлов

5. **Оптимизация:**
   - Используйте один материал для нескольких объектов с одинаковыми свойствами
   - Группируйте объекты с одинаковыми материалами
   - Используйте атласы текстур для уменьшения количества draw calls

## Расширение системы

### Добавление нового типа материала

1. Добавьте новый case в метод `process`:
```typescript
case 'custom':
  material = await this.createCustomMaterial(materialData);
  break;
```

2. Реализуйте метод создания материала:
```typescript
private createCustomMaterial(materialData: MaterialData): THREE.Material {
  // Реализация создания кастомного материала
}
```

### Добавление новых свойств материала

1. Расширьте интерфейс `MaterialProperties`:
```typescript
interface MaterialProperties {
  // ... существующие свойства
  customProperty?: number;
}
```

2. Примените свойство в соответствующем методе создания материала

## Отладка

### Проверка загрузки текстур
```typescript
const processor = new MaterialProcessor();
const materials = await processor.process([materialData]);

// Проверка загруженных текстур
const material = materials[0] as THREE.MeshStandardMaterial;
console.log('Base color map:', material.map);
console.log('Normal map:', material.normalMap);
```

### Визуализация свойств материала
```typescript
function logMaterialProperties(material: THREE.Material) {
  if (material instanceof THREE.MeshStandardMaterial) {
    console.log('Color:', material.color);
    console.log('Roughness:', material.roughness);
    console.log('Metalness:', material.metalness);
    console.log('Opacity:', material.opacity);
  }
}
```

## Известные ограничения

1. Поддержка только одного типа материала на меш (или массива материалов)
2. Текстуры загружаются асинхронно, требуется обработка ошибок
3. Нет автоматической оптимизации текстур при загрузке
4. Поддержка только стандартных форматов текстур Three.js

## Связанные компоненты

- `NewUniversalModelLoader` - Загрузчик моделей с поддержкой материалов
- `ThreeDViewer` - Компонент просмотра 3D моделей
- `Scene3D` - Управление 3D сценой

## Дополнительные ресурсы

- [Three.js Materials Documentation](https://threejs.org/docs/#api/en/materials/Material)
- [PBR Material Guide](https://learnopengl.com/PBR/Theory)
- [Texture Optimization Guide](https://threejs.org/manual/#en/textures)

