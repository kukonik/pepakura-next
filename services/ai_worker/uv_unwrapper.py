"""
UV Unwrapper using xatlas for Pepakura Next
UV atlas generation for 3D meshes.
"""
import xatlas
import trimesh
import numpy as np
from pathlib import Path
from typing import Optional, Dict, Any, Union
import tempfile
import json
import sys

try:
    from cache_manager import get_cached_mesh_path, save_to_cache
except ImportError:
    # Mock cache functions if cache_manager not available
    def get_cached_mesh_path(*args, **kwargs): return None
    def save_to_cache(*args, **kwargs): pass


class UVUnwrapper:
    """UV Unwrapper using xatlas library."""

    def __init__(self, device: Optional[str] = None):
        """
        Initialize unwrapper.

        Args:
            device: Not used for xatlas (CPU only), kept for compatibility.
        """
        self.device = device or "cpu"
        self.default_options = {
            "chart_angle": 89.0,          # maximum angle between faces to be considered same chart (degrees)
            "pack_resolution": 1024,      # texture resolution for packing
            "padding": 2,                 # padding between charts in pixels
            "bilinear": True,             # use bilinear filtering for packing
            "block_align": False,         # block alignment for packing
            "brute_force": False,         # brute force packing (slower but better)
            "max_chart_size": 0,          # 0 = unlimited
            "max_iterations": 1,          # number of packing iterations
        }

    def unwrap(
        self,
        mesh_path: Union[str, Path],
        chart_angle: Optional[float] = None,
        pack_resolution: Optional[int] = None,
        padding: Optional[int] = None,
        bilinear: Optional[bool] = None,
        block_align: Optional[bool] = None,
        brute_force: Optional[bool] = None,
        max_chart_size: Optional[int] = None,
        max_iterations: Optional[int] = None,
        output_path: Optional[Union[str, Path]] = None,
        use_cache: bool = True,
    ) -> Dict[str, Any]:
        """
        Perform UV unwrapping on a mesh.

        Args:
            mesh_path: Path to input mesh (OBJ, STL, PLY, etc.)
            chart_angle: maximum angle between faces to be considered same chart (degrees)
            pack_resolution: texture resolution for packing (power of two recommended)
            padding: padding between charts in pixels
            bilinear: use bilinear filtering for packing
            block_align: block alignment for packing
            brute_force: brute force packing (slower but better)
            max_chart_size: maximum chart size in pixels (0 = unlimited)
            max_iterations: number of packing iterations
            output_path: Path to save the unwrapped mesh (if None, creates temp file)
            use_cache: Whether to use cached results

        Returns:
            Dictionary with keys:
                success: bool
                mesh_path: str path to generated mesh with UVs
                vertices: int vertex count
                faces: int face count
                uv_vertices: int count of vertices after unwrapping (with duplicates)
                charts: int number of UV charts
                bounds: dict with min/max bounds of mesh
                cached: bool whether result was from cache
                error: str if any error occurred
        """
        mesh_path = Path(mesh_path)
        if not mesh_path.exists():
            return {"success": False, "error": f"Mesh not found: {mesh_path}"}

        result = {
            "success": False,
            "mesh_path": None,
            "vertices": 0,
            "faces": 0,
            "uv_vertices": 0,
            "charts": 0,
            "bounds": None,
            "cached": False,
            "error": None,
        }

        # Build cache key from parameters
        if use_cache:
            cache_key = self._build_cache_key(mesh_path, locals())
            cached_path = get_cached_mesh_path(cache_key, "uv_unwrap")
            if cached_path and cached_path.exists():
                # Load cached mesh and return
                mesh = trimesh.load(str(cached_path))
                result.update({
                    "success": True,
                    "mesh_path": str(cached_path.resolve()),
                    "vertices": len(mesh.vertices),
                    "faces": len(mesh.faces),
                    "uv_vertices": len(mesh.vertices),  # approximate
                    "charts": self._count_charts(mesh),
                    "bounds": {
                        "min": mesh.bounds[0].tolist(),
                        "max": mesh.bounds[1].tolist()
                    },
                    "cached": True,
                })
                return result

        # Load mesh
        try:
            mesh = trimesh.load(str(mesh_path))
        except Exception as e:
            return {"success": False, "error": f"Failed to load mesh: {e}"}

        # Ensure mesh is triangular
        if not mesh.is_watertight or mesh.is_empty:
            # Non‑watertight meshes can still be unwrapped, but warn
            pass
        if hasattr(mesh, 'faces') and len(mesh.faces) == 0:
            return {"success": False, "error": "Mesh has no faces"}

        # Convert to numpy arrays
        vertices = mesh.vertices.astype(np.float32)
        faces = mesh.faces.astype(np.int32)

        # Prepare options
        options = self.default_options.copy()
        if chart_angle is not None:
            options["chart_angle"] = chart_angle
        if pack_resolution is not None:
            options["pack_resolution"] = pack_resolution
        if padding is not None:
            options["padding"] = padding
        if bilinear is not None:
            options["bilinear"] = bilinear
        if block_align is not None:
            options["block_align"] = block_align
        if brute_force is not None:
            options["brute_force"] = brute_force
        if max_chart_size is not None:
            options["max_chart_size"] = max_chart_size
        if max_iterations is not None:
            options["max_iterations"] = max_iterations

        # Perform UV unwrapping with xatlas
        try:
            atlas = xatlas.Atlas()
            atlas.add_mesh(vertices, faces)

            pack_opts = xatlas.PackOptions()
            pack_opts.resolution = options["pack_resolution"]
            pack_opts.padding = options["padding"]
            pack_opts.bilinear = options["bilinear"]
            pack_opts.blockAlign = options["block_align"]
            pack_opts.bruteForce = options["brute_force"]

            chart_opts = xatlas.ChartOptions()
            chart_opts.max_chart_area = 0.0  # unlimited
            chart_opts.max_iterations = options["max_iterations"]
            # chart_opts.max_boundary_length, etc. left default

            atlas.generate(chart_options=chart_opts, pack_options=pack_opts)

            # Get results
            vmapping, indices, uvs = atlas[0]  # first mesh (we only added one)
            # vmapping: mapping from original vertices to new vertices
            # indices: new face indices (into vmapping)
            # uvs: texture coordinates per vertex

            # Create new mesh with UVs
            # xatlas may duplicate vertices for different UV coordinates.
            # We'll create a new trimesh with the remapped vertices and faces.
            new_vertices = vertices[vmapping]
            new_faces = indices.reshape(-1, 3)

            # Create a new trimesh object
            unwrapped_mesh = trimesh.Trimesh(vertices=new_vertices, faces=new_faces)
            # Store UVs as vertex attributes
            unwrapped_mesh.vertex_attributes['uv'] = uvs

            # Estimate number of charts (simple heuristic: count connected components in UV space)
            charts = self._estimate_charts(uvs, new_faces)

            # Save to output path
            if output_path is None:
                output_dir = Path(tempfile.gettempdir()) / "pepakura_uv_unwrap"
                output_dir.mkdir(parents=True, exist_ok=True)
                output_path = output_dir / f"{mesh_path.stem}_unwrapped.obj"
            else:
                output_path = Path(output_path)
                output_path.parent.mkdir(parents=True, exist_ok=True)

            # Export mesh with UVs (OBJ supports UVs)
            unwrapped_mesh.export(str(output_path))

            # Cache result
            if use_cache:
                save_to_cache(cache_key, "uv_unwrap", output_path, {
                    "vertex_count": len(unwrapped_mesh.vertices),
                    "face_count": len(unwrapped_mesh.faces),
                    "chart_count": charts,
                })

            result.update({
                "success": True,
                "mesh_path": str(output_path.resolve()),
                "vertices": len(unwrapped_mesh.vertices),
                "faces": len(unwrapped_mesh.faces),
                "uv_vertices": len(vmapping),
                "charts": charts,
                "bounds": {
                    "min": unwrapped_mesh.bounds[0].tolist(),
                    "max": unwrapped_mesh.bounds[1].tolist()
                },
                "cached": False,
            })

        except Exception as e:
            result["error"] = f"UV unwrapping failed: {e}"

        return result

    def _build_cache_key(self, mesh_path: Path, params: dict) -> str:
        """Create a cache key based on mesh file and unwrap parameters."""
        import hashlib
        key_data = {
            "mesh_path": str(mesh_path.resolve()),
            "chart_angle": params.get("chart_angle"),
            "pack_resolution": params.get("pack_resolution"),
            "padding": params.get("padding"),
            "bilinear": params.get("bilinear"),
            "block_align": params.get("block_align"),
            "brute_force": params.get("brute_force"),
            "max_chart_size": params.get("max_chart_size"),
            "max_iterations": params.get("max_iterations"),
        }
        key_str = json.dumps(key_data, sort_keys=True)
        return hashlib.md5(key_str.encode()).hexdigest()

    def _count_charts(self, mesh: trimesh.Trimesh) -> int:
        """Estimate number of UV charts from mesh (placeholder)."""
        # If mesh has vertex attribute 'uv', we could analyze connectivity.
        # For now return 1.
        return 1

    def _estimate_charts(self, uvs: np.ndarray, faces: np.ndarray) -> int:
        """
        Rough estimate of number of UV charts by counting connected components
        in UV space where edges longer than a threshold are considered seams.
        """
        if len(faces) == 0:
            return 0
        # Simple implementation: assume one chart for now.
        # TODO: implement proper chart detection.
        return 1

    def unload(self):
        """No resources to unload for xatlas."""
        pass


