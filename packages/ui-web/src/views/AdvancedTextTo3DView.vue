<template>
  <div class="advanced-text-to-3d-view">
    <header>
      <h1>Продвинутая генерация 3D моделей</h1>
      <p>Создавайте сложные 3D модели с помощью текстового описания и современных ИИ моделей</p>
    </header>
    
    <main>
      <AdvancedTextTo3DGenerator />
    </main>
    
    <section class="model-examples">
      <h2>Примеры моделей</h2>
      <div class="example-grid">
        <div class="example-item" v-for="example in examples" :key="example.id">
          <img :src="example.image" :alt="example.title" />
          <h3>{{ example.title }}</h3>
          <p>{{ example.description }}</p>
          <button @click="loadExample(example.prompt)">Использовать пример</button>
        </div>
      </div>
    </section>
    
    <section class="top-rated-models">
      <h2>Лучшие модели</h2>
      <div v-if="topRatedModels.length === 0">
        <p>Пока нет оцененных моделей</p>
      </div>
      <div v-else>
        <div class="model-grid">
          <div class="model-item" v-for="model in topRatedModels" :key="model.hash">
            <h3>{{ model.prompt }}</h3>
            <p>Рейтинг: {{ model.averageRating.toFixed(1) }} ({{ model.totalRatings }} оценок)</p>
            <div class="model-item-actions">
              <button @click="downloadModel(model.modelUrl)">Скачать</button>
              <button @click="viewModel(model.modelUrl)">Просмотреть</button>
            </div>
          </div>
        </div>
      </div>
    </section>
    
    <section class="model-history">
      <h2>История генераций</h2>
      <div v-if="generationHistory.length === 0">
        <p>Пока нет сохраненных генераций</p>
      </div>
      <div v-else>
        <div class="history-item" v-for="item in generationHistory" :key="item.id">
          <div class="history-item-content">
            <h3>{{ item.prompt }}</h3>
            <p>Модель: {{ item.model }} | Дата: {{ formatDate(item.date) }}</p>
            <div class="history-item-actions">
              <button @click="downloadModel(item.modelUrl)">Скачать</button>
              <button @click="viewModel(item.modelUrl)">Просмотреть</button>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import AdvancedTextTo3DGenerator from '../components/AdvancedTextTo3DGenerator.vue';
import { useModelHistory } from '../composables/useModelHistory';
import AdvancedTextTo3DClient from '../modules/ai-service/advancedTextTo3dClient';

// Примеры моделей
const examples = ref([
  {
    id: 1,
    title: "Киберпанк город",
    description: "Футуристический город в стиле киберпанк с небоскребами и неоновыми вывесками",
    prompt: "A futuristic cyberpunk city with skyscrapers and neon signs, detailed, high quality",
    image: "/images/cyberpunk-city.jpg"
  },
  {
    id: 2,
    title: "Фэнтезийный дракон",
    description: "Величественный дракон в фэнтезийном стиле с детализированной текстурой",
    prompt: "A majestic fantasy dragon with detailed texture, realistic style",
    image: "/images/fantasy-dragon.jpg"
  },
  {
    id: 3,
    title: "Реалистичный автомобиль",
    description: "Спортивный автомобиль в реалистичном стиле с хромированными деталями",
    prompt: "A realistic sports car with chrome details, high quality, detailed",
    image: "/images/sports-car.jpg"
  },
  {
    id: 4,
    title: "Минималистичный стол",
    description: "Современный стол в минималистичном стиле с деревянной столешницей",
    prompt: "A modern minimalist table with wooden top, clean design, realistic style",
    image: "/images/minimalist-table.jpg"
  },
  {
    id: 5,
    title: "Винтажная камера",
    description: "Классическая винтажная камера с кожаными деталями",
    prompt: "A classic vintage camera with leather details, retro style, detailed texture",
    image: "/images/vintage-camera.jpg"
  },
  {
    id: 6,
    title: "Киберпанк робот",
    description: "Робот в стиле киберпанк с неоновыми акцентами",
    prompt: "A cyberpunk robot with neon accents, futuristic style, detailed design",
    image: "/images/cyberpunk-robot.jpg"
  }
]);

// Лучшие модели
const topRatedModels = ref<Array<{
  hash: string;
  prompt: string;
  averageRating: number;
  totalRatings: number;
  modelUrl: string;
}>>([]);

// История генераций
const { generationHistory, addGeneration, formatDate } = useModelHistory();

// Инициализация клиента
const client = new AdvancedTextTo3DClient('http://localhost:8080');

// Функция загрузки примера
const loadExample = (prompt: string) => {
  // Отправляем событие с промптом в генератор
  const event = new CustomEvent('load-example-prompt', { detail: prompt });
  window.dispatchEvent(event);
};

