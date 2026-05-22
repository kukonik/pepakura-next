<script setup lang="ts">
import { computed } from "vue";
import { useProjectStore } from "../../stores/projectStore";
import type { LocaleCode } from "../../i18n/messages";
import { useI18nShared } from "../../i18n/useI18nShared";

const props = defineProps<{
  activeStage: string;
  locale: { value: LocaleCode };
}>();

const { t } = useI18nShared(props.locale);
const projectStore = useProjectStore();

const modelInfoText = computed(() => {
  if (!projectStore.loadedModel) return t("rightPanel.noModel");
  const kb = (projectStore.loadedModel.sizeBytes / 1024).toFixed(1);
  return `${projectStore.loadedModel.name} (${kb} KB)`;
});
</script>

<template>
  <div class="right-panel">
    <header class="right-panel-header">
      <span class="right-panel-title">Stage: {{ activeStage }}</span>
    </header>
    <section class="right-panel-body">
      <p class="right-panel-section-title">
        {{ t("rightPanel.projectTitle") }}
      </p>
      <p class="right-panel-line">
        {{ t("rightPanel.modelLabel") }}: {{ modelInfoText }}
      </p>
      <hr class="right-panel-separator" />
      <p class="right-panel-placeholder">
        Панель для стейджа "{{ activeStage }}".
      </p>
    </section>
  </div>
</template>

<style scoped>
.right-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  color: #ddd;
  font-size: 12px;
}

.right-panel-header {
  padding: 8px 12px;
  border-bottom: 1px solid #333;
  background-color: #202020;
}

.right-panel-title {
  font-weight: 500;
}

.right-panel-body {
  padding: 8px 12px;
  overflow: auto;
  flex: 1 1 auto;
}

.right-panel-section-title {
  font-weight: 500;
  margin: 0 0 4px 0;
}

.right-panel-line {
  margin: 0 0 8px 0;
}

.right-panel-separator {
  border: none;
  border-top: 1px solid #333;
  margin: 8px 0;
}

.right-panel-placeholder {
  opacity: 0.75;
}
</style>
