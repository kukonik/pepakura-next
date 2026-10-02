from fastapi import FastAPI, File, UploadFile
from fastapi.middleware.cors import CORSMiddleware
from app.api import ai_routes
from app.control_api import router as control_router

app = FastAPI(title="Pepakura Next AI Backend")

# CORS (чтобы фронтенд мог стучаться)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(ai_routes.router, prefix="/api/ai", tags=["AI"])
app.include_router(control_router, prefix="/api/control", tags=["Control"])

@app.get("/")
def root():
    return {"status": "Pepakura Next AI Service Running"}
