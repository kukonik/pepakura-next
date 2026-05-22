<script setup lang="ts">
import { computed } from "vue";
import type { LocaleCode } from "../../i18n/messages";
import { useI18nShared } from "../../i18n/useI18nShared";

const props = defineProps<{
  locale: { value: LocaleCode };
}>();

const { t } = useI18nShared(props.locale);

const statusText = computed(() => t("status.ready"));
const coords = computed(() => "x: 0, y: 0, z: 0");
</script>

<template>
  <div class="statusbar">
    <span class="statusbar-text">{{ statusText }}</span>
    <span class="statusbar-spacer" />
    <span class="statusbar-coords">{{ coords }}</span>
  </div>
</template>

<style scoped>
.statusbar {
  display: flex;
  align-items: center;
  padding: 0 8px;
  height: 22px;
  font-size: 11px;
  color: #ccc;
}

.statusbar-text {
  min-width: 80px;
}

.statusbar-spacer {
  flex: 1 1 auto;
}

.statusbar-coords {
  opacity: 0.75;
}
</style>
