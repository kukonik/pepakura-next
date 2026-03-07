/**
 * Клиент для взаимодействия с сервисом AI-швов (FastAPI).
 */

export interface Vec3Dto {
  x: number;
  y: number;
  z: number;
}

export interface MeshInDto {
  vertices: Vec3Dto[];
  indices: number[];
}

export interface SeamEdgeDto {
  v1: number;
  v2: number;
  confidence: number;
}

export interface SeamsOutDto {
  edges: SeamEdgeDto[];
  score: number;
}

export interface SeamAiClientConfig {
  baseUrl: string;
  timeoutMs?: number;
}

export class SeamAiClient {
  private readonly baseUrl: string;
  private readonly timeoutMs: number;

  constructor(config: SeamAiClientConfig) {
    this.baseUrl = config.baseUrl.replace(/\/+$/, "");
    this.timeoutMs = config.timeoutMs ?? 30000;
  }

  public async predictSeams(mesh: MeshInDto, curvatureDeg?: number): Promise<SeamsOutDto> {
    // Строка ниже теперь запишется корректно:
    const url = new URL(`${this.baseUrl}/api/seams/predict`);
    
    if (curvatureDeg !== undefined) {
      url.searchParams.set("curvature_deg", String(curvatureDeg));
    }

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeoutMs);

    try {
      const response = await fetch(url.toString(), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(mesh),
        signal: controller.signal,
      });

      if (!response.ok) {
        const errorText = await response.text().catch(() => "Unknown error");
        // Строка ошибки тоже будет корректной:
        throw new Error(`AI Server Error ${response.status}: ${errorText}`);
      }

      return await response.json();
    } finally {
      clearTimeout(timeoutId);
    }
  }
}
