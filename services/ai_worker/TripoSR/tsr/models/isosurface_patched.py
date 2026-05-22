# Patched isosurface.py - using PyMCubes instead of torchmcubes
import torch
import numpy as np

try:
    import mcubes
    HAS_MCUBES = True
except ImportError:
    HAS_MCUBES = False
    from skimage.measure import marching_cubes as skimage_mcubes

def marching_cubes(volume, threshold=0.5):
    """Extract mesh from volumetric data"""
    volume_np = volume.detach().cpu().numpy() if torch.is_tensor(volume) else volume
    
    if HAS_MCUBES:
        v, f = mcubes.marching_cubes(volume_np, threshold)
    else:
        v, f, _, _ = skimage_mcubes(volume_np, level=threshold)
    
    return torch.from_numpy(v.astype(np.float32)), torch.from_numpy(f.astype(np.int64))
