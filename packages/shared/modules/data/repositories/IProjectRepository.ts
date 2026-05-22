import type { Project, ProjectScene } from '../../types/project.types'

export interface IProjectRepository {
  getAll(): Promise<Project[]>
  getById(id: string): Promise<Project | null>
  create(project: Omit<Project, 'id' | 'createdAt' | 'updatedAt'>): Promise<Project>
  update(id: string, updates: Partial<Project>): Promise<Project | null>
  delete(id: string): Promise<boolean>
  
  getScene(projectId: string): Promise<ProjectScene | null>
  updateScene(projectId: string, scene: ProjectScene): Promise<ProjectScene>
}
