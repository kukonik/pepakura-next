"""
Hunyuan3D-2 Integration for Pepakura Next
Text-to-3D generation using Tencent's Hunyuan3D-2
"""
import torch
import numpy as np
import trimesh
import json
import sys
import shutil
from pathlib import Path
from typing import Optional, Dict, Literal, Union
import tempfile

# Add Hunyuan3D-2 directory to path if exists
CURRENT_DIR = Path(__file__).parent
HUNYUAN_DIR = CURRENT_DIR / "Hunyuan3D-2"
if HUNYUAN_DIR.exists():
    sys.path.append(str(HUNYUAN_DIR))
else:
    # If not present, we'll try to import from installed package
    pass

try:
    # Try to import Hunyuan3D-2 modules
    # Actual import may vary; adjust based on actual module structure
    from hunyuan3d import Hunyuan3DGenerator as HunyuanModel
    HAS_HUNYUAN = True
except ImportError:
    try:
        from Hunyuan3D2 import Hunyuan3DGenerator as HunyuanModel
        HAS_HUNYUAN = True
    except ImportError:
        HAS_HUNYUAN = False

try:
    from cache_manager import get_cached_mesh_path, save_to_cache
except ImportError:
    # Mock cache functions if cache_manager not available
    def get_cached_mesh_path(*args, **kwargs): return None
    def save_to_cache(*args, **kwargs): pass

Quality = Literal["low", "medium", "high"]
Format = Literal["obj", "stl", "glb"]

