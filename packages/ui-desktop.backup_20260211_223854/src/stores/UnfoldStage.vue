<script lang="ts" setup>
import { computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useProjectStore } from "@/stores/project";
import { useUnfoldStore } from "@/stores/unfoldStore";

const projectStore = useProjectStore();
const unfoldStore = useUnfoldStore();

const hasModel = computed(() => !!projectStore.currentModelPath);
const isUnfolding = computed(() => unfoldStore.isUnfolding);
const sheets = computed(() => unfoldStore.sheets);
const lastError = computed(() => unfoldStore.lastError);
const params = computed(() => unfoldStore.params);

async function onUnfoldClick() {
  await unfoldStore.unfoldCurrentModel();
}

async function onExportPdfClick() {
  await unfoldStore.exportPdf();
}

async function pingBackend() {
  if (!projectStore.currentModelPath) return;

  try {
    await invoke("read_model_file", {
      path: projectStore.currentModelPath,
    });
  } catch (e) {
    console.error("[UnfoldStage] pingBackend error", e);
  }
}
</script>

<template>
  <div class="unfold-stage">
    <div class="toolbar">
      <button
        :disabled="!hasModel || isUnfolding"
        @click="onUnfoldClick"
      >
        Unfold model
      </button>

      <button
        :disabled="!sheets.length || isUnfolding"
        @click="onExportPdfClick"
      >
        Export PDF
      </button>

      <button
        :disabled="!hasModel"
        @click="pingBackend"
      >
        Ping backend
      </button>
    </div>

    <div class="params">
      <div>
        Paper: {{ params.paper_format }},
        margin {{ params.margin_mm }} mm,
        max sheets {{ params.max_sheets }},
        scale ×{{ params.scale }}
      </div>
    </div>

    <div v-if="lastError" class="error">
      {{ lastError }}
    </div>

    <div v-if="sheets.length" class="sheets">
      <div
        v-for="sheet in sheets"
        :key="sheet.id"
        class="sheet"
      >
        <h3>Sheet {{ sheet.index }}</h3>
        <p>
          {{ sheet.width_mm }} × {{ sheet.height_mm }} mm,
          margin {{ sheet.margin_mm }} mm
        </p>
        <p>Parts: {{ sheet.parts.length }}</p>
      </div>
    </div>

    <div v-else class="empty">
      No unfold data yet.
    </div>
  </div>
</template>

<style scoped>
.unfold-stage {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.toolbar {
  display: flex;
  gap: 8px;
}

.params {
  font-size: 0.9rem;
  color: #666;
}

.error {
  color: #f56c6c;
}

.sheets {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.sheet {
  border: 1px solid #ccc;
  padding: 8px;
}

.empty {
  color: #999;
}
</style>
