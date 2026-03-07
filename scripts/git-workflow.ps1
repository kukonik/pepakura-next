# Мини-обертка над git add / commit / push
# Использование: .\scripts\git-workflow.ps1 "Сообщение коммита"

param(
    [Parameter(Mandatory=$true)]
    [string]$CommitMessage
)

Set-Location -Path "$PSScriptRoot\.."
Write-Host "Выполнение Git workflow..." -ForegroundColor Green

# Добавление всех изменений
Write-Host "Добавление всех изменений..." -ForegroundColor Yellow
git add .

# Коммит с переданным сообщением
Write-Host "Создание коммита: $CommitMessage" -ForegroundColor Yellow
git commit -m $CommitMessage

# Пуш изменений
Write-Host "Отправка изменений на удаленный репозиторий..." -ForegroundColor Yellow
git push

Write-Host "Git workflow завершен!" -ForegroundColor Green