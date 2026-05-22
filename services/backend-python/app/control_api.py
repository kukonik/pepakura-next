from fastapi import APIRouter, WebSocket, BackgroundTasks
from fastapi.responses import HTMLResponse
import subprocess
import asyncio
import platform

router = APIRouter(prefix="/control")

@router.websocket("/ws/status")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    try:
        while True:
            status_data = get_status()
            await websocket.send_json(status_data)
            await asyncio.sleep(2)  # Обновляем статус каждые 2 секунды
    except Exception as e:
        print(f"[WS Error] {e}")


def get_status():
    import psutil
    backend_running = any(proc.name() == "python.exe" for proc in psutil.process_iter())
    frontend_running = False  # TODO: можно проверять по порту 5173

    return {
        "backend": backend_running,
        "frontend": frontend_running,
        "cpu_usage": psutil.cpu_percent(interval=1),
        "mem_usage": psutil.virtual_memory().percent
    }


@router.post("/exec/{action}")
def execute_action(action: str):
    if action == "start_frontend":
        if platform.system() == "Windows":
            subprocess.Popen(['cmd', '/c', 'cd /d D:\\Dev\\pepakura-next && pnpm dev'], shell=True)
        else:
            subprocess.Popen(['sh', '-c', 'cd D:/Dev/pepakura-next && pnpm dev'])
        return {"message": "Frontend started"}

    elif action == "stop_frontend":
        subprocess.call(["taskkill", "/F", "/IM", "node.exe"])
        return {"message": "Frontend stopped"}

    elif action == "start_backend":
        subprocess.Popen([
            "python", "-m", "uvicorn", "app.main:app", "--reload"
        ])
        return {"message": "Backend started"}

    elif action == "stop_backend":
        subprocess.call(["taskkill", "/F", "/IM", "python.exe"])
        return {"message": "Backend stopped"}

    return {"error": "Action unknown"}
