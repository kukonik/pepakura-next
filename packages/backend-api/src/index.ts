// API Types for Communication with Python Backend

export interface MeshAnalysisRequest {
  objFileUrl: string;
}

export interface MeshAnalysisResponse {
  isValid: boolean;
  issues: string[];
  suggestedFix: string;
}

export interface UnfoldRequest {
  modelData: any; // serialized geometry
  mode: 'auto' | 'manual';
}

export interface UnfoldResponse {
  flatPieces: any[]; // 2D coordinates
  cutEdges: number[]; // IDs of edges to cut
}
