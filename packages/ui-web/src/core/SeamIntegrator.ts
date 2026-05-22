/**
 * D:\Dev\pepakura-next\src\core\SeamIntegrator.ts
 *
 * Модуль интеграции AI-швов в проект.
 * Требует запущенного Python-сервера:
 *   POST http://127.0.0.1:8000/api/seams/predict?curvature_deg=...
 * тело:
 *   { vertices: [{x,y,z},...], indices: [i0,i1,i2,...] }
 * ответ:
 *   { edges: [{v1,v2},...], score: number }
 */

export interface SeamIntegratorResult {
  edges: Uint32Array;
  score: number;
}

export class SeamIntegrator {
  private static readonly API_URL: string =
    "http://127.0.0.1:8000/api/seams/predict";

  public static async analyzeSeams(
    vertices: Float32Array,
    indices: Uint16Array | Uint32Array,
    curvatureDeg: number = 30.0
  ): Promise<SeamIntegratorResult> {
    const vertexList: { x: number; y: number; z: number }[] = [];
    for (let i = 0; i < vertices.length; i += 3) {
      vertexList.push({
        x: vertices[i],
        y: vertices[i + 1],
        z: vertices[i + 2],
      });
    }

    const indicesArray: number[] = Array.from(indices);

    const payload = {
      vertices: vertexList,
      indices: indicesArray,
    };

    const url = new URL(SeamIntegrator.API_URL);
    url.searchParams.set("curvature_deg", String(curvatureDeg));

    try {
      const response = await fetch(url.toString(), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(
          "SeamIntegrator: AI Server Error: " + String(response.status)
        );
      }

      const data: {
        edges: { v1: number; v2: number }[];
        score: number;
      } = await response.json();

      const flatEdges: number[] = [];
      for (const edge of data.edges) {
        flatEdges.push(edge.v1, edge.v2);
      }

      return {
        edges: new Uint32Array(flatEdges),
        score: data.score,
      };
    } catch (error) {
      console.error(
        "SeamIntegrator: Failed to connect to AI Service",
        error
      );
      throw error;
    }
  }
}
