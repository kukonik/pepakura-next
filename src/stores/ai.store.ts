// src/stores/ai.store.ts
import { defineStore } from "pinia";
import { ref, computed } from "vue";
import {
  type AiMessage,
  type AiCompletionRequest,
  type AiCompletionResponse,
  type AiServiceConfig,
  createAiClient
} from "@modules/ai-service";

export type AiBackendType = "openai" | "ollama";

export interface AiSessionMessage extends AiMessage {
  id: string;
  createdAt: Date;
}

export interface AiSession {
  id: string;
  projectId?: string;
  title: string;
  createdAt: Date;
  updatedAt: Date;
  messages: AiSessionMessage[];
}

export interface AiSettings {
  backend: AiBackendType;
  endpointUrl: string;
  apiKey?: string;
  model: string;
  temperature: number;
  maxTokens: number;
}

const DEFAULT_AI_SETTINGS: AiSettings = {
  backend: "ollama",
  endpointUrl: "http://localhost:11434",
  apiKey: "",
  model: "gemma3:1b",
  temperature: 0.2,
  maxTokens: 512
};

export const useAiStore = defineStore("ai", () => {
  const sessions = ref<AiSession[]>([]);
  const currentSessionId = ref<string | null>(null);
  const settings = ref<AiSettings>({ ...DEFAULT_AI_SETTINGS });
  const isRequestInProgress = ref(false);
  const lastError = ref<string | null>(null);

  const currentSession = computed<AiSession | null>(() => {
    if (!currentSessionId.value) return null;
    return sessions.value.find(s => s.id === currentSessionId.value) ?? null;
  });

  const hasActiveSession = computed(() => currentSession.value !== null);

  const aiConfig = computed<AiServiceConfig>(() => ({
    backend: settings.value.backend,
    endpointUrl: settings.value.endpointUrl,
    apiKey: settings.value.apiKey || undefined,
    model: settings.value.model,
    timeoutMs: 60000
  }));

  const setSettings = (partial: Partial<AiSettings>) => {
    settings.value = {
      ...settings.value,
      ...partial
    };
    localStorage.setItem("ai-settings", JSON.stringify(settings.value));
  };

  const loadSettingsFromStorage = () => {
    try {
      const raw = localStorage.getItem("ai-settings");
      if (raw) {
        const parsed = JSON.parse(raw) as AiSettings;
        settings.value = {
          ...DEFAULT_AI_SETTINGS,
          ...parsed
        };
      }
    } catch {
      settings.value = { ...DEFAULT_AI_SETTINGS };
    }
  };

  const createSession = (params?: { projectId?: string; title?: string }): AiSession => {
    const now = new Date();
    const session: AiSession = {
      id: generateId(),
      projectId: params?.projectId,
      title: params?.title ?? "Новая AI-сессия",
      createdAt: now,
      updatedAt: now,
      messages: []
    };

    sessions.value.unshift(session);
    currentSessionId.value = session.id;
    saveSessionsToStorage();
    return session;
  };

  const setCurrentSession = (sessionId: string | null) => {
    currentSessionId.value = sessionId;
  };

  const removeSession = (sessionId: string) => {
    const idx = sessions.value.findIndex(s => s.id === sessionId);
    if (idx >= 0) {
      sessions.value.splice(idx, 1);
      if (currentSessionId.value === sessionId) {
        currentSessionId.value = sessions.value[0]?.id ?? null;
      }
      saveSessionsToStorage();
    }
  };

  const addMessageToSession = (
    sessionId: string,
    message: AiMessage
  ): AiSessionMessage | null => {
    const session = sessions.value.find(s => s.id === sessionId);
    if (!session) return null;

    const msg: AiSessionMessage = {
      id: generateId(),
      role: message.role,
      content: message.content,
      createdAt: new Date()
    };

    session.messages.push(msg);
    session.updatedAt = new Date();
    saveSessionsToStorage();
    return msg;
  };

  const sendMessage = async (userContent: string) => {
    if (!userContent.trim()) return;

    if (!currentSession.value) {
      createSession();
    }

    const session = currentSession.value;
    if (!session) return;

    lastError.value = null;

    const userMessage: AiMessage = {
      role: "user",
      content: userContent
    };
    addMessageToSession(session.id, userMessage);

    const client = createAiClient(aiConfig.value);

    const payload: AiCompletionRequest = {
      model: settings.value.model,
      messages: session.messages.map(m => ({
        role: m.role,
        content: m.content
      })),
      temperature: settings.value.temperature,
      maxTokens: settings.value.maxTokens
    };

    isRequestInProgress.value = true;

    try {
      const response: AiCompletionResponse = await client.createChatCompletion(payload);
      const assistantMessage = response.choices[0]?.message;
      if (assistantMessage) {
        addMessageToSession(session.id, assistantMessage);
      }
    } catch (error: unknown) {
      lastError.value = error instanceof Error ? error.message : String(error);
    } finally {
      isRequestInProgress.value = false;
    }
  };

  const loadSessionsFromStorage = () => {
    try {
      const raw = localStorage.getItem("ai-sessions");
      if (!raw) return;

      const parsed = JSON.parse(raw) as {
        id: string;
        projectId?: string;
        title: string;
        createdAt: string;
        updatedAt: string;
        messages: {
          id: string;
          role: AiMessage["role"];
          content: string;
          createdAt: string;
        }[];
      }[];

      sessions.value = parsed.map(s => ({
        ...s,
        createdAt: new Date(s.createdAt),
        updatedAt: new Date(s.updatedAt),
        messages: s.messages.map(m => ({
          ...m,
          createdAt: new Date(m.createdAt)
        }))
      }));
    } catch {
      sessions.value = [];
    }
  };

  const saveSessionsToStorage = () => {
    try {
      const serializable = sessions.value.map(s => ({
        ...s,
        createdAt: s.createdAt.toISOString(),
        updatedAt: s.updatedAt.toISOString(),
        messages: s.messages.map(m => ({
          ...m,
          createdAt: m.createdAt.toISOString()
        }))
      }));
      localStorage.setItem("ai-sessions", JSON.stringify(serializable));
    } catch {
      // ignore
    }
  };

  const initializeAiStore = () => {
    loadSettingsFromStorage();
    loadSessionsFromStorage();
  };

  const generateId = (): string => {
    return Date.now().toString(36) + Math.random().toString(36).substring(2);
  };

  return {
    sessions,
    currentSessionId,
    settings,
    isRequestInProgress,
    lastError,

    currentSession,
    hasActiveSession,
    aiConfig,

    setSettings,
    createSession,
    setCurrentSession,
    removeSession,
    addMessageToSession,
    sendMessage,
    initializeAiStore
  };
});
