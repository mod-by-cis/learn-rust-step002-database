// src/db/cmd/create.rs

use crate::db::connect::DatabaseConnection;
use surrealdb::Result;
use uuid::Uuid;

pub struct Creator<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> Creator<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // 📚 EDU: &str to "pożyczony string" (lekki), String to "własny string" (cięższy).
    // Do argumentów funkcji zazwyczaj lepiej brać &str.
    pub async fn add_cat(&self, imie: &str, kolor: &str) -> Result<String> {
        let new_id = Uuid::now_v7();

        // format! działa jak template string w TS: `CREATE ... ${imie}`
        let sql = format!(
            "CREATE kot:`{}` SET imie = '{}', kolor = '{}', zrodlo = 'Editor Window';",
            new_id, imie, kolor
        );

        // Wykonujemy zapytanie
        self.db.client.query(&sql).await?;

        Ok(new_id.to_string())
    }
}
