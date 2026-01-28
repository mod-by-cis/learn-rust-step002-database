# 🌲 [Arboretum](https://github.com/mod-by-cis/learn-rust-step002-database/) - Genealogy System [![Verify CLI Branch (Manual)](https://github.com/mod-by-cis/learn-rust-step002-database/actions/workflows/manual_verify_cli.yml/badge.svg?branch=main-cli)](https://github.com/mod-by-cis/learn-rust-step002-database/actions/workflows/manual_verify_cli.yml)

Nowoczesny system do zarządzania danymi genealogicznymi, napisany w języku **Rust** z wykorzystaniem wbudowanej bazy **SurrealDB**.

Projekt wyróżnia się unikalnym podejściem do atomizacji danych ("Mastykony") oraz bezwzględnym priorytetem źródeł historycznych.

Projekt wykorzystuje architekturę wieloprocesową CLI:

1. **Manager**: Zarządza plikami baz danych (tworzenie, wybór).
2. **Editor**: Osobne okno procesu do edycji konkretnych danych, uruchamiane dynamicznie przez Managera.

## 🛠️ Tech Stack & 🌍 Target Platforms 

### 🛠️ Tech Stack (OBECNE)

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg?logo=rust&logoColor=white)
![Database](https://img.shields.io/badge/database-SurrealDB_Embedded-ff00a0.svg)

### 🛠️ Tech Stack (W PRZYSZŁOŚCI)

![Tauri](https://img.shields.io/badge/GUI-Tauri_v2-24C8DB.svg?logo=tauri&logoColor=white)

### 🌍 Target Platforms (OBECNE)

![Windows](https://img.shields.io/badge/Windows-0078D6?style=flat&logo=windows&logoColor=white)

### 🌍 Target Platforms (W PRZYSZŁOŚCI)

![macOS](https://img.shields.io/badge/macOS-000000?style=flat&logo=apple&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat&logo=linux&logoColor=black)
![Android](https://img.shields.io/badge/Android-3DDC84?style=flat&logo=android&logoColor=white)

---

## 🏗️ Status Budowania (CI/CD)

Automatyczna weryfikacja kodu i budowanie wersji Release odbywa się tutaj:

👉 **[Zobacz Status GitHub Actions](https://github.com/mod-by-cis/learn-rust-step002-database/actions)**

---

## 👁️ (PLANY) Wizja i Filozofia Projektu (nie wdrożone)

Arboretum to nie tylko baza danych – to system oparty na trzech fundamentalnych filarach, które odwracają tradycyjne podejście do genealogii:

### 1. 📜 Źródło Ponad Wszystko (Source First)

Większość programów stawia w centrum "Osobę". W Arboretum **królem jest Źródło**.

- Nie tworzymy "Jana Kowalskiego" z powietrza.
- Najpierw wprowadzamy **dokument** (metryka, akt notarialny, zdjęcie nagrobka).
- Dopiero z dokumentu "ekstrahujemy" fakty i osoby.
- Dzięki temu każda informacja w systemie ma swoje pokrycie w rzeczywistości.

### 2. ⚛️ Atomizacja Danych

Zrywamy z monolitycznymi rekordami. Informacja nie jest "cechą osoby", lecz niezależnym bytem.

- Imię to osobny atom. Data urodzenia to osobny atom.
- Pozwala to na przechowywanie **wersji alternatywnych** i sprzecznych (np. trzy różne daty urodzenia z trzech różnych dokumentów) bez sztucznego uśredniania prawdy.

### 3. 💠 Mastykony

To nasza autorska nazwa na najmniejsze, niepodzielne cząstki informacji genealogicznej.

- **Mastykon** to kontener na pojedynczy fakt, nierozerwalnie złączony ze swoim źródłem.
- Drzewo genealogiczne w Arboretum to w rzeczywistości sieć powiązanych Mastykonów, które dopiero w warstwie wizualnej składają się w sylwetki przodków.

---

## 🚀 Jak uruchomić?

### Wymagania wstępne

- **System:** Windows 10/11
- **Terminal:** [PowerShell Core 7.x](https://github.com/PowerShell/PowerShell) (komenda `pwsh`)
- **Rust:** Zainstalowany toolchain (`cargo`).

### 1. Budowanie (Kompilacja)

Pierwsze budowanie może potrwać kilka minut, ponieważ kompilowany jest silnik SurrealDB.

```powershell
# Pobierz zależności i zbuduj wersję Release
cargo build --release

```

### 2. Uruchamianie

Projekt korzysta ze specjalnego launchera, który ustawia środowisko PowerShell Core.

**Kliknij dwukrotnie plik:**
`START.bat`

---

## ⚙️ Architektura Techniczna

System składa się z dwóch binarek (`src/bin/`):

### 1. `manager.exe`

- Główne okno startowe ("Arboretum - MANAGER").
- Pozwala tworzyć nowe bazy danych (foldery w `./data`).
- **Funkcja specjalna:** Uruchamia Edytora w nowym, niezależnym oknie terminala.
- **Zarządzanie:** Potrafi zdalnie zamknąć otwarte okno Edytora (`taskkill`).

### 2. `editor.exe`

- Okno potomne ("Arboretum - EDITOR").
- Łączy się z wybraną bazą danych (SurrealDB Embedded).
- To tutaj odbywa się praca na **Mastykonach** (dodawanie faktów i źródeł).

---

## 📝 Plany na przyszłość

Obecnie prace trwają na gałęzi `main-cli`, skupiając się na logice backendowej w Rust.

- [ ] **Pełna implementacja Mastykonów** w strukturze grafowej SurrealDB.
- [ ] **Interfejs Graficzny (GUI):** Docelowo CLI zostanie zastąpione przez nowoczesną aplikację opartą na **Tauri**.
- Wizualizacja grafu połączeń między faktami a źródłami.
- Przeciąganie dokumentów (Drag & Drop) w celu ekstrakcji danych.

- [ ] Eksport danych do formatów naukowych i GEDCOM.
