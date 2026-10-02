<template>
  <div class="pepakura-web">
    <header class="header">
      <h1>Pepakura Next Web</h1>
      <nav>
        <a href="#import">Импорт</a>
        <a href="#unfold">Развёртка</a>
        <a href="#export">Экспорт</a>
      </nav>
      <div class="wasm-status" :class="wasmReady ? 'ok' : 'loading'">
        {{ wasmStatus }}
      </div>
    </header>

    <main class="main-content">
      <!-- Импорт -->
      <section id="import" class="section">
        <h2>Импорт 3D модели</h2>
        <div class="import-area">
          <input
            type="file"
            ref="fileInput"
            accept=".obj,.pdo"
            @change="handleFileImport"
            class="file-input"
          />
          <button @click="triggerFileInput" class="btn-primary">
            Выбрать файл
          </button>
          <button @click="loadTestCube" class="btn-secondary">
            Тестовый куб
          </button>
          <div v-if="importStatus" class="status">{{ importStatus }}</div>
        </div>
      </section>

      <!-- Настройки развёртки -->
      <section id="unfold" class="section">
        <h2>Развёртка</h2>
        <div class="unfold-settings">
          <label>
            Алгоритм:
            <select v-model="unfoldConfig.algorithm">
              <option value="mds">MDS (классический)</option>
              <option value="lscm">LSCM (конформный)</option>
            </select>
          </label>
          <label>
            Макс. итераций:
            <input type="number" v-model.number="unfoldConfig.maxIterations" min="10" max="1000" />
          </label>
          <button @click="runUnfold" :disabled="!meshLoaded || isUnfolding || !wasmReady" class="btn-primary">
            {{ isUnfolding ? 'Развёртка...' : 'Создать развёртку' }}
          </button>
        </div>
      </section>

      <!-- 2D Развёртка -->
      <section class="section">
        <h2>2D Развёртка</h2>
        <div v-if="unfoldedSvg" class="unfold-preview" v-html="unfoldedSvg"></div>
        <div v-else class="no-data">
          <p>Развёртка ещё не создана. Загрузите модель и нажмите "Создать развёртку".</p>
        </div>
      </section>

      <!-- Экспорт -->
      <section id="export" class="section">
        <h2>Экспорт</h2>
        <div class="export-buttons">
          <button @click="exportSVG" :disabled="!unfoldedSvg" class="btn-secondary">
            Скачать SVG
          </button>
        </div>
      </section>
    </main>

    <footer class="footer">
      <p>Pepakura Next Web v{{ wasmVersion }} - WASM Powered</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const wasmReady = ref(false)
const wasmStatus = ref('WASM: загрузка...')
const wasmVersion = ref('?')
const meshLoaded = ref(false)
const isUnfolding = ref(false)
const importStatus = ref('')
const unfoldedSvg = ref('')
const currentMesh = ref<any>(null)

const fileInput = ref<HTMLInputElement | null>(null)

const unfoldConfig = ref({
  algorithm: 'mds',
  maxIterations: 100,
  tolerance: 1e-6,
})

// --- Инициализация WASM ---
onMounted(async () => {
  try {
    const wasm = await import('../public/wasm/pepakura_wasm.js')
    await wasm.default()
    wasmVersion.value = wasm.version()
    wasmReady.value = true
    wasmStatus.value = 'WASM: готов'
    console.log('WASM инициализирован, версия:', wasm.version())
  } catch (error) {
    wasmStatus.value = 'WASM: ошибка!'
    console.error('Ошибка инициализации WASM:', error)
  }
})

// --- Загрузка тестового куба ---
function loadTestCube() {
  const vertices = [
    [0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0],
    [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1],
  ]
  const faces = [
    [0, 1, 2], [0, 2, 3], [4, 6, 5], [4, 7, 6],
    [0, 4, 5], [0, 5, 1], [2, 6, 7], [2, 7, 3],
    [0, 3, 7], [0, 7, 4], [1, 5, 6], [1, 6, 2],
  ]

  currentMesh.value = { name: 'Cube', vertices, faces }
  meshLoaded.value = true
  importStatus.value = 'Тестовый куб загружен: 8 вершин, 12 граней'
}

