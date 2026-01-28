// src/utils.rs

use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

// 📚 EDU (Generics):
// <T> - To oznacza "Dla dowolnego typu T" (jak w TypeScript).
// W Rust `where` pozwala na dodatkowe ograniczenia typów.
//  - Serialize = Typ T musi umieć się zamienić na tekst.
//  - DeserializeOwned = Typ T musi umieć się stworzyć z tekstu
//  - Default = Typ T musi mieć domyślną wartość (np. pusty obiekt).

/// 📥 Generyczna funkcja do ładowania dowolnego pliku TOML.
/// Odpowiednik TS: function loadToml<T>(path: string): T
pub fn load_toml<T>(path: &Path) -> T
where
    T: DeserializeOwned + Default,
{
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        T::default()
    }
}

/// 💾 Generyczna funkcja do zapisywania dowolnego obiektu do TOML.
/// Automatycznie tworzy foldery, jeśli ich brakuje!
pub fn save_toml<T>(path: &Path, data: &T)
where
    T: Serialize,
{
    // 1. Jeśli plik ma być w folderze (np. config/x.toml), upewnij się, że folder istnieje.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Nie udało się utworzyć folderu");
    }

    // 2. Serializacja i zapis
    let content = toml::to_string_pretty(data).expect("Błąd formatowania TOML");
    fs::write(path, content).expect("Błąd zapisu pliku");
}

/// 🚀 Uruchamia nową instancję programu (binarkę) w nowym oknie terminala.
/// SRP: Ta funkcja martwi się o to JAK otworzyć okno, a nie KIEDY.
/*pub fn open_terminal_window(bin_name: &str) {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args([
                "/C",    // Zamknij proces cmd uruchamiający, ale zostaw okno
                "start", // Polecenie Windows "start" otwiera nowe okno
                "cargo", "run", "--bin", bin_name,
            ])
            .spawn()
            .expect("❌ Fail Fast: Nie udało się uruchomić procesu potomnego!")
            .wait() // 👈 DODANO: Czekamy aż 'cmd' skończy odpalać okno (trwa to milisekundy)
            .expect("Błąd oczekiwania na proces");
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!(
            "⚠️ Fail Fast: Twój system nie wspiera automatycznego otwierania okien w tym kodzie."
        );
    }
}
*/
pub fn open_terminal_window(bin_name: &str) {
    #[cfg(target_os = "windows")]
    {
        // Budujemy komendę PowerShell.
        // 1. "pwsh" -> Uruchamiamy proces-matkę.
        // 2. "-Command" -> Każe mu wykonać polecenie Start-Process.
        // 3. "Start-Process pwsh" -> Otwiera NOWE okno z PowerShellem.
        // 4. "-ArgumentList" -> Przekazuje parametry do tego nowego okna.
        // 5. "-NoExit" -> KLUCZOWE: Nie zamykaj okna po wykonaniu (odpowiednik cmd /K).

        let ps_command = format!(
            //"Start-Process pwsh -ArgumentList '-NoExit', '-Command', 'cargo run --release --bin {}; if ($LASTEXITCODE -eq 0) {{ exit }}'",
            //"Start-Process pwsh -ArgumentList '-NoExit', '-Command', 'cargo run --release --bin {}'",
            "Start-Process pwsh -ArgumentList '-Command', 'cargo run --release --bin {}'",
            bin_name
        );

        Command::new("pwsh")
            .arg("-Command")
            .arg(ps_command)
            .spawn()
            .expect("❌ Błąd: Nie znaleziono 'pwsh'. Upewnij się, że PowerShell Core jest w PATH.")
            .wait()
            .expect("Błąd oczekiwania na proces startowy");
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("⚠️ System nie wspiera automatycznego otwierania okien.");
    }
}

/// 🛑 Zatrzymuje program i czeka na wciśnięcie Enter.
/// Przydatne, żeby zobaczyć błędy przed zamknięciem okna.
pub fn wait_for_enter() {
    print!("\n🔴 Naciśnij ENTER, aby zamknąć okno...");
    // flush() wymaga zaimportowanego traita 'Write' (linia 4)
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap_or_default();
}

/// Zabija proces o podanej nazwie (np. "editor")
pub fn kill_process(bin_name: &str) {
    #[cfg(target_os = "windows")]
    {
        let process_name = format!("{}.exe", bin_name);

        // Uruchamiamy taskkill
        // /F - Force (wymuś zamknięcie)
        // /IM - Image Name (nazwa pliku obrazu)
        // /T - Tree (zabij też procesy potomne, jeśli jakieś stworzył)
        let _ = Command::new("taskkill")
            .args(["/F", "/IM", &process_name, "/T"])
            .output(); // .output() czeka na wykonanie, ale ignorujemy wynik (czy się udało czy nie)

        println!("💀 Wysłano sygnał zamknięcia dla {}", process_name);
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("⚠️ Zabijanie procesów zaimplementowane tylko dla Windows.");
    }
}
