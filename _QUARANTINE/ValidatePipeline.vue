<script setup lang="ts">
import { ref } from 'vue'
import { usePlatform } from '@pepakura/shared/composables/usePlatform'
import type { ParseResult } from '@pepakura/shared/types/core'

const { getBridge } = usePlatform()
const verticesCount = ref<number | null>(null)
const facesCount = ref<number | null>(null)
const error = ref<string | null>(null)

const MOCK_OBJ = `v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 1.0 1.0 0.0
v 0.0 1.0 0.0
f 1 2 3
f 1 3 4`

const handleParse = async () => {
  try {
    const bridge = getBridge()
    const result: ParseResult = await bridge.parseMockObj(MOCK_OBJ)
    verticesCount.value = result.vertices_count
    facesCount.value = result.faces_count
    error.value = null
  } catch (err: unknown) {
    error.value = err instanceof Error ? err.message : 'Unknown error'
    verticesCount.value = null
    facesCount.value = null
  }
}
</script>

<template>
  <div class="p-4 border rounded">
    <button @click="handleParse" class="bg-blue-600 text-white px-4 py-2 rounded">Parse Mock OBJ</button>
    <div v-if="verticesCount !== null" class="mt-4 text-green-600">
      Успех! Vertices: {{ verticesCount }}, Faces: {{ facesCount }}
    </div>
    <div v-if="error" class="mt-4 text-red-600">Ошибка: {{ error }}</div>
  </div>
</template>

<style scoped>
/* Можно добавить стили при необходимости */
</style>