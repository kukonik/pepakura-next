<template>
  <div>
    <h2>🧪 Тест Three.js</h2>
    <button @click="testThreeJS">Проверить Three.js</button>
    <button @click="testScene">Проверить сцену</button>
    <div id="test-canvas" style="width: 400px; height: 400px; border: 1px solid red;"></div>
    <pre>{{ log }}</pre>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as THREE from 'three'

const log = ref<string[]>([])

const testThreeJS = () => {
  log.value.push('🔍 Проверка Three.js...')
  log.value.push(`  THREE.version: ${THREE.REVISION}`)
  log.value.push(`  THREE.Scene: ${typeof THREE.Scene}`)
  log.value.push(`  THREE.Mesh: ${typeof THREE.Mesh}`)
}

const testScene = () => {
  log.value.push('\n🔍 Создание тестовой сцены...')
  
  try {
    const scene = new THREE.Scene()
    log.value.push(`  ✅ Сцена создана: ${scene}`)
    
    const camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000)
    log.value.push(`  ✅ Камера создана: ${camera}`)
    
    const renderer = new THREE.WebGLRenderer()
    log.value.push(`  ✅ Рендерер создан: ${renderer}`)
    
    const geometry = new THREE.BoxGeometry()
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 })
    const cube = new THREE.Mesh(geometry, material)
    log.value.push(`  ✅ Куб создан: ${cube}`)
    
    scene.add(cube)
    log.value.push(`  ✅ Куб добавлен в сцену`)
    
    log.value.push('\n✅ Все компоненты Three.js работают!')
  } catch (error: any) {
    log.value.push(`\n❌ Ошибка: ${error.message}`)
    console.error(error)
  }
}

onMounted(() => {
  log.value.push('✅ Тестовый компонент смонтирован')
})
</script>
