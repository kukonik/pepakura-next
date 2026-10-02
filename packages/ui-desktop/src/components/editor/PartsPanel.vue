<template>
  <div class="parts-panel">
    <div class="panel-header">
      <h3>Список деталей</h3>
      <div class="search-box">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Поиск по ID или имени..."
          @input="onSearch"
        />
        <span class="search-count" v-if="filteredParts.length !== totalParts">
          {{ filteredParts.length }} из {{ totalParts }}
        </span>
      </div>
    </div>

    <div class="parts-list-container">
      <RecycleScroller
        v-if="filteredParts.length > 0"
        class="scroller"
        :items="filteredParts"
        :item-size="itemHeight"
        key-field="id"
        :buffer="200"
        :prerender="10"
        @scroll="onScroll"
        ref="scroller"
      >
        <template v-slot="{ item: part, index }">
          <div
            class="part-item"
            :class="{ selected: selectedPartId === part.id }"
            :style="{ height: itemHeight + 'px' }"
            @click="selectPart(part)"
            @contextmenu.prevent="openContextMenu($event, part)"
          >
            <div class="part-id">#{{ part.id }}</div>
            <div class="part-name">{{ part.name || `Деталь ${part.id}` }}</div>
            <div class="part-details">
              <span class="part-size">
                {{ part.bounds.width.toFixed(1) }} × {{ part.bounds.height.toFixed(1) }} мм
              </span>
              <span class="part-lines">
                {{ part.lines.length }} линий
              </span>
            </div>
            <div class="part-sheet" v-if="getSheetForPart(part)">
              Лист {{ getSheetForPart(part)!.index + 1 }}
            </div>
          </div>
        </template>
      </RecycleScroller>

      <div v-else class="empty-state">
        <p v-if="searchQuery">Ничего не найдено</p>
        <p v-else>Нет деталей для отображения</p>
      </div>
    </div>

    <div class="panel-footer">
      <div class="selection-info" v-if="selectedPartId !== null">
        Выбрана деталь #{{ selectedPartId }}
      </div>
      <div class="total-info">
        Всего деталей: {{ totalParts }}
      </div>
    </div>

    <!-- Контекстное меню -->
    <ContextMenu
      v-if="contextMenu.visible"
      :x="contextMenu.x"
      :y="contextMenu.y"
      :items="contextMenuItems"
      @close="closeContextMenu"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { useUnfoldStore } from '@/stores/unfoldStore'
import { Part2D, Sheet } from '@/stores/unfoldStore'
import ContextMenu from '@/components/ui/ContextMenu.vue'

const unfoldStore = useUnfoldStore()

// Props (если нужно передавать части извне)
const props = defineProps<{
  parts?: Part2D[]
  sheets?: Sheet[]
}>()

// Refs
const searchQuery = ref('')
const selectedPartId = ref<number | null>(null)
const scroller = ref<InstanceType<typeof RecycleScroller> | null>(null)
const itemHeight = 70 // высота элемента в пикселях

// Контекстное меню
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  part: null as Part2D | null
})

// Вычисляемые свойства
const allParts = computed(() => {
  // Если переданы части через props, используем их
  if (props.parts && props.parts.length > 0) {
    return props.parts
  }
  // Иначе берём из store
  const sheets = unfoldStore.sheets
  return sheets.flatMap(sheet => sheet.parts)
})

const totalParts = computed(() => allParts.value.length)

const filteredParts = computed(() => {
  if (!searchQuery.value.trim()) {
    return allParts.value
  }
  const query = searchQuery.value.toLowerCase()
  return allParts.value.filter(part => {
    return (
      part.id.toString().includes(query) ||
      (part.name && part.name.toLowerCase().includes(query))
    )
  })
})

// Поиск листа для детали
const getSheetForPart = (part: Part2D): Sheet | undefined => {
  return unfoldStore.sheets.find(sheet =>
    sheet.parts.some(p => p.id === part.id)
  )
}

// Обработчики
const selectPart = (part: Part2D) => {
  selectedPartId.value = part.id
  emit('select', part)
}

const onSearch = () => {
  // При поиске можно прокрутить к первому найденному элементу
  if (filteredParts.value.length > 0 && searchQuery.value.trim()) {
    scrollToPart(filteredParts.value[0].id)
  }
}

const scrollToPart = (partId: number) => {
  const index = filteredParts.value.findIndex(p => p.id === partId)
  if (index >= 0 && scroller.value) {
    scroller.value.scrollToItem(index)
  }
}

const onScroll = (event: Event) => {
  // Можно добавить логику lazy loading или другие действия при скролле
}

// Контекстное меню
const openContextMenu = (event: MouseEvent, part: Part2D) => {
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    part
  }
}

const closeContextMenu = () => {
  contextMenu.value.visible = false
}

const contextMenuItems = computed(() => [
  { label: 'Выделить', action: () => selectPart(contextMenu.value.part!) },
  { label: 'Показать на листе', action: () => emit('showOnSheet', contextMenu.value.part!) },
  { label: 'Скопировать ID', action: () => copyToClipboard(contextMenu.value.part!.id.toString()) },
  { label: 'Удалить', action: () => emit('delete', contextMenu.value.part!), danger: true }
])

const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text)
}

// Emits
const emit = defineEmits<{
  select: [part: Part2D]
  showOnSheet: [part: Part2D]
  delete: [part: Part2D]
}>()

// Инициализация
onMounted(() => {
  // Можно подписаться на изменения store
})

onUnmounted(() => {
  // Очистка
})

// Watch для внешнего изменения выделения
watch(() => selectedPartId.value, (newId) => {
  if (newId !== null) {
    scrollToPart(newId)
  }
})
</script>

<style scoped>
.parts-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #fff;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.panel-header {
  padding: 16px;
  border-bottom: 1px solid #e0e0e0;
  background: #f9f9f9;
}

.panel-header h3 {
  margin: 0 0 12px 0;
  font-size: 18px;
  font-weight: 600;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 8px;
}

.search-box input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
}

.search-count {
  font-size: 12px;
  color: #666;
  white-space: nowrap;
}

.parts-list-container {
  flex: 1;
  overflow: hidden;
  position: relative;
}

.scroller {
  height: 100%;
}

.part-item {
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  justify-content: center;
  transition: background-color 0.2s;
}

.part-item:hover {
  background-color: #f5f5f5;
}

.part-item.selected {
  background-color: #e3f2fd;
  border-left: 4px solid #2196f3;
}

.part-id {
  font-weight: bold;
  font-size: 14px;
  color: #333;
}

.part-name {
  font-size: 13px;
  color: #555;
  margin-top: 2px;
}

.part-details {
  display: flex;
  gap: 12px;
  margin-top: 4px;
  font-size: 12px;
  color: #777;
}

.part-sheet {
  margin-top: 4px;
  font-size: 11px;
  color: #999;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
  font-size: 14px;
}

.panel-footer {
  padding: 12px 16px;
  border-top: 1px solid #e0e0e0;
  background: #f9f9f9;
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #666;
}

.selection-info {
  font-weight: 500;
  color: #2196f3;
}
</style>