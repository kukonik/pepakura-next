import { ref, computed } from "vue";
import { defineStore } from "pinia";

export interface LoadedModelInfo {
  name: string;
  sizeBytes: number;
}

export const useProjectStore = defineStore("project", () => {
  const loadedModel = ref<LoadedModelInfo | null>(null);

  const hasModel = computed(() => loadedModel.value !== null);

  function setLoadedModel(info: LoadedModelInfo | null) {
    loadedModel.value = info;
  }

  return {
    loadedModel,
    hasModel,
    setLoadedModel,
  };
});
