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
