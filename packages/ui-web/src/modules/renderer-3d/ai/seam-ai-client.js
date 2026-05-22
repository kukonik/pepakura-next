"use strict";
/**
 * Клиент для взаимодействия с сервисом AI-швов (FastAPI).
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.SeamAiClient = void 0;
class SeamAiClient {
    constructor(config) {
        var _a;
        this.baseUrl = config.baseUrl.replace(/\/+$/, "");
        this.timeoutMs = (_a = config.timeoutMs) !== null && _a !== void 0 ? _a : 30000;
    }
    async predictSeams(mesh, curvatureDeg) {
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
        }
        finally {
            clearTimeout(timeoutId);
        }
    }
}
exports.SeamAiClient = SeamAiClient;
