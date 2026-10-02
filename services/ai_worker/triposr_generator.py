"""
TripoSR Integration for Pepakura Next
Image-to-3D generation using Stability AI's TripoSR
"""
import torch
import numpy as np
from PIL import Image
import trimesh
import json
import sys
import shutil
from pathlib import Path
from typing import Optional, Dict, Literal, Union
import tempfile

# Add TripoSR directory to path
CURRENT_DIR = Path(__file__).parent
TRIPOSR_DIR = CURRENT_DIR / "TripoSR"
if TRIPOSR_DIR.exists():
    sys.path.append(str(TRIPOSR_DIR))
else:
    raise ImportError(f"TripoSR directory not found at {TRIPOSR_DIR}")

try:
    from tsr.system import TSR
except ImportError as e:
    raise ImportError(f"Failed to import TripoSR: {e}. Check dependencies.")

try:
    from cache_manager import get_cached_mesh_path, save_to_cache
except ImportError:
    # Mock cache functions if cache_manager not available
    def get_cached_mesh_path(*args, **kwargs): return None
    def save_to_cache(*args, **kwargs): pass

Quality = Literal["fast", "balanced", "high"]
Format = Literal["obj", "glb", "stl", "ply"]

class TripoSRGenerator:
    """TripoSR image-to-3D generator."""
    
    def __init__(self, device: Optional[str] = None):
        """
        Initialize generator.
        
        Args:
            device: 'cuda' or 'cpu'. If None, auto-select.
        """
        self.device = device or ("cuda" if torch.cuda.is_available() else "cpu")
        self.model: Optional[TSR] = None
        self.quality_params = {
            "fast": {"image_size": 256, "chunk_size": 16384},
            "balanced": {"image_size": 384, "chunk_size": 8192},
            "high": {"image_size": 512, "chunk_size": 4096},
        }
    
    def load_model(self, quality: Quality = "balanced") -> None:
        """
        Load TripoSR model with given quality preset.

        Args:
            quality: "fast", "balanced", or "high"
        """
        if self.model is not None:
            return

        params = self.quality_params[quality]
        print(f"Loading TripoSR model on {self.device} with quality {quality}...", file=sys.stderr)
        self.model = TSR.from_pretrained(
            "stabilityai/TripoSR",
            config_name="config.yaml",
            weight_name="model.ckpt"
        )
        self.model.renderer.set_chunk_size(params["chunk_size"])
        self.model = self.model.to(self.device)
        self.model.eval()
        print("Model loaded successfully!", file=sys.stderr)
    
    def generate(
        self,
        image_path: Union[str, Path],
        output_format: Format = "obj",
        quality: Quality = "balanced",
        use_cache: bool = True,
        output_dir: Optional[Union[str, Path]] = None
    ) -> Dict:
        """
        Generate 3D mesh from a single image.
        
        Args:
            image_path: Path to input image (JPEG, PNG, etc.)
            output_format: "obj", "glb", "stl", "ply"
            quality: "fast", "balanced", "high"
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
        image_path = Path(image_path)
        if not image_path.exists():
            return {"success": False, "error": f"Image not found: {image_path}"}
        
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
            cached_path = get_cached_mesh_path(image_path, quality)
            if cached_path and cached_path.exists():
                # Copy to output location
                if output_dir is None:
                    output_dir = Path(tempfile.gettempdir()) / "pepakura_triposr"
                output_dir = Path(output_dir)
                output_dir.mkdir(parents=True, exist_ok=True)
                mesh_filename = f"triposr_mesh_{image_path.stem}.{output_format}"
                mesh_path = output_dir / mesh_filename
                shutil.copy2(cached_path, mesh_path)
                # Convert format if needed
                if output_format != "obj":
                    mesh = trimesh.load(str(cached_path))
                    mesh.export(str(mesh_path))
                
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
            # Load and preprocess image
            image_np = self._load_image(image_path, params["image_size"])
            
            # Generate scene codes
            scene_codes = self.model.from_images([image_np], device=self.device)
            
            # Extract mesh
            meshes = self.model.extract_mesh(scene_codes, has_vertex_color=False)
            if not meshes:
                raise RuntimeError("No meshes generated")
            mesh = meshes[0]  # trimesh.Trimesh
            
            # Save to output directory
            if output_dir is None:
                output_dir = Path(tempfile.gettempdir()) / "pepakura_triposr"
            output_dir = Path(output_dir)
            output_dir.mkdir(parents=True, exist_ok=True)
            
            mesh_filename = f"triposr_mesh_{image_path.stem}.{output_format}"
            mesh_path = output_dir / mesh_filename
            
            # Export in desired format
            mesh.export(str(mesh_path))
            
            # Cache the result (original obj)
            if use_cache:
                cache_obj_path = output_dir / f"triposr_mesh_{image_path.stem}.obj"
                mesh.export(str(cache_obj_path))
                save_to_cache(image_path, quality, cache_obj_path, {
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
    
    def _load_image(self, image_path: Path, size: int) -> np.ndarray:
        """Load and preprocess image."""
        img = Image.open(image_path).convert("RGB")
        img = img.resize((size, size), Image.BICUBIC)
        return np.array(img)
    
    def unload_model(self):
        """Unload model to free memory."""
        self.model = None
        if torch.cuda.is_available():
            torch.cuda.empty_cache()


# CLI interface for standalone usage
if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="TripoSR Image-to-3D Generator")
    parser.add_argument("--input", type=str, required=True, help="Path to input image")
    parser.add_argument("--output-dir", type=str, default=None, help="Output directory")
    parser.add_argument("--quality", type=str, default="balanced", choices=["fast", "balanced", "high"])
    parser.add_argument("--format", type=str, default="obj", choices=["obj", "glb", "stl", "ply"])
    parser.add_argument("--no-cache", action="store_true", help="Disable cache")
    
    args = parser.parse_args()
    
    generator = TripoSRGenerator()
    result = generator.generate(
        image_path=args.input,
        output_format=args.format,
        quality=args.quality,
        use_cache=not args.no_cache,
        output_dir=args.output_dir
    )
    
    print(json.dumps(result, indent=2))