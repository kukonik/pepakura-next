<template>
  <header class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
    <div class="container mx-auto flex h-16 items-center justify-between px-4">
      <!-- Логотип и название -->
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6 text-primary"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        </div>
        <div>
          <h1 class="text-xl font-bold tracking-tight">{{ $t('common.appName') }}</h1>
          <p class="text-xs text-muted-foreground">{{ $t('common.appTagline') }}</p>
        </div>
      </div>

      <!-- Центральная навигация (опционально) -->
      <nav class="hidden md:flex items-center gap-6">
        <a
          href="#"
          class="text-sm font-medium transition-colors hover:text-primary"
          @click.prevent="handleNav('dashboard')"
        >
          {{ $t('dashboard.title') }}
        </a>
        <a
          href="#"
          class="text-sm font-medium transition-colors hover:text-primary"
          @click.prevent="handleNav('unfold')"
        >
          {{ $t('unfold.title') }}
        </a>
        <a
          href="#"
          class="text-sm font-medium transition-colors hover:text-primary"
          @click.prevent="handleNav('export')"
        >
          {{ $t('unfold.export') }}
        </a>
      </nav>

      <!-- Правая часть: переключатели и кнопки -->
      <div class="flex items-center gap-4">
        <!-- Переключатель языка -->
        <div class="flex items-center gap-2">
          <button
            type="button"
            class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 w-9"
            :title="currentLanguage === 'ru' ? $t('common.switchToEnglish') : $t('common.switchToRussian')"
            @click="toggleLanguage"
          >
            {{ currentLanguage === 'ru' ? '🇷🇺' : '🇺🇸' }}
          </button>
          <span class="text-sm text-muted-foreground hidden sm:inline">
            {{ currentLanguage === 'ru' ? $t('common.russian') : $t('common.english') }}
          </span>
        </div>

        <!-- Переключатель темы -->
        <div class="flex items-center gap-2">
          <button
            type="button"
            class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 w-9"
            :title="theme === 'dark' ? $t('common.switchToLight') : $t('common.switchToDark')"
            @click="toggleTheme"
          >
            <svg
              v-if="theme === 'dark'"
              xmlns="http://www.w3.org/2000/svg"
              class="h-4 w-4"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
              />
            </svg>
            <svg
              v-else
              xmlns="http://www.w3.org/2000/svg"
              class="h-4 w-4"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
              />
            </svg>
          </button>
          <span class="text-sm text-muted-foreground hidden sm:inline">
            {{ theme === 'dark' ? $t('common.dark') : $t('common.light') }}
          </span>
        </div>

        <!-- Кнопка "О программе" -->
        <button
          type="button"
          class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-4"
          @click="showAbout"
        >
          {{ $t('common.about') }}
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSettingsStore } from '@/stores/settings.store'

const { locale } = useI18n()
const settingsStore = useSettingsStore()

const currentLanguage = ref<'ru' | 'en'>('ru')
const theme = ref<'light' | 'dark'>('light')

onMounted(() => {
  // Загружаем настройки из store
  currentLanguage.value = settingsStore.settings.language
  theme.value = settingsStore.settings.theme === 'system'
    ? (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
    : settingsStore.settings.theme
})

const toggleLanguage = () => {
  const newLang = currentLanguage.value === 'ru' ? 'en' : 'ru'
  currentLanguage.value = newLang
  locale.value = newLang
  settingsStore.settings.language = newLang
  settingsStore.save()
}

const toggleTheme = () => {
  const newTheme = theme.value === 'light' ? 'dark' : 'light'
  theme.value = newTheme
  document.documentElement.classList.toggle('dark', newTheme === 'dark')
  settingsStore.settings.theme = newTheme
  settingsStore.save()
}

const handleNav = (section: string) => {
  console.log('Navigate to:', section)
  // В будущем можно использовать router
}

const showAbout = () => {
  // Открыть модальное окно "О программе"
  console.log('Show about dialog')
}
</script>

<style scoped>
/* Дополнительные стили не требуются, используем Tailwind */
</style>