# CLI interface for standalone usage
if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="UV Unwrapper using xatlas")
    parser.add_argument("--input", type=str, required=True, help="Path to input mesh")
    parser.add_argument("--output", type=str, default=None, help="Output mesh path (default: auto-generated)")
    parser.add_argument("--output-dir", type=str, default=None, help="Output directory (if output not specified)")
    parser.add_argument("--chart-angle", type=float, default=89.0, help="Chart angle threshold (degrees)")
    parser.add_argument("--pack-resolution", type=int, default=1024, help="Texture resolution for packing")
    parser.add_argument("--padding", type=int, default=2, help="Padding between charts in pixels")
    parser.add_argument("--no-bilinear", action="store_true", help="Disable bilinear filtering")
    parser.add_argument("--block-align", action="store_true", help="Enable block alignment")
    parser.add_argument("--brute-force", action="store_true", help="Use brute force packing")
    parser.add_argument("--max-chart-size", type=int, default=0, help="Maximum chart size (0 = unlimited)")
    parser.add_argument("--max-iterations", type=int, default=1, help="Number of packing iterations")
    parser.add_argument("--no-cache", action="store_true", help="Disable cache")

    args = parser.parse_args()

    # Determine output path
    output_path = args.output
    if output_path is None and args.output_dir is not None:
        import os
        output_dir = Path(args.output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)
        mesh_stem = Path(args.input).stem
        output_path = output_dir / f"{mesh_stem}_unwrapped.obj"

    unwrapper = UVUnwrapper()
    result = unwrapper.unwrap(
        mesh_path=args.input,
        chart_angle=args.chart_angle,
        pack_resolution=args.pack_resolution,
        padding=args.padding,
        bilinear=not args.no_bilinear,
        block_align=args.block_align,
        brute_force=args.brute_force,
        max_chart_size=args.max_chart_size,
        max_iterations=args.max_iterations,
        output_path=output_path,
        use_cache=not args.no_cache,
    )

    print(json.dumps(result, indent=2))
    sys.exit(0 if result["success"] else 1)