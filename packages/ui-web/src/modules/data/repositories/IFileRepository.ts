export interface IFileRepository {
  upload(file: File, projectId: string): Promise<string> // returns file ID
  download(fileId: string): Promise<Blob>
  delete(fileId: string): Promise<boolean>
  getUrl(fileId: string): Promise<string>
}
