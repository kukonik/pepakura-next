# 🚀 Стратегия развития Pepakura Next

**Версия**: 1.0  
**Дата**: 22 марта 2026 г.  
**Фокус**: Ручное и автоматическое редактирование, импорт/экспорт, сохранение файлов

---

## 📋 Содержание

1. [Обзор и видение](#обзор-и-видение)
2. [Архитектурные улучшения](#архитектурные-улучшения)
3. [Система импорта](#система-импорта)
4. [Система экспорта](#система-экспорта)
5. [Редактор развёрток](#редактор-развёрток)
6. [Автосохранение и персистентность](#автосохранение-и-персистентность)
7. [Автоматизация и AI](#автоматизация-и-ai)
8. [Дорожная карта](#дорожная-карта)

---

## 🎯 Обзор и видение

### Миссия
Создать **лучшее open-source приложение** для генерации развёрток бумажных моделей с интуитивным редактированием и умной автоматизацией.

### Целевые пользователи
1. **Любители papercraft** — хобби-моделисты
2. **Профессиональные дизайнеры** — создание упаковки, макетов
3. **Образовательные учреждения** — обучение 3D-моделированию
4. **Разработчики плагинов** — расширение функциональности

### Ключевые принципы
- 🔹 **Гибридное редактирование**: баланс между автоматикой и ручным контролем
- 🔹 **Непрерывное сохранение**: никакая работа не должна быть потеряна
- 🔹 **Универсальная совместимость**: поддержка всех популярных форматов
- 🔹 **Производительность**: работа с моделями до 100K вершин без лагов
- 🔹 **Расширяемость**: плагины для любых сценариев использования

---

## 🏗️ Архитектурные улучшения

### 1. Переход на workspace структуру

**Проблема**: Монолитная структура затрудняет навигацию и тестирование.

**Решение**:

```
pepakura-next/
├── crates/
│   ├── pepakura_core/        # Ядро (без изменений)
│   ├── pepakura_types/       # Общие типы (typeshare)
│   ├── pepakura_wasm/        # WASM для веба
│   └── pepakura_cli/         # CLI утилиты
├── apps/
│   ├── desktop/              # Tauri приложение
│   ├── web/                  # Веб-версия
│   └── mobile/               # Будущее мобильное приложение
├── packages/
│   ├── ui-kit/               # Общие Vue компоненты
│   └── utils/                # Общие утилиты
└── plugins/
    ├── official/             # Официальные плагины
    └── community/            # Плагины сообщества
```

**Преимущества**:
- Чёткое разделение ответственности
- Независимое тестирование компонентов
- Возможность переиспользования ядра в других проектах

### 2. Event-driven архитектура

**Новая система событий**:

```rust
// crates/pepakura_core/src/events/mod.rs

#[derive(Clone, Debug)]
pub enum AppEvent {
    // Импорт
    ImportStarted { path: PathBuf, format: ImportFormat },
    ImportProgress { current: u32, total: u32 },
    ImportCompleted { mesh_id: MeshId, info: MeshInfo },
    ImportFailed { error: String },
    
    // Развёртка
    UnfoldRequested { mesh_id: MeshId, config: UnfoldConfig },
    UnfoldProgress { percent: f32, iteration: usize },
    UnfoldCompleted { unfolded: UnfoldedMesh },
    UnfoldFailed { error: String },
    
    // Редактирование
    EditStarted { edit_type: EditType },
    EditApplied { changes: Vec<EditChange> },
    EditUndone { changes: Vec<EditChange> },
    
    // Экспорт
    ExportRequested { format: ExportFormat, path: PathBuf },
    ExportProgress { current: u32, total: u32 },
    ExportCompleted { path: PathBuf },
    ExportFailed { error: String },
    
    // Сохранение
    AutoSaveTriggered,
    SaveCompleted { path: PathBuf, timestamp: DateTime<Utc> },
    SaveFailed { error: String },
}

pub trait EventBus: Send + Sync {
    fn subscribe<F>(&self, handler: F) -> Subscription
    where
        F: Fn(&AppEvent) + Send + Sync + 'static;
    
    fn publish(&self, event: AppEvent);
}
```

**Преимущества**:
- Слабая связанность компонентов
- Легко добавлять новые функции
- Прозрачное логирование всех операций
- Поддержка undo/redo через историю событий

### 3. Генерация TypeScript типов из Rust

**typeshare конфигурация**:

```toml
# crates/pepakura_types/Cargo.toml
[dependencies]
typeshare = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

```rust
// crates/pepakura_types/src/lib.rs
use typeshare::typeshare;

#[typeshare(typescript)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshInfo {
    pub id: String,
    pub name: String,
    pub vertex_count: u32,
    pub face_count: u32,
    pub bounds: BoundingBox,
}

#[typeshare(typescript)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldedFace {
    pub face_id: String,
    pub vertices_2d: Vec<[f64; 2]>,
    pub center: [f64; 2],
    pub bounds: Rect2D,
}
```

**Автоматическая генерация**:
```bash
# scripts/generate-types.sh
typeshare crates/pepakura_types/src --output=ui-desktop/src/generated/
```

---

## 📥 Система импорта

### Текущие проблемы
- ⚠️ Базовая поддержка PDO (60%)
- ⚠️ Нет валидации файлов
- ⚠️ Нет прогресс-баров для больших файлов
- ⚠️ Ошибки импорта не информативны

### План улучшений

#### 1.1 Универсальный импортёр

**Структура**:

```rust
// crates/pepakura_core/src/import/mod.rs

pub trait ImportPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn supported_extensions(&self) -> &[&str];
    fn supported_mime_types(&self) -> &[&str];
    
    // Основная имплементация
    fn import(&self, path: &Path) -> Result<ImportResult, ImportError>;
    
    // Опционально: превью без полной загрузки
    fn preview(&self, path: &Path) -> Result<ImportPreview, ImportError> {
        Ok(ImportPreview::default())
    }
    
    // Опционально: валидация
    fn validate(&self, path: &Path) -> Result<ValidationReport, ImportError> {
        Ok(ValidationReport::valid())
    }
}

pub struct ImportResult {
    pub mesh: Mesh,
    pub metadata: ImportMetadata,
    pub warnings: Vec<ImportWarning>,
    pub textures: Vec<TextureInfo>,
}

pub struct ImportMetadata {
    pub source_path: PathBuf,
    pub format: String,
    pub file_size: u64,
    pub import_time: Duration,
    pub original_units: LengthUnit,
}
```

#### 1.2 Поддерживаемые форматы (приоритеты)

| Формат | Статус | Приорит | Описание |
|--------|--------|---------|----------|
| **PDO** | ⚠️ 60% | 🔴 High | Родной формат Pepakura Designer |
| **OBJ** | ✅ 80% | 🔴 High | Wavefront OBJ с MTL |
| **STL** | ✅ 70% | 🔴 High | Стереолитография (binary/ascii) |
| **PLY** | ✅ 50% | 🟡 Medium | Polygon File Format |
| **FBX** | ❌ 0% | 🟡 Medium | Autodesk FBX |
| **glTF/GLB** | ❌ 0% | 🟡 Medium | Modern 3D format |
| **3DS** | ❌ 0% | 🟢 Low | Legacy 3DS Max |
| **BLEND** | ❌ 0% | 🟢 Low | Blender native |
| **STEP/IGES** | ❌ 0% | 🟢 Low | CAD форматы |

#### 1.3 Валидация при импорте

```rust
// crates/pepakura_core/src/import/validator.rs

pub struct ImportValidator;

impl ImportValidator {
    pub fn validate(mesh: &Mesh) -> ValidationResult {
        let mut report = ValidationResult::new();
        
        // Проверка на замкнутость (manifold)
        if !Self::is_manifold(mesh) {
            report.add_error(ValidationError::NonManifold);
        }
        
        // Проверка на самопересечения
        if Self::has_self_intersections(mesh) {
            report.add_warning(ValidationWarning::SelfIntersections);
        }
        
        // Проверка нормалей
        if !Self::has_consistent_normals(mesh) {
            report.add_warning(ValidationWarning::InconsistentNormals);
        }
        
        // Проверка размера
        let file_size = std::fs::metadata(&mesh.source_path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        if file_size > 100 * 1024 * 1024 {
            report.add_warning(ValidationWarning::LargeFile(file_size));
        }
        
        // Проверка количества вершин
        if mesh.vertices.len() > 100_000 {
            report.add_warning(ValidationWarning::HighPoly(mesh.vertices.len()));
        }
        
        report
    }
}
```

#### 1.4 Прогресс импорта

```rust
// crates/pepakura_core/src/import/progress.rs

pub trait ImportProgress: Send + Sync {
    fn report_progress(&self, current: u64, total: u64, stage: ImportStage);
}

pub enum ImportStage {
    ReadingFile,
    ParsingGeometry,
    ParsingMaterials,
    ParsingTextures,
    BuildingMesh,
    Validating,
}

// Пример использования
pub async fn import_with_progress<P: ImportProgress>(
    path: &Path,
    format: ImportFormat,
    progress: P,
) -> Result<ImportResult, ImportError> {
    let file_size = std::fs::metadata(path)?.len();
    progress.report_progress(0, file_size, ImportStage::ReadingFile);
    
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    
    // Чтение с прогрессом
    let mut total_read = 0;
    while let Ok(chunk) = file.read_to_end(&mut buffer).await {
        if chunk == 0 { break; }
        total_read += chunk;
        progress.report_progress(total_read, file_size, ImportStage::ReadingFile);
    }
    
    // Парсинг с прогрессом
    progress.report_progress(0, buffer.len() as u64, ImportStage::ParsingGeometry);
    let mesh = parse_format(&buffer, format)?;
    
    Ok(ImportResult::from(mesh))
}
```

#### 1.5 Пакетный импорт

```typescript
// ui-desktop/src/composables/useBatchImport.ts

export async function useBatchImport() {
  const { invoke } = useTauri()
  
  const importBatch = async (paths: string[], options: BatchImportOptions) => {
    const results = await Promise.allSettled(
      paths.map(path => invoke<ImportResult>('import_3d_model', { path }))
    )
    
    const successful = results
      .filter((r): r is PromiseFulfilledResult<ImportResult> => r.status === 'fulfilled')
      .map(r => r.value)
    
    const failed = results
      .filter((r): r is PromiseRejectedResult => r.status === 'rejected')
      .map(r => r.reason)
    
    return { successful, failed }
  }
  
  return { importBatch }
}
```

---

## 📤 Система экспорта

### Текущие проблемы
- ⚠️ Только SVG экспорт
- ⚠️ PDF через browser print (костыль)
- ⚠️ Нет оптимизации выходных файлов
- ⚠️ Нет предпросмотра перед экспортом

### План улучшений

#### 2.1 Нативный PDF экспорт

```rust
// crates/pepakura_core/src/export/pdf.rs

use printpdf::*;

pub struct PdfExporter {
    config: PdfConfig,
}

pub struct PdfConfig {
    pub page_size: PageSize,
    pub orientation: Orientation,
    pub layers: PdfLayers,
    pub include_metadata: bool,
    pub compression: CompressionLevel,
}

pub struct PdfLayers {
    pub cut_lines: bool,      // Линии реза
    pub fold_lines: bool,     // Линии сгиба
    pub glue_tabs: bool,      // Клапаны для склейки
    pub part_numbers: bool,   // Номера деталей
    pub fold_marks: bool,     // Маркеры сгиба
    pub grid: bool,           // Сетка
}

impl PdfExporter {
    pub fn export(
        &self,
        unfolded: &UnfoldedMesh,
        config: &PdfConfig,
    ) -> Result<Vec<u8>, ExportError> {
        let (width, height) = config.page_size.size_mm();
        
        // Создаём PDF документ
        let mut doc = PdfDocument::new(
            "Pepakura Next Export",
            Mm(width),
            Mm(height),
            "Layer 1",
        );
        
        let page = doc.get_page(0);
        let mut layer = page.get_layer("Cut Lines");
        
        // Добавляем линии реза (красные, сплошные)
        for face in &unfolded.faces {
            let path = self.create_face_path(face, unfolded.vertices_2d);
            let outline = LineOutline {
                line_width: Mm(0.2),
                dash_pattern: None,
                color: Rgb(1.0, 0.0, 0.0),
            };
            layer.add_shape(path, outline);
        }
        
        // Добавляем линии сгиба (синие, пунктирные)
        if config.layers.fold_lines {
            let mut fold_layer = page.add_layer("Fold Lines");
            // ... добавление линий сгиба
        }
        
        // Добавляем номера деталей
        if config.layers.part_numbers {
            let mut text_layer = page.add_layer("Part Numbers");
            // ... добавление текста
        }
        
        // Сериализуем в bytes
        let mut bytes = Vec::new();
        doc.save_to(&mut bytes)?;
        
        Ok(bytes)
    }
}
```

**Зависимости**:
```toml
# crates/pepakura_core/Cargo.toml
[dependencies]
printpdf = "0.5"
lopdf = "0.31"  # Альтернатива для сложных PDF
```

#### 2.2 DXF экспорт (для лазерной резки)

```rust
// crates/pepakura_core/src/export/dxf.rs

use dxf::entities::*;
use dxf::Drawing;

pub struct DxfExporter;

impl DxfExporter {
    pub fn export(
        unfolded: &UnfoldedMesh,
        config: &DxfConfig,
    ) -> Result<String, ExportError> {
        let mut drawing = Drawing::new();
        
        // Создаём слои
        let mut cut_layer = Layer::new("CUT_LINES");
        cut_layer.color = Color::Red;
        drawing.layers.insert("CUT_LINES".to_string(), cut_layer);
        
        let mut fold_layer = Layer::new("FOLD_LINES");
        fold_layer.color = Color::Blue;
        drawing.layers.insert("FOLD_LINES".to_string(), fold_layer);
        
        // Добавляем грани
        for (i, face) in unfolded.faces.iter().enumerate() {
            let vertices: Vec<Point3> = face.vertices
                .iter()
                .map(|&v| {
                    let [x, y] = unfolded.vertices_2d[v];
                    Point3::new(x, y, 0.0)
                })
                .collect();
            
            // Полилиния для грани
            let polyline = Polyline {
                vertices: vertices.iter().map(|p| Vertex::from_point(*p)).collect(),
                is_closed: true,
                layer: "CUT_LINES".to_string(),
                ..Default::default()
            };
            
            drawing.entities.push(Entity::Polyline(polyline));
        }
        
        // Сериализуем в строку
        let mut output = String::new();
        drawing.write(&mut output)?;
        
        Ok(output)
    }
}
```

#### 2.3 PNG/TIFF экспорт (растровый)

```rust
// crates/pepakura_core/src/export/raster.rs

use image::{ImageBuffer, Rgba, DynamicImage};

pub struct RasterExporter {
    dpi: u32,
    antialias: bool,
}

pub struct RasterConfig {
    pub format: RasterFormat,  // PNG, TIFF, JPEG
    pub width_px: u32,
    pub height_px: u32,
    pub dpi: u32,
    pub background: RgbaColor,
    pub antialias: bool,
}

impl RasterExporter {
    pub fn export(
        &self,
        unfolded: &UnfoldedMesh,
        config: &RasterConfig,
    ) -> Result<Vec<u8>, ExportError> {
        // Создаём буфер изображения
        let mut img = ImageBuffer::new(config.width_px, config.height_px);
        
        // Заполняем фон
        img.fill(config.background);
        
        // Рендерим грани
        for face in &unfolded.faces {
            self.render_face(&mut img, face, unfolded.vertices_2d, config);
        }
        
        // Конвертируем в нужный формат
        let dynamic = DynamicImage::ImageRgba8(img);
        let mut bytes = Vec::new();
        
        match config.format {
            RasterFormat::Png => {
                dynamic.write_to(&mut Cursor::new(bytes), ImageFormat::Png)?;
            }
            RasterFormat::Tiff => {
                dynamic.write_to(&mut Cursor::new(bytes), ImageFormat::Tiff)?;
            }
            RasterFormat::Jpeg => {
                dynamic.write_to(&mut Cursor::new(bytes), ImageFormat::Jpeg)?;
            }
        }
        
        Ok(bytes)
    }
}
```

#### 2.4 Оптимизация SVG

```rust
// crates/pepakura_core/src/export/svg_optimizer.rs

pub struct SvgOptimizer {
    precision: u8,
    remove_metadata: bool,
    minify: bool,
}

impl SvgOptimizer {
    pub fn optimize(&self, svg: &str) -> String {
        let mut result = svg.to_string();
        
        // Удаляем метаданные
        if self.remove_metadata {
            result = regex::Regex::new(r"<!--.*?-->")
                .unwrap()
                .replace_all(&result, "")
                .to_string();
        }
        
        // Округляем координаты
        result = self.round_coordinates(&result);
        
        // Удаляем лишние пробелы
        if self.minify {
            result = self.minify(&result);
        }
        
        // Объединяем пути с одинаковыми стилями
        result = self.merge_paths(&result);
        
        result
    }
    
    fn round_coordinates(&self, svg: &str) -> String {
        let precision = self.precision as i32;
        let multiplier = 10f64.powi(precision);
        
        regex::Regex::new(r"(\d+\.\d+)")
            .unwrap()
            .replace_all(svg, |caps: &regex::Captures| {
                let num: f64 = caps[1].parse().unwrap();
                format!("{:.precision$}", (num * multiplier).round() / multiplier, precision = precision)
            })
            .to_string()
    }
}
```

#### 2.5 Предэкспортный превью

```vue
<!-- ui-desktop/src/components/export/ExportPreview.vue -->

<template>
  <div class="export-preview">
    <div class="preview-header">
      <h3>Предпросмотр экспорта</h3>
      <div class="preview-controls">
        <button @click="zoomIn">+</button>
        <button @click="zoomOut">-</button>
        <span>{{ zoom }}%</span>
      </div>
    </div>
    
    <div class="preview-content">
      <div class="page-simulation" :style="pageStyle">
        <svg ref="svgRef" :width="svgWidth" :height="svgHeight">
          <!-- Рендер превью -->
        </svg>
        
        <div class="page-info">
          <span>{{ pageSize }}</span>
          <span>{{ orientation }}</span>
          <span>{{ scale }} мм/ед</span>
        </div>
      </div>
    </div>
    
    <div class="preview-footer">
      <div class="export-stats">
        <span>Страниц: {{ pageCount }}</span>
        <span>Деталей: {{ partCount }}</span>
        <span>Размер файла: ~{{ estimatedSize }}</span>
      </div>
      
      <div class="export-actions">
        <button @click="export" :disabled="!isValid">
          Экспортировать
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  unfolded: UnfoldedMesh
  format: ExportFormat
  config: ExportConfig
}>()

const zoom = ref(100)
const svgRef = ref<SVGSVGElement>()

const pageStyle = computed(() => ({
  width: `${props.config.pageWidth}mm`,
  height: `${props.config.pageHeight}mm`,
  transform: `scale(${zoom.value / 100})`
}))

const isValid = computed(() => {
  // Проверка валидности конфигурации экспорта
  return true
})

const export = async () => {
  await invoke('export_unfold', {
    format: props.format,
    config: props.config
  })
}
</script>
```

---

## ✏️ Редактор развёрток

### Текущие проблемы
- ⚠️ Нет перемещения деталей
- ⚠️ Нет выравнивания
- ⚠️ Нет snap to grid
- ⚠️ Нет привязки 3D ↔ 2D

### План улучшений

#### 3.1 Интерактивное перемещение

```vue
<!-- ui-desktop/src/components/editor/UnfoldEditor.vue -->

<template>
  <div class="unfold-editor" @keydown="handleKeyDown" @keyup="handleKeyUp">
    <!-- Toolbar -->
    <EditorToolbar
      :active-tool="activeTool"
      @select-tool="activeTool = $event"
      @undo="undo"
      @redo="redo"
    />
    
    <!-- Canvas -->
    <div
      ref="canvasRef"
      class="editor-canvas"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
      @wheel="handleWheel"
    >
      <svg :width="canvasWidth" :height="canvasHeight">
        <!-- Grid -->
        <GridPattern
          v-if="showGrid"
          :cell-size="gridSize"
          :snap-enabled="snapToGrid"
        />
        
        <!-- Parts -->
        <g v-for="part in parts" :key="part.id">
          <UnfoldedPart
            :part="part"
            :selected="selectedPartIds.includes(part.id)"
            :highlighted="highlightedPartIds.includes(part.id)"
            @select="selectPart(part.id)"
            @deselect="deselectPart(part.id)"
            @start-drag="startDrag(part.id, $event)"
          />
        </g>
        
        <!-- Selection box -->
        <SelectionBox
          v-if="isSelecting"
          :start="selectionStart"
          :end="selectionEnd"
        />
        
        <!-- Measurement overlay -->
        <MeasurementOverlay
          v-if="isMeasuring"
          :start="measureStart"
          :end="measureEnd"
        />
      </svg>
    </div>
    
    <!-- Properties panel -->
    <PropertiesPanel
      v-if="selectedPartIds.length === 1"
      :part="selectedPart"
      @update="updatePart"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useUndoRedo } from '@/composables/useUndoRedo'
import { useSnap } from '@/composables/useSnap'

const props = defineProps<{
  unfolded: UnfoldedMesh
}>()

const emit = defineEmits<{
  update: [unfolded: UnfoldedMesh]
}>()

// State
const activeTool = ref<'select' | 'move' | 'rotate' | 'scale' | 'measure'>('select')
const selectedPartIds = ref<string[]>([])
const highlightedPartIds = ref<string[]>([])
const showGrid = ref(true)
const gridSize = ref(10)
const snapToGrid = ref(true)

// Drag & drop
const isDragging = ref(false)
const dragStart = ref<{ x: number, y: number } | null>(null)
const dragOffset = ref<{ x: number, y: number }>({ x: 0, y: 0 })

// Undo/Redo
const { undo, redo, pushState } = useUndoRedo()

// Snap utilities
const { snapToGridPoint, snapToOtherParts } = useSnap()

const selectPart = (partId: string) => {
  if (activeTool.value !== 'select') return
  
  if (!selectedPartIds.value.includes(partId)) {
    selectedPartIds.value = [partId]
  }
}

const deselectPart = (partId: string) => {
  selectedPartIds.value = selectedPartIds.value.filter(id => id !== partId)
}

const startDrag = (partId: string, event: MouseEvent) => {
  if (activeTool.value !== 'move') return
  
  isDragging.value = true
  dragStart.value = { x: event.clientX, y: event.clientY }
  
  // Сохраняем состояние для undo
  pushState({
    type: 'part_move_start',
    partIds: [partId]
  })
}

const handleMouseMove = (event: MouseEvent) => {
  if (!isDragging.value || !dragStart.value) return
  
  const dx = event.clientX - dragStart.value.x
  const dy = event.clientY - dragStart.value.y
  
  // Перемещаем выбранные части
  for (const partId of selectedPartIds.value) {
    const part = getPart(partId)
    const snappedDx = snapToGrid ? snapToGridPoint(dx, gridSize.value) : dx
    const snappedDy = snapToGrid ? snapToGridPoint(dy, gridSize.value) : dy
    
    part.x += snappedDx
    part.y += snappedDy
    
    // Проверяем коллизии с другими частями
    if (snapToOtherParts) {
      const snapped = snapToOtherParts(part, allParts.value)
      if (snapped) {
        part.x = snapped.x
        part.y = snapped.y
      }
    }
  }
  
  emit('update', { ...props.unfolded })
}

const handleMouseUp = () => {
  if (!isDragging.value) return
  
  isDragging.value = false
  dragStart.value = null
  
  // Фиксируем состояние для undo
  pushState({
    type: 'part_move_end',
    changes: getMoveChanges()
  })
}

// Вращение
const rotatePart = (partId: string, angle: number) => {
  const part = getPart(partId)
  const center = getPartCenter(part)
  
  // Поворот вокруг центра
  part.vertices_2d = part.vertices_2d.map(v => 
    rotatePoint(v, center, angle)
  )
  part.rotation += angle
  
  emit('update', { ...props.unfolded })
}

// Выравнивание
const alignParts = (alignment: 'left' | 'right' | 'top' | 'bottom' | 'center' | 'middle') => {
  const selected = selectedPartIds.value.map(id => getPart(id))
  if (selected.length < 2) return
  
  const bounds = getCombinedBounds(selected)
  
  for (const part of selected) {
    switch (alignment) {
      case 'left':
        part.x = bounds.left
        break
      case 'right':
        part.x = bounds.right - part.width
        break
      case 'top':
        part.y = bounds.top
        break
      case 'bottom':
        part.y = bounds.bottom - part.height
        break
      case 'center':
        part.x = bounds.left + (bounds.width - part.width) / 2
        break
      case 'middle':
        part.y = bounds.top + (bounds.height - part.height) / 2
        break
    }
  }
  
  emit('update', { ...props.unfolded })
}
</script>
```

#### 3.2 Привязка 3D ↔ 2D

```typescript
// ui-desktop/src/composables/use3d2dLink.ts

export function use3d2dLink() {
  const { invoke } = useTauri()
  
  // Выделение грани в 3D → подсветка в 2D
  const highlightFace3d = async (faceIndex: number) => {
    const unfoldedFace = await invoke<UnfoldedFace>('get_unfolded_face', { faceIndex })
    
    // Подсвечиваем соответствующую часть в 2D
    emit('highlight-part', unfoldedFace.part_id)
  }
  
  // Клик в 2D → выделение в 3D
  const selectFace2d = async (partId: string) => {
    const faceIndex = await invoke<number>('get_face_index_by_part', { partId })
    
    // Выделяем грань в 3D вьювере
    emit('select-face-3d', faceIndex)
  }
  
  // Синхронизация камеры
  const syncCamera = async (view3d: Camera, view2d: Camera2D) => {
    // Проекция 3D камеры на 2D плоскость
    const projection = await invoke<Camera2D>('project_camera_to_2d', {
      camera: view3d
    })
    
    // Обновляем 2D камеру
    view2d.copyFrom(projection)
  }
  
  return {
    highlightFace3d,
    selectFace2d,
    syncCamera
  }
}
```

#### 3.3 Система измерений

```vue
<!-- ui-desktop/src/components/editor/MeasurementTool.vue -->

<template>
  <div class="measurement-tool">
    <button
      :class="{ active: isActive }"
      @click="toggleMeasurement"
      title="Измерить расстояние (M)"
    >
      <i class="icon-ruler"></i>
    </button>
    
    <div v-if="isActive" class="measurement-overlay">
      <svg>
        <line
          :x1="startPoint.x"
          :y1="startPoint.y"
          :x2="currentPoint.x"
          :y2="currentPoint.y"
          class="measurement-line"
        />
        <text
          :x="(startPoint.x + currentPoint.x) / 2"
          :y="(startPoint.y + currentPoint.y) / 2"
          class="measurement-label"
        >
          {{ distance }} мм
        </text>
      </svg>
      
      <div class="measurement-info">
        <span>ΔX: {{ deltaX }} мм</span>
        <span>ΔY: {{ deltaY }} мм</span>
        <span>Угол: {{ angle }}°</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const isActive = ref(false)
const startPoint = ref({ x: 0, y: 0 })
const currentPoint = ref({ x: 0, y: 0 })

const toggleMeasurement = () => {
  isActive.value = !isActive.value
}

const distance = computed(() => {
  const dx = currentPoint.value.x - startPoint.value.x
  const dy = currentPoint.value.y - startPoint.value.y
  return Math.sqrt(dx * dx + dy * dy).toFixed(2)
})

const deltaX = computed(() => 
  (currentPoint.value.x - startPoint.value.x).toFixed(2)
)

const deltaY = computed(() => 
  (currentPoint.value.y - startPoint.value.y).toFixed(2)
)

const angle = computed(() => {
  const dx = currentPoint.value.x - startPoint.value.x
  const dy = currentPoint.value.y - startPoint.value.y
  return (Math.atan2(dy, dx) * 180 / Math.PI).toFixed(1)
})
</script>
```

#### 3.4 Редактирование клапанов

```vue
<!-- ui-desktop/src/components/editor/GlueTabEditor.vue -->

<template>
  <div class="glue-tab-editor">
    <div class="tab-toolbar">
      <button @click="addTab" title="Добавить клапан">
        <i class="icon-plus"></i> Добавить
      </button>
      <button @click="removeSelectedTab" title="Удалить выбранный">
        <i class="icon-trash"></i> Удалить
      </button>
    </div>
    
    <svg class="tab-canvas">
      <g v-for="tab in glueTabs" :key="tab.id">
        <!-- Контур клапана -->
        <path
          :d="tab.path"
          :class="['glue-tab', { selected: selectedTabId === tab.id }]"
          @click="selectTab(tab.id)"
        />
        
        <!-- Точки редактирования -->
        <g v-if="selectedTabId === tab.id">
          <circle
            v-for="(point, index) in tab.controlPoints"
            :key="index"
            :cx="point.x"
            :cy="point.y"
            r="5"
            class="control-point"
            @mousedown="startDragPoint(tab.id, index)"
          />
        </g>
        
        <!-- Размерные метки -->
        <text :x="tab.center.x" :y="tab.center.y">
          {{ tab.width }}×{{ tab.height }} мм
        </text>
      </g>
    </svg>
    
    <div class="tab-properties">
      <label>
        Тип клапана:
        <select v-model="selectedTab.type">
          <option value="rectangular">Прямоугольный</option>
          <option value="trapezoidal">Трапециевидный</option>
          <option value="custom">Произвольный</option>
        </select>
      </label>
      
      <label>
        Ширина:
        <input type="number" v-model.number="selectedTab.width" />
      </label>
      
      <label>
        Высота:
        <input type="number" v-model.number="selectedTab.height" />
      </label>
      
      <label>
        Угол наклона:
        <input type="range" v-model.number="selectedTab.angle" min="0" max="180" />
      </label>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface GlueTab {
  id: string
  path: string
  controlPoints: { x: number, y: number }[]
  width: number
  height: number
  angle: number
  type: 'rectangular' | 'trapezoidal' | 'custom'
  center: { x: number, y: number }
}

const glueTabs = ref<GlueTab[]>([])
const selectedTabId = ref<string | null>(null)

const selectedTab = computed(() => 
  glueTabs.value.find(t => t.id === selectedTabId.value)
)

const addTab = () => {
  const newTab: GlueTab = {
    id: generateId(),
    path: '',
    controlPoints: [],
    width: 15,
    height: 10,
    angle: 90,
    type: 'rectangular',
    center: { x: 0, y: 0 }
  }
  glueTabs.value.push(newTab)
  selectedTabId.value = newTab.id
}

const removeSelectedTab = () => {
  if (!selectedTabId.value) return
  glueTabs.value = glueTabs.value.filter(t => t.id !== selectedTabId.value)
  selectedTabId.value = null
}
</script>
```

---

## 💾 Автосохранение и персистентность

### Текущие проблемы
- ⚠️ Нет персистентности (сброс при рестарте)
- ⚠️ Автосохранение базовое (30 сек)
- ⚠️ Нет истории версий
- ⚠️ Нет восстановления после краша

### План улучшений

#### 4.1 SQLite хранилище состояния

```rust
// src-tauri/src/state/persistence.rs

use rusqlite::{Connection, Result};
use serde_json;
use chrono::{DateTime, Utc};

pub struct StatePersistence {
    conn: Connection,
    auto_save_interval: Duration,
    last_save: Option<DateTime<Utc>>,
}

impl StatePersistence {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Создаём таблицы
        conn.execute(
            "CREATE TABLE IF NOT EXISTS project_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS undo_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id TEXT NOT NULL,
                action TEXT NOT NULL,
                state_before TEXT NOT NULL,
                state_after TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS auto_save_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id TEXT NOT NULL,
                save_time TEXT NOT NULL,
                success BOOLEAN NOT NULL,
                error_message TEXT
            )",
            [],
        )?;
        
        Ok(Self {
            conn,
            auto_save_interval: Duration::from_secs(30),
            last_save: None,
        })
    }
    
    pub fn save_state<T: serde::Serialize>(
        &self,
        key: &str,
        value: &T,
    ) -> Result<()> {
        let value_json = serde_json::to_string(value)?;
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT OR REPLACE INTO project_state (key, value, updated_at) 
             VALUES (?1, ?2, ?3)",
            [key, &value_json, &now],
        )?;
        
        self.last_save = Some(Utc::now());
        
        Ok(())
    }
    
    pub fn load_state<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>> {
        let value: String = self.conn.query_row(
            "SELECT value FROM project_state WHERE key = ?1",
            [key],
            |row| row.get(0),
        )?;
        
        Ok(serde_json::from_str(&value).ok())
    }
    
    pub fn push_undo_action(
        &self,
        project_id: &str,
        action: &str,
        state_before: &serde_json::Value,
        state_after: &serde_json::Value,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO undo_history 
             (project_id, action, state_before, state_after, timestamp) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                project_id,
                action,
                &state_before.to_string(),
                &state_after.to_string(),
                &now,
            ],
        )?;
        
        // Очищаем старую историю (>100 записей)
        self.conn.execute(
            "DELETE FROM undo_history 
             WHERE id NOT IN (
                 SELECT id FROM undo_history 
                 WHERE project_id = ?1 
                 ORDER BY id DESC LIMIT 100
             )",
            [project_id],
        )?;
        
        Ok(())
    }
    
    pub fn get_last_undo(
        &self,
        project_id: &str,
    ) -> Result<Option<UndoAction>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, action, state_before, state_after, timestamp 
             FROM undo_history 
             WHERE project_id = ?1 
             ORDER BY id DESC LIMIT 1",
        )?;
        
        let result = stmt.query_row([project_id], |row| {
            Ok(UndoAction {
                id: row.get(0)?,
                action: row.get(1)?,
                state_before: row.get(2)?,
                state_after: row.get(3)?,
                timestamp: row.get(4)?,
            })
        });
        
        match result {
            Ok(action) => Ok(Some(action)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    pub fn log_auto_save(
        &self,
        project_id: &str,
        success: bool,
        error_message: Option<&str>,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO auto_save_log (project_id, save_time, success, error_message) 
             VALUES (?1, ?2, ?3, ?4)",
            [
                project_id,
                &now,
                &success,
                &error_message.unwrap_or(""),
            ],
        )?;
        
        Ok(())
    }
    
    pub fn recover_from_crash(&self) -> Result<Vec<RecoveryData>> {
        let mut stmt = self.conn.prepare(
            "SELECT key, value, updated_at FROM project_state 
             ORDER BY updated_at DESC",
        )?;
        
        let recovery_data = stmt
            .query_map([], |row| {
                Ok(RecoveryData {
                    key: row.get(0)?,
                    value: row.get(1)?,
                    updated_at: row.get(2)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(recovery_data)
    }
}

pub struct UndoAction {
    pub id: i64,
    pub action: String,
    pub state_before: serde_json::Value,
    pub state_after: serde_json::Value,
    pub timestamp: String,
}

pub struct RecoveryData {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}
```

#### 4.2 Умное автосохранение

```typescript
// ui-desktop/src/stores/autoSaveStore.ts

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useAutoSaveStore = defineStore('autoSave', () => {
  const isEnabled = ref(true)
  const intervalSeconds = ref(30)
  const lastSaveAt = ref<Date | null>(null)
  const nextSaveAt = ref<Date | null>(null)
  const saveCount = ref(0)
  const failedSaves = ref(0)
  const isSaving = ref(false)
  
  let timer: ReturnType<typeof setInterval> | null = null
  
  // Адаптивный интервал (уменьшается при частых изменениях)
  const adaptiveInterval = computed(() => {
    if (saveCount.value < 5) return 15000 // 15 сек в начале
    if (failedSaves.value > 0) return 60000 // 1 мин при ошибках
    return intervalSeconds.value * 1000
  })
  
  const start = (save: () => Promise<void>) => {
    if (timer || !isEnabled.value) return
    
    const tick = async () => {
      if (isSaving.value) return
      
      isSaving.value = true
      try {
        await save()
        lastSaveAt.value = new Date()
        nextSaveAt.value = new Date(Date.now() + adaptiveInterval.value)
        saveCount.value++
        failedSaves.value = 0
        
        // Логирование успешного сохранения
        await invoke('log_auto_save', {
          projectId: getCurrentProjectId(),
          success: true
        })
      } catch (e) {
        console.error('[AutoSave] error', e)
        failedSaves.value++
        
        // Логирование ошибки
        await invoke('log_auto_save', {
          projectId: getCurrentProjectId(),
          success: false,
          errorMessage: e instanceof Error ? e.message : String(e)
        })
      } finally {
        isSaving.value = false
      }
    }
    
    // Первое сохранение через 5 секунд
    setTimeout(() => {
      tick()
      timer = setInterval(tick, adaptiveInterval.value)
    }, 5000)
  }
  
  const stop = () => {
    if (!timer) return
    clearInterval(timer)
    timer = null
  }
  
  const forceSave = async (save: () => Promise<void>) => {
    if (isSaving.value) return
    await save()
    lastSaveAt.value = new Date()
    saveCount.value++
  }
  
  // Восстановление после краша
  const recoverFromCrash = async () => {
    try {
      const recoveryData = await invoke<RecoveryData[]>('recover_from_crash')
      
      if (recoveryData.length > 0) {
        // Показываем диалог восстановления
        showRecoveryDialog(recoveryData)
      }
    } catch (e) {
      console.error('[Recovery] error', e)
    }
  }
  
  return {
    isEnabled,
    intervalSeconds,
    lastSaveAt,
    nextSaveAt,
    saveCount,
    failedSaves,
    isSaving,
    start,
    stop,
    forceSave,
    recoverFromCrash
  }
})
```

#### 4.3 История версий проекта

```vue
<!-- ui-desktop/src/components/project/VersionHistory.vue -->

<template>
  <div class="version-history">
    <h3>История версий</h3>
    
    <div class="version-timeline">
      <div
        v-for="version in versions"
        :key="version.id"
        :class="['version-item', { selected: selectedVersionId === version.id }]"
        @click="selectVersion(version.id)"
      >
        <div class="version-header">
          <span class="version-name">{{ version.name }}</span>
          <span class="version-date">{{ formatDate(version.timestamp) }}</span>
        </div>
        
        <div class="version-info">
          <span class="version-action">{{ version.action }}</span>
          <span class="version-size">{{ formatSize(version.stateSize) }}</span>
        </div>
        
        <div class="version-actions">
          <button @click.stop="restoreVersion(version.id)" title="Восстановить">
            <i class="icon-restore"></i>
          </button>
          <button @click.stop="deleteVersion(version.id)" title="Удалить">
            <i class="icon-trash"></i>
          </button>
        </div>
      </div>
    </div>
    
    <div class="version-controls">
      <button @click="createManualVersion" :disabled="isCreating">
        {{ isCreating ? 'Создание...' : 'Создать версию' }}
      </button>
      
      <button @click="exportVersions" title="Экспорт истории">
        <i class="icon-download"></i> Экспорт
      </button>
      
      <button @click="clearHistory" class="danger" title="Очистить историю">
        <i class="icon-trash"></i> Очистить всё
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface ProjectVersion {
  id: number
  name: string
  timestamp: string
  action: string
  stateSize: number
  stateBefore: string
  stateAfter: string
}

const versions = ref<ProjectVersion[]>([])
const selectedVersionId = ref<number | null>(null)
const isCreating = ref(false)

onMounted(async () => {
  versions.value = await invoke<ProjectVersion[]>('get_version_history', {
    projectId: getCurrentProjectId()
  })
})

const selectVersion = (id: number) => {
  selectedVersionId.value = id
}

const restoreVersion = async (id: number) => {
  const confirmed = await showConfirmDialog(
    'Вы уверены? Текущее состояние будет потеряно.'
  )
  
  if (!confirmed) return
  
  await invoke('restore_version', {
    projectId: getCurrentProjectId(),
    versionId: id
  })
  
  // Перезагружаем проект
  await reloadProject()
}

const createManualVersion = async () => {
  isCreating.value = true
  
  try {
    await invoke('create_manual_version', {
      projectId: getCurrentProjectId(),
      name: `Версия ${versions.value.length + 1}`,
      action: 'Ручное сохранение'
    })
    
    // Обновляем список
    versions.value = await invoke('get_version_history', {
      projectId: getCurrentProjectId()
    })
  } finally {
    isCreating.value = false
  }
}

const deleteVersion = async (id: number) => {
  await invoke('delete_version', { versionId: id })
  versions.value = versions.value.filter(v => v.id !== id)
}

const clearHistory = async () => {
  const confirmed = await showConfirmDialog(
    'Это действие необратимо. Удалить всю историю версий?'
  )
  
  if (!confirmed) return
  
  await invoke('clear_version_history', {
    projectId: getCurrentProjectId()
  })
  
  versions.value = []
}
</script>
```

#### 4.4 Экспорт/Импорт проекта

```rust
// src-tauri/src/commands/project.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use zip::write::ZipWriter;
use zip::ZipArchive;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectExportData {
    pub version: String,
    pub export_date: String,
    pub project: ProjectState,
    pub unfolded: Option<UnfoldedMesh>,
    pub settings: ProjectSettings,
    pub history: Vec<UndoAction>,
    pub assets: Vec<AssetInfo>,
}

#[tauri::command]
pub async fn export_project(
    project_id: String,
    output_path: PathBuf,
) -> Result<ExportResult, String> {
    // Собираем все данные проекта
    let export_data = collect_project_data(&project_id)?;
    
    // Создаём ZIP архив
    let file = File::create(&output_path)
        .map_err(|e| format!("Ошибка создания файла: {}", e))?;
    
    let mut zip = ZipWriter::new(file);
    
    // Добавляем метаданные
    zip.start_file("metadata.json", zip::write::FileOptions::default())?;
    let metadata = ProjectMetadata {
        version: export_data.version.clone(),
        export_date: export_data.export_date.clone(),
        project_name: export_data.project.name.clone(),
    };
    zip.write_all(serde_json::to_string_pretty(&metadata)?.as_bytes())?;
    
    // Добавляем состояние проекта
    zip.start_file("project_state.json", zip::write::FileOptions::default())?;
    zip.write_all(serde_json::to_string_pretty(&export_data.project)?.as_bytes())?;
    
    // Добавляем развёртку
    if let Some(unfolded) = &export_data.unfolded {
        zip.start_file("unfolded.json", zip::write::FileOptions::default())?;
        zip.write_all(serde_json::to_string_pretty(unfolded)?.as_bytes())?;
    }
    
    // Добавляем настройки
    zip.start_file("settings.json", zip::write::FileOptions::default())?;
    zip.write_all(serde_json::to_string_pretty(&export_data.settings)?.as_bytes())?;
    
    // Добавляем историю
    zip.start_file("history.json", zip::write::FileOptions::default())?;
    zip.write_all(serde_json::to_string_pretty(&export_data.history)?.as_bytes())?;
    
    // Добавляем ассеты (текстуры, изображения)
    let mut assets_dir = PathBuf::from("assets");
    for asset in &export_data.assets {
        assets_dir.push(&asset.filename);
        zip.start_file(assets_dir.to_string_lossy(), zip::write::FileOptions::default())?;
        zip.write_all(&asset.data)?;
        assets_dir.pop();
    }
    
    zip.finish()?;
    
    Ok(ExportResult {
        success: true,
        path: output_path,
        size: std::fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0),
    })
}

#[tauri::command]
pub async fn import_project(
    import_path: PathBuf,
) -> Result<ImportResult, String> {
    // Открываем ZIP архив
    let file = File::open(&import_path)
        .map_err(|e| format!("Ошибка открытия файла: {}", e))?;
    
    let mut zip = ZipArchive::new(file)
        .map_err(|e| format!("Ошибка чтения архива: {}", e))?;
    
    // Извлекаем метаданные
    let mut metadata_file = zip.by_name("metadata.json")
        .map_err(|_| "Файл metadata.json не найден")?;
    
    let mut metadata_content = String::new();
    metadata_file.read_to_string(&mut metadata_content)?;
    let metadata: ProjectMetadata = serde_json::from_str(&metadata_content)?;
    
    // Проверяем версию
    if !is_version_compatible(&metadata.version)? {
        return Err(format!(
            "Несовместимая версия проекта: {}",
            metadata.version
        ));
    }
    
    // Извлекаем состояние проекта
    let mut project_file = zip.by_name("project_state.json")?;
    let mut project_content = String::new();
    project_file.read_to_string(&mut project_content)?;
    let project: ProjectState = serde_json::from_str(&project_content)?;
    
    // Извлекаем развёртку (если есть)
    let unfolded = if zip.by_name("unfolded.json").is_ok() {
        let mut unfolded_file = zip.by_name("unfolded.json")?;
        let mut unfolded_content = String::new();
        unfolded_file.read_to_string(&mut unfolded_content)?;
        Some(serde_json::from_str(&unfolded_content)?)
    } else {
        None
    };
    
    // Создаём новый проект с импортированными данными
    let new_project_id = create_project_with_data(project, unfolded)?;
    
    Ok(ImportResult {
        success: true,
        project_id: new_project_id,
        project_name: metadata.project_name,
        import_date: Utc::now().to_rfc3339(),
    })
}
```

---

## 🤖 Автоматизация и AI

### 5.1 AI-кэширование

```rust
// crates/pepakura_core/src/ai/cache.rs

use lru::LruCache;
use sha2::{Sha256, Digest};
use rusqlite::Connection;
use std::num::NonZeroUsize;
use chrono::{DateTime, Utc, Duration};

pub struct AiCache {
    // Быстрый LRU кэш в памяти
    memory_cache: LruCache<String, CacheEntry>,
    // Персистентный кэш в SQLite
    db_conn: Connection,
    // TTL для кэша
    ttl: Duration,
    // Максимальный размер кэша
    max_size: usize,
}

struct CacheEntry {
    response: String,
    created_at: DateTime<Utc>,
    access_count: u32,
    last_accessed: DateTime<Utc>,
}

impl AiCache {
    pub fn new(db_path: &str, max_size: usize, ttl_hours: u64) -> Result<Self, CacheError> {
        let conn = Connection::open(db_path)?;
        
        // Создаём таблицу для персистентного кэша
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ai_cache (
                hash TEXT PRIMARY KEY,
                prompt TEXT NOT NULL,
                response TEXT NOT NULL,
                created_at TEXT NOT NULL,
                access_count INTEGER DEFAULT 0,
                last_accessed TEXT NOT NULL
            )",
            [],
        )?;
        
        // Индекс для очистки по времени
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_created_at ON ai_cache(created_at)",
            [],
        )?;
        
        Ok(Self {
            memory_cache: LruCache::new(NonZeroUsize::new(max_size / 10).unwrap()),
            db_conn: conn,
            ttl: Duration::hours(ttl_hours as i64),
            max_size,
        })
    }
    
    pub fn get(&mut self, prompt: &str) -> Option<String> {
        let hash = self.hash_prompt(prompt);
        
        // Сначала проверяем memory cache
        if let Some(entry) = self.memory_cache.get(&hash) {
            if !self.is_expired(&entry) {
                // Обновляем статистику
                let mut entry = entry.clone();
                entry.access_count += 1;
                entry.last_accessed = Utc::now();
                self.memory_cache.put(hash, entry);
                
                return Some(self.memory_cache.get(&hash).unwrap().response.clone());
            }
        }
        
        // Проверяем SQLite кэш
        let response = self.db_conn.query_row(
            "SELECT response FROM ai_cache WHERE hash = ?1",
            [&hash],
            |row| row.get::<_, String>(0),
        );
        
        match response {
            Ok(resp) => {
                // Проверяем TTL
                let created_at: String = self.db_conn.query_row(
                    "SELECT created_at FROM ai_cache WHERE hash = ?1",
                    [&hash],
                    |row| row.get(0),
                ).ok()?;
                
                let created_at = DateTime::parse_from_rfc3339(&created_at).ok()?;
                if Utc::now() - created_at > self.ttl {
                    self.remove(&hash);
                    return None;
                }
                
                // Обновляем access_count
                self.db_conn.execute(
                    "UPDATE ai_cache 
                     SET access_count = access_count + 1, last_accessed = ?1 
                     WHERE hash = ?2",
                    [Utc::now().to_rfc3339(), &hash],
                ).ok();
                
                // Добавляем в memory cache
                let entry = CacheEntry {
                    response: resp.clone(),
                    created_at: Utc::now(),
                    access_count: 1,
                    last_accessed: Utc::now(),
                };
                self.memory_cache.put(hash, entry);
                
                Some(resp)
            }
            Err(_) => None,
        }
    }
    
    pub fn put(&mut self, prompt: &str, response: String) {
        let hash = self.hash_prompt(prompt);
        let now = Utc::now();
        
        // Добавляем в memory cache
        let entry = CacheEntry {
            response: response.clone(),
            created_at: now,
            access_count: 1,
            last_accessed: now,
        };
        self.memory_cache.put(hash.clone(), entry);
        
        // Добавляем в SQLite
        self.db_conn.execute(
            "INSERT OR REPLACE INTO ai_cache 
             (hash, prompt, response, created_at, access_count, last_accessed) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &hash,
                prompt,
                &response,
                &now.to_rfc3339(),
                &1,
                &now.to_rfc3339(),
            ],
        ).ok();
        
        // Очищаем старый кэш если превышен размер
        self.cleanup();
    }
    
    pub fn remove(&mut self, prompt: &str) {
        let hash = self.hash_prompt(prompt);
        self.memory_cache.pop(&hash);
        self.db_conn.execute(
            "DELETE FROM ai_cache WHERE hash = ?1",
            [&hash],
        ).ok();
    }
    
    pub fn clear(&mut self) {
        self.memory_cache.clear();
        self.db_conn.execute("DELETE FROM ai_cache", []).ok();
    }
    
    fn hash_prompt(&self, prompt: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(prompt.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    fn is_expired(&self, entry: &CacheEntry) -> bool {
        Utc::now() - entry.created_at > self.ttl
    }
    
    fn cleanup(&mut self) {
        // Удаляем просроченные записи из SQLite
        let cutoff = (Utc::now() - self.ttl).to_rfc3339();
        self.db_conn.execute(
            "DELETE FROM ai_cache WHERE created_at < ?1",
            [&cutoff],
        ).ok();
        
        // Если memory cache переполнен, удаляем наименее используемые
        while self.memory_cache.len() > self.max_size / 10 {
            self.memory_cache.pop_lru();
        }
    }
    
    pub fn stats(&self) -> CacheStats {
        let memory_size = self.memory_cache.len();
        
        let db_size: i64 = self.db_conn.query_row(
            "SELECT COUNT(*) FROM ai_cache",
            [],
            |row| row.get(0),
        ).unwrap_or(0);
        
        let total_hits: i64 = self.db_conn.query_row(
            "SELECT SUM(access_count) FROM ai_cache",
            [],
            |row| row.get(0),
        ).unwrap_or(0);
        
        CacheStats {
            memory_entries: memory_size,
            db_entries: db_size as usize,
            total_hits: total_hits as u32,
        }
    }
}

pub struct CacheStats {
    pub memory_entries: usize,
    pub db_entries: usize,
    pub total_hits: u32,
}
```

### 5.2 AI стриминг

```rust
// crates/pepakura_core/src/ai/streaming.rs

use futures::stream::Stream;
use futures::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct AiStreamingClient {
    client: Client,
    base_url: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct StreamRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct StreamResponse {
    model: String,
    created_at: String,
    message: ChatMessage,
    done: bool,
}

impl AiStreamingClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            model: model.to_string(),
        }
    }
    
    pub async fn chat_stream(
        &self,
        messages: &[ChatMessage],
    ) -> Result<impl Stream<Item = Result<String, AiError>>, AiError> {
        let request = StreamRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
            stream: true,
            max_tokens: None,
            temperature: Some(0.7),
        };
        
        let response = self.client
            .post(&format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AiError::NetworkError(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(AiError::ApiError(response.status().to_string()));
        }
        
        // Создаём стрим из response body
        let stream = response
            .bytes_stream()
            .map(|chunk| -> Result<String, AiError> {
                let chunk = chunk.map_err(|e| AiError::StreamError(e.to_string()))?;
                let text = String::from_utf8_lossy(&chunk);
                
                // Парим NDJSON (newline-delimited JSON)
                let mut result = String::new();
                for line in text.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    
                    let response: StreamResponse = serde_json::from_str(line)
                        .map_err(|e| AiError::ParseError(e.to_string()))?;
                    
                    if let Some(content) = &response.message.content {
                        result.push_str(content);
                    }
                }
                
                Ok(result)
            });
        
        Ok(stream)
    }
}

