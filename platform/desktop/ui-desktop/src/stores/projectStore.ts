import { defineStore } from 'pinia';
import { ref } from 'vue';
import { usePlatform } from '@pepakura/shared/composables/usePlatform';
import type { ProjectInfo, ProjectId } from '@/types';

export const useProjectStore = defineStore('project', () => {
  const projects = ref<ProjectInfo[]>([]);
  const currentProject = ref<ProjectId | null>(null);
  const { invoke } = usePlatform();

  /**
   * Загрузить список последних проектов из платформенного бэкенда
   */
  const loadRecent = async () => {
    try {
      // Вызов команды get_recent_projects
      const recent = await invoke<ProjectInfo[]>('get_recent_projects');
      projects.value = recent;
    } catch (error) {
      console.error('Failed to load recent projects:', error);
      // В случае ошибки оставляем пустой список
      projects.value = [];
    }
  };

  /**
   * Создать новый проект
   * @param name - Название проекта
   * @returns ID созданного проекта
   */
  const create = async (name: string): Promise<ProjectId> => {
    try {
      const projectId = await invoke<ProjectId>('create_project', { name });
      // После создания перезагружаем список проектов
      await loadRecent();
      return projectId;
    } catch (error) {
      console.error('Failed to create project:', error);
      throw error;
    }
  };

  /**
   * Удалить проект
   * @param id - ID проекта
   */
  const remove = async (id: ProjectId) => {
    try {
      await invoke('delete_project', { id });
      // Удаляем проект из локального списка
      projects.value = projects.value.filter(p => p.id !== id);
      if (currentProject.value === id) {
        currentProject.value = null;
      }
    } catch (error) {
      console.error('Failed to delete project:', error);
      throw error;
    }
  };

  /**
   * Импортировать модель в текущий проект
   * @param path - Путь к файлу модели
   * @param format - Формат файла
   * @returns Загруженная модель
   */
  const importModel = async (path: string, format: string) => {
    return await invoke('import_model', { path, format });
  };

  /**
   * Установить текущий проект
   * @param id - ID проекта
   */
  const setCurrentProject = (id: ProjectId | null) => {
    currentProject.value = id;
  };

  return {
    projects,
    currentProject,
    loadRecent,
    create,
    remove,
    importModel,
    setCurrentProject,
  };
});