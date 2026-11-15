import { invoke } from "@tauri-apps/api/core";
import type { PepaScene } from "@/types/pepa-types";

interface ParsePdoResult {
  success: boolean;
  error?: string;
  scene?: PepaScene;
}

/**
 * Parse a PDO file and convert to PepaScene using Rust backend
 */
export async function parsePdoToPepaScene(fileData: ArrayBuffer): Promise<PepaScene> {
  try {
    const result: ParsePdoResult = await invoke("parse_pdo_to_pepa", {
      data: Array.from(new Uint8Array(fileData))
    });

    if (!result.success || !result.scene) {
      throw new Error(result.error || "Failed to parse PDO file");
    }

    console.log("✅ PDO файл успешно разобран через Rust-парсер");
    return result.scene;
  } catch (error) {
    console.error("❌ Ошибка парсинга PDO:", error);
    throw error;
  }
}
