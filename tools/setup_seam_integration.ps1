# D:\Dev\pepakura-next\tools\setup_seam_integration.ps1
# Создаёт модули интеграции AI-швов в проект Pepakura Next.

param()

$projectRoot = "D:\Dev\pepakura-next"
$srcCore     = Join-Path $projectRoot "src\core"
$srcRenderer = Join-Path $projectRoot "src\modules\renderer-3d"

New-Item -ItemType Directory -Force -Path $srcCore      | Out-Null
New-Item -ItemType Directory -Force -Path $srcRenderer  | Out-Null

# ----- 1) SeamIntegrator.ts -----
$seamIntegratorTs = @"
/**
 * D:\Dev\pepakura-next\src\core\SeamIntegrator.ts
 *
 * Модуль интеграции AI-швов в проект.
 * Требует запущенного Python-сервера:
 *   POST http://127.0.0.1:8000/api/seams/predict?curvature_deg=...
 * тело:
 *   { vertices: [{x,y,z},...], indices: [i0,i1,i2,...] }
 * ответ:
 *   { edges: [{v1,v2},...], score: number }
 */

export interface SeamIntegratorResult {
  edges: Uint32Array;
  score: number;
}

export class SeamIntegrator {
  private static readonly API_URL: string =
    "http://127.0.0.1:8000/api/seams/predict";

  public static async analyzeSeams(
    vertices: Float32Array,
    indices: Uint16Array | Uint32Array,
    curvatureDeg: number = 30.0
  ): Promise<SeamIntegratorResult> {
    const vertexList: { x: number; y: number; z: number }[] = [];
    for (let i = 0; i < vertices.length; i += 3) {
      vertexList.push({
        x: vertices[i],
        y: vertices[i + 1],
        z: vertices[i + 2],
      });
    }

    const indicesArray: number[] = Array.from(indices);

    const payload = {
      vertices: vertexList,
      indices: indicesArray,
    };

    const url = new URL(SeamIntegrator.API_URL);
    url.searchParams.set("curvature_deg", String(curvatureDeg));

    try {
      const response = await fetch(url.toString(), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(
          "SeamIntegrator: AI Server Error: " + String(response.status)
        );
      }

      const data: {
        edges: { v1: number; v2: number }[];
        score: number;
      } = await response.json();

      const flatEdges: number[] = [];
      for (const edge of data.edges) {
        flatEdges.push(edge.v1, edge.v2);
      }

      return {
        edges: new Uint32Array(flatEdges),
        score: data.score,
      };
    } catch (error) {
      console.error(
        "SeamIntegrator: Failed to connect to AI Service",
        error
      );
      throw error;
    }
  }
}
"@

Set-Content -Path (Join-Path $srcCore "SeamIntegrator.ts") -Value $seamIntegratorTs -Encoding UTF8

# ----- 2) SeamAutoIntegration.ts -----
$seamAutoIntegrationTs = @"
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
"@

Set-Content -Path (Join-Path $srcRenderer "SeamAutoIntegration.ts") -Value $seamAutoIntegrationTs -Encoding UTF8

# ----- 3) SeamAutoUiBridge.ts -----
$seamAutoUiBridgeTs = @"
// D:\Dev\pepakura-next\src\modules\renderer-3d\SeamAutoUiBridge.ts

import {
  SeamAutoIntegration,
  SeamAutoOptions,
  MeshLike,
  ManualSeamEditor,
  SeamAutoResult,
} from "D:\\Dev\\pepakura-next\\src\\modules\\renderer-3d\\SeamAutoIntegration";

export async function runAutoSeamsFromUI(
  mesh: MeshLike,
  editor: ManualSeamEditor,
  curvatureDeg: number = 30.0,
  mergeMode: "replace" | "union" | "union-prefer-manual" = "union-prefer-manual"
): Promise<SeamAutoResult> {
  const options: SeamAutoOptions = {
    curvatureDeg,
    mergeMode,
  };

  const integrator = new SeamAutoIntegration(mesh, editor);
  const result = await integrator.runAutoSeams(options);

  console.log(
    "runAutoSeamsFromUI:",
    "autoEdges=" + String(result.autoEdges.length / 2),
    "mergedEdges=" + String(result.mergedEdges.length / 2),
    "score=" + String(result.score)
  );

  return result;
}
"@

Set-Content -Path (Join-Path $srcRenderer "SeamAutoUiBridge.ts") -Value $seamAutoUiBridgeTs -Encoding UTF8

Write-Host "[OK] Созданы файлы:"
Write-Host " - $srcCore\SeamIntegrator.ts"
Write-Host " - $srcRenderer\SeamAutoIntegration.ts"
Write-Host " - $srcRenderer\SeamAutoUiBridge.ts"