// --- Импорт файла ---
function triggerFileInput() {
  fileInput.value?.click()
}

async function handleFileImport(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  importStatus.value = 'Загрузка ' + file.name + '...'

  try {
    const text = await file.text()
    const lines = text.split('\n')

    if (file.name.endsWith('.obj')) {
      const vertices: number[][] = []
      const faces: number[][] = []

      for (const line of lines) {
        const parts = line.trim().split(/\s+/)
        if (parts[0] === 'v' && parts.length >= 4) {
          vertices.push([parseFloat(parts[1]), parseFloat(parts[2]), parseFloat(parts[3])])
        } else if (parts[0] === 'f') {
          const face = parts.slice(1).map(p => (parseInt(p.split('/')[0]) - 1))
          // Триангулируем (простой случай)
          for (let i = 1; i < face.length - 1; i++) {
            faces.push([face[0], face[i], face[i + 1]])
          }
        }
      }

      currentMesh.value = { name: file.name, vertices, faces }
      meshLoaded.value = true
      importStatus.value = 'Загружено: ' + vertices.length + ' вершин, ' + faces.length + ' граней'
    } else {
      importStatus.value = 'Формат ' + file.name.split('.').pop() + ' пока не поддерживается'
    }
  } catch (error) {
    importStatus.value = 'Ошибка: ' + error
    meshLoaded.value = false
  }
}

// --- Развёртка ---
async function runUnfold() {
  if (!meshLoaded.value || !currentMesh.value || !wasmReady.value) return

  isUnfolding.value = true
  importStatus.value = 'Выполняется развёртка...'

  try {
    const wasm = await import('../public/wasm/pepakura_wasm.js')

    // Создаём WASM объекты
    const wasmVertices = currentMesh.value.vertices.map(
      (v: number[], i: number) => new wasm.VertexWasm(i, new Float64Array(v))
    )
    const wasmFaces = currentMesh.value.faces.map(
      (f: number[]) => new wasm.FaceWasm(new Uint32Array(f))
    )
    const wasmMesh = new wasm.MeshWasm(currentMesh.value.name, wasmVertices, wasmFaces)

    const config = new wasm.UnfoldConfigWasm(
      unfoldConfig.value.algorithm,
      unfoldConfig.value.maxIterations,
      unfoldConfig.value.tolerance,
      true
    )

    const result = wasm.unfold_mesh(wasmMesh, config)

    // Получаем данные
    const vertices2d = Array.from(result.vertices_2d())
    const facesData = result.faces()
    const metadata = result.metadata()

    console.log('Результат развёртки:', { vertices2d: vertices2d.length, faces: facesData.length, metadata })

    // Рендерим SVG
    unfoldedSvg.value = renderToSvg(vertices2d, facesData, currentMesh.value.faces)

    importStatus.value = 'Развёртка создана! ' + facesData.length + ' деталей'
    console.log('unfoldedSvg set to:', unfoldedSvg.value ? unfoldedSvg.value.substring(0, 200) : 'null', 'length:', unfoldedSvg.value?.length)

    // Освобождаем память WASM

  } catch (error) {
    importStatus.value = 'Ошибка развёртки: ' + error
    console.error('Ошибка развёртки:', error)
  } finally {
    isUnfolding.value = false
  }
}

