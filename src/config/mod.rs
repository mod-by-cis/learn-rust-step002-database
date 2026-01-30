// src/config/mod.rs

// 📦 Zewnętrzne crate'y (odpowiednik importów z npm/deno.land)
use serde::{Deserialize, Serialize}; // Do zamiany Struct <-> TOML/JSON

// 📦 Biblioteka standardowa (std)
use std::collections::HashMap; // Jak Map<string, string> w TS
use std::fs; // System plików
// use std::path::{Path, PathBuf};
use std::path::Path;

use crate::consts::{
    // CONFIG_FILE, CONFIG_FOLDER, DATA_FOLDER, 
    DB_NAME, DB_NAMESPACE // 👈 UPEWNIJ SIĘ, ŻE TE SĄ
};

// 🧩 Moduły własne (nasze lokalne importy)
// ⚠️ Poprawka: DatabaseConnection jest teraz w db::connect, nie w db::Database
use crate::db::connect::DatabaseConnection;
use crate::utils::{load_toml, save_toml};

// --- STRUKTURY DANYCH  ---

// 📚 EDU (TypeScript):
// #[derive(...)] to makro. W TS musiałbyś ręcznie pisać funkcję toJson() i fromJson().
// Tutaj Rust generuje ten kod za Ciebie podczas kompilacji.

/// 🧱 1. GlobalConfig (./config/config.toml)
/// Przechowuje informację, gdzie aktualnie szukamy baz (folder data).
/// Odpowiednik TS: interface GlobalConfig { current_data_path: string | null }
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    pub current_data_path: Option<String>,
}

/// 🧱 2. DataIndex (data/data.toml)
/// Lista dostępnych baz w danym folderze.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataIndex {
    // HashMap<String, String> to w TS: Record<string, string>
    pub databases: HashMap<String, String>,
}

/// 🧱 3. CurrentState (data/data_current.toml)
/// Informacja, która baza jest teraz "otwarta" przez edytor.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CurrentState {
    pub active_db_name: Option<String>,
}

// --- LOGIKA OBSŁUGI ---

/// ⚙️ ConfigManager - Klasa statyczna do zarządzania plikami.
/// W Rust "metody statyczne" nie mają `&self`. Wywołujemy je ConfigManager::metoda().
pub struct ConfigManager;

impl ConfigManager {
    // 📑 === GLOBAL CONFIG (./config/config.toml) ===

    /// 💾 Wczytuje globalną konfigurację.
    /// Zwraca GlobalConfig. Jeśli plik nie istnieje, zwraca domyślny (pusty).
    pub fn load_global_config() -> GlobalConfig {
        load_toml(Path::new("config/config.toml"))
    }

    /// 💾 Zapisuje globalną konfigurację
    pub fn save_global_config(config: &GlobalConfig) {
        save_toml(Path::new("config/config.toml"), config);
    }

    // --- 🗂️ DATA INDEX (data/data.toml) ---

    /// 🧪 Wczytuje indeks baz i CZYŚCI nieistniejące wpisy (Walidacja)
    pub fn load_and_clean_data_index(data_folder: &Path) -> DataIndex {
        let index_path = data_folder.join("data.toml");

        let mut index: DataIndex = load_toml(&index_path);

        // 📚 EDU: Iterujemy i sprawdzamy, czy foldery fizycznie istnieją.
        // Jeśli usunąłeś folder ręcznie, ten kod naprawi plik toml.
        let mut clean_dbs = HashMap::new();
        let mut changes_made = false;

        for (name, relative_path) in index.databases {
            let db_path = data_folder.join(&relative_path);
            if db_path.exists() {
                clean_dbs.insert(name, relative_path);
            } else {
                println!("🧹 [AUTO-CLEAN] Baza '{}' nie istnieje. Usuwam wpis.", name);
                changes_made = true;
            }
        }

        index.databases = clean_dbs;

        if changes_made || !index_path.exists() {
            Self::save_data_index(data_folder, &index);
        }

        index
    }

    /// 💾 Zapisuje indeks baz
    pub fn save_data_index(data_folder: &Path, index: &DataIndex) {
        save_toml(&data_folder.join("data.toml"), index);
    }

    // === 🗄️ ZARZĄDZANIE BAZAMI ===

    /// ⚙️ Tworzy nową bazę danych FIZYCZNIE (pliki .clog, .manifest)
    /// 📚 EDU (Async): `async fn` zwraca `Future` (jak Promise w TS).
    /// Musisz użyć `.await` żeby to wykonać.
    pub async fn create_new_db(
        data_folder: &Path,
        db_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db_path = data_folder.join(db_name);

        // 1. Tworzenie pustego folderu
        if !db_path.exists() {
            fs::create_dir_all(&db_path)?;
            println!("✨ Utworzono folder: {:?}", db_path);
        }

        // 2. FIZYCZNA INICJALIZACJA SURREALKV
        println!("⚙️ Inicjalizacja struktury plików SurrealKV...");
        let db_path_str = db_path.to_str().unwrap();

        // ⚠️ Poprawka: Używamy DatabaseConnection, a nie Database
        let _temp_conn = DatabaseConnection::init(db_path_str, DB_NAMESPACE, DB_NAME).await?;
        // Zmienna _temp_conn tutaj "umiera" (jest dropowana), co zamyka połączenie i zwalnia plik.

        // 3. Aktualizacja indeksu
        let mut index = Self::load_and_clean_data_index(data_folder);
        index
            .databases
            .insert(db_name.to_string(), db_name.to_string());
        Self::save_data_index(data_folder, &index);

        Ok(())
    }

    /// 🚦 Ustawia aktywną bazę
    pub fn set_active_db(data_folder: &Path, db_name: &str) {
        let state = CurrentState {
            active_db_name: Some(db_name.to_string()),
        };
        save_toml(&data_folder.join("data_current.toml"), &state);
    }

    /// 🚦 Pobiera nazwę aktywnej bazy
    pub fn get_active_db(data_folder: &Path) -> Option<String> {
        // 👇 UŻYWAMY UTILS (Rust sam się domyśli, że T = CurrentState)
        let state: CurrentState = load_toml(&data_folder.join("data_current.toml"));
        state.active_db_name
    }
}
