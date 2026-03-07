/**
 * Клиент для работы с AI (Mock или HTTP).
 * Использует `fetch` из браузера (Tauri использует Node.js).
 */

// --- Интерфейс ---

export interface SeamPredictionResult {
  edges: {
    v1: number;
    v2: number;
    confidence: number;
  }[];
  score: number;
}

// --- Реализация ---

/**
 * Выполняет запрос на сервер для предсказки швов.
 */
export const predictSeams = async (
  mesh: THREE.BufferGeometry,
  curvatureDeg: number
): Promise<SeamPredictionResult> => {
  console.log(`[SeamClient] Запрос швов, порог: ${curvatureDeg}...`);

  try {
    const vertices: { x: number; y: number; z: number; }[] = [];
    const indices = mesh.index ? mesh.index.array : new Uint32Array(geometry.attributes.position.count);
    
    for (let i = 0; i < geometry.attributes.position.count; i += 3) {
      vertices.push({
        x: geometry.attributes.position.array[i],
        y: geometry.attributes.position.array[i + 1],
        z: geometry.attributes.position.array[i + 2]
      });
    }

    // Конвертация для отправки в JSON (Mock данные для прототипа)
    const payload = {
      vertices,
      indices
    };

    const url = `http://127.0.0.1:8000/api/seams/predict`;
    const query = new URLSearchParams();
    if (curvatureDeg) {
      query.set("curvature_deg", String(curvatureDeg));
    }

    const finalUrl = `${url}?${query.toString()}`;
    console.log(`[SeamClient] Отправка на ${finalUrl}`);

    try {
      const response = await fetch(finalUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      const data = await response.json();
      console.log(`[SeamClient] Ответ получен: ${data.edges?.length || 0} швов.`);
      
      // Форматирование для SeamAssistant
      return data;

    } catch (error) {
      console.error("[SeamClient] Ошибка запроса:", error);
      throw error; // Пробрасываем ошибку, чтобы её видел и Vue и AI Assistant
    }
};
