// src/db/connect.rs

use surrealdb::Result;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, SurrealKv}; 
use serde::de::DeserializeOwned;
use serde::Deserialize;

// 📚 EDU (Visibility):
// Domyślnie pola w struct są PRYWATNE (nawet jeśli struct jest publiczny).
// Musimy dodać `pub` przed `client`, żeby `read.rs` mógł zrobić `db.client.query()`.
// Jedynym zadaniem pliku jest dać nam czynne połączenie do bazy.

#[derive(Clone)]
pub struct DatabaseConnection {
    pub client: Surreal<Db>, // ⚠️ ZMIANA: dodano `pub`
}

impl DatabaseConnection {
    /// 💎 UNIWERSALNY INIT
    /// path: ścieżka do FOLDERU bazy (nie pliku!)
    /// namespace: nazwa przestrzeni logicznej
    /// db_name: nazwa bazy logicznej
    /// Inicjalizacja bazy w trybie Embedded (zapis do folderu na dysku).
    pub async fn init(path: &str, namespace: &str, db_name: &str) -> Result<Self> {
        // Fail Fast: Sprawdźmy czy ścieżka nie jest pusta
        if path.trim().is_empty() {
            panic!("⛔ Fail Fast: Próba inicjalizacji bazy z pustą ścieżką!");
        }

        // Tworzymy silnik bazy danych w podanej ścieżce
        let db = Surreal::new::<SurrealKv>(path).await?;

        // Namespace i Database są wymagane logicznie przez SurrealDB
        db.use_ns(namespace).use_db(db_name).await?;

        // println!("💽 [DB] Połączono z SurrealKv w folderze: {}", path);
        Ok(DatabaseConnection { client: db })
    }

    /// Pomocnicza funkcja do wykonywania surowego SQL (opcjonalna, ale przydatna)
    pub async fn execute_surrealql(&self, query: &str) -> Result<()> {
        self.client.query(query).await?;
        Ok(())
    }

    /// 💎 UNIWERSALNA funkcja pobierająca dane.
    /// Przyjmuje surowe zapytanie SQL i zwraca listę obiektów typu T.
    /// Dzięki temu connect.rs nie musi wiedzieć, czym jest T (Kotem, Imieniem, Liczbą).
    pub async fn query_as<T: DeserializeOwned>(&self, query: &str) -> Result<Vec<T>> {
        // 1. Wykonaj zapytanie
        let mut response = self.client.query(query).await?;
        
        // 2. Weź pierwszy wynik (zakładamy, że query to jeden SELECT)
        // 3. Zmapuj go na typ T (np. Vec<AntroponimXml>)
        let result: Vec<T> = response.take(0)?;
        
        Ok(result)
    }

    /// 🔢 Zlicza rekordy w podanej tabeli
    /// Użycie: db.get_count("concept").await?;
    pub async fn get_count(&self, table_name: &str) -> Result<usize> {
        // Pomocnicza struktura do odebrania wyniku z bazy.
        // Definiujemy ją w środku, bo jest potrzebna tylko tutaj.
        #[derive(Deserialize)]
        struct CountResult { count: usize }

        // GROUP ALL sprawia, że dostajemy jeden wynik z podsumowaniem całej tabeli
        let query = format!("SELECT count() FROM {} GROUP ALL", table_name);

        let mut response = self.client.query(&query).await?;
        
        // Bierzemy pierwszy wynik. Jeśli tabela jest pusta, Surreal może zwrócić pustą listę.
        let result: Option<CountResult> = response.take(0)?;

        // Jeśli result to Some, zwracamy count. Jeśli None (pusta tabela), zwracamy 0.
        Ok(result.map(|r| r.count).unwrap_or(0))
    }
    
    // 🧹 Czyści całą zawartość bazy danych (przydatne przy re-imporcie XML)
    //pub async fn clean_db(&self) -> Result<()> {
        // INFO: W SurrealDB "REMOVE DATABASE" w trybie embedded może wymagać ponownego wybrania NS/DB,
        // dlatego bezpieczniej jest usunąć rekordy lub tabele.
        
        // Opcja A: Jeśli znasz nazwy tabel (bezpieczniejsza)
        // self.client.query("DELETE concept; DELETE lexeme; DELETE case;").await?;
        
        // Opcja B: Bardziej radykalna (dla embedded często wystarcza usunięcie plików .db ręcznie,
        // ale z poziomu kodu można spróbować usunąć wszystko transakcyjnie).
        
        // Najprostsza uniwersalna metoda czyszczenia w trakcie developmentu:
        //self.client.query("DELETE *").await?; // Usuwa wszystkie rekordy, do których mamy dostęp
        
        //Ok(())
    //}
}
