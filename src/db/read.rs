// src/db/read.rs

use crate::api::model::Kot;
use crate::db::connect::DatabaseConnection;
use surrealdb::Result;

// 📚 EDU (Lifetimes 'a):
// Reader trzyma REFERENCJĘ (&) do połączenia, a nie samo połączenie.
// <'a> mówi kompilatorowi: "Reader nie może żyć dłużej niż DatabaseConnection, do którego się odnosi".
// To chroni nas przed używaniem zamkniętego połączenia (Use After Free).
pub struct Reader<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> Reader<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all_cats(&self) -> Result<Vec<Kot>> {
        // Dostęp do .client jest teraz możliwy, bo daliśmy `pub` w connect.rs
        let mut response = self.db.client.query("SELECT * FROM kot").await?;

        // Jawnie mówimy Rustowi: "Oczekuję, że to będzie lista kotów"
        let cats: Vec<Kot> = response.take(0)?;

        Ok(cats)
    }
}