// TypeScript frontend интеграция
```

```typescript
// ui-desktop/src/composables/useAiStream.ts

import { ref } from 'vue'

export function useAiStream() {
  const isStreaming = ref(false)
  const currentResponse = ref('')
  const error = ref<string | null>(null)
  
  const streamChat = async function* (message: string) {
    isStreaming.value = true
    currentResponse.value = ''
    error.value = null
    
    try {
      const response = await fetch(`${config.ollamaUrl}/api/chat`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: config.model,
          messages: [{ role: 'user', content: message }],
          stream: true
        })
      })
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`)
      }
      
      const reader = response.body?.getReader()
      if (!reader) throw new Error('No response body')
      
      const decoder = new TextDecoder()
      
      while (true) {
        const { done, value } = await reader.read()
        if (done) break
        
        const chunk = decoder.decode(value)
        for (const line of chunk.split('\n')) {
          if (!line.trim()) continue
          
          try {
            const data = JSON.parse(line)
            if (data.message?.content) {
              currentResponse.value += data.message.content
              yield data.message.content
            }
          } catch (e) {
            console.warn('Failed to parse stream chunk', e)
          }
        }
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isStreaming.value = false
    }
  }
  
  return {
    isStreaming,
    currentResponse,
    error,
    streamChat
  }
}
```

