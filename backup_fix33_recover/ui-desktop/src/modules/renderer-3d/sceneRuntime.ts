import type { Mesh } from "three";

export interface SeamEdge {
  id: string;
  vertex1: [number, number, number];
  vertex2: [number, number, number];
  isCut: boolean;
}

export type SeamSet = SeamEdge[];

export class SceneRuntime {
  private readonly baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async generateSeams(_mesh: Mesh): Promise<SeamSet> {
    // Временная заглушка для desktop-версии:
    // возвращаем пустой набор швов, чтобы не ломать UI.
    return [];
  }
}
