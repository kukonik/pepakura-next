import Dexie, { type Table } from 'dexie';
import { useProjectStore, type ProjectState } from '../stores/projectStore';
import { compress, decompress } from 'lz-string';

export interface AutosaveRecord {
  id?: number;
  timestamp: number;
  state: string; // сжатое JSON-состояние
  version: string;
  isCorrupted?: boolean;
}

export class AutosaveDatabase extends Dexie {
  autosaves!: Table<AutosaveRecord>;

  constructor() {
    super('pepakura-autosave');
    // @ts-ignore
    this.version(1).stores({
      autosaves: '++id, timestamp, isCorrupted'
    });
  }
}

export class AutosaveService {
  private db: AutosaveDatabase;
  private projectStore = useProjectStore();
  private saveInterval: number | null = null;
  private lastSavedState: string | null = null;
  private readonly SAVE_INTERVAL_MS = 60 * 1000; // 60 секунд
  private readonly MAX_AUTOSAVES = 10;

  constructor() {
    this.db = new AutosaveDatabase();
  }

  /**
   * Инициализация сервиса автосохранения
   */
  async initialize(): Promise<void> {
    // Проверяем наличие несохранённой сессии при старте
    const hasUnfinishedSession = await this.hasUnfinishedSession();
    if (hasUnfinishedSession) {
      // Флаг о наличии сессии, восстановление будет инициировано извне
      console.log('Обнаружена несохранённая сессия');
    }

    // Запускаем периодическое автосохранение
    this.startAutoSave();
  }

  /**
   * Проверяет наличие незавершённой сессии
   */
  async hasUnfinishedSession(): Promise<boolean> {
    const latest = await this.getLatestAutosave();
    return !!latest && !latest.isCorrupted;
  }

  /**
   * Получает последнее автосохранение
   */
  async getLatestAutosave(): Promise<AutosaveRecord | undefined> {
    try {
      const autosave = await this.db.autosaves
        .orderBy('timestamp')
        .reverse()
        .filter((record: AutosaveRecord) => !record.isCorrupted)
        .first();
      return autosave || undefined;
    } catch (error) {
      console.error('Ошибка при получении автосохранения:', error);
      return undefined;
    }
  }

  /**
   * Получает все автосохранения (для отладки)
   */
  async getAllAutosaves(): Promise<AutosaveRecord[]> {
    return await this.db.autosaves.orderBy('timestamp').reverse().toArray();
  }

  /**
   * Сохраняет текущее состояние проекта
   */
  async save(): Promise<void> {
    try {
      const state = this.projectStore.$state;
      const serialized = JSON.stringify(state);
      
      // Сравниваем с предыдущим состоянием, чтобы избежать лишних сохранений
      const compressed = compress(serialized);
      if (this.lastSavedState === compressed) {
        return; // Состояние не изменилось
      }

      const record: AutosaveRecord = {
        timestamp: Date.now(),
        state: compressed,
        version: '1.0'
      };

      await this.db.autosaves.add(record);
      this.lastSavedState = compressed;

      // Ограничиваем количество автосохранений
      await this.cleanupOldAutosaves();

      console.log('Автосохранение выполнено', new Date(record.timestamp).toLocaleString());
    } catch (error) {
      console.error('Ошибка автосохранения:', error);
    }
  }

  /**
   * Восстанавливает состояние из автосохранения
   */
  async restore(autosaveId?: number): Promise<boolean> {
    try {
      let record: AutosaveRecord | undefined;
      if (autosaveId) {
        record = await this.db.autosaves.get(autosaveId);
      } else {
        record = await this.getLatestAutosave();
      }

      if (!record) {
        return false;
      }

      const decompressed = decompress(record.state);
      if (!decompressed) {
        throw new Error('Не удалось распаковать состояние');
      }

      const state: ProjectState = JSON.parse(decompressed);
      
      // Восстанавливаем состояние в store
      this.projectStore.$patch(state);

      // Помечаем как восстановленное (можно удалить или пометить как corrupted)
      await this.db.autosaves.update(record.id!, { isCorrupted: true });

      console.log('Состояние восстановлено из автосохранения', new Date(record.timestamp).toLocaleString());
      return true;
    } catch (error) {
      console.error('Ошибка восстановления:', error);
      return false;
    }
  }

  /**
   * Удаляет старые автосохранения, оставляя только последние MAX_AUTOSAVES
   */
  private async cleanupOldAutosaves(): Promise<void> {
    try {
      const all = await this.db.autosaves.orderBy('timestamp').toArray();
      if (all.length > this.MAX_AUTOSAVES) {
        const toDelete = all.slice(0, all.length - this.MAX_AUTOSAVES);
        const ids = toDelete.map((r: AutosaveRecord) => r.id!).filter((id: number | undefined) => id !== undefined);
        await this.db.autosaves.bulkDelete(ids);
      }
    } catch (error) {
      console.error('Ошибка очистки старых автосохранений:', error);
    }
  }

  /**
   * Запускает периодическое автосохранение
   */
  startAutoSave(): void {
    if (this.saveInterval) {
      clearInterval(this.saveInterval);
    }

    this.saveInterval = window.setInterval(() => {
      // Используем requestIdleCallback для избежания блокировки UI
      if ('requestIdleCallback' in window) {
        (window as any).requestIdleCallback(() => this.save(), { timeout: 2000 });
      } else {
        setTimeout(() => this.save(), 0);
      }
    }, this.SAVE_INTERVAL_MS);
  }

  /**
   * Останавливает автосохранение
   */
  stopAutoSave(): void {
    if (this.saveInterval) {
      clearInterval(this.saveInterval);
      this.saveInterval = null;
    }
  }

  /**
   * Удаляет все автосохранения (очистка)
   */
  async clearAll(): Promise<void> {
    await this.db.autosaves.clear();
    this.lastSavedState = null;
  }

  /**
   * Помечает автосохранение как повреждённое
   */
  async markAsCorrupted(autosaveId: number): Promise<void> {
    await this.db.autosaves.update(autosaveId, { isCorrupted: true });
  }
}

// Экспортируем синглтон
export const autosaveService = new AutosaveService();