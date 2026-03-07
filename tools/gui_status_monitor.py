import sys
import requests
from PyQt5.QtWidgets import QApplication, QWidget, QLabel, QVBoxLayout, QPushButton, QProgressBar
from PyQt5.QtCore import QTimer, Qt
import subprocess
import threading
import os

class ServerMonitor(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Pepakura Stack Monitor")
        self.resize(400, 200)
        layout = QVBoxLayout()

        self.label_fe = QLabel("Frontend: Проверка...", self)
        self.label_be = QLabel("Backend: Проверка...", self)
        self.progress = QProgressBar(self)
        self.restart_btn = QPushButton("Перезапустить всё", self)
        self.restart_btn.clicked.connect(self.restart_all)

        layout.addWidget(self.label_fe)
        layout.addWidget(self.label_be)
        layout.addWidget(self.progress)
        layout.addWidget(self.restart_btn)

        self.setLayout(layout)

        self.timer = QTimer()
        self.timer.timeout.connect(self.check_status)
        self.timer.start(3000)  # каждые 3 секунды

        self.check_status()

    def check_status(self):
        fe_ok = self.is_server_up("http://localhost:5173")
        be_ok = self.is_server_up("http://localhost:8000")

        self.label_fe.setText(f"Frontend: {'✅ ОК' if fe_ok else '❌ Недоступен'}")
        self.label_be.setText(f"Backend: {'✅ ОК' if be_ok else '❌ Недоступен'}")

        status = sum([fe_ok, be_ok])
        self.progress.setValue(int((status / 2.0) * 100))

    def is_server_up(self, url):
        try:
            resp = requests.get(url, timeout=2)
            return resp.status_code == 200
        except Exception:
            return False

    def restart_all(self):
        self.label_fe.setText("Запуск... Ждите")
        self.label_be.setText("Запуск... Ждите")
        thread = threading.Thread(target=self._restart_processes)
        thread.daemon = True
        thread.start()

    def _restart_processes(self):
        # Kill existing processes
        subprocess.run(["taskkill", "/F", "/IM", "node.exe"], capture_output=True)
        subprocess.run(["taskkill", "/F", "/IM", "python.exe"], capture_output=True)

        # Restart frontend
        fe_thread = threading.Thread(target=lambda: os.system('cd .. && pnpm dev'), shell=True)
        fe_thread.daemon = True
        fe_thread.start()

        # Restart backend
        be_thread = threading.Thread(target=lambda: os.system('cd ../services/backend-python && .\\venv\\Scripts\\activate && uvicorn app.main:app --reload'), shell=True)
        be_thread.daemon = True
        be_thread.start()


if __name__ == '__main__':
    app = QApplication(sys.argv)
    monitor = ServerMonitor()
    monitor.show()
    sys.exit(app.exec_())
