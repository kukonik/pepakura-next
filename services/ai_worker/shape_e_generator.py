"""
Shape-E Integration for Pepakura Next
Text-to-3D generation using OpenAI's Shap-E
"""
import torch
import shap_e
from shap_e.models import config, create_model_from_pretrained
from shap_e.diffusion.sample import sample_latents
from shap_e.util.notebooks import decode_latent_mesh
import tempfile
import os
from pathlib import Path
import trimesh

class ShapeEGenerator:
    def __init__(self, device: str = None):
        self.device = device or ("cuda" if torch.cuda.is_available() else "cpu")
        print(f"Loading Shape-E model on {self.device}...", file=sys.stderr)
        self.xm, self.model = create_model_from_pretrained("text300M", device=self.device)
        print("Model loaded successfully!", file=sys.stderr)

    def generate(
        self,
        prompt: str,
        quality: str = "medium",
        output_format: str = "obj"
    ) -> dict:
        """
        Generate 3D model from text prompt

        Args:
            prompt: Text description
            quality: "low" (64 steps), "medium" (128), "high" (256)
            output_format: "obj", "stl", "glb"

        Returns:
            dict with model_path, vertices, faces, preview_path
        """
        steps = {"low": 64, "medium": 128, "high": 256}.get(quality, 128)

        print(f"Generating 3D model for: '{prompt}'", file=sys.stderr)
        print(f"Quality: {quality} ({steps} steps)", file=sys.stderr)
        
        # Sample from diffusion model
        latents = sample_latents(
            batch_size=1,
            model=self.model,
            diffusion=self.model.diffusion,
            guidance_scale=15.0,
            model_kwargs=dict(texts=[prompt]),
            steps=steps,
            clip_denoised=True,
            use_model=True,
            use_fp16=self.device == "cuda",
        )
        
        # Decode latent to mesh
        latent = latents[0]
        t = decode_latent_mesh(self.xm, latent).tri_mesh()
        
        # Create trimesh object
        mesh = trimesh.Trimesh(
            vertices=t.verts,
            faces=t.faces,
            process=True
        )
        
        # Save to temporary file
        output_dir = Path(tempfile.gettempdir()) / "pepakura_ai"
        output_dir.mkdir(exist_ok=True)
        
        model_id = abs(hash(prompt)) % 10**8
        output_path = output_dir / f"model_{model_id}.{output_format}"
        
        # Export mesh
        if output_format == "obj":
            mesh.export(str(output_path))
        elif output_format == "stl":
            mesh.export(str(output_path))
        elif output_format == "glb":
            mesh.export(str(output_path))
        
        # Generate preview image
        preview_path = self._generate_preview(mesh, output_dir, model_id)
        
        return {
            "success": True,
            "model_path": str(output_path),
            "vertices": len(mesh.vertices),
            "faces": len(mesh.faces),
            "preview_path": str(preview_path),
            "bounds": {
                "min": mesh.bounds[0].tolist(),
                "max": mesh.bounds[1].tolist()
            }
        }
    
    def _generate_preview(self, mesh, output_dir, model_id):
        """Generate preview image of the mesh"""
        import matplotlib.pyplot as plt
        from mpl_toolkits.mplot3d import Axes3D
        
        fig = plt.figure(figsize=(8, 8))
        ax = fig.add_subplot(111, projection='3d')
        
        # Plot mesh
        ax.plot_trisurf(
            mesh.vertices[:, 0],
            mesh.vertices[:, 1],
            mesh.vertices[:, 2],
            triangles=mesh.faces,
            cmap='viridis',
            edgecolor='none'
        )
        
        ax.axis('off')
        ax.view_init(elev=30, azim=45)
        
        preview_path = output_dir / f"preview_{model_id}.png"
        plt.savefig(preview_path, dpi=150, bbox_inches='tight', facecolor='white')
        plt.close()
        
        return preview_path

# CLI interface
if __name__ == "__main__":
    import argparse
    import json
    
    parser = argparse.ArgumentParser(description="Shape-E 3D Generator")
    parser.add_argument("--prompt", type=str, required=True, help="Text prompt")
    parser.add_argument("--quality", type=str, default="medium", choices=["low", "medium", "high"])
    parser.add_argument("--format", type=str, default="obj", choices=["obj", "stl", "glb"])
    
    args = parser.parse_args()
    
    generator = ShapeEGenerator()
    result = generator.generate(args.prompt, args.quality, args.format)
    
    print(json.dumps(result, indent=2))
