# Формат TXT описания модели

## Обзор

Формат TXT описания модели используется в Pepakura Next для описания всего процесса создания бумажной модели:
- **AI-генерация** → создание 3D модели из текста
- **2D/3D модель** → работа с моделью
- **2D бумажная развёртка** → создание развёртки для печати

## Структура файла

Файл состоит из секций, каждая из которых описывает определённый этап процесса.

### Пример полного описания

```
# Описание модели Pepakura Next
# Версия формата: 1.0.0

[METADATA]
ID: 550e8400-e29b-41d4-a716-446655440000
Name: Лиса для детской поделки
Author: Иван Иванов
CreatedAt: 2026-02-06T10:00:00.000Z
UpdatedAt: 2026-02-06T11:30:00.000Z
ModelVersion: 1.0

[GENERATION]
Prompt: Простая лиса для детской поделки, мультяшный стиль
Mode: high_quality
Steps: 100
Resolution: 1024
Seed: 12345
Style: cartoon
Color: #FF8C00
Material: paper

[MODEL_3D]
Name: Лиса
Description: Мультяшная лиса для детской поделки
##Dimensions
Width: 150.0
Height: 200.0
Depth: 100.0
##Geometry
FaceCount: 1248
VertexCount: 625
Format: glb
ModelPath: /models/fox.glb
MaterialPath: /models/fox.mtl
TexturePaths: /textures/fox_diffuse.jpg, /textures/fox_normal.jpg

[UNFOLD]
##Parameters
MinSeamAngle: 75.0
MaxSeamLength: 10.0
AutoSeams: true
##Paper
PaperWidth: 210.0
PaperHeight: 297.0
MinGap: 5.0
##Tabs
MinTabWidth: 10.0
MaxAutoAngle: 90.0
##Margins
MarginsEnabled: true
MarginSize: 10.0

[OPTIMIZATION]
SheetCount: 3
UsagePercentage: 85.5
ModelArea: 0.125
UsedArea: 0.107
##AssemblyTips
Tip: Начните сборку с головы лисы
Tip: Используйте клей ПВА для склеивания
Tip: Следуйте нумерации на развёртке
```

## Секции

### [METADATA]

Метаданные проекта.

- `ID` - Уникальный идентификатор проекта (UUID)
- `Name` - Имя проекта
- `Author` - Автор проекта (опционально)
- `CreatedAt` - Дата создания (ISO 8601)
- `UpdatedAt` - Дата последнего обновления (ISO 8601)
- `ModelVersion` - Версия модели (опционально)

### [GENERATION]

Описание для AI-генерации модели.

- `Prompt` - Текстовое описание модели для AI
- `Mode` - Режим генерации: `preview` или `high_quality`
- `Steps` - Количество шагов генерации (число)
- `Resolution` - Разрешение модели (число)
- `Seed` - Сид для воспроизводимости (опционально, число)
- `Style` - Стиль модели: `realistic`, `cartoon`, `pixel_art`, `low_poly` (опционально)
- `Color` - Основной цвет в формате HEX (опционально)
- `Material` - Тип материала: `plastic`, `metal`, `wood`, `paper` (опционально)

### [MODEL_3D]

Описание 3D модели.

- `Name` - Имя модели
- `Description` - Описание модели

#### Подсекция ##Dimensions

- `Width` - Ширина модели в мм
- `Height` - Высота модели в мм
- `Depth` - Глубина модели в мм

#### Подсекция ##Geometry

- `FaceCount` - Количество граней
- `VertexCount` - Количество вершин
- `Format` - Формат файла: `obj`, `gltf`, `glb`, `fbx`, `stl`
- `ModelPath` - Путь к файлу модели
- `MaterialPath` - Путь к файлу материалов (опционально)
- `TexturePaths` - Пути к текстурам через запятую (опционально)

### [UNFOLD]

Описание параметров развёртки.

#### Подсекция ##Parameters

- `MinSeamAngle` - Минимальный угол для создания шва в градусах
- `MaxSeamLength` - Максимальная длина шва в мм
- `AutoSeams` - Автоматическое создание швов: `true` или `false`

#### Подсекция ##Paper

