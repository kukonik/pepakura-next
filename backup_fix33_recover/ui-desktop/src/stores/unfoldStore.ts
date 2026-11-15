import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useProjectStore } from "@/stores/project";

export type LineKind = "Cut" | "Valley" | "Mountain" | "GlueTab";

export interface Line2D {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  kind: LineKind;
}

export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface Part2D {
  id: number;
  name?: string;
  bounds: Rect;
  lines: Line2D[];
}

export interface Sheet {
  id: number;
  index: number;
  width_mm: number;
  height_mm: number;
  margin_mm: number;
  parts: Part2D[];
}

export interface UnfoldResult {
  sheets: Sheet[];
}

export interface UnfoldParams {
  paper_format: string;
  margin_mm: number;
  max_sheets: number;
  scale: number;
}

interface UnfoldState {
  result: UnfoldResult | null;
  isUnfolding: boolean;
  lastError: string | null;
  params: UnfoldParams;
}

export const useUnfoldStore = defineStore("unfold", {
  state: (): UnfoldState => ({
    result: null,
    isUnfolding: false,
    lastError: null,
    params: {
      paper_format: "A4",
      margin_mm: 5,
      max_sheets: 4,
      scale: 1.0,
    },
  }),

  getters: {
    hasResult: (state) => !!state.result,
    sheets: (state) => state.result?.sheets ?? [],
  },

  actions: {
    setParams(params: Partial<UnfoldParams>) {
      this.params = { ...this.params, ...params };
    },

    clearResult() {
      this.result = null;
      this.lastError = null;
    },

    async unfoldCurrentModel() {
      const projectStore = useProjectStore();
      const modelPath = projectStore.currentModelPath;

      if (!modelPath) {
        this.lastError = "3D модель не загружена";
        return;
      }

      this.isUnfolding = true;
      this.lastError = null;

      try {
        const result = await invoke<UnfoldResult>("unfold_3d_model", {
          modelPath,
          params: this.params,
        });

        this.result = result;
      } catch (err: any) {
        console.error("[UnfoldStore] unfoldCurrentModel error", err);
        this.lastError = String(err?.message ?? err ?? "Unknown error");
      } finally {
        this.isUnfolding = false;
      }
    },

    async exportPdf() {
      if (!this.result) {
        this.lastError = "Нет данных развёртки для экспорта";
        return;
      }

      this.isUnfolding = true;
      this.lastError = null;

      try {
        await invoke("export_unfold_pdf", {
          unfold: this.result,
        });
      } catch (err: any) {
        console.error("[UnfoldStore] exportPdf error", err);
        this.lastError = String(err?.message ?? err ?? "Unknown error");
      } finally {
        this.isUnfolding = false;
      }
    },
  },
});
