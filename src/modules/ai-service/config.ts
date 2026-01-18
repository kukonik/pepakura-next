// src/modules/ai-service/config.ts
import { HttpAiClient, type HttpAiClientOptions } from "./httpAiClient";
import type { IAiClient } from "./client";
import { OllamaClient, type OllamaClientOptions } from "./ollamaClient";

export type AiBackendType = "openai" | "ollama";

export interface AiServiceConfig {
  backend: AiBackendType;
  endpointUrl: string;
  apiKey?: string;
  model: string;
  timeoutMs?: number;
}

export const createAiClient = (config: AiServiceConfig): IAiClient => {
  if (config.backend === "ollama") {
    const options: OllamaClientOptions = {
      baseUrl: config.endpointUrl || "http://localhost:11434",
      model: config.model,
      timeoutMs: config.timeoutMs
    };
    return new OllamaClient(options);
  }

  const options: HttpAiClientOptions = {
    baseUrl: config.endpointUrl,
    apiKey: config.apiKey,
    timeoutMs: config.timeoutMs
  };

  return new HttpAiClient(options);
};
