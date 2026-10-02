import { mount } from '@vue/test-utils';
import UnfoldStage from './UnfoldStage.vue';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { ref } from 'vue';

// Мокаем хранилище проектов
const mockProjectStore = {
  currentProject: ref({
    id: 'test-project',
    name: 'Test Project',
    model: {
      vertices: [],
      faces: [],
      seams: [],
      paperOptimization: {
        sheetWidth: 210,
        sheetHeight: 297,
        minGap: 2,
        minTabWidth: 5,
        maxAutoTabAngle: 60,
        addPrintMargins: true,
        marginSize: 5
      }
    },
    unfoldResult: null
  }),
  saveProject: vi.fn()
};

// Мокаем клиент оптимизации бумаги
const mockPaperOptimizeClient = {
  optimizePaperLayout: vi.fn(),
  analyzeModelForPaper: vi.fn()
};

// Мокаем уведомления
const mockNotifications = {
  show: vi.fn()
};

// Мокаем Tauri API для развертки
const mockTauriUnfold = {
  generateUnfold: vi.fn()
};

vi.mock('../../../stores/projectStore', () => ({
  useProjectStore: () => mockProjectStore
}));

vi.mock('../../../modules/paper-optimization/paperOptimizeClient', () => ({
  ...mockPaperOptimizeClient
}));

vi.mock('../../../composables/useNotifications', () => ({
  useNotifications: () => mockNotifications
}));

vi.mock('../../../modules/unfold/unfoldClient', () => ({
  ...mockTauriUnfold
}));

describe('UnfoldStage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    const wrapper = mount(UnfoldStage);

    // Проверяем, что компонент отображается
    expect(wrapper.find('.unfold-stage').exists()).toBe(true);
    
    // Проверяем, что отображаются кнопки управления
    expect(wrapper.find('button.generate-unfold-btn').exists()).toBe(true);
    expect(wrapper.find('button.optimize-paper-btn').exists()).toBe(true);
    expect(wrapper.find('button.export-btn').exists()).toBe(true);
  });

  it('calls unfold generation when generate button is clicked', async () => {
    // Настраиваем мок для функции развертки
    mockTauriUnfold.generateUnfold.mockResolvedValue({
      faces: [],
      seams: []
    });

    const wrapper = mount(UnfoldStage);

    // Нажимаем кнопку генерации развертки
    const generateButton = wrapper.find('button.generate-unfold-btn');
    await generateButton.trigger('click');

    // Проверяем, что функция развертки была вызвана
    expect(mockTauriUnfold.generateUnfold).toHaveBeenCalled();
    
    // Проверяем, что проект сохраняется
    expect(mockProjectStore.saveProject).toHaveBeenCalled();
  });

  it('shows error notification when unfold generation fails', async () => {
    // Настраиваем мок для выбрасывания ошибки
    mockTauriUnfold.generateUnfold.mockRejectedValue(
      new Error('Unfold failed')
    );

    const wrapper = mount(UnfoldStage);

    // Нажимаем кнопку генерации развертки
    const generateButton = wrapper.find('button.generate-unfold-btn');
    await generateButton.trigger('click');

    // Проверяем, что отображается уведомление об ошибке
    expect(mockNotifications.show).toHaveBeenCalledWith(
      'Ошибка развертки', 
      'Не удалось сгенерировать развертку модели', 
      'error'
    );
  });

  it('opens paper optimization modal when optimize button is clicked', async () => {
    const wrapper = mount(UnfoldStage);

    // Нажимаем кнопку оптимизации бумаги
    const optimizeButton = wrapper.find('button.optimize-paper-btn');
    await optimizeButton.trigger('click');

    // Проверяем, что модальное окно оптимизации открылось
    expect(wrapper.find('PaperOptimizationModal').exists()).toBe(true);
  });

  it('applies optimized layout when received from modal', async () => {
    // Настраиваем мок для функции оптимизации
    mockPaperOptimizeClient.optimizePaperLayout.mockResolvedValue({
      faces: [],
      width: 210,
      height: 297
    });

    const wrapper = mount(UnfoldStage);

    // Открываем модальное окно оптимизации
    const optimizeButton = wrapper.find('button.optimize-paper-btn');
    await optimizeButton.trigger('click');

    // Эмитируем событие применения оптимизации
    const modal = wrapper.find('PaperOptimizationModal');
    await modal.vm.$emit('apply', {
      faces: [],
      width: 210,
      height: 297
    });

    // Проверяем, что оптимизированная развертка сохраняется в проекте
    expect(mockProjectStore.currentProject.value.unfoldResult).toBeDefined();
    expect(mockProjectStore.saveProject).toHaveBeenCalled();
  });

  it('exports unfolded model when export button is clicked', async () => {
    // Устанавливаем развертку в проекте
    mockProjectStore.currentProject.value.unfoldResult = {
      faces: [],
      seams: []
    };

    const wrapper = mount(UnfoldStage);

    // Нажимаем кнопку экспорта
    const exportButton = wrapper.find('button.export-btn');
    await exportButton.trigger('click');

    // Проверяем, что отображается меню экспорта
    expect(wrapper.find('.export-menu').exists()).toBe(true);
  });

  it('handles export format selection', async () => {
    // Устанавливаем развертку в проекте
    mockProjectStore.currentProject.value.unfoldResult = {
      faces: [],
      seams: []
    };

    const wrapper = mount(UnfoldStage);

    // Нажимаем кнопку экспорта
    const exportButton = wrapper.find('button.export-btn');
    await exportButton.trigger('click');

    // Выбираем формат экспорта PDF
    const pdfExportButton = wrapper.find('button.export-pdf');
    await pdfExportButton.trigger('click');

    // Проверяем, что отображается диалог экспорта PDF
    expect(wrapper.find('.pdf-export-dialog').exists()).toBe(true);
  });

  it('performs paper analysis when analyze button is clicked', async () => {
    // Настраиваем мок для функции анализа
    mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
      faceCount: 100,
      estimatedSheetCount: 5,
      complexityScore: 75.5
    });

    const wrapper = mount(UnfoldStage);

    // Нажимаем кнопку анализа
    const analyzeButton = wrapper.find('button.analyze-btn');
    await analyzeButton.trigger('click');

    // Проверяем, что функция анализа была вызвана
    expect(mockPaperOptimizeClient.analyzeModelForPaper).toHaveBeenCalled();
    
    // Проверяем, что отображаются результаты анализа
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.analysis-results').exists()).toBe(true);
  });

  it('shows error notification when paper analysis fails', async () => {
    // Настраиваем мок для выбрасывания ошибки
    mockPaperOptimizeClient.analyzeModelForPaper.mockRejectedValue(
      new Error('Analysis failed')
    );

    const wrapper = mount(UnfoldStage);

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