export interface TextPromptRequest {
  prompt: string;
}

export interface ModelShapeAttributes {
  type: 'box' | 'sphere' | 'cylinder' | 'pyramid';
  color?: string;
  dimensions?: {
    width?: number;
    height?: number;
    depth?: number;
    radius?: number;
  };
  textureUrl?: string;
}
