import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { ref } from 'vue';

// Мокаем все зависимости для отладочного теста
const mockProjectStore = {
  currentProject: ref({
    id: 'debug-project',
    name: 'Debug Project',
    model: null,
    unfoldResult: null,
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

const mockPaperOptimizeClient = {
  optimizePaperLayout: vi.fn(),
  analyzeModelForPaper: vi.fn(),
  generateAssemblyTips: vi.fn()
};

const mockUnfoldClient = {
  generateUnfold: vi.fn()
};

const mockAutoSaveStore = {
  enableAutoSave: vi.fn(),
  disableAutoSave: vi.fn(),
  saveProject: vi.fn(),
  isAutoSaveEnabled: ref(true)
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

vi.mock('../../modules/paper-optimization/paperOptimizeClient', () => ({
  ...mockPaperOptimizeClient
}));

vi.mock('../../modules/unfold/unfoldClient', () => ({
  ...mockUnfoldClient
}));

vi.mock('../../stores/autoSaveStore', () => ({
  useAutoSaveStore: () => mockAutoSaveStore
}));

vi.mock('../../composables/useNotifications', () => ({
  useNotifications: () => mockNotifications
}));

// Импортируем основные компоненты для тестирования
import TextTo3DView from '../../views/TextTo3DView.vue';
import ImageTo3DView from '../../views/ImageTo3DView.vue';
import UnfoldStage from '../../components/stages/UnfoldStage.vue';
import AutoSaveIndicator from '../../components/auto-save/AutoSaveIndicator.vue';

describe('System Debug and Integration Test', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    
    // Сброс состояния хранилища
    mockProjectStore.currentProject.value = {
      id: 'debug-project',
      name: 'Debug Project',
      model: null,
      unfoldResult: null,
      history: []
    };
  });

  afterEach(() => {
    // Очистка после каждого теста
    vi.restoreAllMocks();
  });

  describe('Complete Workflow Debug Test', () => {
    it('should execute complete workflow from text to optimized paper layout', async () => {
      // Настраиваем моки для всего процесса
      mockTextTo3DClient.generateModel.mockResolvedValue({
        taskId: 'task-text-123',
        status: 'processing'
      });
      
      mockTextTo3DClient.checkGenerationStatus.mockResolvedValue({
        status: 'completed',
        progress: 100
      });
      
      mockTextTo3DClient.getModelResult.mockResolvedValue({
        model: {
          vertices: [{ x: 0, y: 0, z: 0 }, { x: 1, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }],
          faces: [{ vertexIndices: [0, 1, 2] }]
        },
        previewImage: 'base64image'
      });
      
      mockUnfoldClient.generateUnfold.mockResolvedValue({
        faces: [
          {
            id: 0,
            vertices: [
              { x: 0, y: 0 },
              { x: 10, y: 0 },
              { x: 10, y: 10 },
              { x: 0, y: 10 }
            ],
            seams: []
          }
        ],
        seams: []
      });
      
      mockPaperOptimizeClient.optimizePaperLayout.mockResolvedValue({
        faces: [
          {
            id: 0,
            vertices: [
              { x: 5, y: 5 },
              { x: 15, y: 5 },
              { x: 15, y: 15 },
              { x: 5, y: 15 }
            ],
            position: { x: 5, y: 5 },
            rotation: 0
          }
        ],
        width: 210,
        height: 297
      });
      
      mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
        faceCount: 1,
        estimatedSheetCount: 1,
        complexityScore: 10.0
      });
      
      mockPaperOptimizeClient.generateAssemblyTips.mockResolvedValue([
        "Аккуратно вырежьте детали по контуру",
        "Склейте вкладыши перед финальной сборкой"
      ]);

      // Тест 1: Text-to-3D Generation
      console.log('=== Testing Text-to-3D Generation ===');
      
      const textTo3DWrapper = mount(TextTo3DView, {
        props: {
          modelValue: ''
        }
      });

      // Вводим текстовое описание
      const textInput = textTo3DWrapper.find('textarea');
      await textInput.setValue('Simple pyramid for testing');
      
      // Нажимаем кнопку генерации
      const generateButton = textTo3DWrapper.find('button.generate-btn');
      await generateButton.trigger('click');

      // Проверяем вызовы API
      expect(mockTextTo3DClient.generateModel).toHaveBeenCalledWith(
        'Simple pyramid for testing'
      );
      
      // Ждем завершения генерации
      await textTo3DWrapper.vm.$nextTick();
      
      // Проверяем, что модель была получена
      expect(mockTextTo3DClient.getModelResult).toHaveBeenCalled();
      
      // Проверяем, что проект был сохранен
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      expect(mockAutoSaveStore.saveProject).toHaveBeenCalled();
      
      // Проверяем, что модель была сохранена в проекте
      expect(mockProjectStore.currentProject.value.model).toBeDefined();
      expect(mockProjectStore.currentProject.value.model.vertices.length).toBe(3);
      expect(mockProjectStore.currentProject.value.model.faces.length).toBe(1);
      
      console.log('✓ Text-to-3D Generation completed successfully');

      // Тест 2: Unfold Generation
      console.log('=== Testing Unfold Generation ===');
      
      const unfoldWrapper = mount(UnfoldStage);

      // Нажимаем кнопку генерации развертки
      const unfoldButton = unfoldWrapper.find('button.generate-unfold-btn');
      await unfoldButton.trigger('click');

      // Проверяем вызовы API развертки
      expect(mockUnfoldClient.generateUnfold).toHaveBeenCalled();
      
      // Ждем завершения развертки
      await unfoldWrapper.vm.$nextTick();
      
      // Проверяем, что развертка была сохранена
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      expect(mockProjectStore.currentProject.value.unfoldResult).toBeDefined();
      expect(mockProjectStore.currentProject.value.unfoldResult.faces.length).toBe(1);
      
      console.log('✓ Unfold Generation completed successfully');

      // Тест 3: Paper Optimization
      console.log('=== Testing Paper Optimization ===');
      
      // Нажимаем кнопку оптимизации бумаги
      const optimizeButton = unfoldWrapper.find('button.optimize-paper-btn');
      await optimizeButton.trigger('click');
      
      // Проверяем, что открылось модальное окно оптимизации
      const modal = unfoldWrapper.findComponent({ name: 'PaperOptimizationModal' });
      expect(modal.exists()).toBe(true);
      
      // Нажимаем кнопку оптимизации в модальном окне
      const modalOptimizeButton = modal.find('button.optimize-btn');
      await modalOptimizeButton.trigger('click');
      
      // Проверяем вызовы API оптимизации
      expect(mockPaperOptimizeClient.optimizePaperLayout).toHaveBeenCalled();
      expect(mockPaperOptimizeClient.analyzeModelForPaper).toHaveBeenCalled();
      
      // Эмитируем событие применения оптимизации
      await modal.vm.$emit('apply', {
        faces: [
          {
            id: 0,
            vertices: [
              { x: 5, y: 5 },
              { x: 15, y: 5 },
              { x: 15, y: 15 },
              { x: 5, y: 15 }
            ],
            position: { x: 5, y: 5 },
            rotation: 0
          }
        ],
        width: 210,
        height: 297
      });
      
      // Проверяем, что оптимизированная развертка была сохранена
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      expect(mockAutoSaveStore.saveProject).toHaveBeenCalled();
      expect(mockProjectStore.currentProject.value.unfoldResult.optimizedLayout).toBeDefined();
      
      console.log('✓ Paper Optimization completed successfully');

      // Тест 4: Auto-save System
      console.log('=== Testing Auto-save System ===');
      
      const autoSaveIndicator = mount(AutoSaveIndicator);
      
      // Проверяем, что индикатор автосохранения отображается правильно
      expect(autoSaveIndicator.find('.save-status').text()).toContain('Сохранено');
      expect(autoSaveIndicator.find('.auto-save-enabled').exists()).toBe(true);
      
      console.log('✓ Auto-save System working correctly');

      // Тест 5: Assembly Tips Generation
      console.log('=== Testing Assembly Tips Generation ===');
      
      // Нажимаем кнопку советов по сборке
      const tipsButton = unfoldWrapper.find('button.tips-btn');
      await tipsButton.trigger('click');
      
      // Проверяем вызовы API для генерации советов
      expect(mockPaperOptimizeClient.generateAssemblyTips).toHaveBeenCalled();
      
      console.log('✓ Assembly Tips Generation completed successfully');

      // Финальная проверка состояния проекта
      console.log('=== Final Project State Verification ===');
      
      const finalProject = mockProjectStore.currentProject.value;
      
      // Проверяем, что все данные проекта корректны
      expect(finalProject.id).toBe('debug-project');
      expect(finalProject.name).toBe('Debug Project');
      expect(finalProject.model).toBeDefined();
      expect(finalProject.unfoldResult).toBeDefined();
      expect(finalProject.unfoldResult.optimizedLayout).toBeDefined();
      expect(finalProject.history.length).toBeGreaterThan(0);
      
      console.log('✓ Final Project State is correct');
      console.log('=== Complete Workflow Test PASSED ===');
    });

    it('should handle errors gracefully at each stage', async () => {
      // Настраиваем моки для ошибок на каждом этапе
      mockTextTo3DClient.generateModel.mockRejectedValue(
        new Error('Text-to-3D generation failed')
      );
      
      mockUnfoldClient.generateUnfold.mockRejectedValue(
        new Error('Unfold generation failed')
      );
      
      mockPaperOptimizeClient.optimizePaperLayout.mockRejectedValue(
        new Error('Paper optimization failed')
      );

      console.log('=== Testing Error Handling ===');
      
      // Тест 1: Ошибка Text-to-3D
      const textTo3DWrapper = mount(TextTo3DView, {
        props: {
          modelValue: ''
        }
      });

      const textInput = textTo3DWrapper.find('textarea');
      await textInput.setValue('Error test model');
      
      const generateButton = textTo3DWrapper.find('button.generate-btn');
      await generateButton.trigger('click');

      // Проверяем, что отображается уведомление об ошибке
      expect(mockNotifications.show).toHaveBeenCalledWith(
        'Ошибка генерации', 
        'Не удалось сгенерировать модель', 
        'error'
      );
      
      console.log('✓ Text-to-3D Error Handling working correctly');

      // Тест 2: Ошибка развертки
      const unfoldWrapper = mount(UnfoldStage);
      
      const unfoldButton = unfoldWrapper.find('button.generate-unfold-btn');
      await unfoldButton.trigger('click');

      // Проверяем, что отображается уведомление об ошибке
      expect(mockNotifications.show).toHaveBeenCalledWith(
        'Ошибка развертки', 
        'Не удалось сгенерировать развертку модели', 
        'error'
      );
      
      console.log('✓ Unfold Error Handling working correctly');

      // Тест 3: Ошибка оптимизации бумаги
      const optimizeButton = unfoldWrapper.find('button.optimize-paper-btn');
      await optimizeButton.trigger('click');
      
      const modal = unfoldWrapper.findComponent({ name: 'PaperOptimizationModal' });
      const modalOptimizeButton = modal.find('button.optimize-btn');
      await modalOptimizeButton.trigger('click');

      // Проверяем, что отображается уведомление об ошибке
      expect(mockNotifications.show).toHaveBeenCalledWith(
        'Ошибка оптимизации', 
        'Не удалось оптимизировать размещение деталей', 
        'error'
      );
      
      console.log('✓ Paper Optimization Error Handling working correctly');
      console.log('=== Error Handling Test PASSED ===');
    });

    it('should maintain data consistency throughout the workflow', async () => {
      // Настраиваем моки для успешного выполнения всех этапов
      mockTextTo3DClient.generateModel.mockResolvedValue({
        taskId: 'task-consistency-123',
        status: 'completed'
      });
      
      mockTextTo3DClient.getModelResult.mockResolvedValue({
        model: {
          vertices: [{ x: 0, y: 0, z: 0 }, { x: 1, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }],
          faces: [{ vertexIndices: [0, 1, 2] }]
        },
        previewImage: 'base64image'
      });
      
      mockUnfoldClient.generateUnfold.mockResolvedValue({
        faces: [
          {
            id: 0,
            vertices: [
              { x: 0, y: 0 },
              { x: 10, y: 0 },
              { x: 10, y: 10 },
              { x: 0, y: 10 }
            ],
            seams: []
          }
        ],
        seams: []
      });
      
      mockPaperOptimizeClient.optimizePaperLayout.mockResolvedValue({
        faces: [
          {
            id: 0,
            vertices: [
              { x: 5, y: 5 },
              { x: 15, y: 5 },
              { x: 15, y: 15 },
              { x: 5, y: 15 }
            ],
            position: { x: 5, y: 5 },
            rotation: 0
          }
        ],
        width: 210,
        height: 297
      });

      console.log('=== Testing Data Consistency ===');
      
      // Выполняем весь процесс
      const textTo3DWrapper = mount(TextTo3DView, {
        props: {
          modelValue: ''
        }
      });

      // Генерация модели
      const textInput = textTo3DWrapper.find('textarea');
      await textInput.setValue('Consistency test model');
      
      const generateButton = textTo3DWrapper.find('button.generate-btn');
      await generateButton.trigger('click');
      
      // Генерация развертки
      const unfoldWrapper = mount(UnfoldStage);
      const unfoldButton = unfoldWrapper.find('button.generate-unfold-btn');
      await unfoldButton.trigger('click');
      
      // Оптимизация бумаги
      const optimizeButton = unfoldWrapper.find('button.optimize-paper-btn');
      await optimizeButton.trigger('click');
      
      const modal = unfoldWrapper.findComponent({ name: 'PaperOptimizationModal' });
      const modalOptimizeButton = modal.find('button.optimize-btn');
      await modalOptimizeButton.trigger('click');
      
      await modal.vm.$emit('apply', {
        faces: [
          {
            id: 0,
            vertices: [
              { x: 5, y: 5 },
              { x: 15, y: 5 },
              { x: 15, y: 15 },
              { x: 5, y: 15 }
            ],
            position: { x: 5, y: 5 },
            rotation: 0
          }
        ],
        width: 210,
        height: 297
      });

      // Проверяем согласованность данных
      const project = mockProjectStore.currentProject.value;
      
      // Проверяем, что все этапы имеют связанные данные
      expect(project.model).toBeDefined();
      expect(project.unfoldResult).toBeDefined();
      expect(project.unfoldResult.optimizedLayout).toBeDefined();
      
      // Проверяем, что количество вершин и граней согласовано
      expect(project.model.vertices.length).toBe(3);
      expect(project.model.faces.length).toBe(1);
      expect(project.unfoldResult.faces.length).toBe(1);
      expect(project.unfoldResult.optimizedLayout.faces.length).toBe(1);
      
      // Проверяем, что история проекта содержит все этапы
      expect(project.history.length).toBeGreaterThan(0);
      const historyDescriptions = project.history.map(h => h.description);
      expect(historyDescriptions.some(desc => desc.includes('AI генерация'))).toBe(true);
      expect(historyDescriptions.some(desc => desc.includes('развертка'))).toBe(true);
      expect(historyDescriptions.some(desc => desc.includes('оптимизация'))).toBe(true);
      
      console.log('✓ Data Consistency maintained throughout workflow');
      console.log('=== Data Consistency Test PASSED ===');
    });
  });
});