### 5.3 AI-рекомендации для развёртки

```rust
// crates/pepakura_core/src/ai/recommendations.rs

use crate::geometry::Mesh;
use crate::unfold::UnfoldedMesh;

pub struct UnfoldRecommendations {
    pub algorithm: RecommendedAlgorithm,
    pub config_suggestions: Vec<ConfigSuggestion>,
    pub warnings: Vec<UnfoldWarning>,
    pub estimated_quality: QualityEstimate,
}

pub enum RecommendedAlgorithm {
    MDS { iterations: usize, tolerance: f64 },
    LSCM { preserve_angles: bool },
    SimpleProjection { axis: char },
}

pub struct ConfigSuggestion {
    pub parameter: String,
    pub current_value: String,
    pub suggested_value: String,
    pub reason: String,
}

pub struct QualityEstimate {
    pub score: f32,  // 0.0 - 1.0
    pub area_distortion: f32,
    pub angle_distortion: f32,
    pub overlap_risk: f32,
}

pub async fn analyze_and_recommend(
    mesh: &Mesh,
) -> Result<UnfoldRecommendations, AiError> {
    let mut recommendations = UnfoldRecommendations {
        algorithm: RecommendedAlgorithm::MDS {
            iterations: 100,
            tolerance: 1e-6,
        },
        config_suggestions: Vec::new(),
        warnings: Vec::new(),
        estimated_quality: QualityEstimate {
            score: 0.8,
            area_distortion: 0.0,
            angle_distortion: 0.0,
            overlap_risk: 0.0,
        },
    };
    
    // Анализируем сложность меша
    let vertex_count = mesh.vertices.len();
    let face_count = mesh.faces.len();
    
    if vertex_count > 10_000 {
        recommendations.warnings.push(UnfoldWarning::HighPoly(vertex_count));
        recommendations.config_suggestions.push(ConfigSuggestion {
            parameter: "max_iterations".to_string(),
            current_value: "100".to_string(),
            suggested_value: "50".to_string(),
            reason: "Большое количество вершин может замедлить развёртку".to_string(),
        });
    }
    
    // Проверяем на замкнутость
    if !is_manifold(mesh) {
        recommendations.warnings.push(UnfoldWarning::NonManifold);
        recommendations.estimated_quality.score -= 0.2;
        recommendations.estimated_quality.overlap_risk = 0.7;
    }
    
    // Анализируем распределение размеров граней
    let face_sizes = analyze_face_sizes(mesh);
    if face_sizes.has_extreme_variation {
        recommendations.warnings.push(UnfoldWarning::ExtremeFaceSizes);
        recommendations.config_suggestions.push(ConfigSuggestion {
            parameter: "preserve_detail".to_string(),
            current_value: "true".to_string(),
            suggested_value: "false".to_string(),
            reason: "Сильное различие в размерах граней может вызвать искажения".to_string(),
        });
    }
    
    // Рекомендации по алгоритму
    if face_count < 100 {
        recommendations.algorithm = RecommendedAlgorithm::LSCM {
            preserve_angles: true,
        };
    } else if vertex_count > 5000 {
        recommendations.algorithm = RecommendedAlgorithm::MDS {
            iterations: 50,
            tolerance: 1e-4,
        };
    }
    
    // Оценка качества
    recommendations.estimated_quality.area_distortion = estimate_area_distortion(mesh);
    recommendations.estimated_quality.angle_distortion = estimate_angle_distortion(mesh);
    
    Ok(recommendations)
}
```

