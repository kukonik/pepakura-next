<template>
  <div class="nesting-editor-page">
    <NestingEditor />
  </div>
</template>

<script setup lang="ts">
import NestingEditor from '@/components/paper-optimization/NestingEditor.vue'
</script>

<style scoped>
.nesting-editor-page {
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style>