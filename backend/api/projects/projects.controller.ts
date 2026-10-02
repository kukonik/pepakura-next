// Заглушка для контроллера проектов
export class ProjectsController {
  async getAllProjects(req: any, res: any) {
    res.json({ projects: [] })
  }
  
  async getProjectById(req: any, res: any) {
    res.json({ project: null })
  }
  
  async createProject(req: any, res: any) {
    res.json({ project: { id: 'new-project' } })
  }
  
  async updateProject(req: any, res: any) {
    res.json({ project: req.body })
  }
  
  async deleteProject(req: any, res: any) {
    res.json({ success: true })
  }
}