---

## 📅 Дорожная карта

### Phase 1: Критичные улучшения (1-2 месяца)

| № | Задача | Приоритет | Оценка | Зависимости |
|---|--------|-----------|--------|-------------|
| 1.1 | Нативный PDF экспорт | 🔴 High | 1 неделя | - |
| 1.2 | Персистентность состояния (SQLite) | 🔴 High | 1.5 недели | - |
| 1.3 | AI кэширование | 🔴 High | 3 дня | - |
| 1.4 | AI стриминг | 🔴 High | 1 неделя | 1.3 |
| 1.5 | Интерактивный 3D viewer | 🔴 High | 2 недели | - |
| 1.6 | Редактор развёрток (перемещение) | 🔴 High | 2 недели | 1.5 |
| 1.7 | Умное автосохранение | 🔴 High | 1 неделя | 1.2 |
| 1.8 | Тесты покрытие >80% | 🔴 High | 2 недели | - |

**Итого Phase 1**: ~10 недель

### Phase 2: Важные улучшения (2-3 месяца)

| № | Задача | Приоритет | Оценка | Зависимости |
|---|--------|-----------|--------|-------------|
| 2.1 | LSCM алгоритм | 🟡 Medium | 2 недели | - |
| 2.2 | MDS оптимизация (parallel) | 🟡 Medium | 1.5 недели | - |
| 2.3 | DXF экспорт | 🟡 Medium | 1 неделя | - |
| 2.4 | Nesting оптимизация (genetic) | 🟡 Medium | 2 недели | - |
| 2.5 | История версий проекта | 🟡 Medium | 1.5 недели | 1.2 |
| 2.6 | Экспорт/Импорт проекта (ZIP) | 🟡 Medium | 1 неделя | 1.2 |
| 2.7 | Валидация при импорте | 🟡 Medium | 1 неделя | - |
| 2.8 | Привязка 3D ↔ 2D | 🟡 Medium | 1.5 недели | 1.5, 1.6 |
| 2.9 | Virtual scrolling | 🟡 Medium | 1 неделя | - |
| 2.10 | Система событий (event bus) | 🟡 Medium | 2 недели | - |

