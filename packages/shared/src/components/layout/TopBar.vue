<script setup lang="ts">
import type { LocaleCode } from "../../i18n/messages";
import { useI18nShared } from "../../i18n/useI18nShared";

const props = defineProps<{
  locale: { value: LocaleCode };
}>();

const { t } = useI18nShared(props.locale);
</script>

<template>
  <div class="topbar">
    <div class="topbar-title">
      {{ t("app.title") }}
    </div>
    <div class="topbar-spacer" />
    <div class="topbar-actions">
      <!-- позже можно заменить на реальное меню -->
      <button type="button" class="topbar-button">
        File
      </button>
      <button type="button" class="topbar-button">
        Edit
      </button>
      <button type="button" class="topbar-button">
        View
      </button>
    </div>
  </div>
</template>

<style scoped>
.topbar {
  display: flex;
  align-items: center;
  padding: 0 12px;
  height: 40px;
  color: #f0f0f0;
  background: linear-gradient(to bottom, #2a2a2a, #1e1e1e);
  font-size: 13px;
}

.topbar-title {
  font-weight: 500;
}

.topbar-spacer {
  flex: 1 1 auto;
}

.topbar-actions {
  display: flex;
  gap: 8px;
}

.topbar-button {
  border: none;
  background: transparent;
  color: #ddd;
  font-size: 12px;
  cursor: pointer;
  padding: 2px 6px;
}

.topbar-button:hover {
  background-color: #3b82f6;
  color: #fff;
  border-radius: 3px;
}
</style>
