// src/modules/ai-service/ollamaClient.ts
import type { IAiClient } from "./client";
import type {
  AiCompletionRequest,
  AiCompletionResponse,
  AiMessage
} from "./types";

export interface OllamaClientOptions {
  baseUrl?: string;
  model: string;
  timeoutMs?: number;
}

interface OllamaChatRequest {
  model: string;
  messages: AiMessage[];
  stream?: boolean;
}

interface OllamaChatResponseChunk {
  model: string;
  message: AiMessage;
  done: boolean;
}

export class OllamaClient implements IAiClient {
  private readonly baseUrl: string;
  private readonly model: string;
  private readonly timeoutMs: number;

  constructor(options: OllamaClientOptions) {
    this.baseUrl = (options.baseUrl ?? "http://localhost:11434").replace(/\/+$/, "");
    this.model = options.model;
    this.timeoutMs = options.timeoutMs ?? 60000;
  }

  async createChatCompletion(
    payload: AiCompletionRequest,
    externalSignal?: AbortSignal
  ): Promise<AiCompletionResponse> {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeoutMs);

    const signal = externalSignal
      ? this.combineSignals([controller.signal, externalSignal])
      : controller.signal;

    try {
      const body: OllamaChatRequest = {
        model: this.model,
        messages: payload.messages,
        stream: false
      };

      const res = await fetch(`${this.baseUrl}/api/chat`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(body),
        signal
      });

      if (!res.ok) {
        const text = await res.text().catch(() => "");
        throw new Error(`Ollama request failed: ${res.status} ${res.statusText} ${text}`);
      }

      const data = (await res.json()) as OllamaChatResponseChunk;

      const response: AiCompletionResponse = {
        id: `${Date.now()}`,
        created: Math.floor(Date.now() / 1000),
        model: data.model,
        choices: [
          {
            index: 0,
            message: data.message,
            finishReason: "stop"
          }
        ]
      };

      return response;
    } finally {
      clearTimeout(timeoutId);
    }
  }

  private combineSignals(signals: AbortSignal[]): AbortSignal {
    const controller = new AbortController();

    for (const s of signals) {
      if (s.aborted) {
        controller.abort(s.reason);
        return controller.signal;
      }

      s.addEventListener(
        "abort",
        () => controller.abort(s.reason),
        { once: true }
      );
    }

    return controller.signal;
  }
}
