/**
 * E2E тесты для Pepakura Next с использованием Playwright.
 * 
 * Запуск: pnpm test:e2e
 */

import { test, expect, type Page } from '@playwright/test';

// Базовый URL приложения
const BASE_URL = 'http://localhost:5173';

/**
 * Тесты главной страницы (Dashboard)
 */
test.describe('Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(BASE_URL);
  });

  test('должна загружаться с заголовком', async ({ page }) => {
    await expect(page).toHaveTitle(/Pepakura Next/);
    await expect(page.locator('h1, h2, h3').first()).toBeVisible();
  });

  test('должна показывать кнопку нового проекта', async ({ page }) => {
    const newProjectBtn = page.locator('[data-testid="new-project-btn"]');
    await expect(newProjectBtn).toBeVisible();
  });

  test('должна показывать список последних проектов', async ({ page }) => {
    const projectsPanel = page.locator('[data-testid="recent-projects"]');
    await expect(projectsPanel).toBeVisible();
  });
});

/**
 * Тесты импорта моделей
 */
test.describe('Import Model', () => {
  test('должна открывать диалог импорта', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const importBtn = page.locator('[data-testid="import-model-btn"]');
    await importBtn.click();
    
    // Проверяем что диалог открылся
    const dialog = page.locator('[role="dialog"]');
    await expect(dialog).toBeVisible();
  });

  test('должна показывать ошибку для несуществующего файла', async ({ page }) => {
    await page.goto(BASE_URL);
    
    // Пытаемся импортировать несуществующий файл
    await page.evaluate(async () => {
      // Эмуляция ошибки импорта
      window.dispatchEvent(new CustomEvent('import-error', {
        detail: { message: 'Файл не найден' }
      }));
    });
    
    const errorToast = page.locator('[data-testid="error-toast"]');
    await expect(errorToast).toBeVisible();
  });
});

/**
 * Тесты развёртки
 */
test.describe('Unfold Model', () => {
  test('должна показывать прогресс развёртки', async ({ page }) => {
    await page.goto(BASE_URL);
    
    // Эмуляция начала развёртки
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('unfold-start'));
    });
    
    const progressBar = page.locator('[data-testid="unfold-progress"]');
    await expect(progressBar).toBeVisible();
  });

  test('должна показывать результат развёртки', async ({ page }) => {
    await page.goto(BASE_URL);
    
    // Эмуляция завершения развёртки
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('unfold-complete', {
        detail: { vertices: 8, faces: 12 }
      }));
    });
    
    const viewer = page.locator('[data-testid="unfold-viewer"]');
    await expect(viewer).toBeVisible();
  });
});

/**
 * Тесты экспорта
 */
test.describe('Export', () => {
  test('должна открывать настройки экспорта', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const exportBtn = page.locator('[data-testid="export-btn"]');
    await exportBtn.click();
    
    const exportDialog = page.locator('[data-testid="export-dialog"]');
    await expect(exportDialog).toBeVisible();
  });

  test('должна позволять выбрать формат SVG', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const exportBtn = page.locator('[data-testid="export-btn"]');
    await exportBtn.click();
    
    const svgOption = page.locator('[data-testid="format-svg"]');
    await expect(svgOption).toBeVisible();
  });

  test('должна позволять выбрать размер страницы', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const exportBtn = page.locator('[data-testid="export-btn"]');
    await exportBtn.click();
    
    const pageSizeSelect = page.locator('[data-testid="page-size-select"]');
    await expect(pageSizeSelect).toBeVisible();
  });
});

/**
 * Тесты AI-помощника
 */
test.describe('AI Assistant', () => {
  test('должна показывать панель AI', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const aiPanel = page.locator('[data-testid="ai-assistant"]');
    await expect(aiPanel).toBeVisible();
  });

  test('должна показывать статус подключения', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const aiStatus = page.locator('[data-testid="ai-status"]');
    await expect(aiStatus).toBeVisible();
  });

  test('должна позволять отправить сообщение', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const input = page.locator('[data-testid="ai-input"]');
    await input.fill('Привет!');
    
    const sendBtn = page.locator('[data-testid="ai-send"]');
    await sendBtn.click();
    
    // Проверяем что сообщение отправлено
    const message = page.locator('[data-testid="message-user"]');
    await expect(message).toBeVisible();
  });

  test('должна показывать ответ AI', async ({ page }) => {
    await page.goto(BASE_URL);
    
    // Эмуляция ответа AI
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('ai-response', {
        detail: { message: 'Привет! Чем могу помочь?' }
      }));
    });
    
    const aiMessage = page.locator('[data-testid="message-assistant"]');
    await expect(aiMessage).toBeVisible();
  });

  test('должна показывать настройки AI', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const settingsBtn = page.locator('[data-testid="ai-settings"]');
    await settingsBtn.click();
    
    const settingsPanel = page.locator('[data-testid="ai-settings-panel"]');
    await expect(settingsPanel).toBeVisible();
  });
});

/**
 * Тесты настроек
 */