// Функция скачивания модели
const downloadModel = (url: string) => {
  const link = document.createElement('a');
  link.href = url;
  link.download = 'model.glb';
  link.click();
};

// Функция просмотра модели
const viewModel = (url: string) => {
  // Открываем модель в просмотрщике
  const event = new CustomEvent('view-model', { detail: url });
  window.dispatchEvent(event);
};

// Загрузка лучших моделей
const loadTopRatedModels = async () => {
  try {
    // В реальной реализации здесь будет вызов API для получения лучших моделей
    // Пока используем заглушку
    topRatedModels.value = [
      {
        hash: "abc123",
        prompt: "Киберпанк город",
        averageRating: 4.8,
        totalRatings: 24,
        modelUrl: "/models/cyberpunk-city.glb"
      },
      {
        hash: "def456",
        prompt: "Фэнтезийный дракон",
        averageRating: 4.6,
        totalRatings: 18,
        modelUrl: "/models/fantasy-dragon.glb"
      },
      {
        hash: "ghi789",
        prompt: "Реалистичный автомобиль",
        averageRating: 4.5,
        totalRatings: 15,
        modelUrl: "/models/realistic-car.glb"
      }
    ];
  } catch (err) {
    console.error('Ошибка при загрузке лучших моделей:', err);
  }
};

// При монтировании компонента
onMounted(() => {
  // Добавляем обработчик события для загрузки примера
  window.addEventListener('load-example-prompt' as any, (event: CustomEvent) => {
    const prompt = event.detail;
    // Здесь можно обновить промпт в генераторе
    console.log('Загрузка примера:', prompt);
  });
  
  // Добавляем обработчик события для просмотра модели
  window.addEventListener('view-model' as any, (event: CustomEvent) => {
    const modelUrl = event.detail;
    // Здесь можно открыть просмотрщик модели
    console.log('Просмотр модели:', modelUrl);
  });
  
  // Загружаем лучшие модели
  loadTopRatedModels();
});
</script>

<style scoped>
.advanced-text-to-3d-view {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

header {
  text-align: center;
  margin-bottom: 40px;
}

header h1 {
  font-size: 2.5rem;
  margin-bottom: 10px;
  color: #333;
}

header p {
  font-size: 1.2rem;
  color: #666;
}

.model-examples {
  margin: 50px 0;
}

.model-examples h2 {
  text-align: center;
  margin-bottom: 30px;
  font-size: 2rem;
  color: #333;
}

.example-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 30px;
}

.example-item {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 20px;
  text-align: center;
  background-color: #fff;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.example-item img {
  max-width: 100%;
  height: 200px;
  object-fit: cover;
  border-radius: 4px;
  margin-bottom: 15px;
}

.example-item h3 {
  margin: 10px 0;
  color: #333;
}

.example-item p {
  color: #666;
  margin-bottom: 15px;
}

.example-item button {
  padding: 8px 16px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
}

.example-item button:hover {
  background-color: #0056b3;
}

.top-rated-models {
  margin: 50px 0;
}

.top-rated-models h2 {
  text-align: center;
  margin-bottom: 30px;
  font-size: 2rem;
  color: #333;
}

.model-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 20px;
}

.model-item {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 15px;
  background-color: #fff;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

.model-item h3 {
  margin: 0 0 10px 0;
  color: #333;
  font-size: 1.1rem;
}

.model-item p {
  color: #666;
  margin-bottom: 15px;
  font-size: 0.9rem;
}

.model-item-actions {
  display: flex;
  gap: 10px;
}

.model-item-actions button {
  padding: 5px 10px;
  background-color: #28a745;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
}

.model-item-actions button:last-child {
  background-color: #17a2b8;
}

.model-history {
  margin: 50px 0;
}

.model-history h2 {
  text-align: center;
  margin-bottom: 30px;
  font-size: 2rem;
  color: #333;
}

.history-item {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
  background-color: #fff;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.history-item-content h3 {
  margin: 0 0 10px 0;
  color: #333;
}

.history-item-content p {
  color: #666;
  margin-bottom: 15px;
}

.history-item-actions {
  display: flex;
  gap: 10px;
}

.history-item-actions button {
  padding: 6px 12px;
  background-color: #28a745;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
}

.history-item-actions button:last-child {
  background-color: #17a2b8;
}

@media (max-width: 768px) {
  .example-grid,
  .model-grid {
    grid-template-columns: 1fr;
  }
  
  header h1 {
    font-size: 2rem;
  }
  
  header p {
    font-size: 1rem;
  }
}
</style>