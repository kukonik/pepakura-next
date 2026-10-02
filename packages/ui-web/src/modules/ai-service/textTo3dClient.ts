import {
  TextTo3dRequest,
  TextTo3dResponse,
  ImageTo3dRequest,
  ImageTo3dResponse
} from '@/shared/types/textTo3d.types';

export class TextTo3dClient {
  private baseUrl: string;

  constructor(baseUrl: string = '/api/v1') {
    this.baseUrl = baseUrl;
  }

  /**
   * Генерация 3D модели из текстового описания
   * @param request Параметры генерации
   * @returns Результат генерации с артефактами
   */
  async generateFromText(request: TextTo3dRequest): Promise<TextTo3dResponse> {
    const response = await fetch(`${this.baseUrl}/text-to-3d/generate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  }
}

export class ImageTo3dClient {
  private baseUrl: string;

  constructor(baseUrl: string = '/api/v1') {
    this.baseUrl = baseUrl;
  }

  /**
   * Генерация 3D модели из 2D изображения
   * @param request Параметры генерации
   * @returns Результат генерации с артефактами
   */
  async generateFromImage(request: ImageTo3dRequest): Promise<ImageTo3dResponse> {
    const response = await fetch(`${this.baseUrl}/image-to-3d/generate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  }
}

// Экспортируем общий клиент для удобства использования
export const textTo3dClient = new TextTo3dClient();
export const imageTo3dClient = new ImageTo3dClient();