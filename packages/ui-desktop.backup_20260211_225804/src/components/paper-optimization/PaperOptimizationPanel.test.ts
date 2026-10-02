import { mount } from '@vue/test-utils';
import PaperOptimizationPanel from './PaperOptimizationPanel.vue';
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
      paperOptimization: {
        sheetWidth: 210,
        sheetHeight: 297,
        minGap: 2,
        minTabWidth: 5,
        maxAutoTabAngle: 60,
        addPrintMargins: true,
        marginSize: 5
      }
    }
  }),
  saveProject: vi.fn()
};

// Мокаем клиент оптимизации бумаги
const mockPaperOptimizeClient = {
  analyzeModelForPaper: vi.fn(),
  generateAssemblyTips: vi.fn()
};

// Мокаем уведомления
const mockNotifications = {
  show: vi.fn()
};

vi.mock('../../stores/projectStore', () => ({
  useProjectStore: () => mockProjectStore
}));

vi.mock('../../modules/paper-optimization/paperOptimizeClient', () => ({
  ...mockPaperOptimizeClient
}));

vi.mock('../../composables/useNotifications', () => ({
  useNotifications: () => mockNotifications
}));

describe('PaperOptimizationPanel', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly with project data', () => {
    const wrapper = mount(PaperOptimizationPanel);

    // Проверяем, что панель отображается
    expect(wrapper.find('.paper-optimization-panel').exists()).toBe(true);
    
    // Проверяем, что отображаются параметры оптимизации
    expect(wrapper.find('input#sheet-width').exists()).toBe(true);
    expect(wrapper.find('input#sheet-height').exists()).toBe(true);
    expect(wrapper.find('input#min-gap').exists()).toBe(true);
    
    // Проверяем, что отображаются метрики модели
    expect(wrapper.find('.model-metrics').exists()).toBe(true);
  });

  it('updates parameters when inputs change', async () => {
    const wrapper = mount(PaperOptimizationPanel);

    // Изменяем значение ширины листа
    const sheetWidthInput = wrapper.find('input#sheet-width');
    await sheetWidthInput.setValue('297');
    
    // Проверяем, что значение обновилось
    expect(sheetWidthInput.element.value).toBe('297');
    
    // Проверяем, что проект сохраняется
    expect(mockProjectStore.saveProject).toHaveBeenCalled();
  });

  it('calls analysis function when analyze button is clicked', async () => {
    // Настраиваем мок для функции анализа
    mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
      faceCount: 100,
      estimatedSheetCount: 5,
      complexityScore: 75.5
    });
    
    mockPaperOptimizeClient.generateAssemblyTips.mockResolvedValue([
      "Начните сборку с центральных элементов",
      "Склейте все вкладыши перед финальной сборкой"
    ]);

    const wrapper = mount(PaperOptimizationPanel);

    // Нажимаем кнопку анализа
    const analyzeButton = wrapper.find('button.analyze-btn');
    await analyzeButton.trigger('click');

    // Проверяем, что функция анализа была вызвана
    expect(mockPaperOptimizeClient.analyzeModelForPaper).toHaveBeenCalled();
    
    // Проверяем, что отображаются результаты анализа
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.analysis-results').exists()).toBe(true);
  });

  it('shows error notification when analysis fails', async () => {
    // Настраиваем мок для выбрасывания ошибки
    mockPaperOptimizeClient.analyzeModelForPaper.mockRejectedValue(
      new Error('Analysis failed')
    );

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

  it('emits optimize event when optimize button is clicked', async () => {
    const wrapper = mount(PaperOptimizationPanel);

    // Нажимаем кнопку оптимизации
    const optimizeButton = wrapper.find('button.optimize-btn');
    await optimizeButton.trigger('click');

    // Проверяем, что событие оптимизации было отправлено
    expect(wrapper.emitted('optimize')).toBeTruthy();
  });

  it('shows assembly tips when tips button is clicked', async () => {
    // Настраиваем мок для функции генерации советов
    mockPaperOptimizeClient.generateAssemblyTips.mockResolvedValue([
      "Начните сборку с центральных элементов",
      "Склейте все вкладыши перед финальной сборкой"
    ]);

    const wrapper = mount(PaperOptimizationPanel);

    // Нажимаем кнопку советов
    const tipsButton = wrapper.find('button.tips-btn');
    await tipsButton.trigger('click');

    // Проверяем, что отображаются советы по сборке
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.assembly-tips').exists()).toBe(true);
  });

  it('handles empty assembly tips gracefully', async () => {
    // Настраиваем мок для функции генерации советов с пустым результатом
    mockPaperOptimizeClient.generateAssemblyTips.mockResolvedValue([]);

    const wrapper = mount(PaperOptimizationPanel);

    // Нажимаем кнопку советов
    const tipsButton = wrapper.find('button.tips-btn');
    await tipsButton.trigger('click');

    // Проверяем, что отображается сообщение об отсутствии советов
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.no-tips-message').exists()).toBe(true);
  });
});