test.describe('Settings', () => {
  test('должна открывать настройки', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const settingsBtn = page.locator('[data-testid="settings-btn"]');
    await settingsBtn.click();
    
    const settingsDialog = page.locator('[data-testid="settings-dialog"]');
    await expect(settingsDialog).toBeVisible();
  });

  test('должна позволять изменить язык', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const settingsBtn = page.locator('[data-testid="settings-btn"]');
    await settingsBtn.click();
    
    const languageSelect = page.locator('[data-testid="language-select"]');
    await languageSelect.selectOption('en');
    
    // Проверяем что язык изменился
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
  });

  test('должна позволять изменить тему', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const settingsBtn = page.locator('[data-testid="settings-btn"]');
    await settingsBtn.click();
    
    const themeSelect = page.locator('[data-testid="theme-select"]');
    await themeSelect.selectOption('dark');
    
    // Проверяем что тема изменилась
    const html = page.locator('html');
    await expect(html).toHaveClass(/dark/);
  });
});

/**
 * Тесты навигации
 */
test.describe('Navigation', () => {
  test('должна переключаться между вкладками', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const dashboardTab = page.locator('[data-testid="tab-dashboard"]');
    const projectsTab = page.locator('[data-testid="tab-projects"]');
    
    await expect(dashboardTab).toBeVisible();
    await expect(projectsTab).toBeVisible();
    
    await projectsTab.click();
    await expect(page.locator('[data-testid="projects-view"]')).toBeVisible();
  });
});

/**
 * Тесты производительности
 */
test.describe('Performance', () => {
  test('должна загружаться быстрее 2 секунд', async ({ page }) => {
    const startTime = Date.now();
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
    const loadTime = Date.now() - startTime;
    
    expect(loadTime).toBeLessThan(2000);
  });

  test('должна рендерить список из 100 проектов', async ({ page }) => {
    await page.goto(BASE_URL);
    
    // Эмуляция загрузки 100 проектов
    await page.evaluate(() => {
      const projects = Array.from({ length: 100 }, (_, i) => ({
        id: i,
        name: `Project ${i}`,
        createdAt: new Date().toISOString()
      }));
      
      window.dispatchEvent(new CustomEvent('projects-loaded', {
        detail: { projects }
      }));
    });
    
    const projectList = page.locator('[data-testid="project-list"]');
    await expect(projectList).toBeVisible();
  });
});

/**
 * Тесты доступности (a11y)
 */
test.describe('Accessibility', () => {
  test('должна иметь правильную структуру заголовков', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const h1 = page.locator('h1');
    await expect(h1).toHaveCount(1);
  });

  test('должна иметь alt текст для изображений', async ({ page }) => {
    await page.goto(BASE_URL);
    
    const images = page.locator('img');
    const count = await images.count();
    
    for (let i = 0; i < count; i++) {
      const img = images.nth(i);
      await expect(img).toHaveAttribute('alt', /.+/);
    }
  });

  test('должна поддерживать навигацию с клавиатуры', async ({ page }) => {
    await page.goto(BASE_URL);
    
    // Tab навигация
    await page.keyboard.press('Tab');
    const focusedElement = page.locator(':focus');
    await expect(focusedElement).toBeVisible();
  });
});

/**
 * E2E сценарий: импорт → развёртка → экспорт
 */
test.describe('Full workflow: import → unfold → export', () => {
  test('user can import, unfold, and export', async ({ page }) => {
    await page.goto(BASE_URL);
    
    // 1. Импорт модели
    const importBtn = page.locator('[data-testid="import-model-btn"]');
    await importBtn.click();
    
    const fileInput = page.locator('[data-testid="file-input"]');
    await expect(fileInput).toBeVisible();
    
    // Эмуляция выбора файла (используем тестовый файл)
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('import-success', {
        detail: {
          name: 'test-cube.obj',
          vertices: 8,
          faces: 12,
          meshId: 'test-mesh-123'
        }
      }));
    });
    
    // Проверяем что модель загружена
    const modelViewer = page.locator('[data-testid="model-viewer"]');
    await expect(modelViewer).toBeVisible();
    
    // 2. Развёртка
    const unfoldBtn = page.locator('[data-testid="unfold-btn"]');
    await unfoldBtn.click();
    
    // Эмуляция прогресса развёртки
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('unfold-start'));
    });
    
    const progressBar = page.locator('[data-testid="unfold-progress"]');
    await expect(progressBar).toBeVisible();
    
    // Эмуляция завершения развёртки
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('unfold-complete', {
        detail: {
          vertices: 8,
          faces: 12,
          unfoldedMeshId: 'unfolded-123',
          svgData: '<svg>...</svg>'
        }
      }));
    });
    
    const unfoldResult = page.locator('[data-testid="unfold-result"]');
    await expect(unfoldResult).toBeVisible();
    
    // 3. Экспорт в SVG
    const exportBtn = page.locator('[data-testid="export-btn"]');
    await exportBtn.click();
    
    const exportDialog = page.locator('[data-testid="export-dialog"]');
    await expect(exportDialog).toBeVisible();
    
    const svgOption = page.locator('[data-testid="format-svg"]');
    await svgOption.click();
    
    const exportConfirm = page.locator('[data-testid="export-confirm"]');
    await exportConfirm.click();
    
    // Эмуляция успешного экспорта
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('export-success', {
        detail: { format: 'svg', path: '/downloads/test-cube.svg' }
      }));
    });
    
    const successToast = page.locator('[data-testid="success-toast"]');
    await expect(successToast).toBeVisible();
  });
});
