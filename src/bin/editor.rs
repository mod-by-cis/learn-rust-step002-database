/*
// src/bin/editor.rs

use inquire::{Select, Text};
use std::error::Error;
use std::path::PathBuf;

use my_libs::config::ConfigManager;
use my_libs::consts::msg;
use my_libs::db::cmd::create::Creator;
use my_libs::db::connect::DatabaseConnection;
use my_libs::db::read::Reader;
use my_libs::utils::wait_for_enter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 🌲 Ustawiamy tytuł okna
    print!("\x1b]0;Arboretum - EDITOR\x07");
    // Ustawiamy tytuł okna (żebyś wiedział, że to Edytor)
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(["/c", "title", "EDYTOR (Okno 2)"])
        .status();

    // Wywołujemy "prawdziwą" logikę i sprawdzamy wynik
    match run_editor_setup().await {
        Ok(_) => {
            // Program zakończył się poprawnie (przez opcję WYJŚCIE)
            println!("👋 Do zobaczenia!");
            // Opcjonalnie: wait_for_enter(); // jeśli chcesz czekać też po sukcesie
        }
        Err(e) => {
            // 🛑 WYSTĄPIŁ BŁĄD! (Dlatego okno się zamykało)
            eprintln!("\n❌❌❌ KRYTYCZNY BŁĄD ❌❌❌");
            eprintln!("Powód: {}", e);
            eprintln!("-----------------------------");

            // TU JEST KLUCZ: Czekamy, żebyś zdążył przeczytać
            wait_for_enter();
        }
    }
    Ok(())
}

// ==========================================
// 🔌 WARSTWA 2: SETUP (Magazynier)
// Odpowiada za: Config, Ścieżki, Połączenie z DB, Stworzenie narzędzi (Reader/Creator)
// ==========================================
async fn run_editor_setup() -> Result<(), Box<dyn Error>> {
    println!("{}", msg::EDITOR_TITLE);

    // 1. Ładujemy Config
    let config = ConfigManager::load_global_config();
    let data_path_str = config
        .current_data_path
        .ok_or("Brak folderu data w configu!")?;
    let data_path = PathBuf::from(data_path_str);

    let active_db = ConfigManager::get_active_db(&data_path)
        .ok_or("❌ Nie wybrano aktywnej bazy! Uruchom najpierw 'manager'.")?;

    println!("📂 Folder danych: {:?}", data_path);
    println!("🗃️ Pracujemy na bazie: '{}'", active_db);

    // 2. Łączymy z Bazą
    let full_db_path = data_path.join(&active_db);
    let connection = DatabaseConnection::init(full_db_path.to_str().unwrap()).await?;

    // 3. Tworzymy Narzędzia (CQRS)
    // To tutaj powołujemy do życia obiekty, które potrafią czytać i pisać
    let reader = Reader::new(&connection);
    let creator = Creator::new(&connection);

    // 4. Przekazujemy gotowe narzędzia do Pętli (Warstwa 3)
    // Zauważ: Pętla nie musi się łączyć z bazą. Ona dostaje "gotowca".
    run_editor_loop(&reader, &creator).await
}

/// 🧠 Prawdziwa logika programu (wydzielona, żeby złapać błędy)
/* 
async fn run_editor() -> Result<(), Box<dyn Error>> {
    println!("{}", msg::EDITOR_TITLE);

    // 1. Odczytujemy, którą bazę wybrał Manager
    let config = ConfigManager::load_global_config();
    let data_path_str = config
        .current_data_path
        .ok_or("Brak folderu data w configu!")?;
    let data_path = PathBuf::from(data_path_str);

    let active_db = ConfigManager::get_active_db(&data_path)
        .ok_or("❌ Nie wybrano aktywnej bazy! Uruchom najpierw 'manager'.")?;

    println!("📂 Folder danych: {:?}", data_path);
    println!("🗃️ Pracujemy na bazie: '{}'", active_db);

    // 2. Łączymy się z bazą (Embedded)
    let full_db_path = data_path.join(&active_db);
    let connection = DatabaseConnection::init(full_db_path.to_str().unwrap()).await?;

    // 3. Inicjujemy narzędzia (CQRS)
    let reader = Reader::new(&connection);
    let creator = Creator::new(&connection);

    // D. Start pętli
    run_editor_loop(&reader, &creator).await
}

*/
// ==========================================
// 🔄 WARSTWA 3: LOOP (Pracownik przy taśmie)
// Odpowiada za: Gadanie z użytkownikiem (Menu), Używanie narzędzi.
// Nie wie: Gdzie jest baza, jak się nazywa plik.
// ==========================================
/// 🔄 Czysta pętla interakcji.
/// Nie martwi się o config ani połączenie - dostaje gotowe narzędzia.
async fn run_editor_loop(reader: &Reader<'_>, creator: &Creator<'_>) -> Result<(), Box<dyn Error>> {
    // To dzięki temu okno się nie zamyka!
    loop {
        println!("\n--------------------------------");

        // Definiujemy opcje menu
        let options = vec![
            msg::EDITOR_MENU_LIST,
            msg::EDITOR_MENU_ADD,
            msg::EDITOR_MENU_EXIT,
        ];

        // Czekamy na wybór użytkownika (Program tu PAUZUJE)
        let choice = Select::new(msg::ASK_ACTION, options).prompt();

        match choice {
            Ok(action) => {
                match action {
                    // ❌ WYJŚCIE
                    val if val == msg::EDITOR_MENU_EXIT => {
                        println!("👋 Zamykanie Edytora...");
                        break; // To przerywa pętlę i kończy program
                    }

                    // 📝 DODAWANIE (Interaktywne)
                    val if val == msg::EDITOR_MENU_ADD => {
                        // Pytamy o dane wewnątrz pętli
                        let imie = Text::new("Podaj imię kota:").prompt().unwrap_or_default();
                        let kolor = Text::new("Podaj kolor:").prompt().unwrap_or_default();

                        if !imie.is_empty() {
                            let new_id = creator.add_cat(&imie, &kolor).await?;
                            println!("{} {}", msg::SUCCESS_REC_ADDED, new_id);
                        }
                    }

                    // 📊 ODCZYT
                    val if val == msg::EDITOR_MENU_LIST => {
                        let koty = reader.get_all_cats().await?;
                        println!("{} {}", msg::INFO_DB_COUNT, koty.len());
                        for k in koty {
                            println!(" - 🐈 {} ({})", k.imie, k.kolor);
                        }
                    }

                    _ => {}
                }
            }
            Err(_) => {
                // Jeśli user wciśnie Ctrl+C lub Esc
                println!("Anulowano akcję.");
                break;
            }
        }
    }

    Ok(())
}


*/