// src/api/model.rs

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing; // Specjalny typ SurrealDB dla ID rekordów (np. kot:ulv8...)

// 📚 EDU (Serde):
// Serialize -> pozwala zamienić obiekt Rust na JSON/TOML (do zapisu).
// Deserialize -> pozwala zamienić JSON/TOML na obiekt Rust (do odczytu).
// Debug -> pozwala wyświetlić obiekt w println!("{:?}", kot);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kot {
    // 📚 EDU (Option):
    // Odpowiednik `string | null` w TS.
    // ID jest Option, bo jak tworzymy nowego kota w RAM, to jeszcze nie ma ID.
    pub id: Option<Thing>,
    pub imie: String,
    pub kolor: String,

    // Opcjonalne pola - jeśli w bazie nie ma tego pola, Rust wstawi tu `None` zamiast błędu.
    pub zrodlo: Option<String>,
    pub wiek: Option<i32>,
}
