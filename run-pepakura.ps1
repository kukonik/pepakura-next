$shortcut = "C:\Users\kukon\OneDrive\Desktop\Pepakura Next Desktop.lnk"

if (Test-Path $shortcut) {
    Start-Process $shortcut
} else {
    Write-Host "Ярлык не найден: $shortcut"
}
