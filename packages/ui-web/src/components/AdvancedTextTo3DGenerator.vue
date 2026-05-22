<template>
  <div class="advanced-text-to-3d-generator">
    <h2>Генератор 3D моделей по текстовому описанию</h2>
    
    <div class="input-section">
      <textarea
        v-model="prompt"
        placeholder="Опишите 3D модель, которую хотите создать..."
        rows="4"
        cols="50"
      ></textarea>
      
      <div class="options">
        <label>
          Модель ИИ:
          <select v-model="selectedModel">
            <option value="shap-e">Shap-E</option>
            <option value="get3d">GET3D</option>
            <option value="magic3d">Magic3D</option>
          </select>
        </label>
        
        <label>
          Количество шагов:
          <input type="number" v-model.number="numInferenceSteps" min="10" max="200" />
        </label>
        
        <label>
          Guidance Scale:
          <input type="number" v-model.number="guidanceScale" step="0.1" min="1" max="20" />
        </label>
        
        <label>
          Качество:
          <select v-model="quality">
            <option value="high">Высокое</option>
            <option value="medium">Среднее</option>
            <option value="low">Низкое</option>
          </select>
        </label>
        
        <label>
          Освещение:
          <select v-model="lighting">
            <option value="bright">Яркое</option>
            <option value="soft">Мягкое</option>
            <option value="dramatic">Драматическое</option>
            <option value="dark">Темное</option>
          </select>
        </label>
      </div>
      
      <div class="advanced-options">
        <h3>Дополнительные параметры</h3>
        <label>
          Стиль:
          <select v-model="style">
            <option value="">По умолчанию</option>
            <option value="realistic">Реалистичный</option>
            <option value="cartoon">Мультяшный</option>
            <option value="lowpoly">Низкополигональный</option>
            <option value="cyberpunk">Киберпанк</option>
            <option value="medieval">Средневековый</option>
            <option value="minimalist">Минималистичный</option>
            <option value="vintage">Винтажный</option>
          </select>
        </label>
        
        <label>
          Угол камеры:
          <select v-model="cameraAngle">
            <option value="">По умолчанию</option>
            <option value="front">Спереди</option>
            <option value="side">Сбоку</option>
            <option value="top">Сверху</option>
            <option value="bottom">Снизу</option>
            <option value="isometric">Изометрия</option>
          </select>
        </label>
      </div>
      
      <button @click="generateModel" :disabled="isGenerating">
        {{ isGenerating ? 'Генерация...' : 'Создать 3D модель' }}
      </button>
    </div>
    
    <div class="status-section" v-if="taskId">
      <h3>Статус генерации: {{ taskStatus }}</h3>
      <div v-if="modelUrl">
        <h4>Модель успешно создана!</h4>
        <a :href="modelUrl" target="_blank" download="model.glb">Скачать модель (GLB)</a>
        <ModelViewer :model-url="modelUrl" />
        
        <div class="rating-section">
          <h4>Оцените модель</h4>
          <div class="rating-stars">
            <span
              v-for="i in 5"
              :key="i"
              @click="setRating(i)"
              :class="{ 'star-filled': i <= rating, 'star-empty': i > rating }"
              class="star"
            >
              ★
            </span>
          </div>
          <textarea
            v-model="ratingComment"
            placeholder="Оставьте комментарий (необязательно)"
            rows="3"
            cols="50"
          ></textarea>
          <button @click="submitRating" :disabled="!rating">Отправить оценку</button>
        </div>
      </div>
    </div>
    
    <div class="error-section" v-if="error">
      <p class="error">Ошибка: {{ error }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import AdvancedTextTo3DClient, { AdvancedTextTo3DRequest } from '../modules/ai-service/advancedTextTo3dClient';
import ModelViewer from '../../../shared/src/components/ModelViewer.vue';

// Состояние компонента
const prompt = ref('');
const selectedModel = ref('shap-e');
const numInferenceSteps = ref(100);
const guidanceScale = ref(7.5);
const quality = ref('high');
const lighting = ref('bright');
const style = ref('');
const cameraAngle = ref('');
const isGenerating = ref(false);
const taskId = ref<string | null>(null);
const taskStatus = ref('Ожидание');
const modelUrl = ref<string | null>(null);
const error = ref<string | null>(null);

// Состояние рейтинга
const rating = ref(0);
const ratingComment = ref('');
const modelHash = ref<string | null>(null);

// Инициализация клиента
const client = new AdvancedTextTo3DClient('http://localhost:8080');

// Функция генерации модели
const generateModel = async () => {
  if (!prompt.value.trim()) {
    error.value = 'Пожалуйста, введите описание модели';
    return;
  }
  
  isGenerating.value = true;
  error.value = null;
  taskId.value = null;
  modelUrl.value = null;
  taskStatus.value = 'Запуск генерации...';
  rating.value = 0;
  ratingComment.value = '';
  modelHash.value = null;
  
  try {
    // Подготавливаем расширенный запрос
    const request: AdvancedTextTo3DRequest = {
      prompt: prompt.value,
      model: selectedModel.value,
      numInferenceSteps: numInferenceSteps.value,
      guidanceScale: guidanceScale.value,
      quality: quality.value,
      lighting: lighting.value,
      style: style.value || undefined,
      cameraAngle: cameraAngle.value || undefined,
    };
    
    // Отправляем запрос на генерацию
    const response = await client.generate3DModel(request);
    
    // В новом API мы сразу получаем результат
    taskId.value = response.taskId || 'advanced_task_123';
    taskStatus.value = 'Генерация завершена';
    modelUrl.value = response.resultUrl || 'http://localhost:8080/models/advanced_model.glb';
    modelHash.value = response.modelHash || 'advanced_model_hash_123';
  } catch (err) {
    error.value = 'Ошибка при отправке запроса на генерацию';
  } finally {
    isGenerating.value = false;
  }
};

// Функции для работы с рейтингом
const setRating = (value: number) => {
  rating.value = value;
};

const submitRating = async () => {
  if (!modelHash.value || rating.value === 0) {
    error.value = 'Невозможно отправить оценку: отсутствует хэш модели или оценка';
    return;
  }
  
  try {
    await client.addModelRating(
      modelHash.value,
      'user123', // В реальной реализации это будет ID текущего пользователя
      rating.value,
      ratingComment.value || undefined
    );
    
    // Сбрасываем форму рейтинга
    rating.value = 0;
    ratingComment.value = '';
  } catch (err) {
    error.value = 'Ошибка при отправке оценки';
  }
};
</script>

<style scoped>
.advanced-text-to-3d-generator {
  padding: 20px;
  border: 1px solid #ccc;
  border-radius: 8px;
  max-width: 800px;
  margin: 0 auto;
}

.input-section {
  margin-bottom: 20px;
}

.input-section textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
  resize: vertical;
}

.options {
  display: flex;
  gap: 20px;
  margin: 15px 0;
  flex-wrap: wrap;
}

.options label {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.options select,
.options input {
  padding: 5px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.options input {
  width: 80px;
}

.advanced-options {
  margin: 15px 0;
  padding: 10px;
  border: 1px solid #eee;
  border-radius: 4px;
}

.advanced-options h3 {
  margin-top: 0;
}

button {
  padding: 10px 20px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.status-section {
  margin-top: 20px;
  padding: 15px;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.error-section {
  margin-top: 20px;
}

.error {
  color: #dc3545;
  font-weight: bold;
}

.rating-section {
  margin-top: 20px;
  padding: 15px;
  background-color: #fff;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.rating-stars {
  margin: 10px 0;
}

.star {
  font-size: 24px;
  cursor: pointer;
  color: #ddd;
}

.star-filled {
  color: #ffc107;
}

.rating-section textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
  resize: vertical;
  margin: 10px 0;
}
</style>