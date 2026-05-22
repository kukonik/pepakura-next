<script setup lang="ts">
import ThreeDViewerCanvas from "../viewer/ThreeDViewerCanvas.vue";
import { useProjectStore } from "../../stores/projectStore";
import type { LocaleCode } from "../../i18n/messages";

const props = defineProps<{
  locale: { value: LocaleCode };
}>();

const projectStore = useProjectStore();

function handleModelLoaded(payload: { name: string; sizeBytes: number }) {
  projectStore.setLoadedModel({
    name: payload.name,
    sizeBytes: payload.sizeBytes,
  });
}
</script>

<template>
  <div class="stage3d-root">
    <ThreeDViewerCanvas :locale="locale" @modelLoaded="handleModelLoaded" />
  </div>
</template>

<style scoped>
.stage3d-root {
  width: 100%;
  height: 100%;
  display: flex;
}
</style>
