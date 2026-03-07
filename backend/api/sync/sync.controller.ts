// Заглушка для синхронизации
export class SyncController {
  async pushOperations(req: any, res: any) {
    res.json({ success: true, processed: req.body.operations.length })
  }
  
  async pullOperations(req: any, res: any) {
    res.json({ operations: [] })
  }
  
  async resolveConflicts(req: any, res: any) {
    res.json({ success: true })
  }
}
