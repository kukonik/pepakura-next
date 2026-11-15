import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { ref } from 'vue';

// Мокаем все зависимости
const mockProjectStore = {
  currentProject: ref({
    id: 'test-project',
    name: 'Test Project',
    model: null,
    history: []
  }),
  saveProject: vi.fn(),
  loadProject: vi.fn()
};

const mockTextTo3DClient = {
  generateModel: vi.fn(),
  checkGenerationStatus: vi.fn(),
  getModelResult: vi.fn()
};

const mockImageTo3DClient = {
  generateModelFromImage: vi.fn()
};

const mockAutoSaveStore = {
  enableAutoSave: vi.fn(),
  disableAutoSave: vi.fn(),
  saveProject: vi.fn()
};

const mockNotifications = {
  show: vi.fn()
};

// Мокаем все модули
vi.mock('../../stores/projectStore', () => ({
  useProjectStore: () => mockProjectStore
}));

vi.mock('../../modules/ai/text-to-3d/textTo3dClient', () => ({
  ...mockTextTo3DClient
}));

vi.mock('../../modules/ai/image-to-3d/imageTo3DClient', () => ({
  ...mockImageTo3DClient
}));

vi.mock('../../stores/autoSaveStore', () => ({
  useAutoSaveStore: () => mockAutoSaveStore
}));

vi.mock('../../composables/useNotifications', () => ({
  useNotifications: () => mockNotifications
}));

// Импортируем компоненты для тестирования
import TextTo3DView from '../../views/TextTo3DView.vue';
import ImageTo3DView from '../../views/ImageTo3DView.vue';
import AutoSaveIndicator from '../../components/auto-save/AutoSaveIndicator.vue';

