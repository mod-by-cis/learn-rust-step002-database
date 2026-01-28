// src/bin/manager.rs (Dawniej dev_db_manager)

use inquire::{Confirm, Select, Text};
use std::fs;
use std::path::PathBuf;

use my_libs::config::ConfigManager;
use my_libs::consts::DATA_FOLDER;
use my_libs::consts::msg;
use my_libs::utils::{kill_process, open_terminal_window};

#[tokio::main]
async fn main() {
    // 🌲 Ustawiamy tytuł okna (ANSI Escape Code)
    print!("\x1b]0;Arboretum - MANAGER\\x07");
    println!("{}", msg::MANAGER_TITLE);

    // --- KROK 1: Ładowanie Configu ---
    let mut config = ConfigManager::load_global_config();
    // DRY: Używamy stałej DATA_FOLDER zamiast wpisywać "data"
    // Pobieramy zapisaną ścieżkę lub domyślną "data"
    let default_path = config
        .current_data_path
        .clone()
        .unwrap_or_else(|| DATA_FOLDER.to_string());

    // Pytamy usera (Biblioteka Inquire)
    let data_path_str = Text::new(msg::ASK_DATA_DIR)
        .with_default(&default_path)
        .prompt()
        .unwrap(); // unwrap() = "Zakładam, że user coś wpisał, jak nie to crash"

    let data_path = PathBuf::from(&data_path_str);

    // Walidacja nazwy folderu
    if !data_path.ends_with("data")
        && !data_path_str.ends_with("data/")
        && !data_path_str.ends_with("data\\")
    {
        println!("{}", msg::ERR_DIR_NAME);
        // return; // Odkomentuj jeśli chcesz być restrykcyjny
    }

    // Tworzenie folderu data jeśli brak
    if !data_path.exists() {
        let create = Confirm::new(msg::ASK_CREATE_DIR)
            .with_default(true)
            .prompt()
            .unwrap();

        if create {
            fs::create_dir_all(&data_path).expect(msg::ERR_DIR_CREATE);
            println!("{} {:?}", msg::OK_DIR_CREATED, data_path);
        } else {
            // Jeśli użytkownik nie chce tworzyć, kończymy program
            println!("{}", msg::CANCELLED);
            return;
        }
    }

    // Zapisujemy wybór do configu globalnego
    config.current_data_path = Some(data_path_str.clone());
    ConfigManager::save_global_config(&config);

    // --- KROK 2: Pętla Menu Głównego ---
    loop {
        // Ładujemy listę baz (automatycznie czyści śmieci)
        let index = ConfigManager::load_and_clean_data_index(&data_path);
        let mut db_names: Vec<String> = index.databases.keys().cloned().collect();
        db_names.sort();

        // Budujemy menu
        let mut menu_items = vec![];
        menu_items.push(msg::MENU_NEW_DB.to_string());
        for name in db_names {
            menu_items.push(format!("📂 {}", name));
        }
        let kill_option = "💀 ZAMKNIJ OTWARTEGO EDYTORA".to_string();
        menu_items.push(kill_option.clone());
        menu_items.push(msg::MENU_EXIT.to_string());

        let selection = Select::new(msg::ASK_SELECT_ACTION, menu_items)
            .with_page_size(10)
            .prompt()
            .unwrap();

        match selection.as_str() {
            val if val == msg::MENU_EXIT => break, // Wychodzimy z pętli loop -> koniec programu

            val if val == kill_option => {
                kill_process("editor");
                // Mała pauza, żebyś zdążył zobaczyć komunikat w konsoli
                std::thread::sleep(std::time::Duration::from_millis(1500));
            }

            val if val == msg::MENU_NEW_DB => {
                let new_name = Text::new(msg::ASK_DB_NAME).prompt().unwrap();
                if !new_name.is_empty() {
                    // Wywołujemy naszą funkcję z ConfigManagera, która robi init bazy
                    match ConfigManager::create_new_db(&data_path, &new_name).await {
                        Ok(_) => {
                            // Ustawiamy jako aktywną
                            ConfigManager::set_active_db(&data_path, &new_name);
                            println!("{} {}", msg::OK_DB_CREATED, new_name);
                        }
                        Err(e) => println!("{} {}", msg::ERR_DB_CREATE, e),
                    }
                }
            }

            val => {
                // Wybrano istniejącą bazę. Wyciągamy nazwę po "📂 "
                let db_name = val.trim_start_matches("📂 ");
                ConfigManager::set_active_db(&data_path, db_name);
                println!("{} {}", msg::OK_DB_SET_ACTIVE, db_name);
                println!("{}", msg::INFO_EDITOR_LAUNCHING);
                open_terminal_window("editor");
            }
        }
    }
}
