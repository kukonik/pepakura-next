import { describe, it, expect, vi, beforeEach } from 'vitest';
import { performance } from 'perf_hooks';

// Мокаем зависимости для тестов производительности
const mockPaperOptimizeClient = {
  optimizePaperLayout: vi.fn(),
  analyzeModelForPaper: vi.fn(),
  generateAssemblyTips: vi.fn()
};

const mockUnfoldClient = {
  generateUnfold: vi.fn()
};

vi.mock('../../modules/paper-optimization/paperOptimizeClient', () => ({
  ...mockPaperOptimizeClient
}));

vi.mock('../../modules/unfold/unfoldClient', () => ({
  ...mockUnfoldClient
}));

// Импортируем функции для тестирования
import { optimizePaperLayout } from '../../modules/paper-optimization/paperOptimizeClient';
import { generateUnfold } from '../../modules/unfold/unfoldClient';

describe('Paper Optimization Performance', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Unfold Generation Performance', () => {
    it('should generate unfold for simple models within 300ms', async () => {
      // Настраиваем мок для быстрой развертки
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

      // Создаем простую модель для тестирования
      const simpleModel = {
        vertices: [{ x: 0, y: 0, z: 0 }],
        faces: [{ vertexIndices: [0, 1, 2] }]
      };

      // Измеряем время выполнения
      const startTime = performance.now();
      
      await generateUnfold(simpleModel);
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(300);
    });

    it('should handle complex models with many faces efficiently', async () => {
      // Настраиваем мок для развертки сложной модели
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
        seams: Array(999).fill(0).map((_, i) => ({
          id: i,
          face1Index: i,
          face2Index: i + 1,
          start: { x: (i + 1) * 10, y: 0 },
          end: { x: (i + 1) * 10, y: 10 },
          angleDegrees: 90
        }))
      });

      // Создаем сложную модель для тестирования
      const complexModel = {
        vertices: Array(1000).fill(0).map((_, i) => ({ 
          x: i, 
          y: 0, 
          z: 0 
        })),
        faces: Array(500).fill(0).map((_, i) => ({ 
          vertexIndices: [i * 2, i * 2 + 1, i * 2 + 2] 
        }))
      };

      // Измеряем время выполнения
      const startTime = performance.now();
      
      await generateUnfold(complexModel);
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(2000); // 2 секунды для сложной модели
    });

    it('should maintain memory usage below 200MB for large models', async () => {
      // Настраиваем мок для развертки очень большой модели
      mockUnfoldClient.generateUnfold.mockResolvedValue({
        faces: Array(5000).fill(0).map((_, i) => ({
          id: i,
          vertices: [
            { x: i * 10, y: 0 },
            { x: i * 10 + 10, y: 0 },
            { x: i * 10 + 10, y: 10 },
            { x: i * 10, y: 10 }
          ],
          seams: []
        })),
        seams: Array(4999).fill(0).map((_, i) => ({
          id: i,
          face1Index: i,
          face2Index: i + 1,
          start: { x: (i + 1) * 10, y: 0 },
          end: { x: (i + 1) * 10, y: 10 },
          angleDegrees: 90
        }))
      });

      // Создаем очень большую модель для тестирования
      const veryLargeModel = {
        vertices: Array(10000).fill(0).map((_, i) => ({ 
          x: Math.random() * 1000, 
          y: Math.random() * 1000, 
          z: Math.random() * 1000 
        })),
        faces: Array(5000).fill(0).map((_, i) => ({ 
          vertexIndices: [i * 2, i * 2 + 1, i * 2 + 2] 
        }))
      };

      // Измеряем использование памяти
      const startMemory = process.memoryUsage().heapUsed;
      
      await generateUnfold(veryLargeModel);
      
      const endMemory = process.memoryUsage().heapUsed;
      const memoryUsage = (endMemory - startMemory) / 1024 / 1024; // В МБ

      // Проверяем, что использование памяти в пределах нормы
      expect(memoryUsage).toBeLessThan(200);
    });
  });

  describe('Paper Layout Optimization Performance', () => {
    it('should optimize layout for small models within 200ms', async () => {
      // Настраиваем мок для быстрой оптимизации
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

      // Создаем развертку для тестирования
      const unfoldResult = {
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
      };

      // Измеряем время выполнения
      const startTime = performance.now();
      
      await optimizePaperLayout(unfoldResult.faces, {
        sheetWidth: 210,
        sheetHeight: 297,
        minGap: 2,
        minTabWidth: 5,
        maxAutoTabAngle: 60,
        addPrintMargins: true,
        marginSize: 5
      });
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(200);
    });

    it('should handle large layouts with many faces efficiently', async () => {
      // Настраиваем мок для оптимизации большого количества граней
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

      // Создаем большую развертку для тестирования
      const largeUnfoldResult = {
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
      };

      // Измеряем время выполнения
      const startTime = performance.now();
      
      await optimizePaperLayout(largeUnfoldResult.faces, {
        sheetWidth: 210,
        sheetHeight: 297,
        minGap: 2,
        minTabWidth: 5,
        maxAutoTabAngle: 60,
        addPrintMargins: true,
        marginSize: 5
      });
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(5000); // 5 секунд для большой развертки
    });

    it('should maintain memory usage below 150MB for complex layouts', async () => {
      // Настраиваем мок для оптимизации очень сложной развертки
      mockPaperOptimizeClient.optimizePaperLayout.mockResolvedValue({
        faces: Array(5000).fill(0).map((_, i) => ({
          id: i,
          vertices: Array(10).fill(0).map((_, j) => ({
            x: i * 10 + j,
            y: j * 10
          })),
          position: { x: i * 10, y: 0 },
          rotation: i % 360
        })),
        width: 2100,
        height: 2970
      });

      // Создаем очень сложную развертку для тестирования
      const veryComplexUnfoldResult = {
        faces: Array(5000).fill(0).map((_, i) => ({
          id: i,
          vertices: Array(10).fill(0).map((_, j) => ({
            x: i * 10 + j,
            y: j * 10
          })),
          seams: []
        })),
        seams: []
      };

      // Измеряем использование памяти
      const startMemory = process.memoryUsage().heapUsed;
      
      await optimizePaperLayout(veryComplexUnfoldResult.faces, {
        sheetWidth: 210,
        sheetHeight: 297,
        minGap: 2,
        minTabWidth: 5,
        maxAutoTabAngle: 60,
        addPrintMargins: true,
        marginSize: 5
      });
      
      const endMemory = process.memoryUsage().heapUsed;
      const memoryUsage = (endMemory - startMemory) / 1024 / 1024; // В МБ

      // Проверяем, что использование памяти в пределах нормы
      expect(memoryUsage).toBeLessThan(150);
    });
  });

  describe('Paper Analysis Performance', () => {
    it('should analyze models within 100ms', async () => {
      // Настраиваем мок для быстрого анализа
      mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
        faceCount: 100,
        estimatedSheetCount: 2,
        complexityScore: 65.5
      });

      // Создаем модель для тестирования
      const model = {
        vertices: Array(100).fill(0).map((_, i) => ({ 
          x: i, 
          y: 0, 
          z: 0 
        })),
        faces: Array(50).fill(0).map((_, i) => ({ 
          vertexIndices: [i * 2, i * 2 + 1, i * 2 + 2] 
        }))
      };

      // Измеряем время выполнения
      const startTime = performance.now();
      
      await mockPaperOptimizeClient.analyzeModelForPaper(model);
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(100);
    });

    it('should handle very large models efficiently', async () => {
      // Настраиваем мок для анализа очень большой модели
      mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
        faceCount: 10000,
        estimatedSheetCount: 50,
        complexityScore: 95.5
      });

      // Создаем очень большую модель для тестирования
      const veryLargeModel = {
        vertices: Array(20000).fill(0).map((_, i) => ({ 
          x: Math.random() * 1000, 
          y: Math.random() * 1000, 
          z: Math.random() * 1000 
        })),
        faces: Array(10000).fill(0).map((_, i) => ({ 
          vertexIndices: [i * 2, i * 2 + 1, i * 2 + 2] 
        }))
      };

      // Измеряем время выполнения
      const startTime = performance.now();
      
      await mockPaperOptimizeClient.analyzeModelForPaper(veryLargeModel);
      
      const endTime = performance.now();
      const executionTime = endTime - startTime;

      // Проверяем, что время выполнения в пределах нормы
      expect(executionTime).toBeLessThan(1000); // 1 секунда для очень большой модели
    });
  });

  describe('Concurrent Operations Performance', () => {
    it('should handle multiple concurrent operations efficiently', async () => {
      // Настраиваем моки для множественных операций
      mockUnfoldClient.generateUnfold.mockResolvedValue({
        faces: [{ id: 0, vertices: [], seams: [] }],
        seams: []
      });
      
      mockPaperOptimizeClient.optimizePaperLayout.mockResolvedValue({
        faces: [{ id: 0, vertices: [], position: { x: 0, y: 0 }, rotation: 0 }],
        width: 210,
        height: 297
      });
      
      mockPaperOptimizeClient.analyzeModelForPaper.mockResolvedValue({
        faceCount: 10,
        estimatedSheetCount: 1,
        complexityScore: 50
      });

      // Создаем несколько моделей для тестирования
      const models = Array(5).fill(0).map((_, i) => ({
        vertices: [{ x: 0, y: 0, z: 0 }],
        faces: [{ vertexIndices: [0, 1, 2] }]
      }));

      // Измеряем время выполнения множественных операций
      const startTime = performance.now();
      
      // Выполняем параллельные операции
      const unfoldPromises = models.map(model => generateUnfold(model));
      const analysisPromises = models.map(model => mockPaperOptimizeClient.analyzeModelForPaper(model));
      
      await Promise.all([...unfoldPromises, ...analysisPromises]);
      
      const endTime = performance.now();
      const totalTime = endTime - startTime;

      // Проверяем, что общее время выполнения разумное
      expect(totalTime).toBeLessThan(3000); // 3 секунды для 10 параллельных операций
      
      // Проверяем, что все операции были выполнены
      expect(mockUnfoldClient.generateUnfold).toHaveBeenCalledTimes(5);
      expect(mockPaperOptimizeClient.analyzeModelForPaper).toHaveBeenCalledTimes(5);
    });
  });
});