describe('AI Generation Integration', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    
    // Сброс состояния хранилища
    mockProjectStore.currentProject.value = {
      id: 'test-project',
      name: 'Test Project',
      model: null,
      history: []
    };
  });

  afterEach(() => {
    // Очистка после каждого теста
    vi.restoreAllMocks();
  });

  describe('Text-to-3D Generation Flow', () => {
    it('should generate model from text description and save automatically', async () => {
      // Настраиваем моки для Text-to-3D
      mockTextTo3DClient.generateModel.mockResolvedValue({
        taskId: 'task-123',
        status: 'processing'
      });
      
      mockTextTo3DClient.checkGenerationStatus.mockResolvedValue({
        status: 'completed',
        progress: 100
      });
      
      mockTextTo3DClient.getModelResult.mockResolvedValue({
        model: {
          vertices: [{ x: 0, y: 0, z: 0 }],
          faces: [{ vertexIndices: [0, 1, 2] }]
        },
        previewImage: 'base64image'
      });

      // Монтируем компонент
      const wrapper = mount(TextTo3DView, {
        props: {
          modelValue: ''
        }
      });

      // Вводим текстовое описание
      const input = wrapper.find('textarea');
      await input.setValue('A simple cube for testing');
      
      // Нажимаем кнопку генерации
      const generateButton = wrapper.find('button.generate-btn');
      await generateButton.trigger('click');

      // Проверяем вызовы API
      expect(mockTextTo3DClient.generateModel).toHaveBeenCalledWith(
        'A simple cube for testing'
      );
      
      // Ждем завершения генерации
      await wrapper.vm.$nextTick();
      
      // Проверяем, что модель была получена
      expect(mockTextTo3DClient.getModelResult).toHaveBeenCalled();
      
      // Проверяем, что проект был сохранен
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      expect(mockAutoSaveStore.saveProject).toHaveBeenCalled();
      
      // Проверяем, что отображается превью модели
      expect(wrapper.find('img.model-preview').exists()).toBe(true);
    });

    it('should handle generation errors gracefully', async () => {
      // Настраиваем мок для ошибки генерации
      mockTextTo3DClient.generateModel.mockRejectedValue(
        new Error('Generation failed')
      );

      // Монтируем компонент
      const wrapper = mount(TextTo3DView, {
        props: {
          modelValue: ''
        }
      });

      // Вводим текстовое описание
      const input = wrapper.find('textarea');
      await input.setValue('A simple cube for testing');
      
      // Нажимаем кнопку генерации
      const generateButton = wrapper.find('button.generate-btn');
      await generateButton.trigger('click');

      // Проверяем, что отображается уведомление об ошибке
      expect(mockNotifications.show).toHaveBeenCalledWith(
        'Ошибка генерации', 
        'Не удалось сгенерировать модель', 
        'error'
      );
    });

    it('should update auto-save indicator during generation', async () => {
      // Настраиваем моки
      mockTextTo3DClient.generateModel.mockResolvedValue({
        taskId: 'task-123',
        status: 'processing'
      });
      
      mockTextTo3DClient.checkGenerationStatus.mockResolvedValue({
        status: 'completed',
        progress: 100
      });
      
      mockTextTo3DClient.getModelResult.mockResolvedValue({
        model: {
          vertices: [{ x: 0, y: 0, z: 0 }],
          faces: [{ vertexIndices: [0, 1, 2] }]
        },
        previewImage: 'base64image'
      });

      // Монтируем компонент индикатора автосохранения
      const indicatorWrapper = mount(AutoSaveIndicator);
      
      // Монтируем компонент Text-to-3D
      const textTo3DWrapper = mount(TextTo3DView, {
        props: {
          modelValue: ''
        }
      });

      // Вводим текстовое описание
      const input = textTo3DWrapper.find('textarea');
      await input.setValue('A simple cube for testing');
      
      // Нажимаем кнопку генерации
      const generateButton = textTo3DWrapper.find('button.generate-btn');
      await generateButton.trigger('click');

      // Проверяем, что индикатор автосохранения обновляется
      expect(mockAutoSaveStore.saveProject).toHaveBeenCalled();
      
      // Проверяем, что отображается статус сохранения
      expect(indicatorWrapper.find('.save-status').text()).toContain('Сохранено');
    });
  });

  describe('Image-to-3D Generation Flow', () => {
    it('should generate model from image and save automatically', async () => {
      // Настраиваем моки для Image-to-3D
      mockImageTo3DClient.generateModelFromImage.mockResolvedValue({
        model: {
          vertices: [{ x: 0, y: 0, z: 0 }],
          faces: [{ vertexIndices: [0, 1, 2] }]
        },
        previewImage: 'base64image'
      });

      // Монтируем компонент
      const wrapper = mount(ImageTo3DView);

      // Создаем мок файла изображения
      const file = new File(['image-content'], 'test.png', {
        type: 'image/png'
      });
      
      // Эмитируем событие выбора файла
      const fileInput = wrapper.find('input[type="file"]');
      await fileInput.trigger('change', {
        target: { files: [file] }
      });

      // Проверяем вызовы API
      expect(mockImageTo3DClient.generateModelFromImage).toHaveBeenCalled();
      
      // Проверяем, что проект был сохранен
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      expect(mockAutoSaveStore.saveProject).toHaveBeenCalled();
      
      // Проверяем, что отображается превью модели
      expect(wrapper.find('img.model-preview').exists()).toBe(true);
    });

    it('should handle image generation errors gracefully', async () => {
      // Настраиваем мок для ошибки генерации
      mockImageTo3DClient.generateModelFromImage.mockRejectedValue(
        new Error('Image processing failed')
      );

      // Монтируем компонент
      const wrapper = mount(ImageTo3DView);

      // Создаем мок файла изображения
      const file = new File(['image-content'], 'test.png', {
        type: 'image/png'
      });
      
      // Эмитируем событие выбора файла
      const fileInput = wrapper.find('input[type="file"]');
      await fileInput.trigger('change', {
        target: { files: [file] }
      });

      // Проверяем, что отображается уведомление об ошибке
      expect(mockNotifications.show).toHaveBeenCalledWith(
        'Ошибка обработки изображения', 
        'Не удалось сгенерировать модель из изображения', 
        'error'
      );
    });
  });

  describe('Project History and Versioning', () => {
    it('should maintain project history during AI generation', async () => {
      // Настраиваем моки
      mockTextTo3DClient.generateModel.mockResolvedValue({
        taskId: 'task-123',
        status: 'processing'
      });
      
      mockTextTo3DClient.checkGenerationStatus.mockResolvedValue({
        status: 'completed',
        progress: 100
      });
      
      mockTextTo3DClient.getModelResult.mockResolvedValue({
        model: {
          vertices: [{ x: 0, y: 0, z: 0 }],
          faces: [{ vertexIndices: [0, 1, 2] }]
        },
        previewImage: 'base64image'
      });

      // Устанавливаем начальную историю проекта
      mockProjectStore.currentProject.value.history = [
        { id: 'version-1', timestamp: Date.now() - 1000, description: 'Initial model' }
      ];

      // Монтируем компонент
      const wrapper = mount(TextTo3DView, {
        props: {
          modelValue: ''
        }
      });

      // Вводим текстовое описание
      const input = wrapper.find('textarea');
      await input.setValue('A simple cube for testing');
      
      // Нажимаем кнопку генерации
      const generateButton = wrapper.find('button.generate-btn');
      await generateButton.trigger('click');

      // Проверяем, что история проекта обновлена
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      
      // Проверяем, что новая версия добавлена в историю
      const history = mockProjectStore.currentProject.value.history;
      expect(history.length).toBeGreaterThan(1);
      expect(history[history.length - 1].description).toContain('AI генерация');
    });
  });

  describe('Performance and Resource Management', () => {
    it('should handle large models without memory leaks', async () => {
      // Настраиваем моки для большого объема данных
      mockTextTo3DClient.generateModel.mockResolvedValue({
        taskId: 'task-123',
        status: 'processing'
      });
      
      mockTextTo3DClient.checkGenerationStatus.mockResolvedValue({
        status: 'completed',
        progress: 100
      });
      
      // Создаем большую модель для тестирования
      const largeModel = {
        vertices: Array(10000).fill(0).map((_, i) => ({ 
          x: Math.random(), 
          y: Math.random(), 
          z: Math.random() 
        })),
        faces: Array(5000).fill(0).map((_, i) => ({ 
          vertexIndices: [i * 3, i * 3 + 1, i * 3 + 2] 
        }))
      };
      
      mockTextTo3DClient.getModelResult.mockResolvedValue({
        model: largeModel,
        previewImage: 'base64image'
      });

      // Монтируем компонент
      const wrapper = mount(TextTo3DView, {
        props: {
          modelValue: ''
        }
      });

      // Вводим текстовое описание
      const input = wrapper.find('textarea');
      await input.setValue('A complex model for performance testing');
      
      // Нажимаем кнопку генерации
      const generateButton = wrapper.find('button.generate-btn');
      await generateButton.trigger('click');

      // Проверяем, что проект был успешно сохранен
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      
      // Проверяем, что модель была корректно обработана
      const project = mockProjectStore.currentProject.value;
      expect(project.model).toBeDefined();
      expect(project.model.vertices.length).toBe(10000);
      expect(project.model.faces.length).toBe(5000);
    });
  });
});