import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { ref } from 'vue';

// Мокаем все зависимости
const mockProjectStore = {
  currentProject: ref({
    id: 'test-project',
    name: 'Test Project',
    model: {
      vertices: [{ x: 0, y: 0, z: 0 }],
      faces: [{ vertexIndices: [0, 1, 2] }]
    },
    unfoldResult: null,
    history: []
  }),
  saveProject: vi.fn(),
  loadProject: vi.fn()
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
  saveProject: vi.fn()
};

const mockNotifications = {
  show: vi.fn()
};

// Мокаем все модули
vi.mock('../../stores/projectStore', () => ({
  useProjectStore: () => mockProjectStore
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

// Импортируем компоненты для тестирования
import UnfoldStage from '../../components/stages/UnfoldStage.vue';
import PaperOptimizationPanel from '../../components/paper-optimization/PaperOptimizationPanel.vue';
import PaperOptimizationModal from '../../components/paper-optimization/PaperOptimizationModal.vue';

describe('Paper Optimization Integration', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    
    // Сброс состояния хранилища
    mockProjectStore.currentProject.value = {
      id: 'test-project',
      name: 'Test Project',
      model: {
        vertices: [{ x: 0, y: 0, z: 0 }],
        faces: [{ vertexIndices: [0, 1, 2] }]
      },
      unfoldResult: null,
      history: []
    };
  });

  afterEach(() => {
    // Очистка после каждого теста
    vi.restoreAllMocks();
  });

  describe('Complete Paper Optimization Workflow', () => {
    it('should perform full optimization workflow from model to optimized layout', async () => {
      // Настраиваем моки для развертки
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
      
      // Настраиваем моки для оптимизации бумаги
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
        faceCount: 100,
        estimatedSheetCount: 2,
        complexityScore: 65.5
      });
      
      mockPaperOptimizeClient.generateAssemblyTips.mockResolvedValue([
        "Начните сборку с центральных элементов",
        "Склейте все вкладыши перед финальной сборкой"
      ]);

      // Монтируем компонент развертки
      const wrapper = mount(UnfoldStage);

      // Нажимаем кнопку генерации развертки
      const unfoldButton = wrapper.find('button.generate-unfold-btn');
      await unfoldButton.trigger('click');

      // Проверяем вызовы API развертки
      expect(mockUnfoldClient.generateUnfold).toHaveBeenCalled();
      
      // Ждем завершения развертки
      await wrapper.vm.$nextTick();
      
      // Проверяем, что развертка была сохранена
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      
      // Нажимаем кнопку оптимизации бумаги
      const optimizeButton = wrapper.find('button.optimize-paper-btn');
      await optimizeButton.trigger('click');
      
      // Проверяем, что открылось модальное окно оптимизации
      const modal = wrapper.findComponent(PaperOptimizationModal);
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
      
      // Проверяем, что отображаются результаты оптимизации
      expect(wrapper.find('.optimized-layout').exists()).toBe(true);
    });

    it('should handle optimization errors gracefully', async () => {
      // Настраиваем мок для ошибки оптимизации
      mockPaperOptimizeClient.optimizePaperLayout.mockRejectedValue(
        new Error('Optimization failed')
      );

      // Монтируем компонент развертки
      const wrapper = mount(UnfoldStage);

      // Нажимаем кнопку оптимизации бумаги
      const optimizeButton = wrapper.find('button.optimize-paper-btn');
      await optimizeButton.trigger('click');
      
      // Проверяем, что открылось модальное окно оптимизации
      const modal = wrapper.findComponent(PaperOptimizationModal);
      expect(modal.exists()).toBe(true);
      
      // Нажимаем кнопку оптимизации в модальном окне
      const modalOptimizeButton = modal.find('button.optimize-btn');
      await modalOptimizeButton.trigger('click');
      
      // Проверяем, что отображается уведомление об ошибке
      expect(mockNotifications.show).toHaveBeenCalledWith(
        'Ошибка оптимизации', 
        'Не удалось оптимизировать размещение деталей', 
        'error'
      );
    });
  });

  describe('Paper Analysis and Assembly Tips', () => {
    it('should provide paper analysis and assembly tips', async () => {
      // Настраиваем моки для анализа
      mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
        faceCount: 150,
        estimatedSheetCount: 3,
        complexityScore: 80.5
      });
      
      mockPaperOptimizeClient.generateAssemblyTips.mockResolvedValue([
        "Рекомендуется собирать модель по частям",
        "Используйте клей-карандаш для точечной фиксации",
        "Нумеруйте элементы перед сборкой"
      ]);

      // Монтируем компонент панели оптимизации
      const wrapper = mount(PaperOptimizationPanel);

      // Нажимаем кнопку анализа
      const analyzeButton = wrapper.find('button.analyze-btn');
      await analyzeButton.trigger('click');

      // Проверяем вызовы API анализа
      expect(mockPaperOptimizeClient.analyzeModelForPaper).toHaveBeenCalled();
      
      // Ждем завершения анализа
      await wrapper.vm.$nextTick();
      
      // Проверяем, что отображаются результаты анализа
      expect(wrapper.find('.analysis-results').exists()).toBe(true);
      expect(wrapper.find('.face-count').text()).toContain('150');
      expect(wrapper.find('.sheet-count').text()).toContain('3');
      
      // Нажимаем кнопку советов по сборке
      const tipsButton = wrapper.find('button.tips-btn');
      await tipsButton.trigger('click');
      
      // Проверяем вызовы API для генерации советов
      expect(mockPaperOptimizeClient.generateAssemblyTips).toHaveBeenCalled();
      
      // Ждем завершения генерации советов
      await wrapper.vm.$nextTick();
      
      // Проверяем, что отображаются советы по сборке
      expect(wrapper.find('.assembly-tips').exists()).toBe(true);
      const tips = wrapper.findAll('.tip-item');
      expect(tips.length).toBe(3);
    });

    it('should handle analysis errors gracefully', async () => {
      // Настраиваем мок для ошибки анализа
      mockPaperOptimizeClient.analyzeModelForPaper.mockRejectedValue(
        new Error('Analysis failed')
      );

      // Монтируем компонент панели оптимизации
      const wrapper = mount(PaperOptimizationPanel);

      // Нажимаем кнопку анализа
      const analyzeButton = wrapper.find('button.analyze-btn');
      await analyzeButton.trigger('click');

      // Проверяем, что отображается уведомление об ошибке
      expect(mockNotifications.show).toHaveBeenCalledWith(
        'Ошибка анализа', 
        'Не удалось проанализировать модель для печати', 
        'error'
      );
    });
  });

  describe('Parameter Persistence and Auto-save', () => {
    it('should persist optimization parameters and auto-save changes', async () => {
      // Настраиваем моки
      mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
        faceCount: 100,
        estimatedSheetCount: 2,
        complexityScore: 65.5
      });

      // Монтируем компонент панели оптимизации
      const wrapper = mount(PaperOptimizationPanel);

      // Изменяем параметры оптимизации
      const sheetWidthInput = wrapper.find('input#sheet-width');
      await sheetWidthInput.setValue('297');
      
      const sheetHeightInput = wrapper.find('input#sheet-height');
      await sheetHeightInput.setValue('420');
      
      const minGapInput = wrapper.find('input#min-gap');
      await minGapInput.setValue('3');

      // Проверяем, что проект был сохранен
      expect(mockProjectStore.saveProject).toHaveBeenCalled();
      expect(mockAutoSaveStore.saveProject).toHaveBeenCalled();
      
      // Проверяем, что параметры были обновлены в проекте
      const project = mockProjectStore.currentProject.value;
      expect(project.model.paperOptimization.sheetWidth).toBe(297);
      expect(project.model.paperOptimization.sheetHeight).toBe(420);
      expect(project.model.paperOptimization.minGap).toBe(3);
      
      // Нажимаем кнопку анализа
      const analyzeButton = wrapper.find('button.analyze-btn');
      await analyzeButton.trigger('click');
      
      // Проверяем, что анализ использует обновленные параметры
      expect(mockPaperOptimizeClient.analyzeModelForPaper).toHaveBeenCalled();
    });
  });

  describe('Performance and Resource Management', () => {
    it('should handle complex models with many faces efficiently', async () => {
      // Настраиваем моки для большого объема данных
      mockUnfoldClient.generateUnfold.mockResolvedValue({
        faces: Array(1000).fill(0).map((_, i) => ({
          id: i,
          vertices: [
            { x: i * 10, y: 0 },
            { x: i * 10 + 10, y: 0 },
            { x: i * 10 + 10, y: 10 },
            { x: i * 10, y: 10 }
          ],
          seams: []
        })),
        seams: []
      });
      
      mockPaperOptimizeClient.optimizePaperLayout.mockResolvedValue({
        faces: Array(1000).fill(0).map((_, i) => ({
          id: i,
          vertices: [
            { x: i * 10, y: 0 },
            { x: i * 10 + 10, y: 0 },
            { x: i * 10 + 10, y: 10 },
            { x: i * 10, y: 10 }
          ],
          position: { x: i * 10, y: 0 },
          rotation: 0
        })),
        width: 2100,
        height: 2970
      });
      
      mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
        faceCount: 1000,
        estimatedSheetCount: 15,
        complexityScore: 95.5
      });

      // Монтируем компонент развертки
      const wrapper = mount(UnfoldStage);

      // Нажимаем кнопку генерации развертки
      const unfoldButton = wrapper.find('button.generate-unfold-btn');
      await unfoldButton.trigger('click');

      // Ждем завершения развертки
      await wrapper.vm.$nextTick();
      
      // Проверяем, что развертка была успешно создана
      expect(mockProjectStore.currentProject.value.unfoldResult).toBeDefined();
      expect(mockProjectStore.currentProject.value.unfoldResult.faces.length).toBe(1000);
      
      // Нажимаем кнопку оптимизации бумаги
      const optimizeButton = wrapper.find('button.optimize-paper-btn');
      await optimizeButton.trigger('click');
      
      // Нажимаем кнопку оптимизации в модальном окне
      const modal = wrapper.findComponent(PaperOptimizationModal);
      const modalOptimizeButton = modal.find('button.optimize-btn');
      await modalOptimizeButton.trigger('click');
      
      // Ждем завершения оптимизации
      await wrapper.vm.$nextTick();
      
      // Проверяем, что оптимизация была успешно выполнена
      expect(mockProjectStore.currentProject.value.unfoldResult.optimizedLayout).toBeDefined();
      expect(mockProjectStore.currentProject.value.unfoldResult.optimizedLayout.faces.length).toBe(1000);
    });
  });
});