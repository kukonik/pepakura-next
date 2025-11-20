<template>
  <div style="padding:2em;">
    <h1>Галерея проектов</h1>
    <div
      @dragover.prevent
      @drop.prevent="onDrop"
      style="border: 2px dashed #aaa; padding: 2em; margin-bottom: 1em; text-align:center;">
      Перетащите сюда файлы моделей для добавления
    </div>
    <div v-if="projects.length > 0" style="display: flex; flex-wrap: wrap; gap: 16px;">
      <div v-for="proj in projects" :key="proj.id"
           style="border: 1px solid #ccc; border-radius:8px; width:180px; padding: 1em; background:#fff;">
        <div style="font-size:3em; text-align:center;">📦</div>
        <strong>{{ proj.title }}</strong>
        <div style="font-size:0.9em; color:#888;">{{ proj.filename }}</div>
        <div style="font-size:0.8em; color:#555;">{{ proj.size }}</div>
      </div>
    </div>
    <div v-else>
      <p>Проектов нет. Добавьте первый файл!</p>
    </div>
    <button @click="gotoPrompt" style="margin-top:2em;">Создать проект</button>
  </div>
</template>

<script>
import mockProjects from '../mockProjects'
export default {
  name: "GalleryPage",
  data() {
    return { projects: mockProjects }
  },
  methods: {
    onDrop(e) {
      const files = Array.from(e.dataTransfer.files)
      for (const file of files) {
        this.projects.push({
          id: 'new-' + Date.now() + '-' + Math.random(),
          title: file.name.split('.')[0],
          filename: file.name,
          size: Math.round(file.size/1024) + " КБ"
        })
      }
    },
    gotoPrompt() { this.$router.push('/prompt') }
  }
}
</script>
