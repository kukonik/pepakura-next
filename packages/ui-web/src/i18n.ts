import { ref } from "vue";
import type { LocaleCode } from "@shared/i18n/messages";

export const currentLocale = ref<LocaleCode>("ru");
