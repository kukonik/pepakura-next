import { mount } from '@vue/test-utils';
import PaperOptimizationModal from './PaperOptimizationModal.vue';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { ref } from 'vue';

// Мокаем хранилище проектов
const mockProjectStore = {
  currentProject: ref({
    id: 'test-project',
    name: 'Test Project',
    model: {
      vertices: [],
      faces: []
    }
  }),
  saveProject: vi.fn()
};

// Мокаем клиент оптимизации бумаги
const mockPaperOptimizeClient = {
  optimizePaperLayout: vi.fn(),
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

describe('PaperOptimizationModal', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly when visible', () => {
    const wrapper = mount(PaperOptimizationModal, {
      props: {
        visible: true,
        model: {
          vertices: [],
          faces: []
        }
      }
    });

    // Проверяем, что модальное окно отображается
    expect(wrapper.find('.paper-optimization-modal').exists()).toBe(true);
    
    // Проверяем, что отображаются поля ввода параметров
    expect(wrapper.find('input#sheet-width').exists()).toBe(true);
    expect(wrapper.find('input#sheet-height').exists()).toBe(true);
    expect(wrapper.find('input#min-gap').exists()).toBe(true);
    expect(wrapper.find('input#min-tab-width').exists()).toBe(true);
  });

  it('does not render when not visible', () => {
    const wrapper = mount(PaperOptimizationModal, {
      props: {
        visible: false,
        model: {
          vertices: [],
          faces: []
        }
      }
    });

    // Проверяем, что модальное окно не отображается
    expect(wrapper.find('.paper-optimization-modal').exists()).toBe(false);
  });

  it('updates parameters when inputs change', async () => {
    const wrapper = mount(PaperOptimizationModal, {
      props: {
        visible: true,
        model: {
          vertices: [],
          faces: []
        }
      }
    });

    // Изменяем значение ширины листа
    const sheetWidthInput = wrapper.find('input#sheet-width');
    await sheetWidthInput.setValue('297');
    
    // Проверяем, что значение обновилось
    expect(sheetWidthInput.element.value).toBe('297');
  });

  it('calls optimize function when optimize button is clicked', async () => {
    // Настраиваем мок для функции оптимизации
    mockPaperOptimizeClient.optimizePaperLayout.mockResolvedValue({
      faces: [],
      width: 210,
      height: 297
    });
    
    mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
      faceCount: 10,
      estimatedSheetCount: 1,
      complexityScore: 50
    });
    
    mockPaperOptimizeClient.generateAssemblyTips.mockResolvedValue([
      "Совет по сборке"
    ]);

    const wrapper = mount(PaperOptimizationModal, {
      props: {
        visible: true,
        model: {
          vertices: [],
          faces: []
        }
      }
    });

    // Нажимаем кнопку оптимизации
    const optimizeButton = wrapper.find('button.optimize-btn');
    await optimizeButton.trigger('click');

    // Проверяем, что функция оптимизации была вызвана
    expect(mockPaperOptimizeClient.optimizePaperLayout).toHaveBeenCalled();
    expect(mockPaperOptimizeClient.analyzeModelForPaper).toHaveBeenCalled();
    expect(mockPaperOptimizeClient.generateAssemblyTips).toHaveBeenCalled();
  });

  it('shows error notification when optimization fails', async () => {
    // Настраиваем мок для выбрасывания ошибки
    mockPaperOptimizeClient.optimizePaperLayout.mockRejectedValue(
      new Error('Optimization failed')
    );

    const wrapper = mount(PaperOptimizationModal, {
      props: {
        visible: true,
        model: {
          vertices: [],
          faces: []
        }
      }
    });

    // Нажимаем кнопку оптимизации
    const optimizeButton = wrapper.find('button.optimize-btn');
    await optimizeButton.trigger('click');

    // Проверяем, что отображается уведомление об ошибке
    expect(mockNotifications.show).toHaveBeenCalledWith(
      'Ошибка оптимизации', 
      'Не удалось оптимизировать размещение деталей', 
      'error'
    );
  });

  it('emits close event when close button is clicked', async () => {
    const wrapper = mount(PaperOptimizationModal, {
      props: {
        visible: true,
        model: {
          vertices: [],
          faces: []
        }
      }
    });

    // Нажимаем кнопку закрытия
    const closeButton = wrapper.find('button.close-btn');
    await closeButton.trigger('click');

    // Проверяем, что событие закрытия было отправлено
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('emits apply event with optimized layout when apply button is clicked', async () => {
    // Настраиваем мок для функции оптимизации
    const mockLayout = {
      faces: [],
      width: 210,
      height: 297
    };
    
    mockPaperOptimizeClient.optimizePaperLayout.mockResolvedValue(mockLayout);
    mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
      faceCount: 10,
      estimatedSheetCount: 1,
      complexityScore: 50
    });
    mockPaperOptimizeClient.generateAssemblyTips.mockResolvedValue([
      "Совет по сборке"
    ]);

    const wrapper = mount(PaperOptimizationModal, {
      props: {
        visible: true,
        model: {
          vertices: [],
          faces: []
        }
      }
    });

    // Нажимаем кнопку оптимизации
    const optimizeButton = wrapper.find('button.optimize-btn');
    await optimizeButton.trigger('click');
    
    // Ждем завершения оптимизации
    await wrapper.vm.$nextTick();

    // Нажимаем кнопку применения
    const applyButton = wrapper.find('button.apply-btn');
    await applyButton.trigger('click');

    // Проверяем, что событие применения было отправлено с правильными данными
    const emitted = wrapper.emitted('apply');
    expect(emitted).toBeTruthy();
    expect(emitted?.[0]).toEqual([mockLayout]);
  });
});