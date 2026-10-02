// shared/src/stores/draftStorage.ts
import { get, set, del, keys } from 'idb-keyval';

export interface DraftEntry {
  id: string;
  createdAt: Date;
  updatedAt: Date;
  title?: string;
  description?: string;
  previewImage?: string;
  prompt: string;
  modelData: string;         // obj
  format?: '.obj' | '.glb';
  svgSheet?: string;         // unfolds/svg
  metadata?: Record<string, any>;
}

/**
 * Сохранить черновик
 */
export async function saveDraft(id: string, entry: Omit<DraftEntry, 'id' | 'createdAt' | 'updatedAt'>): Promise<void> {
  const now = new Date();
  const draft: DraftEntry = {
    id,
    createdAt: now,
    updatedAt: now,
    ...entry
  };
  await set(`draft_${id}`, draft);
}

/**
 * Получить черновик по ID
 */
export async function loadDraft(id: string): Promise<DraftEntry | undefined> {
  return await get(`draft_${id}`);
}

/**
 * Удалить черновик
 */
export async function deleteDraft(id: string): Promise<void> {
  await del(`draft_${id}`);
}

/**
 * Получить список всех черновиков
 */
export async function getAllDraftIds(): Promise<string[]> {
  const allKeys = await keys();
  return allKeys
    .filter(k => typeof k === 'string' && k.startsWith('draft_'))
    .map(k => (k as string).substring(6));
}