**Итого Phase 2**: ~14 недель

### Phase 3: Расширенные функции (3-6 месяцев)

| № | Задача | Приоритет | Оценка | Зависимости |
|---|--------|-----------|--------|-------------|
| 3.1 | WASM версия ядра | 🟢 Low | 3 недели | 2.10 |
| 3.2 | Веб-приложение | 🟢 Low | 4 недели | 3.1 |
| 3.3 | Текстурированная развёртка | 🟢 Low | 2 недели | - |
| 3.4 | Редактор клапанов | 🟢 Low | 1.5 недели | 1.6 |
| 3.5 | PNG/TIFF экспорт | 🟢 Low | 1 неделя | - |
| 3.6 | Пакетный импорт | 🟢 Low | 1 неделя | 2.7 |
| 3.7 | Система плагинов (cdylib) | 🟢 Low | 3 недели | 2.10 |
| 3.8 | Мобильное приложение (просмотр) | 🟢 Low | 6 недель | 3.1 |
| 3.9 | Облачная синхронизация | 🟢 Low | 4 недели | 1.2 |
| 3.10 | Маркетплейс плагинов | 🟢 Low | 4 недели | 3.7 |

**Итого Phase 3**: ~30 недель

---

## 📊 Метрики успеха

### Технические метрики