class HunyuanGenerator:
    """Hunyuan3D-2 text-to-3D generator."""
    
    def __init__(self, device: Optional[str] = None):
        """
        Initialize generator.
        
        Args:
            device: 'cuda' or 'cpu'. If None, auto-select.
        """
        self.device = device or ("cuda" if torch.cuda.is_available() else "cpu")
        self.model: Optional[HunyuanModel] = None
        self.quality_params = {
            "low": {"steps": 30, "resolution": 256},
            "medium": {"steps": 60, "resolution": 384},
            "high": {"steps": 100, "resolution": 512},
        }
    
    def load_model(self, quality: Quality = "medium") -> None:
        """
        Load Hunyuan3D-2 model with given quality preset.
        
        Args:
            quality: "low", "medium", or "high"
        """
        if self.model is not None:
            return
        
        if not HAS_HUNYUAN:
            print("WARNING: Hunyuan3D-2 not installed. Using dummy mesh for testing.", file=sys.stderr)
            print("To install Hunyuan3D-2:", file=sys.stderr)
            print("  git clone https://github.com/Tencent-Hunyuan/Hunyuan3D-2.git", file=sys.stderr)
            print("  cd Hunyuan3D-2 && pip install -e .", file=sys.stderr)
            # Keep self.model as None, generation will fallback to dummy mesh
            return

        params = self.quality_params[quality]
        print(f"Loading Hunyuan3D-2 model on {self.device} with quality {quality}...", file=sys.stderr)
        # Actual loading code depends on the model's API
        # This is a placeholder; replace with actual initialization
        self.model = HunyuanModel(device=self.device, **params)
        print("Model loaded successfully!", file=sys.stderr)
    
    def generate_from_text(
        self,
        prompt: str,
        quality: Quality = "medium",
        output_format: Format = "obj",
        use_cache: bool = True,
        output_dir: Optional[Union[str, Path]] = None
    ) -> Dict:
        """
        Generate 3D mesh from a text prompt.
        
        Args:
            prompt: Text description of the 3D object.
            quality: "low", "medium", "high"
            output_format: "obj", "stl", "glb"
            use_cache: Whether to use cached results
            output_dir: Directory to save output mesh. If None, uses temp dir.
        
        Returns:
            Dictionary with keys:
                success: bool
                mesh_path: str path to generated mesh
                vertices: int vertex count
                faces: int face count
                bounds: dict with min/max bounds
                cached: bool whether result was from cache
                device: str device used
                error: str if any error occurred
        """
        result = {
            "success": False,
            "mesh_path": None,
            "vertices": 0,
            "faces": 0,
            "bounds": None,
            "cached": False,
            "device": self.device,
            "error": None
        }
        
        # Check cache
        if use_cache:
            # Use prompt hash as cache key
            import hashlib
            prompt_hash = hashlib.md5(prompt.encode()).hexdigest()
            cached_path = get_cached_mesh_path(prompt_hash, quality)
            if cached_path and cached_path.exists():
                # Copy to output location
                if output_dir is None:
                    output_dir = Path(tempfile.gettempdir()) / "pepakura_hunyuan"
                output_dir = Path(output_dir)
                output_dir.mkdir(parents=True, exist_ok=True)
                mesh_filename = f"hunyuan_mesh_{prompt_hash}.{output_format}"
                mesh_path = output_dir / mesh_filename
                shutil.copy2(cached_path, mesh_path)
                # Convert format if needed
                if output_format != "obj":
                    mesh = trimesh.load(str(cached_path))
                    mesh.export(str(mesh_path))
                else:
                    mesh = trimesh.load(str(mesh_path))

                result.update({
                    "success": True,
                    "mesh_path": str(mesh_path.resolve()),
                    "vertices": len(mesh.vertices),
                    "faces": len(mesh.faces),
                    "bounds": {
                        "min": mesh.bounds[0].tolist(),
                        "max": mesh.bounds[1].tolist()
                    },
                    "cached": True,
                    "device": "cache"
                })
                return result
        
        # Load model if not loaded
        if self.model is None:
            self.load_model(quality)
        
        params = self.quality_params[quality]
        
        try:
            # Generate mesh using Hunyuan3D-2
            # This is a placeholder; replace with actual generation
            if HAS_HUNYUAN and self.model is not None:
                # Assuming the model has a generate method
                mesh_data = self.model.generate(prompt, steps=params["steps"])
                # Convert mesh_data to trimesh
                # This depends on the model's output format
                # For now, create a dummy mesh
                mesh = self._dummy_mesh()
            else:
                # Fallback to a simple mesh for testing
                mesh = self._dummy_mesh()
            
            # Save to output directory
            if output_dir is None:
                output_dir = Path(tempfile.gettempdir()) / "pepakura_hunyuan"
            output_dir = Path(output_dir)
            output_dir.mkdir(parents=True, exist_ok=True)
            
            mesh_filename = f"hunyuan_mesh_{abs(hash(prompt)) % 10**8}.{output_format}"
            mesh_path = output_dir / mesh_filename
            
            # Export in desired format
            mesh.export(str(mesh_path))
            
            # Cache the result (original obj)
            if use_cache:
                cache_obj_path = output_dir / f"hunyuan_mesh_{prompt_hash}.obj"
                mesh.export(str(cache_obj_path))
                save_to_cache(prompt_hash, quality, cache_obj_path, {
                    "vertex_count": len(mesh.vertices),
                    "face_count": len(mesh.faces)
                })
            
            result.update({
                "success": True,
                "mesh_path": str(mesh_path.resolve()),
                "vertices": len(mesh.vertices),
                "faces": len(mesh.faces),
                "bounds": {
                    "min": mesh.bounds[0].tolist(),
                    "max": mesh.bounds[1].tolist()
                },
                "cached": False
            })
            
        except Exception as e:
            result["error"] = str(e)
        
        return result
    
    def _dummy_mesh(self):
        """Generate a simple cube mesh for testing when model is not available."""
        return trimesh.creation.box(extents=(1.0, 1.0, 1.0))
    
    def unload_model(self):
        """Unload model to free memory."""
        self.model = None
        if torch.cuda.is_available():
            torch.cuda.empty_cache()


# CLI interface for standalone usage
if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Hunyuan3D-2 Text-to-3D Generator")
    parser.add_argument("--prompt", type=str, required=True, help="Text prompt")
    parser.add_argument("--output-dir", type=str, default=None, help="Output directory")
    parser.add_argument("--quality", type=str, default="medium", choices=["low", "medium", "high"])
    parser.add_argument("--format", type=str, default="obj", choices=["obj", "stl", "glb"])
    parser.add_argument("--no-cache", action="store_true", help="Disable cache")
    
    args = parser.parse_args()

    generator = HunyuanGenerator()
    result = generator.generate_from_text(
        prompt=args.prompt,
        quality=args.quality,
        output_format=args.format,
        use_cache=not args.no_cache,
        output_dir=args.output_dir
    )

    print(json.dumps(result, indent=2))