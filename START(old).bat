@echo off
:: To jest Launcher, który wymusza PowerShell Core 7
echo 🚀 Uruchamianie Systemu Genealogii w PowerShell Core...

:: Uruchom pwsh, nie zamykaj go po błędzie (-NoExit) i wykonaj cargo run
start pwsh -NoExit -Command "cargo run --release --bin manager"

:: Zamknij to stare okno cmd launchera
exit