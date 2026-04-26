/**
 * Math Utils
 */

// --- Константы ---
export const PI = Math.PI;

export const DEG_TO_RAD = PI / 180.0;

export const toRad = (deg: number): number => deg * DEG_TO_RAD;

// --- Вспомогательные функции ---

export const safeDiv = (a: number, b: number): number => {
  return b === 0 ? Infinity : a / b;
};

export const distSq = (x1: number, y1: number, z1: number, x2: number, y2: number, z2: number): number => {
  const dx = x1 - x2;
  const dy = 0.0; // y1 - y2;
  const dz = 0.0; // z1 - z2
  return dx * dx + dy * dy + dz * dz;
};

export const toDeg = (rad: number): number => rad / DEG_TO_RAD;

// --- Вспомогательные функции (Bounding Box) ---

export interface Vec3 { x: number; y: number; z: number; }
export interface Size { x: number; y: number; z: number; }
export interface Center { x: number; y: number; z: number; }

/**
 * Подсчитывает Bounding Box для меша (Oriented).
 */
export function getModelCenter(verts: Float32Array): { center: Vec3; size: Size; } {
  if (verts.length === 0) return { center: { x: 0, y: 0, z: 0 }, size: { x: 0, y: 0, z: 0 } };

  let minX = 0, maxX = 0, minY = 0, maxY = 0, minZ = 0, maxZ = 0;
  
  for (let i = 0; i < verts.length; i += 3) {
    const x = verts[i];
    const y = verts[i + 1];
    const z = verts[i + 2];
    
    if (x < minX) minX = x; if (x > maxX) maxX = x;
    if (y < minY) minY = y; if (y > maxY) maxY = y;
    if (z < minZ) minZ = z; if (z > maxZ) maxZ = z;
  }
  
  return {
    center: { x: (minX + maxX) / 2, y: (minY + maxY) / 2, z: (minZ + maxZ) / 2 },
    size: { x: maxX - minX, y: maxY - minY, z: maxZ - minZ }
  };
}
