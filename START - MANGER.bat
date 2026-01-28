@echo off
:: Uruchamia już zbudowany plik w PowerShell Core
echo 🌲 Uruchamianie Arboretum..

:: -NoExit = Okno nie zamyka się po zakończeniu/błędzie
:: & '...' = Operator wywołania w PowerShellu (potrzebny do uruchomienia pliku ze ścieżki)
start pwsh  -Command "$Host.UI.RawUI.WindowTitle = 'Arboretum - MANAGER'; & '.\target\release\manager.exe'"

exit