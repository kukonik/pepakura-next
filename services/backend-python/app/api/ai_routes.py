from fastapi import APIRouter, UploadFile, HTTPException, Response
from pydantic import BaseModel
import shutil
import os
from ...utils.prompt_parser import parse_prompt_to_dict
from ...utils.mesh_generator import shape_generators
from ...exporters.obj_to_glb import convert_obj_string_to_glb
from ...exporters.obj_to_fbx import convert_obj_string_to_fbx
from ...exporters.obj_to_stl import convert_obj_string_to_stl
from ...unfolders.svg_unfolder import unfold_mesh_to_svg
from ...exporters.project_zipper import generate_project_package
from ...exporters.instruction_pdf_generator import generate_build_instructions_pdf

router = APIRouter()

class GenerateRequest(BaseModel):
    prompt: str

class MeshAnalysis(BaseModel):
    url: str

@router.post("/generate_model")
async def generate_model(request: GenerateRequest):
    spec = parse_prompt_to_dict(request.prompt)

    if not spec.get('type'):
        raise HTTPException(status_code=400, detail='Не удалось распознать форму')

    generator_fn = shape_generators.get(spec['type'])
    if not generator_fn:
        raise HTTPException(status_code=400, detail=f'Форма "{spec["type"]}" не поддерживается')

    model_data = generator_fn(spec.get("dimensions"), spec.get("color"))

    return {
        "model": model_data,
        "format": ".obj",
        "attributes": spec
    }


@router.post("/export_model/{target_format}")
async def export_model(target_format: str, request: GenerateRequest):
    if target_format not in ['glb', 'fbx', 'stl']:
        raise HTTPException(status_code=400, detail="Only GLB, FBX, and STL supported currently.")

    spec = parse_prompt_to_dict(request.prompt)
    generator_fn = shape_generators.get(spec['type'])
    model_obj_data = generator_fn(spec.get("dimensions"), spec.get("color"))
    
    if target_format == 'glb':
        model_data = convert_obj_string_to_glb(model_obj_data)
    elif target_format == 'fbx':
        model_data = convert_obj_string_to_fbx(model_obj_data)
    elif target_format == 'stl':
        model_data = convert_obj_string_to_stl(model_obj_data)

    headers = {'Content-Disposition': f'attachment; filename=model.{target_format}'}
    return Response(content=model_data, media_type='application/octet-stream', headers=headers)


@router.post("/unfold_svg")
async def unfold_to_svg(request: GenerateRequest):
    spec = parse_prompt_to_dict(request.prompt)
    generator_fn = shape_generators.get(spec['type'])
    obj_data = generator_fn(spec.get("dimensions"), spec.get("color"))
    svg_data = unfold_mesh_to_svg(obj_data)
    return Response(content=svg_data, media_type='image/svg+xml')


@router.post("/export_project_zip")
async def export_project_zip(request: GenerateRequest):
    spec = parse_prompt_to_dict(request.prompt)
    generator_fn = shape_generators.get(spec['type'])
    model_data = generator_fn(spec.get("dimensions"), spec.get("color"))

    zip_data = generate_project_package({"prompt": request.prompt}, model_data)

    headers = {
        'Content-Disposition': 'attachment; filename="project.zip"'
    }

    return Response(content=zip_data, media_type='application/zip', headers=headers)


@router.post("/export_build_instructions_pdf")
async def export_pdf_instructions(request: GenerateRequest):
    spec = parse_prompt_to_dict(request.prompt)
    generator_fn = shape_generators.get(spec['type'])
    model_data = generator_fn(spec.get("dimensions"), spec.get("color"))

    svg_data = unfold_mesh_to_svg(model_data)
    pdf_data = generate_build_instructions_pdf(
        model_title=request.prompt,
        model_description=request.prompt,
        svg_preview=svg_data
    )

    headers = {
        'Content-Disposition': 'inline; filename="build_guide.pdf"'
    }

    return Response(content=pdf_data, media_type='application/pdf', headers=headers)


@router.post("/analyze")
async def analyze_mesh(mesh: MeshAnalysis):
    # Заглушка: В будущем здесь будет Computer Vision анализ
    return {
        "status": "analyzed",
        "is_manifold": True,
        "issues": [],
        "suggestion": "Модель готова к развертке."
    }

@router.post("/unfold")
async def smart_unfold():
    # Заглушка: В будущем здесь будет ML модель
    return {
        "pieces_generated": 12,
        "estimated_sheets": 2
    }

@router.post("/optimize_packing")
async def optimize_packing():
    return {"optimization": "completed", "waste_percent": 5.2}
