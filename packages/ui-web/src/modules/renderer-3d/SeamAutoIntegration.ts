// D:\Dev\pepakura-next\src\modules\renderer-3d\SeamAutoIntegration.ts

import { SeamIntegrator } from "D:\\Dev\\pepakura-next\\src\\core\\SeamIntegrator";

export interface MeshLike {
  geometry: {
    attributes: {
      position?: {
        array: Float32Array | any;
      };
    };
    index?: {
      array: Uint16Array | Uint32Array | any;
    };
  };
}

export interface ManualSeamEditor {
  getManualSeams(): Uint32Array;
  setSeams(allSeams: Uint32Array): void;
}

export type SeamMergeMode = "replace" | "union" | "union-prefer-manual";

export interface SeamAutoOptions {
  curvatureDeg: number;
  mergeMode: SeamMergeMode;
}

export interface SeamAutoResult {
  autoEdges: Uint32Array;
  mergedEdges: Uint32Array;
  score: number;
}

export class SeamAutoIntegration {
  private readonly mesh: MeshLike;
  private readonly editor: ManualSeamEditor;

  public constructor(mesh: MeshLike, editor: ManualSeamEditor) {
    this.mesh = mesh;
    this.editor = editor;
  }

  public async runAutoSeams(
    options: SeamAutoOptions
  ): Promise<SeamAutoResult> {
    const geom = this.mesh.geometry;
    const posAttr = geom && geom.attributes && geom.attributes.position;
    const indexAttr = geom && geom.index;

    const vertices: Float32Array | null =
      posAttr && posAttr.array instanceof Float32Array
        ? (posAttr.array as Float32Array)
        : null;

    const indices: Uint16Array | Uint32Array | null =
      indexAttr && indexAttr.array
        ? (indexAttr.array as Uint16Array | Uint32Array)
        : null;

    if (!vertices || !indices) {
      throw new Error(
        "SeamAutoIntegration.runAutoSeams: no vertices or indices"
      );
    }

    const aiResult = await SeamIntegrator.analyzeSeams(
      vertices,
      indices,
      options.curvatureDeg
    );

    const manualEdges = this.editor.getManualSeams();
    const mergedEdges = this.mergeSeams(
      manualEdges,
      aiResult.edges,
      options.mergeMode
    );

    this.editor.setSeams(mergedEdges);

    return {
      autoEdges: aiResult.edges,
      mergedEdges,
      score: aiResult.score,
    };
  }

  private mergeSeams(
    manual: Uint32Array,
    auto: Uint32Array,
    mode: SeamMergeMode
  ): Uint32Array {
    if (mode === "replace") {
      return auto;
    }

    const edgeKey = (a: number, b: number): string =>
      a < b ? a.toString() + ":" + b.toString() : b.toString() + ":" + a.toString();

    const manualSet = new Set<string>();
    for (let i = 0; i + 1 < manual.length; i += 2) {
      manualSet.add(edgeKey(manual[i], manual[i + 1]));
    }

    const autoSet = new Set<string>();
    for (let i = 0; i + 1 < auto.length; i += 2) {
      autoSet.add(edgeKey(auto[i], auto[i + 1]));
    }

    const resultSet = new Set<string>();

    if (mode === "union") {
      for (const k of manualSet) resultSet.add(k);
      for (const k of autoSet) resultSet.add(k);
    } else if (mode === "union-prefer-manual") {
      for (const k of autoSet) resultSet.add(k);
      for (const k of manualSet) resultSet.add(k);
    } else {
      for (const k of manualSet) resultSet.add(k);
    }

    const result: number[] = [];
    for (const k of resultSet) {
      const parts = k.split(":");
      const a = parseInt(parts[0], 10);
      const b = parseInt(parts[1], 10);
      result.push(a, b);
    }

    return new Uint32Array(result);
  }
}
