<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/plugin-dialog";
import type { ImageTo3DResult } from "../../../../shared/src/types/ai";

const isGenerating = ref(false);
const imagePath = ref<string | null>(null);
const result = ref<ImageTo3DResult | null>(null);

const selectFile = async () => {
  const selected = await open({ multiple: false, filters: [{ name: "Image", extensions: ["png", "jpg"] }] });
  if (selected && typeof selected === "string") imagePath.value = selected;
};

const generate = async () => {
  if (!imagePath.value) return;
  isGenerating.value = true;
  result.value = null;
  
  try {
    const res = await invoke<ImageTo3DResult>("start_image_to_3d_generation", {
      payload: { imagePath: imagePath.value, qualityParam: "balanced" }
    });
    result.value = res;
    if (res.success) {
      console.log("Model generated at:", res.modelPath);
    }
  } catch (e) {
    console.error(e);
  } finally {
    isGenerating.value = false;
  }
};
</script>

<template>
  <div class="p-4 border rounded">
    <h3 class="font-bold mb-2">AI Generator (TripoSR)</h3>
    
    <div class="mb-4">
      <button @click="selectFile" class="btn btn-secondary">
        {{ imagePath ? imagePath.split("\\").pop() : "Select Image" }}
      </button>
    </div>

    <button 
      @click="generate" 
      :disabled="!imagePath || isGenerating"
      class="btn btn-primary"
    >
      {{ isGenerating ? "Generating..." : "Generate 3D" }}
    </button>

    <div v-if="result" class="mt-4 p-2 bg-gray-100 text-sm">
      <div v-if="result.success" class="text-green-600">
        ✅ Success! 
        <span v-if="result.cached" class="text-blue-500">(From Cache)</span>
        <br>Path: {{ result.modelPath }}
      </div>
      <div v-else class="text-red-600">
        ❌ Error: {{ result.errorMessage }}
      </div>
    </div>
  </div>
</template>
