// src/consts.rs

// 📚 EDU: 'pub const' to stała dostępna w całym projekcie.
// Kompilator wstawia te wartości w miejsca użycia (bardzo szybkie).

// Ścieżki folderów
pub const DATA_FOLDER: &str = "data";
pub const CONFIG_FOLDER: &str = "config";

// Nazwy plików
pub const CONFIG_FILE: &str = "config.toml";
pub const DATA_INDEX_FILE: &str = "data.toml";
pub const STATE_FILE: &str = "data_current.toml";

// Baza Danych (Namespace i Database w SurrealDB)
pub const DB_NAMESPACE: &str = "genealogia";
pub const DB_NAME: &str = "core";

// 🗣️ KOMUNIKATY (MESSAGES)
pub mod msg {
    pub const MANAGER_TITLE: &str = "🔧 === MANAGER BAZ DANYCH (Okno 0) === 🔧";
    pub const EDITOR_TITLE: &str = "🚀 === EDYTOR DANYCH (Okno 2) === 🚀";
    pub const EDITOR_MENU_ADD: &str = "📝 [DODAJ TESTOWY REKORD]";
    pub const EDITOR_MENU_LIST: &str = "📊 [WYŚWIETL DANE]";
    pub const EDITOR_MENU_EXIT: &str = "❌ [ZAMKNIJ EDYTOR]";

    pub const ASK_DATA_DIR: &str = "Wskaż folder 'data' z bazami:";
    pub const ASK_CREATE_DIR: &str = "Folder nie istnieje. Utworzyć?";
    pub const ASK_DB_NAME: &str = "Nazwa nowej bazy:";
    pub const ASK_SELECT_ACTION: &str = "Wybierz akcję:";
    pub const ASK_ACTION: &str = "Wybierz działanie:";

    pub const ERR_DIR_CREATE: &str = "❌ Nie udało się utworzyć folderu!";
    pub const ERR_DB_ACTIVE: &str = "❌ Nie wybrano aktywnej bazy!";
    pub const ERR_DIR_NAME: &str = "❌ Błąd: Folder musi nazywać się 'data'!";
    pub const ERR_EDITOR_LAUNCH: &str = "❌ Nie udało się uruchomić okna edytora";
    pub const ERR_DB_CREATE: &str = "❌ Błąd tworzenia bazy:";

    pub const WARN_MANUAL_TERMINAL: &str =
        "⚠️ Na tym systemie musisz ręcznie otworzyć nowe okno terminala.";

    pub const INFO_EDITOR_LAUNCHING: &str = "🚀 Uruchamiam Edytora w nowym oknie...";
    pub const INFO_DB_COUNT: &str = "📊 Liczba rekordów w bazie:";

    pub const OK_DIR_CREATED: &str = "✅ Utworzono folder główny:";
    pub const OK_DB_CREATED: &str = "✅ Utworzono bazę:";
    pub const OK_DB_SET_ACTIVE: &str = "✅ Baza '{}' ustawiona jako AKTYWNA.";
    pub const SUCCESS_REC_ADDED: &str = "✅ Dodano rekord ID:";

    pub const MENU_NEW_DB: &str = "➕ [UTWÓRZ NOWĄ BAZĘ]";
    pub const MENU_EXIT: &str = "❌ [WYJŚCIE]";

    pub const CANCELLED: &str = "Anulowano.";
    
}
