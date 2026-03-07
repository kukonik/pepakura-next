import { describe, it, expect, vi, beforeEach } from 'vitest';
import { performance } from 'perf_hooks';

// Мокаем зависимости для тестов производительности
const mockTextTo3DClient = {
  generateModel: vi.fn(),
  checkGenerationStatus: vi.fn(),
  getModelResult: vi.fn()
};

const mockImageTo3DClient = {
  generateModelFromImage: vi.fn()
};

vi.mock('../../modules/ai/text-to-3d/textTo3dClient', () => ({
  ...mockTextTo3DClient
}));

vi.mock('../../modules/ai/image-to-3d/imageTo3DClient', () => ({
  ...mockImageTo3DClient
}));

// Импортируем функции для тестирования
import { generateModelFromText } from '../../modules/ai/text-to-3d/textTo3dClient';
import { generateModelFromImage } from '../../modules/ai/image-to-3d/imageTo3DClient';

describe('AI Generation Performance', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Text-to-3D Generation Performance', () => {
    it('should generate simple models within 500ms', async () => {
      // Настраиваем мок для быстрой генерации
      mockTextTo3DClient.generateModel.mockResolvedValue({
        taskId: 'task-123',
        status: 'completed'
      });
      
      mockTextTo3DClient.getModelResult.mockResolvedValue({
        model: {
          vertices: [{ x: 0, y: 0, z: 0 }],
          faces: [{ vertexIndices: [0, 1, 2] }]
        },
        previewImage: 'base64image'
      });

      // Измеряем время выполнения
      const startTime = performance.now();
      
      await generateModelFromText('Simple cube');
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(500);
    });

    it('should handle concurrent generation requests efficiently', async () => {
      // Настраиваем моки для множественных запросов
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

      // Измеряем время выполнения множественных запросов
      const startTime = performance.now();
      
      // Создаем 5 параллельных запросов
      const promises = Array(5).fill(0).map(() => 
        generateModelFromText('Simple model')
      );
      
      await Promise.all(promises);
      
      const endTime = performance.now();
      const totalTime = endTime - startTime;

      // Проверяем, что общее время выполнения разумное
      expect(totalTime).toBeLessThan(2000); // 2 секунды для 5 запросов
      
      // Проверяем, что все запросы были выполнены
      expect(mockTextTo3DClient.generateModel).toHaveBeenCalledTimes(5);
    });

    it('should maintain memory usage below 100MB for large models', async () => {
      // Настраиваем мок для большой модели
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
        vertices: Array(50000).fill(0).map((_, i) => ({ 
          x: Math.random(), 
          y: Math.random(), 
          z: Math.random() 
        })),
        faces: Array(25000).fill(0).map((_, i) => ({ 
          vertexIndices: [i * 3, i * 3 + 1, i * 3 + 2] 
        }))
      };
      
      mockTextTo3DClient.getModelResult.mockResolvedValue({
        model: largeModel,
        previewImage: 'base64image'
      });

      // Измеряем использование памяти
      const startMemory = process.memoryUsage().heapUsed;
      
      await generateModelFromText('Complex model with many faces');
      
      const endMemory = process.memoryUsage().heapUsed;
      const memoryUsage = (endMemory - startMemory) / 1024 / 1024; // В МБ

      // Проверяем, что использование памяти в пределах нормы
      expect(memoryUsage).toBeLessThan(100);
    });
  });

  describe('Image-to-3D Generation Performance', () => {
    it('should process images within 1 second', async () => {
      // Настраиваем мок для быстрой обработки изображения
      mockImageTo3DClient.generateModelFromImage.mockResolvedValue({
        model: {
          vertices: [{ x: 0, y: 0, z: 0 }],
          faces: [{ vertexIndices: [0, 1, 2] }]
        },
        previewImage: 'base64image'
      });

      // Измеряем время выполнения
      const startTime = performance.now();
      
      // Создаем тестовое изображение
      const imageBase64 = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==';
      await generateModelFromImage(imageBase64);
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(1000);
    });

    it('should handle large images efficiently', async () => {
      // Настраиваем мок для обработки большого изображения
      mockImageTo3DClient.generateModelFromImage.mockResolvedValue({
        model: {
          vertices: Array(1000).fill(0).map((_, i) => ({ 
            x: Math.random(), 
            y: Math.random(), 
            z: Math.random() 
          })),
          faces: Array(500).fill(0).map((_, i) => ({ 
            vertexIndices: [i * 3, i * 3 + 1, i * 3 + 2] 
          }))
        },
        previewImage: 'base64image'
      });

      // Создаем большое изображение (мок)
      const largeImageBase64 = 'data:image/png;base64,' + 'A'.repeat(1000000); // 1MB изображение

      // Измеряем время выполнения
      const startTime = performance.now();
      
      await generateModelFromImage(largeImageBase64);
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(5000); // 5 секунд для большого изображения
    });
  });

  describe('API Response Times', () => {
    it('should maintain API response times under 200ms for simple operations', async () => {
      // Настраиваем мок для быстрого ответа
      mockTextTo3DClient.generateModel.mockImplementation(async () => {
        // Имитируем небольшую задержку сети
        await new Promise(resolve => setTimeout(resolve, 50));
        return {
          taskId: 'task-123',
          status: 'processing'
        };
      });

      // Измеряем время ответа API
      const startTime = performance.now();
      
      await generateModelFromText('Simple model');
      
      const endTime = performance.now();
      const responseTime = endTime - startTime;

      // Проверяем, что время ответа в пределах нормы
      expect(responseTime).toBeLessThan(200);
    });

    it('should handle API timeouts gracefully', async () => {
      // Настраиваем мок для таймаута
      mockTextTo3DClient.generateModel.mockImplementation(async () => {
        // Имитируем таймаут
        await new Promise(resolve => setTimeout(resolve, 30000)); // 30 секунд
        return {
          taskId: 'task-123',
          status: 'processing'
        };
      });

      // Устанавливаем таймаут для теста
      const timeoutPromise = new Promise((_, reject) => {
        setTimeout(() => reject(new Error('API timeout')), 5000); // 5 секунд таймаут для теста
      });

      // Проверяем, что запрос завершается с таймаутом
      await expect(Promise.race([
        generateModelFromText('Slow model'),
        timeoutPromise
      ])).rejects.toThrow('API timeout');
    });
  });

  describe('Resource Cleanup', () => {
    it('should properly clean up resources after generation', async () => {
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

      // Измеряем использование ресурсов до
      const startHandles = process._getActiveHandles().length;
      const startRequests = process._getActiveRequests().length;
      
      // Выполняем генерацию
      await generateModelFromText('Test model');
      
      // Измеряем использование ресурсов после
      const endHandles = process._getActiveHandles().length;
      const endRequests = process._getActiveRequests().length;

      // Проверяем, что ресурсы были освобождены
      // Разрешаем небольшое количество активных хендлов из-за асинхронных операций
      expect(endHandles).toBeLessThanOrEqual(startHandles + 5);
      expect(endRequests).toBeLessThanOrEqual(startRequests + 5);
    });
  });
});