- `PaperWidth` - Ширина листа бумаги в мм
- `PaperHeight` - Высота листа бумаги в мм
- `MinGap` - Минимальный зазор между элементами в мм

#### Подсекция ##Tabs

- `MinTabWidth` - Минимальная ширина вкладыша в мм
- `MaxAutoAngle` - Максимальный угол для автовкладышей в градусах

#### Подсекция ##Margins

- `MarginsEnabled` - Добавить поля: `true` или `false`
- `MarginSize` - Размер полей в мм

### [OPTIMIZATION]

Результаты оптимизации для бумаги.

- `SheetCount` - Количество листов бумаги
- `UsagePercentage` - Процент использования бумаги
- `ModelArea` - Площадь модели в м²
- `UsedArea` - Использованная площадь в м²

#### Подсекция ##AssemblyTips

- `Tip` - Рекомендация по сборке (может быть несколько)

## Использование

### Создание описания из AI-генерации

```typescript
import { ModelDescriptionGenerator } from '@/modules/model-description/ModelDescriptionGenerator';
import { TextTo3dRequest, TextTo3dResponse } from '@/shared/types/textTo3d.types';

const request: TextTo3dRequest = {
  model_id: 'default',
  preset_id: 'default',
  mode: 'high_quality',
  prompt: 'Простая лиса для детской поделки',
  params: { steps: 100, resolution: 1024 }
};

const response: TextTo3dResponse = {
  artifacts: [
    { kind: 'mesh', format: 'glb', path: '/models/fox.glb' }
  ]
};

const description = ModelDescriptionGenerator.fromAIGeneration(
  request,
  response,
  'Лиса'
);

// Сохранение в файл
ModelDescription.saveToFile(description, 'fox-description.txt');
```

### Загрузка описания из файла

```typescript
import { ModelDescription } from '@/modules/model-description/ModelDescription';

const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement;
const file = fileInput.files[0];

const description = await ModelDescription.loadFromFile(file);
console.log('Загружено описание:', description);
```

### Генерация TXT из объекта

```typescript
import { ModelDescription } from '@/modules/model-description/ModelDescription';

const description: ModelProjectDescription = {
  version: '1.0.0',
  metadata: {
    id: '123',
    name: 'Test Model',
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  },
  // ... остальные данные
};

const txtContent = ModelDescription.generateTXT(description);
console.log(txtContent);
```

## Интеграция с процессом

### 1. AI-генерация

После генерации модели создаётся описание с секцией `[GENERATION]`:

```typescript
const description = ModelDescriptionGenerator.fromAIGeneration(
  generationRequest,
  generationResponse
);
```

### 2. Загрузка 3D модели

После загрузки модели обновляется секция `[MODEL_3D]`:

```typescript
const description = ModelDescriptionGenerator.fromThreeModel(
  threeModel,
  modelPath
);
```

### 3. Развёртка

После развёртки добавляется секция `[UNFOLD]`:

```typescript
const description = ModelDescriptionGenerator.addUnfoldInfo(
  description,
  {
    minSeamAngle: 75,
    maxSeamLength: 10,
    paperWidth: 210,
    paperHeight: 297
  }
);
```

### 4. Оптимизация

После оптимизации добавляется секция `[OPTIMIZATION]`:

```typescript
const description = ModelDescriptionGenerator.addOptimizationResults(
  description,
  {
    sheetCount: 3,
    usagePercentage: 85.5,
    modelArea: 0.125,
    usedArea: 0.107,
    assemblyTips: [
      'Начните сборку с головы',
      'Используйте клей ПВА'
    ]
  }
);
```

## Расширение формата

Формат легко расширяется добавлением новых секций:

```
[NEW_SECTION]
Key1: Value1
Key2: Value2
```

Парсер автоматически обработает новые секции, если они не требуют специальной логики.

## Совместимость

- Версия формата: 1.0.0
- Кодировка: UTF-8
- Разделитель строк: `\n` (LF) или `\r\n` (CRLF)
- Комментарии: строки, начинающиеся с `#`

## Примеры

Полные примеры описаний моделей можно найти в директории `examples/model-descriptions/`.