| Метрика | Сейчас | Phase 1 | Phase 2 | Phase 3 |
|---------|--------|---------|---------|---------|
| Тесты покрытие | 65% | >80% | >85% | >90% |
| Время развёртки (1000 вершин) | 500ms | <300ms | <100ms | <50ms |
| Время запуска приложения | 3 сек | <2 сек | <1.5 сек | <1 сек |
| Размер бинарника | 50 MB | <45 MB | <40 MB | <35 MB |
| Потребление памяти | 200 MB | <180 MB | <150 MB | <120 MB |
| AI ответ (стриминг) | 5-10 сек | 2-3 сек | 1-2 сек | <1 сек |

### Пользовательские метрики

| Метрика | Сейчас | Цель |
|---------|--------|------|
| Время до первой развёртки | 2 мин | <30 сек |
| Успешность импорта | 85% | >95% |
| Удовлетворённость AI | 70% | >90% |
| NPS (Net Promoter Score) | N/A | >50 |
| Retention (7 дней) | N/A | >60% |
| Средняя сессия | N/A | >15 мин |

---

## 🔧 Инструменты разработки

### Рекомендуемые зависимости

```toml
# crates/pepakura_core/Cargo.toml

[dependencies]
# Существующие
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
nalgebra = "0.32"
image = "0.24"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }

# Новые для улучшений
printpdf = "0.5"              # PDF экспорт
dxf = "0.4"                   # DXF экспорт
rusqlite = { version = "0.31", features = ["bundled"] }  # Персистентность
lru = "0.12"                  # Кэширование
sha2 = "0.10"                 # Хеширование
zip = "0.6"                   # Экспорт проектов
futures = "0.3"               # Стриминг
tokio-stream = "0.1"          # Асинхронные стримы
typeshare = "1.0"             # Генерация TypeScript типов

[dev-dependencies]
criterion = "0.5"             # Бенчмарки
proptest = "1.4"              # Fuzz тесты
```

