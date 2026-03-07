import { TextTo3DRequest, TextTo3DResponse } from './types';

interface AdvancedTextTo3DRequest extends TextTo3DRequest {
  style?: string;
  colors?: string[];
  materials?: string[];
  quality?: string;
  lighting?: string;
  cameraAngle?: string;
  dimensions?: Record<string, number>;
  language?: string;
}

class AdvancedTextTo3DClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async generate3DModel(request: AdvancedTextTo3DRequest): Promise<TextTo3DResponse> {
    const response = await fetch(`${this.baseUrl}/api/v1/text-to-3d/advanced`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        prompt: request.prompt,
        model: request.model || 'shap-e', // По умолчанию используем Shap-E
        num_inference_steps: request.numInferenceSteps,
        guidance_scale: request.guidanceScale,
        style: request.style,
        colors: request.colors,
        materials: request.materials,
        quality: request.quality,
        lighting: request.lighting,
        camera_angle: request.cameraAngle,
        dimensions: request.dimensions,
        language: request.language,
      }),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  }

  async generate3DModelWithSpecificModel(modelType: string, request: AdvancedTextTo3DRequest): Promise<TextTo3DResponse> {
    const response = await fetch(`${this.baseUrl}/api/v1/text-to-3d/${modelType}/generate`, {
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

  async getModelStatus(taskId: string): Promise<any> {
    // В новом API мы возвращаем статус напрямую, без необходимости дополнительного запроса
    // Эта функция может быть упрощена или удалена
    return { status: 'completed', taskId };
  }

  async downloadModel(modelUrl: string): Promise<Blob> {
    const response = await fetch(modelUrl);
    
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.blob();
  }

  async getModelRating(modelHash: string): Promise<any> {
    const response = await fetch(`${this.baseUrl}/api/v1/rating/get`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ model_hash: modelHash }),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  }

  async addModelRating(modelHash: string, userId: string, rating: number, comment?: string): Promise<any> {
    const response = await fetch(`${this.baseUrl}/api/v1/rating/add`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        model_hash: modelHash,
        user_id: userId,
        rating: rating,
        comment: comment
      }),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  }
}

export default AdvancedTextTo3DClient;
export type { AdvancedTextTo3DRequest };