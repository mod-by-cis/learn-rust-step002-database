# 🛠️ Skrypty pomocnicze (Scripts / Tooling)

W tym folderze znajdują się narzędzia wspomagające rozwój projektu `🦀☯️🦕`.

## 📜 Zasady uruchamiania

System Windows domyślnie blokuje uruchamianie skryptów PowerShell pobranych z sieci lub niepodpisanych. Aby móc uruchamiać skrypty w tym folderze bez zmiany globalnych ustawień systemu, używaj poniższej komendy w terminalu:

```powershell
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process
```

*Działa to tylko dla bieżącej sesji terminala (procesu).*

---