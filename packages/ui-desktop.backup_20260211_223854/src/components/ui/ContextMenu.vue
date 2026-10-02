<template>
  <Teleport to="body">
    <div 
      v-if="visible" 
      class="context-menu" 
      :style="{ top: position.y + 'px', left: position.x + 'px' }"
      @click.stop
    >
      <div 
        v-for="(option, index) in options" 
        :key="index" 
        class="menu-item" 
        :class="{ 'danger': option.danger }"
        @click="handleClick(option)"
      >
        <span class="menu-icon">
          <component :is="getIconComponent(option.icon)" />
        </span>
        <span class="menu-label">{{ option.label }}</span>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps({
  options: {
    type: Array as () => Array<{ label: string, icon: string, action: string, danger?: boolean }>,
    required: true
  },
  position: {
    type: Object as () => { x: number, y: number },
    required: true
  },
  visible: {
    type: Boolean,
    default: false
  }
})

const emits = defineEmits(['action', 'close'])

const getIconComponent = (iconName: string) => {
  return iconName.split(':')[1] || 'mdi:help'
}

const handleClick = (option: any) => {
  emits('action', option.action)
  emits('close')
}
</script>

<style scoped>
.context-menu {
  position: fixed;
  background: var(--surface-900);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  z-index: 1000;
  min-width: 200px;
  transform: translateX(-10px) translateY(5px);
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 16px;
  cursor: pointer;
  transition: background 0.2s;
}

.menu-item:hover {
  background: var(--surface-800);
}

.menu-item.danger:hover {
  background: var(--error-900);
}

.menu-icon {
  font-size: 18px;
  color: var(--text-secondary);
}

.menu-label {
  color: var(--text-color);
  font-size: 14px;
}
</style>
