// src/db/connect.rs

use crate::api::model::Kot;
use crate::consts::{DB_NAME, DB_NAMESPACE};
use surrealdb::Result;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, SurrealKv}; // ✅ Importujemy model Kota, żeby funkcja wiedziała co zwracać // 👈 Używamy stałych!

// 📚 EDU (Visibility):
// Domyślnie pola w struct są PRYWATNE (nawet jeśli struct jest publiczny).
// Musimy dodać `pub` przed `client`, żeby `read.rs` mógł zrobić `db.client.query()`.
#[derive(Clone)]
pub struct DatabaseConnection {
    pub client: Surreal<Db>, // ⚠️ ZMIANA: dodano `pub`
}

impl DatabaseConnection {
    /// Inicjalizacja bazy w trybie Embedded (zapis do folderu na dysku).
    pub async fn init(path: &str) -> Result<Self> {
        // Fail Fast: Sprawdźmy czy ścieżka nie jest pusta
        if path.trim().is_empty() {
            panic!("⛔ Fail Fast: Próba inicjalizacji bazy z pustą ścieżką!");
        }

        // Tworzymy silnik bazy danych w podanej ścieżce
        let db = Surreal::new::<SurrealKv>(path).await?;

        // Namespace i Database są wymagane logicznie przez SurrealDB
        db.use_ns(DB_NAMESPACE).use_db(DB_NAME).await?;

        // println!("💽 [DB] Połączono z SurrealKv w folderze: {}", path);
        Ok(DatabaseConnection { client: db })
    }

    /// Wykonuje surowe zapytanie SQL
    pub async fn execute_surrealql(&self, query: &str) -> Result<()> {
        self.client.query(query).await?;
        Ok(())
    }

    /// Pobiera wszystkie koty
    /// 📚 EDU (Typy): -> Result<Vec<Kot>> oznacza:
    /// "Obiecuję zwrócić Listę Kotów (Vec<Kot>) ALBO Błąd (Result)".
    pub async fn get_all_cats(&self) -> Result<Vec<Kot>> {
        // `query` wykonuje SQL
        let mut response = self.client.query("SELECT * FROM kot").await?;

        // `take(0)` bierze wynik pierwszego zapytania SQL (można wysłać kilka po średniku).
        // Rust spróbuje automatycznie zmapować JSON z bazy na struct Kot.
        let cats: Vec<Kot> = response.take(0)?;

        Ok(cats)
    }
}
