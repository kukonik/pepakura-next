<template>
  <div class="quick-actions-panel">
    <h2>Быстрые действия</h2>
    <div class="actions-grid">
      <Button 
        variant="primary" 
        class="action-button"
        @click="emit('create-project')"
      >
        <div class="action-content">
          <div class="action-icon">&#10133;</div>
          <div class="action-text">Создать проект</div>
        </div>
      </Button>
      
      <Button 
        variant="secondary" 
        class="action-button"
        @click="emit('import-model')"
      >
        <div class="action-content">
          <div class="action-icon">&#128229;</div>
          <div class="action-text">Импорт модели</div>
        </div>
      </Button>
      
      <Button 
        variant="secondary" 
        class="action-button"
        @click="emit('open-settings')"
      >
        <div class="action-content">
          <div class="action-icon">&#9881;</div>
          <div class="action-text">Настройки</div>
        </div>
      </Button>
      
      <Button 
        variant="secondary" 
        class="action-button"
        @click="emit('open-tutorial')"
      >
        <div class="action-content">
          <div class="action-icon">&#128214;</div>
          <div class="action-text">Обучение</div>
        </div>
      </Button>
    </div>
  </div>
</template>

<script setup>
import Button from '../ui/Button.vue'

const emit = defineEmits(['create-project', 'import-model', 'open-settings', 'open-tutorial'])
</script>

<style scoped>
.quick-actions-panel {
  padding: 20px;
  background: var(--bg-secondary);
  border-radius: 8px;
  box-shadow: var(--shadow-sm);
}

.quick-actions-panel h2 {
  margin: 0 0 20px 0;
  font-size: 1.5rem;
  color: var(--text-primary);
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 16px;
}

.action-button {
  height: 100px;
  padding: 0;
  border: 2px dashed var(--border-color);
  background: var(--bg-primary);
  transition: all 0.2s ease;
}

.action-button:hover {
  border-color: var(--primary-color);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.action-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
}

.action-icon {
  font-size: 2rem;
  margin-bottom: 8px;
}

.action-text {
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--text-primary);
}
</style>
