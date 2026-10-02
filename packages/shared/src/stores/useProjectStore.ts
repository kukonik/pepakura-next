import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { AppMode, Project, ProjectState } from '../types/Project';

export const useProjectStore = defineStore('project', () => {
  // State
  const currentProject = ref<Project | null>(null);
  const appMode = ref<AppMode>(AppMode.VIEWER_3D);
  const isLoading = ref<boolean>(false);

  // Actions
  function createNewProject(name: string = 'Untitled Project') {
    const newProject: Project = {
      id: crypto.randomUUID(),
      name: name,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      modelObj: null,
      modelMtl: null,
      config: {
        scale: 1.0,
        units: 'mm'
      }
    };
    currentProject.value = newProject;
    return newProject;
  }

  function setMode(mode: AppMode) {
    appMode.value = mode;
  }

  function updateProjectModel(objUrl: string | null, mtlUrl: string | null = null) {
    if (!currentProject.value) {
      createNewProject();
    }
    if (currentProject.value) {
      currentProject.value.modelObj = objUrl;
      currentProject.value.modelMtl = mtlUrl;
      currentProject.value.updatedAt = new Date().toISOString();
    }
  }

  return {
    // State
    currentProject,
    appMode,
    isLoading,
    // Actions
    createNewProject,
    setMode,
    updateProjectModel
  };
});
