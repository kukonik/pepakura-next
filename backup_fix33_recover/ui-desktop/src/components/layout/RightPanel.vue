<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useTauriModelLoader } from '@/composables/useTauriModelLoader'

const {
  import3DModel,
  openProject,
  showMessage,
  currentModelPath,
  modelStats
} = useTauriModelLoader()

// Примеры локальных состояний панели
const autoUnwrap = ref(true)
const autoArrange = ref(true)
const quality = ref<'draft' | 'normal' | 'high'>('normal')

// Безопасные computed, чтобы не было undefined
const faces = computed(() => {
  const v = modelStats.value?.faces
  return typeof v === 'number' ? v : null
})

const parts = computed(() => {
  const v = modelStats.value?.parts
  return typeof v === 'number' ? v : null
})

// WATCH: следим только за ref/computed, никаких undefined
watch(
  () => currentModelPath.value,
  newVal => {
    if (newVal) {
      showMessage(`Загружена модель: ${newVal}`, 'info')
    }
  }
)

watch(
  () => modelStats.value,
  newVal => {
    if (newVal) {
      console.debug('[RightPanel] modelStats changed', newVal)
    }
  },
  { deep: true }
)

watch(
  () => quality.value,
  q => {
    console.debug('[RightPanel] quality:', q)
  }
)

async function onImportModelClick() {
  await import3DModel()
}

async function onOpenProjectClick() {
  await openProject()
}
</script>