### Скрипты автоматизации

```bash
#!/bin/bash
# scripts/dev-setup.sh

echo "🚀 Pepakura Next Development Setup"

# Проверка версий
rustc --version
cargo --version
node --version
pnpm --version

# Установка зависимостей
echo "📦 Installing Rust dependencies..."
cd crates/pepakura_core
cargo build

echo "📦 Installing TypeScript dependencies..."
cd ../../ui-desktop
pnpm install

# Генерация типов
echo "🔧 Generating TypeScript types from Rust..."
cd ../crates/pepakura_types
cargo run --bin typeshare -- --output=../../ui-desktop/src/generated/

# Запуск тестов
echo "🧪 Running tests..."
cd ../pepakura_core
cargo test --lib

echo "✅ Setup complete!"
```

```powershell
# scripts/build-release.ps1

param(
    [string]$Target = "current",
    [switch]$SkipTests
)

Write-Host "🏗️  Building Pepakura Next Release" -ForegroundColor Green

# Тесты
if (-not $SkipTests) {
    Write-Host "🧪 Running tests..." -ForegroundColor Yellow
    cd crates/pepakura_core
    cargo test --lib
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Tests failed!" -ForegroundColor Red
        exit 1
    }
}

# Сборка ядра
Write-Host "📦 Building pepakura_core..." -ForegroundColor Yellow
cd crates/pepakura_core
cargo build --release

# Сборка Tauri
Write-Host "📦 Building Tauri app..." -ForegroundColor Yellow
cd ../../src-tauri
cargo tauri build --target $Target

Write-Host "✅ Build complete!" -ForegroundColor Green
Write-Host "📁 Output: src-tauri/target/$Target/release/bundle/"
```

---

## 📝 Заключение

Представленная стратегия развития охватывает **все аспекты** приложения Pepakura Next с фокусом на:

1. **Ручное редактирование**: интерактивный редактор, перемещение деталей, клапаны
2. **Автоматическое редактирование**: AI-рекомендации, умная раскладка, оптимизация
3. **Импорт**: универсальная система, валидация, прогресс, пакетная обработка
4. **Экспорт**: PDF, DXF, PNG, оптимизация SVG, предпросмотр
5. **Сохранение**: персистентность, автосохранение, история версий, восстановление

Реализация плана займёт **6-9 месяцев** при команде из 2-3 разработчиков, но первые улучшения (Phase 1) будут готовы уже через **2 месяца**.

**Следующие шаги**:
1. Приоритизировать задачи Phase 1
2. Создать детальные спецификации для каждой задачи
3. Настроить CI/CD для автоматического тестирования
4. Начать реализацию с PDF экспорта и персистентности

---

*Документ подготовлен на основе анализа проекта*  
*22 марта 2026 г.*
