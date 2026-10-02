export const isTauri: boolean = typeof window !== "undefined" && "__TAURI__" in window;

export function getPlatformName(): string {
  if (isTauri) return "tauri";
  return "web";
}
