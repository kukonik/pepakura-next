export interface ImageTo3DPayload {
  imagePath: string;
  quality?: 'fast' | 'balanced' | 'high';
}

export interface ImageTo3DResult {
  success: boolean;
  modelPath?: string;
  errorMessage?: string;
  logOutput?: string;
  cached: boolean;
}