// --- SVG рендеринг ---
function renderToSvg(vertices2d: number[], wasmFaces: any[], originalFaces: number[][]): string {
  console.log('renderToSvg called:', { vertices2d: vertices2d.length, wasmFaces, originalFaces })

  const colors = ['#ff6b6b', '#4ecdc4', '#45b7d1', '#96ceb4', '#ffeaa7', '#dfe6e9', '#fab1a0', '#a29bfe', '#fd79a8', '#00b894']

  // Вычисляем bounding box всех 2D-координат
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity
  for (let i = 0; i < vertices2d.length; i += 2) {
    const x = vertices2d[i]
    const y = vertices2d[i + 1]
    if (x < minX) minX = x
    if (x > maxX) maxX = x
    if (y < minY) minY = y
    if (y > maxY) maxY = y
  }

  // Добавляем 10% padding
  const width = maxX - minX
  const height = maxY - minY
  const padding = Math.max(width, height) * 0.1
  const viewBoxMinX = minX - padding
  const viewBoxMinY = minY - padding
  const viewBoxWidth = width + 2 * padding
  const viewBoxHeight = height + 2 * padding

  console.log('Bounding box:', { minX, maxX, minY, maxY, viewBox: `${viewBoxMinX} ${viewBoxMinY} ${viewBoxWidth} ${viewBoxHeight}` })

  // SVG с viewBox для правильного масштабирования
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${viewBoxMinX} ${viewBoxMinY} ${viewBoxWidth} ${viewBoxHeight}" preserveAspectRatio="xMidYMid meet" width="800" height="600">`

  // Белый фон для контрастности
  svg += `<rect x="${viewBoxMinX}" y="${viewBoxMinY}" width="${viewBoxWidth}" height="${viewBoxHeight}" fill="white" />`

  // Масштабируемый stroke-width (зависит от размера viewBox)
  const strokeWidth = Math.max(viewBoxWidth, viewBoxHeight) * 0.005

  wasmFaces.forEach((faceData: any, index: number) => {
    let face: number[]
    if (Array.isArray(faceData)) {
      face = faceData
    } else if (faceData && typeof faceData.vertices === 'function') {
      face = Array.from(faceData.vertices())
    } else if (originalFaces && originalFaces[index]) {
      face = originalFaces[index]
    } else {
      console.warn('Unknown face format at index', index, faceData)
      return
    }

    const color = colors[index % colors.length]

    const points = face.map((vi: number) => {
      const x = vertices2d[vi * 2]
      const y = vertices2d[vi * 2 + 1]
      return x.toFixed(2) + ',' + y.toFixed(2)
    }).join(' ')

    svg += `<polygon points="${points}" fill="${color}" fill-opacity="0.3" stroke="#333" stroke-width="${strokeWidth}" />`
  })

  svg += '</svg>'
  console.log('SVG generated, length:', svg.length)
  return svg
}

// --- Экспорт ---
function exportSVG() {
  if (!unfoldedSvg.value) return
  const blob = new Blob([unfoldedSvg.value], { type: 'image/svg+xml' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'pepakura_unfold.svg'
  a.click()
  URL.revokeObjectURL(url)
}
</script>

<style scoped>
.pepakura-web {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  margin: 0;
}

.header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 1rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header h1 { margin: 0; font-size: 1.5rem; }
.header nav { display: flex; gap: 1rem; }
.header nav a { color: white; text-decoration: none; opacity: 0.9; }
.header nav a:hover { opacity: 1; }

.wasm-status {
  padding: 0.3rem 0.8rem;
  border-radius: 4px;
  font-size: 0.85rem;
  font-weight: 500;
}
.wasm-status.ok { background: rgba(0,200,83,0.3); }
.wasm-status.loading { background: rgba(255,255,255,0.2); }

.main-content {
  flex: 1;
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
}

.section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}
.section h2 { margin-top: 0; color: #333; }

.import-area, .unfold-settings, .export-buttons {
  display: flex;
  gap: 1rem;
  align-items: center;
  flex-wrap: wrap;
}

.file-input { display: none; }

.btn-primary, .btn-secondary {
  padding: 0.6rem 1.2rem;
  border: none;
  border-radius: 6px;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s;
}
.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}
.btn-primary:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}
.btn-secondary {
  background: #f5f5f5;
  color: #333;
  border: 1px solid #ddd;
}
.btn-secondary:hover:not(:disabled) { background: #e5e5e5; }
.btn-primary:disabled, .btn-secondary:disabled { opacity: 0.5; cursor: not-allowed; }

.status {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  background: #f0f0f0;
  font-size: 0.9rem;
}

.unfold-preview {
  min-height: 400px;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1rem;
  background: #fafafa;
}
.unfold-preview :deep(svg) { width: 100%; height: 100%; }

.no-data {
  text-align: center;
  padding: 3rem;
  color: #999;
}

.footer {
  background: #f5f5f5;
  padding: 1rem 2rem;
  text-align: center;
  color: #666;
  font-size: 0.9rem;
}

label {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  font-size: 0.9rem;
  color: #555;
}
select, input[type="number"] {
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
}
</style>