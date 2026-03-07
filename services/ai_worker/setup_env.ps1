Write-Host "Setting up Pepakura Next AI Env..." -ForegroundColor Cyan
python -m venv venv
.\venv\Scripts\Activate.ps1
pip install --upgrade pip
pip install -r requirements.txt
Write-Host "Done!" -ForegroundColor Green
